const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__type__matcher__v3__HttpAttributesCelMatchInput_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpAttributesCelMatchInput {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpAttributesCelMatchInput>
}

impl ::protobuf::Message for HttpAttributesCelMatchInput {
  type MessageView<'msg> = HttpAttributesCelMatchInputView<'msg>;
  type MessageMut<'msg> = HttpAttributesCelMatchInputMut<'msg>;
}

impl ::std::default::Default for HttpAttributesCelMatchInput {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpAttributesCelMatchInput {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpAttributesCelMatchInput` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpAttributesCelMatchInputMut`.
unsafe impl ::std::marker::Sync for HttpAttributesCelMatchInput {}

// SAFETY:
// - `HttpAttributesCelMatchInput` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpAttributesCelMatchInput {}

impl ::protobuf::Proxied for HttpAttributesCelMatchInput {
  type View<'msg> = HttpAttributesCelMatchInputView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpAttributesCelMatchInput {}

impl ::protobuf::MutProxied for HttpAttributesCelMatchInput {
  type Mut<'msg> = HttpAttributesCelMatchInputMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpAttributesCelMatchInputView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpAttributesCelMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpAttributesCelMatchInputView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpAttributesCelMatchInputView<'msg> {
  type Message = HttpAttributesCelMatchInput;
}

impl ::std::fmt::Debug for HttpAttributesCelMatchInputView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpAttributesCelMatchInputView<'_> {
  fn default() -> HttpAttributesCelMatchInputView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpAttributesCelMatchInput>> for HttpAttributesCelMatchInputView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpAttributesCelMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpAttributesCelMatchInputView<'msg> {

  pub fn to_owned(&self) -> HttpAttributesCelMatchInput {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `HttpAttributesCelMatchInputView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpAttributesCelMatchInputView<'_> {}

// SAFETY:
// - `HttpAttributesCelMatchInputView` is `Send` because while its alive a `HttpAttributesCelMatchInputMut` cannot.
// - `HttpAttributesCelMatchInputView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpAttributesCelMatchInputView<'_> {}

impl<'msg> ::protobuf::AsView for HttpAttributesCelMatchInputView<'msg> {
  type Proxied = HttpAttributesCelMatchInput;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpAttributesCelMatchInput> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpAttributesCelMatchInputView<'msg> {
  fn into_view<'shorter>(self) -> HttpAttributesCelMatchInputView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpAttributesCelMatchInput> for HttpAttributesCelMatchInputView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpAttributesCelMatchInput {
    let mut dst = HttpAttributesCelMatchInput::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpAttributesCelMatchInput> for HttpAttributesCelMatchInputMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpAttributesCelMatchInput {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpAttributesCelMatchInput {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpAttributesCelMatchInputView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpAttributesCelMatchInputMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpAttributesCelMatchInputMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpAttributesCelMatchInput>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpAttributesCelMatchInputMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpAttributesCelMatchInputMut<'msg> {
  type Message = HttpAttributesCelMatchInput;
}

impl ::std::fmt::Debug for HttpAttributesCelMatchInputMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpAttributesCelMatchInput>> for HttpAttributesCelMatchInputMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpAttributesCelMatchInput>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpAttributesCelMatchInputMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpAttributesCelMatchInput> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpAttributesCelMatchInput {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `HttpAttributesCelMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpAttributesCelMatchInputMut<'_> {}

// SAFETY:
// - `HttpAttributesCelMatchInputMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpAttributesCelMatchInputMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpAttributesCelMatchInputMut<'msg> {
  type Proxied = HttpAttributesCelMatchInput;
  fn as_view(&self) -> ::protobuf::View<'_, HttpAttributesCelMatchInput> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpAttributesCelMatchInputMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpAttributesCelMatchInput>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpAttributesCelMatchInputMut<'msg> {
  type MutProxied = HttpAttributesCelMatchInput;
  fn as_mut(&mut self) -> HttpAttributesCelMatchInputMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpAttributesCelMatchInputMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpAttributesCelMatchInputMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpAttributesCelMatchInput {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpAttributesCelMatchInput> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpAttributesCelMatchInputView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpAttributesCelMatchInputMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl HttpAttributesCelMatchInput

impl ::std::ops::Drop for HttpAttributesCelMatchInput {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpAttributesCelMatchInput {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpAttributesCelMatchInput {
  type Proxied = Self;
  fn as_view(&self) -> HttpAttributesCelMatchInputView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpAttributesCelMatchInput {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpAttributesCelMatchInputMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpAttributesCelMatchInput {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__type__matcher__v3__HttpAttributesCelMatchInput_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__type__matcher__v3__HttpAttributesCelMatchInput_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__type__matcher__v3__HttpAttributesCelMatchInput_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpAttributesCelMatchInput {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpAttributesCelMatchInput {
  type Msg = HttpAttributesCelMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpAttributesCelMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpAttributesCelMatchInput {
  type Msg = HttpAttributesCelMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpAttributesCelMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpAttributesCelMatchInputMut<'_> {
  type Msg = HttpAttributesCelMatchInput;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpAttributesCelMatchInput> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpAttributesCelMatchInputMut<'_> {
  type Msg = HttpAttributesCelMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpAttributesCelMatchInput> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpAttributesCelMatchInputView<'_> {
  type Msg = HttpAttributesCelMatchInput;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpAttributesCelMatchInput> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpAttributesCelMatchInputMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



