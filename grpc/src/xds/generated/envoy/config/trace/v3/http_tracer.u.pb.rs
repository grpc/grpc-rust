const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__trace__v3__Tracing_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Tracing {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Tracing>
}

impl ::protobuf::Message for Tracing {
  type MessageView<'msg> = TracingView<'msg>;
  type MessageMut<'msg> = TracingMut<'msg>;
}

impl ::std::default::Default for Tracing {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Tracing {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Tracing` is `Sync` because it does not implement interior mutability.
//    Neither does `TracingMut`.
unsafe impl ::std::marker::Sync for Tracing {}

// SAFETY:
// - `Tracing` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Tracing {}

impl ::protobuf::Proxied for Tracing {
  type View<'msg> = TracingView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Tracing {}

impl ::protobuf::MutProxied for Tracing {
  type Mut<'msg> = TracingMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TracingView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Tracing>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TracingView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TracingView<'msg> {
  type Message = Tracing;
}

impl ::std::fmt::Debug for TracingView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TracingView<'_> {
  fn default() -> TracingView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Tracing>> for TracingView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Tracing>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TracingView<'msg> {

  pub fn to_owned(&self) -> Tracing {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // http: optional message envoy.config.trace.v3.Tracing.Http
  pub fn has_http(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn http_opt(self) -> ::std::option::Option<super::tracing::HttpView<'msg>> {
    self.has_http().then(|| self.http())
  }
  pub fn http(self) -> super::tracing::HttpView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::tracing::HttpView::default())
  }

}

// SAFETY:
// - `TracingView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TracingView<'_> {}

// SAFETY:
// - `TracingView` is `Send` because while its alive a `TracingMut` cannot.
// - `TracingView` does not use thread-local data.
unsafe impl ::std::marker::Send for TracingView<'_> {}

impl<'msg> ::protobuf::AsView for TracingView<'msg> {
  type Proxied = Tracing;
  fn as_view(&self) -> ::protobuf::View<'msg, Tracing> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TracingView<'msg> {
  fn into_view<'shorter>(self) -> TracingView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Tracing> for TracingView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Tracing {
    let mut dst = Tracing::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Tracing> for TracingMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Tracing {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Tracing {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TracingView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TracingMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TracingMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Tracing>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TracingMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TracingMut<'msg> {
  type Message = Tracing;
}

impl ::std::fmt::Debug for TracingMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Tracing>> for TracingMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Tracing>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TracingMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Tracing> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Tracing {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // http: optional message envoy.config.trace.v3.Tracing.Http
  pub fn has_http(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_opt(&self) -> ::std::option::Option<super::tracing::HttpView<'_>> {
    self.has_http().then(|| self.http())
  }
  pub fn http(&self) -> super::tracing::HttpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::tracing::HttpView::default())
  }
  pub fn http_mut(&mut self) -> super::tracing::HttpMut<'_> {
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
  pub fn set_http(&mut self,
    val: impl ::protobuf::IntoProxied<super::tracing::Http>) {

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
// - `TracingMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TracingMut<'_> {}

// SAFETY:
// - `TracingMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TracingMut<'_> {}

impl<'msg> ::protobuf::AsView for TracingMut<'msg> {
  type Proxied = Tracing;
  fn as_view(&self) -> ::protobuf::View<'_, Tracing> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TracingMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Tracing>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TracingMut<'msg> {
  type MutProxied = Tracing;
  fn as_mut(&mut self) -> TracingMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TracingMut<'msg> {
  fn into_mut<'shorter>(self) -> TracingMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Tracing {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Tracing> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TracingView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TracingMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // http: optional message envoy.config.trace.v3.Tracing.Http
  pub fn has_http(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_opt(&self) -> ::std::option::Option<super::tracing::HttpView<'_>> {
    self.has_http().then(|| self.http())
  }
  pub fn http(&self) -> super::tracing::HttpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::tracing::HttpView::default())
  }
  pub fn http_mut(&mut self) -> super::tracing::HttpMut<'_> {
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
  pub fn set_http(&mut self,
    val: impl ::protobuf::IntoProxied<super::tracing::Http>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl Tracing

impl ::std::ops::Drop for Tracing {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Tracing {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Tracing {
  type Proxied = Self;
  fn as_view(&self) -> TracingView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Tracing {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TracingMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Tracing {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__trace__v3__Tracing_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__trace__v3__Tracing_msg_init.0, &[<super::tracing::Http as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__trace__v3__Tracing_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Tracing {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Tracing {
  type Msg = Tracing;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Tracing {
  type Msg = Tracing;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TracingMut<'_> {
  type Msg = Tracing;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TracingMut<'_> {
  type Msg = Tracing;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TracingView<'_> {
  type Msg = Tracing;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Tracing> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TracingMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod tracing {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__trace__v3__Tracing__Http_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn typed_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  pub fn config_type(self) -> super::super::tracing::http::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::super::tracing::http::ConfigTypeCase::TypedConfig =>
          super::super::tracing::http::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::tracing::http::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::super::tracing::http::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::tracing::http::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
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

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::super::tracing::http::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::tracing::http::ConfigTypeCase::TypedConfig =>
          super::super::tracing::http::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::tracing::http::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::tracing::http::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::tracing::http::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
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

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::super::tracing::http::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::tracing::http::ConfigTypeCase::TypedConfig =>
          super::super::tracing::http::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::tracing::http::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::tracing::http::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::tracing::http::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
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
        super::super::tracing::envoy__config__trace__v3__Tracing__Http_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa3^$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::tracing::envoy__config__trace__v3__Tracing__Http_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::tracing::envoy__config__trace__v3__Tracing__Http_msg_init.0)
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

pub mod http {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigTypeOneof<'msg> {
  TypedConfig(::protobuf::View<'msg, ::protobuf_well_known_types::Any>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigTypeCase {
  TypedConfig = 3,

  not_set = 0
}

impl ConfigTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigTypeCase> {
    match v {
      0 => Some(ConfigTypeCase::not_set),
      3 => Some(ConfigTypeCase::TypedConfig),
      _ => None
    }
  }
}
}  // pub mod http


}  // pub mod tracing


