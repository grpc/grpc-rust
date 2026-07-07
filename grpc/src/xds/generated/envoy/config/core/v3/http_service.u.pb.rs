const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HttpService_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpService {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpService>
}

impl ::protobuf::Message for HttpService {
  type MessageView<'msg> = HttpServiceView<'msg>;
  type MessageMut<'msg> = HttpServiceMut<'msg>;
}

impl ::std::default::Default for HttpService {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpService {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpService` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpServiceMut`.
unsafe impl ::std::marker::Sync for HttpService {}

// SAFETY:
// - `HttpService` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpService {}

impl ::protobuf::Proxied for HttpService {
  type View<'msg> = HttpServiceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpService {}

impl ::protobuf::MutProxied for HttpService {
  type Mut<'msg> = HttpServiceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpServiceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpServiceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpServiceView<'msg> {
  type Message = HttpService;
}

impl ::std::fmt::Debug for HttpServiceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpServiceView<'_> {
  fn default() -> HttpServiceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpService>> for HttpServiceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpServiceView<'msg> {

  pub fn to_owned(&self) -> HttpService {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // http_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_http_uri(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn http_uri_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'msg>> {
    self.has_http_uri().then(|| self.http_uri())
  }
  pub fn http_uri(self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }

  // request_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `HttpServiceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpServiceView<'_> {}

// SAFETY:
// - `HttpServiceView` is `Send` because while its alive a `HttpServiceMut` cannot.
// - `HttpServiceView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpServiceView<'_> {}

impl<'msg> ::protobuf::AsView for HttpServiceView<'msg> {
  type Proxied = HttpService;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpService> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpServiceView<'msg> {
  fn into_view<'shorter>(self) -> HttpServiceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpService> for HttpServiceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpService {
    let mut dst = HttpService::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpService> for HttpServiceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpService {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpService {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpServiceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpServiceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpServiceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpService>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpServiceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpServiceMut<'msg> {
  type Message = HttpService;
}

impl ::std::fmt::Debug for HttpServiceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpService>> for HttpServiceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpService>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpServiceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpService> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpService {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // http_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_http_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_uri_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_>> {
    self.has_http_uri().then(|| self.http_uri())
  }
  pub fn http_uri(&self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }
  pub fn http_uri_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriMut<'_> {
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
  pub fn set_http_uri(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // request_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_request_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `HttpServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpServiceMut<'_> {}

// SAFETY:
// - `HttpServiceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpServiceMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpServiceMut<'msg> {
  type Proxied = HttpService;
  fn as_view(&self) -> ::protobuf::View<'_, HttpService> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpServiceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpService>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpServiceMut<'msg> {
  type MutProxied = HttpService;
  fn as_mut(&mut self) -> HttpServiceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpServiceMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpServiceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpService {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpService> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpServiceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpServiceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // http_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_http_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_uri_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_>> {
    self.has_http_uri().then(|| self.http_uri())
  }
  pub fn http_uri(&self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }
  pub fn http_uri_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriMut<'_> {
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
  pub fn set_http_uri(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // request_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn request_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn request_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_request_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl HttpService

impl ::std::ops::Drop for HttpService {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpService {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpService {
  type Proxied = Self;
  fn as_view(&self) -> HttpServiceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpService {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpServiceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpService {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__HttpService_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__HttpService_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__HttpService_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpService {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpService {
  type Msg = HttpService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpService {
  type Msg = HttpService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpServiceMut<'_> {
  type Msg = HttpService;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpServiceMut<'_> {
  type Msg = HttpService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpServiceView<'_> {
  type Msg = HttpService;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpService> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpServiceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



