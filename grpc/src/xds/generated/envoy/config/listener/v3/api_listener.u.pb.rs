const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__listener__v3__ApiListener_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ApiListener {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ApiListener>
}

impl ::protobuf::Message for ApiListener {
  type MessageView<'msg> = ApiListenerView<'msg>;
  type MessageMut<'msg> = ApiListenerMut<'msg>;
}

impl ::std::default::Default for ApiListener {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ApiListener {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ApiListener` is `Sync` because it does not implement interior mutability.
//    Neither does `ApiListenerMut`.
unsafe impl ::std::marker::Sync for ApiListener {}

// SAFETY:
// - `ApiListener` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ApiListener {}

impl ::protobuf::Proxied for ApiListener {
  type View<'msg> = ApiListenerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ApiListener {}

impl ::protobuf::MutProxied for ApiListener {
  type Mut<'msg> = ApiListenerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ApiListenerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApiListener>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApiListenerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ApiListenerView<'msg> {
  type Message = ApiListener;
}

impl ::std::fmt::Debug for ApiListenerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ApiListenerView<'_> {
  fn default() -> ApiListenerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ApiListener>> for ApiListenerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApiListener>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApiListenerView<'msg> {

  pub fn to_owned(&self) -> ApiListener {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // api_listener: optional message google.protobuf.Any
  pub fn has_api_listener(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn api_listener_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_api_listener().then(|| self.api_listener())
  }
  pub fn api_listener(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

}

// SAFETY:
// - `ApiListenerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ApiListenerView<'_> {}

// SAFETY:
// - `ApiListenerView` is `Send` because while its alive a `ApiListenerMut` cannot.
// - `ApiListenerView` does not use thread-local data.
unsafe impl ::std::marker::Send for ApiListenerView<'_> {}

impl<'msg> ::protobuf::AsView for ApiListenerView<'msg> {
  type Proxied = ApiListener;
  fn as_view(&self) -> ::protobuf::View<'msg, ApiListener> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiListenerView<'msg> {
  fn into_view<'shorter>(self) -> ApiListenerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ApiListener> for ApiListenerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApiListener {
    let mut dst = ApiListener::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ApiListener> for ApiListenerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApiListener {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ApiListener {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ApiListenerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ApiListenerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ApiListenerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiListener>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApiListenerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ApiListenerMut<'msg> {
  type Message = ApiListener;
}

impl ::std::fmt::Debug for ApiListenerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ApiListener>> for ApiListenerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiListener>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApiListenerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ApiListener> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ApiListener {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // api_listener: optional message google.protobuf.Any
  pub fn has_api_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_api_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn api_listener_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_api_listener().then(|| self.api_listener())
  }
  pub fn api_listener(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn api_listener_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_api_listener(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

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
// - `ApiListenerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ApiListenerMut<'_> {}

// SAFETY:
// - `ApiListenerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ApiListenerMut<'_> {}

impl<'msg> ::protobuf::AsView for ApiListenerMut<'msg> {
  type Proxied = ApiListener;
  fn as_view(&self) -> ::protobuf::View<'_, ApiListener> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApiListenerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ApiListener>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ApiListenerMut<'msg> {
  type MutProxied = ApiListener;
  fn as_mut(&mut self) -> ApiListenerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ApiListenerMut<'msg> {
  fn into_mut<'shorter>(self) -> ApiListenerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ApiListener {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ApiListener> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ApiListenerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ApiListenerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // api_listener: optional message google.protobuf.Any
  pub fn has_api_listener(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_api_listener(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn api_listener_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_api_listener().then(|| self.api_listener())
  }
  pub fn api_listener(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn api_listener_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_api_listener(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl ApiListener

impl ::std::ops::Drop for ApiListener {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ApiListener {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ApiListener {
  type Proxied = Self;
  fn as_view(&self) -> ApiListenerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ApiListener {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ApiListenerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ApiListener {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__listener__v3__ApiListener_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__listener__v3__ApiListener_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__listener__v3__ApiListener_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApiListener {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApiListener {
  type Msg = ApiListener;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListener> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiListener {
  type Msg = ApiListener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListener> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApiListenerMut<'_> {
  type Msg = ApiListener;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListener> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiListenerMut<'_> {
  type Msg = ApiListener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListener> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApiListenerView<'_> {
  type Msg = ApiListener;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApiListener> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApiListenerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



