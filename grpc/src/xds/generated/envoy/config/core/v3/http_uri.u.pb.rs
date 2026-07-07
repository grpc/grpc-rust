const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HttpUri_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpUri {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpUri>
}

impl ::protobuf::Message for HttpUri {
  type MessageView<'msg> = HttpUriView<'msg>;
  type MessageMut<'msg> = HttpUriMut<'msg>;
}

impl ::std::default::Default for HttpUri {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpUri {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpUri` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpUriMut`.
unsafe impl ::std::marker::Sync for HttpUri {}

// SAFETY:
// - `HttpUri` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpUri {}

impl ::protobuf::Proxied for HttpUri {
  type View<'msg> = HttpUriView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpUri {}

impl ::protobuf::MutProxied for HttpUri {
  type Mut<'msg> = HttpUriMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpUriView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpUri>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpUriView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpUriView<'msg> {
  type Message = HttpUri;
}

impl ::std::fmt::Debug for HttpUriView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpUriView<'_> {
  fn default() -> HttpUriView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpUri>> for HttpUriView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpUri>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpUriView<'msg> {

  pub fn to_owned(&self) -> HttpUri {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // uri: optional string
  pub fn uri(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // cluster: optional string
  pub fn has_cluster(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn cluster_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  pub fn http_upstream_type(self) -> super::http_uri::HttpUpstreamTypeOneof<'msg> {
    match self.http_upstream_type_case() {
      super::http_uri::HttpUpstreamTypeCase::Cluster =>
          super::http_uri::HttpUpstreamTypeOneof::Cluster(self.cluster()),
      _ => super::http_uri::HttpUpstreamTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn http_upstream_type_case(self) -> super::http_uri::HttpUpstreamTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::http_uri::HttpUpstreamTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HttpUriView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpUriView<'_> {}

// SAFETY:
// - `HttpUriView` is `Send` because while its alive a `HttpUriMut` cannot.
// - `HttpUriView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpUriView<'_> {}

impl<'msg> ::protobuf::AsView for HttpUriView<'msg> {
  type Proxied = HttpUri;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpUri> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpUriView<'msg> {
  fn into_view<'shorter>(self) -> HttpUriView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpUri> for HttpUriView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpUri {
    let mut dst = HttpUri::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpUri> for HttpUriMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpUri {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpUri {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpUriView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpUriMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpUriMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpUri>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpUriMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpUriMut<'msg> {
  type Message = HttpUri;
}

impl ::std::fmt::Debug for HttpUriMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpUri>> for HttpUriMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpUri>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpUriMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpUri> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpUri {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // uri: optional string
  pub fn uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // cluster: optional string
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn cluster_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn http_upstream_type(&self) -> super::http_uri::HttpUpstreamTypeOneof<'_> {
    match &self.http_upstream_type_case() {
      super::http_uri::HttpUpstreamTypeCase::Cluster =>
          super::http_uri::HttpUpstreamTypeOneof::Cluster(self.cluster()),
      _ => super::http_uri::HttpUpstreamTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn http_upstream_type_case(&self) -> super::http_uri::HttpUpstreamTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::http_uri::HttpUpstreamTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HttpUriMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpUriMut<'_> {}

// SAFETY:
// - `HttpUriMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpUriMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpUriMut<'msg> {
  type Proxied = HttpUri;
  fn as_view(&self) -> ::protobuf::View<'_, HttpUri> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpUriMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpUri>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpUriMut<'msg> {
  type MutProxied = HttpUri;
  fn as_mut(&mut self) -> HttpUriMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpUriMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpUriMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpUri {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpUri> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpUriView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpUriMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // uri: optional string
  pub fn uri(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_uri(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // cluster: optional string
  pub fn has_cluster(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_cluster(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn cluster_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_cluster().then(|| self.cluster())
  }
  pub fn cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn http_upstream_type(&self) -> super::http_uri::HttpUpstreamTypeOneof<'_> {
    match &self.http_upstream_type_case() {
      super::http_uri::HttpUpstreamTypeCase::Cluster =>
          super::http_uri::HttpUpstreamTypeOneof::Cluster(self.cluster()),
      _ => super::http_uri::HttpUpstreamTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn http_upstream_type_case(&self) -> super::http_uri::HttpUpstreamTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::http_uri::HttpUpstreamTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl HttpUri

impl ::std::ops::Drop for HttpUri {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpUri {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpUri {
  type Proxied = Self;
  fn as_view(&self) -> HttpUriView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpUri {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpUriMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpUri {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__HttpUri_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1T3^#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__HttpUri_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__HttpUri_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpUri {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpUri {
  type Msg = HttpUri;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpUri> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpUri {
  type Msg = HttpUri;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpUri> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpUriMut<'_> {
  type Msg = HttpUri;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpUri> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpUriMut<'_> {
  type Msg = HttpUri;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpUri> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpUriView<'_> {
  type Msg = HttpUri;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpUri> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpUriMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http_uri {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum HttpUpstreamTypeOneof<'msg> {
  Cluster(&'msg ::protobuf::ProtoStr) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum HttpUpstreamTypeCase {
  Cluster = 2,

  not_set = 0
}

impl HttpUpstreamTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<HttpUpstreamTypeCase> {
    match v {
      0 => Some(HttpUpstreamTypeCase::not_set),
      2 => Some(HttpUpstreamTypeCase::Cluster),
      _ => None
    }
  }
}
}  // pub mod http_uri


