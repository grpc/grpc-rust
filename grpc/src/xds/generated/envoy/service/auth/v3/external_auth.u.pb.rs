const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__CheckRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CheckRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CheckRequest>
}

impl ::protobuf::Message for CheckRequest {
  type MessageView<'msg> = CheckRequestView<'msg>;
  type MessageMut<'msg> = CheckRequestMut<'msg>;
}

impl ::std::default::Default for CheckRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CheckRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CheckRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `CheckRequestMut`.
unsafe impl ::std::marker::Sync for CheckRequest {}

// SAFETY:
// - `CheckRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CheckRequest {}

impl ::protobuf::Proxied for CheckRequest {
  type View<'msg> = CheckRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CheckRequest {}

impl ::protobuf::MutProxied for CheckRequest {
  type Mut<'msg> = CheckRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CheckRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CheckRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CheckRequestView<'msg> {
  type Message = CheckRequest;
}

impl ::std::fmt::Debug for CheckRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CheckRequestView<'_> {
  fn default() -> CheckRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CheckRequest>> for CheckRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CheckRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CheckRequestView<'msg> {

  pub fn to_owned(&self) -> CheckRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // attributes: optional message envoy.service.auth.v3.AttributeContext
  pub fn has_attributes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn attributes_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextView<'msg>> {
    self.has_attributes().then(|| self.attributes())
  }
  pub fn attributes(self) -> crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextView::default())
  }

}

// SAFETY:
// - `CheckRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CheckRequestView<'_> {}

// SAFETY:
// - `CheckRequestView` is `Send` because while its alive a `CheckRequestMut` cannot.
// - `CheckRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for CheckRequestView<'_> {}

impl<'msg> ::protobuf::AsView for CheckRequestView<'msg> {
  type Proxied = CheckRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, CheckRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckRequestView<'msg> {
  fn into_view<'shorter>(self) -> CheckRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckRequest> for CheckRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckRequest {
    let mut dst = CheckRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckRequest> for CheckRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CheckRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CheckRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CheckRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CheckRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CheckRequestMut<'msg> {
  type Message = CheckRequest;
}

impl ::std::fmt::Debug for CheckRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CheckRequest>> for CheckRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CheckRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CheckRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // attributes: optional message envoy.service.auth.v3.AttributeContext
  pub fn has_attributes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_attributes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn attributes_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextView<'_>> {
    self.has_attributes().then(|| self.attributes())
  }
  pub fn attributes(&self) -> crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextView::default())
  }
  pub fn attributes_mut(&mut self) -> crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextMut<'_> {
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
  pub fn set_attributes(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContext>) {

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
// - `CheckRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CheckRequestMut<'_> {}

// SAFETY:
// - `CheckRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CheckRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for CheckRequestMut<'msg> {
  type Proxied = CheckRequest;
  fn as_view(&self) -> ::protobuf::View<'_, CheckRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CheckRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CheckRequestMut<'msg> {
  type MutProxied = CheckRequest;
  fn as_mut(&mut self) -> CheckRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CheckRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> CheckRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CheckRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CheckRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CheckRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CheckRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // attributes: optional message envoy.service.auth.v3.AttributeContext
  pub fn has_attributes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_attributes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn attributes_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextView<'_>> {
    self.has_attributes().then(|| self.attributes())
  }
  pub fn attributes(&self) -> crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextView::default())
  }
  pub fn attributes_mut(&mut self) -> crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContextMut<'_> {
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
  pub fn set_attributes(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl CheckRequest

impl ::std::ops::Drop for CheckRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CheckRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CheckRequest {
  type Proxied = Self;
  fn as_view(&self) -> CheckRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CheckRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CheckRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CheckRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__auth__v3__CheckRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__auth__v3__CheckRequest_msg_init.0, &[<crate::xds::generated::envoy::service::auth::v3::attribute_context::AttributeContext as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__auth__v3__CheckRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CheckRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CheckRequest {
  type Msg = CheckRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckRequest {
  type Msg = CheckRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CheckRequestMut<'_> {
  type Msg = CheckRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckRequestMut<'_> {
  type Msg = CheckRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckRequestView<'_> {
  type Msg = CheckRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CheckRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__DeniedHttpResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeniedHttpResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeniedHttpResponse>
}

impl ::protobuf::Message for DeniedHttpResponse {
  type MessageView<'msg> = DeniedHttpResponseView<'msg>;
  type MessageMut<'msg> = DeniedHttpResponseMut<'msg>;
}

impl ::std::default::Default for DeniedHttpResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeniedHttpResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeniedHttpResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `DeniedHttpResponseMut`.
unsafe impl ::std::marker::Sync for DeniedHttpResponse {}

// SAFETY:
// - `DeniedHttpResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DeniedHttpResponse {}

impl ::protobuf::Proxied for DeniedHttpResponse {
  type View<'msg> = DeniedHttpResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeniedHttpResponse {}

impl ::protobuf::MutProxied for DeniedHttpResponse {
  type Mut<'msg> = DeniedHttpResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeniedHttpResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeniedHttpResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeniedHttpResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeniedHttpResponseView<'msg> {
  type Message = DeniedHttpResponse;
}

impl ::std::fmt::Debug for DeniedHttpResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeniedHttpResponseView<'_> {
  fn default() -> DeniedHttpResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeniedHttpResponse>> for DeniedHttpResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeniedHttpResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeniedHttpResponseView<'msg> {

  pub fn to_owned(&self) -> DeniedHttpResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // status: optional message envoy.type.v3.HttpStatus
  pub fn has_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn status_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'msg>> {
    self.has_status().then(|| self.status())
  }
  pub fn status(self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }

  // headers: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn headers(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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

  // body: optional string
  pub fn body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `DeniedHttpResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DeniedHttpResponseView<'_> {}

// SAFETY:
// - `DeniedHttpResponseView` is `Send` because while its alive a `DeniedHttpResponseMut` cannot.
// - `DeniedHttpResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for DeniedHttpResponseView<'_> {}

impl<'msg> ::protobuf::AsView for DeniedHttpResponseView<'msg> {
  type Proxied = DeniedHttpResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, DeniedHttpResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeniedHttpResponseView<'msg> {
  fn into_view<'shorter>(self) -> DeniedHttpResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeniedHttpResponse> for DeniedHttpResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeniedHttpResponse {
    let mut dst = DeniedHttpResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeniedHttpResponse> for DeniedHttpResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeniedHttpResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DeniedHttpResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeniedHttpResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeniedHttpResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeniedHttpResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeniedHttpResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeniedHttpResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeniedHttpResponseMut<'msg> {
  type Message = DeniedHttpResponse;
}

impl ::std::fmt::Debug for DeniedHttpResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeniedHttpResponse>> for DeniedHttpResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeniedHttpResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeniedHttpResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeniedHttpResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DeniedHttpResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // status: optional message envoy.type.v3.HttpStatus
  pub fn has_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn status_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_>> {
    self.has_status().then(|| self.status())
  }
  pub fn status(&self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }
  pub fn status_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusMut<'_> {
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
  pub fn set_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // headers: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `DeniedHttpResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DeniedHttpResponseMut<'_> {}

// SAFETY:
// - `DeniedHttpResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DeniedHttpResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for DeniedHttpResponseMut<'msg> {
  type Proxied = DeniedHttpResponse;
  fn as_view(&self) -> ::protobuf::View<'_, DeniedHttpResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeniedHttpResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeniedHttpResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DeniedHttpResponseMut<'msg> {
  type MutProxied = DeniedHttpResponse;
  fn as_mut(&mut self) -> DeniedHttpResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeniedHttpResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> DeniedHttpResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeniedHttpResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeniedHttpResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeniedHttpResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeniedHttpResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // status: optional message envoy.type.v3.HttpStatus
  pub fn has_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn status_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_>> {
    self.has_status().then(|| self.status())
  }
  pub fn status(&self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusView::default())
  }
  pub fn status_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::http_status::HttpStatusMut<'_> {
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
  pub fn set_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // headers: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // body: optional string
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl DeniedHttpResponse

impl ::std::ops::Drop for DeniedHttpResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeniedHttpResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeniedHttpResponse {
  type Proxied = Self;
  fn as_view(&self) -> DeniedHttpResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeniedHttpResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeniedHttpResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeniedHttpResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__auth__v3__DeniedHttpResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__auth__v3__DeniedHttpResponse_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__auth__v3__DeniedHttpResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeniedHttpResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeniedHttpResponse {
  type Msg = DeniedHttpResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeniedHttpResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeniedHttpResponse {
  type Msg = DeniedHttpResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeniedHttpResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeniedHttpResponseMut<'_> {
  type Msg = DeniedHttpResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeniedHttpResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeniedHttpResponseMut<'_> {
  type Msg = DeniedHttpResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeniedHttpResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeniedHttpResponseView<'_> {
  type Msg = DeniedHttpResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeniedHttpResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeniedHttpResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__OkHttpResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OkHttpResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OkHttpResponse>
}

impl ::protobuf::Message for OkHttpResponse {
  type MessageView<'msg> = OkHttpResponseView<'msg>;
  type MessageMut<'msg> = OkHttpResponseMut<'msg>;
}

impl ::std::default::Default for OkHttpResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OkHttpResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OkHttpResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `OkHttpResponseMut`.
unsafe impl ::std::marker::Sync for OkHttpResponse {}

// SAFETY:
// - `OkHttpResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OkHttpResponse {}

impl ::protobuf::Proxied for OkHttpResponse {
  type View<'msg> = OkHttpResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OkHttpResponse {}

impl ::protobuf::MutProxied for OkHttpResponse {
  type Mut<'msg> = OkHttpResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OkHttpResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OkHttpResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OkHttpResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OkHttpResponseView<'msg> {
  type Message = OkHttpResponse;
}

impl ::std::fmt::Debug for OkHttpResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OkHttpResponseView<'_> {
  fn default() -> OkHttpResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OkHttpResponse>> for OkHttpResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OkHttpResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OkHttpResponseView<'msg> {

  pub fn to_owned(&self) -> OkHttpResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // headers: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn headers(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // headers_to_remove: repeated string
  pub fn headers_to_remove(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dynamic_metadata: optional message google.protobuf.Struct
  pub fn has_dynamic_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn dynamic_metadata_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_dynamic_metadata().then(|| self.dynamic_metadata())
  }
  pub fn dynamic_metadata(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // response_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn response_headers_to_add(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // query_parameters_to_set: repeated message envoy.config.core.v3.QueryParameter
  pub fn query_parameters_to_set(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::QueryParameter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::QueryParameter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // query_parameters_to_remove: repeated string
  pub fn query_parameters_to_remove(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `OkHttpResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OkHttpResponseView<'_> {}

// SAFETY:
// - `OkHttpResponseView` is `Send` because while its alive a `OkHttpResponseMut` cannot.
// - `OkHttpResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for OkHttpResponseView<'_> {}

impl<'msg> ::protobuf::AsView for OkHttpResponseView<'msg> {
  type Proxied = OkHttpResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, OkHttpResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OkHttpResponseView<'msg> {
  fn into_view<'shorter>(self) -> OkHttpResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OkHttpResponse> for OkHttpResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OkHttpResponse {
    let mut dst = OkHttpResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OkHttpResponse> for OkHttpResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OkHttpResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OkHttpResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OkHttpResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OkHttpResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OkHttpResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OkHttpResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OkHttpResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OkHttpResponseMut<'msg> {
  type Message = OkHttpResponse;
}

impl ::std::fmt::Debug for OkHttpResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OkHttpResponse>> for OkHttpResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OkHttpResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OkHttpResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OkHttpResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OkHttpResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // headers: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // headers_to_remove: repeated string
  pub fn headers_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_headers_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // dynamic_metadata: optional message google.protobuf.Struct
  pub fn has_dynamic_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dynamic_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dynamic_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_dynamic_metadata().then(|| self.dynamic_metadata())
  }
  pub fn dynamic_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn dynamic_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_dynamic_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // response_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn response_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn response_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_response_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // query_parameters_to_set: repeated message envoy.config.core.v3.QueryParameter
  pub fn query_parameters_to_set(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::QueryParameter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::QueryParameter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn query_parameters_to_set_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::QueryParameter> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_query_parameters_to_set(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::QueryParameter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // query_parameters_to_remove: repeated string
  pub fn query_parameters_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn query_parameters_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_query_parameters_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}

// SAFETY:
// - `OkHttpResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OkHttpResponseMut<'_> {}

// SAFETY:
// - `OkHttpResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OkHttpResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for OkHttpResponseMut<'msg> {
  type Proxied = OkHttpResponse;
  fn as_view(&self) -> ::protobuf::View<'_, OkHttpResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OkHttpResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OkHttpResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OkHttpResponseMut<'msg> {
  type MutProxied = OkHttpResponse;
  fn as_mut(&mut self) -> OkHttpResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OkHttpResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> OkHttpResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OkHttpResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OkHttpResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OkHttpResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OkHttpResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // headers: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // headers_to_remove: repeated string
  pub fn headers_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
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
  pub fn set_headers_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // dynamic_metadata: optional message google.protobuf.Struct
  pub fn has_dynamic_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dynamic_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dynamic_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_dynamic_metadata().then(|| self.dynamic_metadata())
  }
  pub fn dynamic_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn dynamic_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_dynamic_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // response_headers_to_add: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn response_headers_to_add(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn response_headers_to_add_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_response_headers_to_add(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // query_parameters_to_set: repeated message envoy.config.core.v3.QueryParameter
  pub fn query_parameters_to_set(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::QueryParameter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::QueryParameter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn query_parameters_to_set_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::QueryParameter> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
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
  pub fn set_query_parameters_to_set(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::QueryParameter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // query_parameters_to_remove: repeated string
  pub fn query_parameters_to_remove(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn query_parameters_to_remove_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_query_parameters_to_remove(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}  // impl OkHttpResponse

impl ::std::ops::Drop for OkHttpResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OkHttpResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OkHttpResponse {
  type Proxied = Self;
  fn as_view(&self) -> OkHttpResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OkHttpResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OkHttpResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OkHttpResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__auth__v3__OkHttpResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$aG3aETGGET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__auth__v3__OkHttpResponse_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::QueryParameter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__auth__v3__OkHttpResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OkHttpResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OkHttpResponse {
  type Msg = OkHttpResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OkHttpResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OkHttpResponse {
  type Msg = OkHttpResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OkHttpResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OkHttpResponseMut<'_> {
  type Msg = OkHttpResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OkHttpResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OkHttpResponseMut<'_> {
  type Msg = OkHttpResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OkHttpResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OkHttpResponseView<'_> {
  type Msg = OkHttpResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OkHttpResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OkHttpResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__auth__v3__CheckResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CheckResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CheckResponse>
}

impl ::protobuf::Message for CheckResponse {
  type MessageView<'msg> = CheckResponseView<'msg>;
  type MessageMut<'msg> = CheckResponseMut<'msg>;
}

impl ::std::default::Default for CheckResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CheckResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CheckResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `CheckResponseMut`.
unsafe impl ::std::marker::Sync for CheckResponse {}

// SAFETY:
// - `CheckResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CheckResponse {}

impl ::protobuf::Proxied for CheckResponse {
  type View<'msg> = CheckResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CheckResponse {}

impl ::protobuf::MutProxied for CheckResponse {
  type Mut<'msg> = CheckResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CheckResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CheckResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CheckResponseView<'msg> {
  type Message = CheckResponse;
}

impl ::std::fmt::Debug for CheckResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CheckResponseView<'_> {
  fn default() -> CheckResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CheckResponse>> for CheckResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CheckResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CheckResponseView<'msg> {

  pub fn to_owned(&self) -> CheckResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // status: optional message google.rpc.Status
  pub fn has_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn status_opt(self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'msg>> {
    self.has_status().then(|| self.status())
  }
  pub fn status(self) -> crate::xds::generated::google::rpc::status::StatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }

  // denied_response: optional message envoy.service.auth.v3.DeniedHttpResponse
  pub fn has_denied_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn denied_response_opt(self) -> ::std::option::Option<super::DeniedHttpResponseView<'msg>> {
    self.has_denied_response().then(|| self.denied_response())
  }
  pub fn denied_response(self) -> super::DeniedHttpResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DeniedHttpResponseView::default())
  }

  // ok_response: optional message envoy.service.auth.v3.OkHttpResponse
  pub fn has_ok_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn ok_response_opt(self) -> ::std::option::Option<super::OkHttpResponseView<'msg>> {
    self.has_ok_response().then(|| self.ok_response())
  }
  pub fn ok_response(self) -> super::OkHttpResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::OkHttpResponseView::default())
  }

  // error_response: optional message envoy.service.auth.v3.DeniedHttpResponse
  pub fn has_error_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn error_response_opt(self) -> ::std::option::Option<super::DeniedHttpResponseView<'msg>> {
    self.has_error_response().then(|| self.error_response())
  }
  pub fn error_response(self) -> super::DeniedHttpResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DeniedHttpResponseView::default())
  }

  // dynamic_metadata: optional message google.protobuf.Struct
  pub fn has_dynamic_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn dynamic_metadata_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_dynamic_metadata().then(|| self.dynamic_metadata())
  }
  pub fn dynamic_metadata(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  pub fn http_response(self) -> super::check_response::HttpResponseOneof<'msg> {
    match self.http_response_case() {
      super::check_response::HttpResponseCase::DeniedResponse =>
          super::check_response::HttpResponseOneof::DeniedResponse(self.denied_response()),
      super::check_response::HttpResponseCase::OkResponse =>
          super::check_response::HttpResponseOneof::OkResponse(self.ok_response()),
      super::check_response::HttpResponseCase::ErrorResponse =>
          super::check_response::HttpResponseOneof::ErrorResponse(self.error_response()),
      _ => super::check_response::HttpResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn http_response_case(self) -> super::check_response::HttpResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::check_response::HttpResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CheckResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CheckResponseView<'_> {}

// SAFETY:
// - `CheckResponseView` is `Send` because while its alive a `CheckResponseMut` cannot.
// - `CheckResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for CheckResponseView<'_> {}

impl<'msg> ::protobuf::AsView for CheckResponseView<'msg> {
  type Proxied = CheckResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, CheckResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckResponseView<'msg> {
  fn into_view<'shorter>(self) -> CheckResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckResponse> for CheckResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckResponse {
    let mut dst = CheckResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CheckResponse> for CheckResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CheckResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CheckResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CheckResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CheckResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CheckResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CheckResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CheckResponseMut<'msg> {
  type Message = CheckResponse;
}

impl ::std::fmt::Debug for CheckResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CheckResponse>> for CheckResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CheckResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CheckResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CheckResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // status: optional message google.rpc.Status
  pub fn has_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn status_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_status().then(|| self.status())
  }
  pub fn status(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn status_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // denied_response: optional message envoy.service.auth.v3.DeniedHttpResponse
  pub fn has_denied_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_denied_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn denied_response_opt(&self) -> ::std::option::Option<super::DeniedHttpResponseView<'_>> {
    self.has_denied_response().then(|| self.denied_response())
  }
  pub fn denied_response(&self) -> super::DeniedHttpResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DeniedHttpResponseView::default())
  }
  pub fn denied_response_mut(&mut self) -> super::DeniedHttpResponseMut<'_> {
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
  pub fn set_denied_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::DeniedHttpResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // ok_response: optional message envoy.service.auth.v3.OkHttpResponse
  pub fn has_ok_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_ok_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn ok_response_opt(&self) -> ::std::option::Option<super::OkHttpResponseView<'_>> {
    self.has_ok_response().then(|| self.ok_response())
  }
  pub fn ok_response(&self) -> super::OkHttpResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::OkHttpResponseView::default())
  }
  pub fn ok_response_mut(&mut self) -> super::OkHttpResponseMut<'_> {
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
  pub fn set_ok_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::OkHttpResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_response: optional message envoy.service.auth.v3.DeniedHttpResponse
  pub fn has_error_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_error_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn error_response_opt(&self) -> ::std::option::Option<super::DeniedHttpResponseView<'_>> {
    self.has_error_response().then(|| self.error_response())
  }
  pub fn error_response(&self) -> super::DeniedHttpResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DeniedHttpResponseView::default())
  }
  pub fn error_response_mut(&mut self) -> super::DeniedHttpResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_error_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::DeniedHttpResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // dynamic_metadata: optional message google.protobuf.Struct
  pub fn has_dynamic_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_dynamic_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn dynamic_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_dynamic_metadata().then(|| self.dynamic_metadata())
  }
  pub fn dynamic_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn dynamic_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dynamic_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn http_response(&self) -> super::check_response::HttpResponseOneof<'_> {
    match &self.http_response_case() {
      super::check_response::HttpResponseCase::DeniedResponse =>
          super::check_response::HttpResponseOneof::DeniedResponse(self.denied_response()),
      super::check_response::HttpResponseCase::OkResponse =>
          super::check_response::HttpResponseOneof::OkResponse(self.ok_response()),
      super::check_response::HttpResponseCase::ErrorResponse =>
          super::check_response::HttpResponseOneof::ErrorResponse(self.error_response()),
      _ => super::check_response::HttpResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn http_response_case(&self) -> super::check_response::HttpResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::check_response::HttpResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CheckResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CheckResponseMut<'_> {}

// SAFETY:
// - `CheckResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CheckResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for CheckResponseMut<'msg> {
  type Proxied = CheckResponse;
  fn as_view(&self) -> ::protobuf::View<'_, CheckResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CheckResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CheckResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CheckResponseMut<'msg> {
  type MutProxied = CheckResponse;
  fn as_mut(&mut self) -> CheckResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CheckResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> CheckResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CheckResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CheckResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CheckResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CheckResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // status: optional message google.rpc.Status
  pub fn has_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn status_opt(&self) -> ::std::option::Option<crate::xds::generated::google::rpc::status::StatusView<'_>> {
    self.has_status().then(|| self.status())
  }
  pub fn status(&self) -> crate::xds::generated::google::rpc::status::StatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::google::rpc::status::StatusView::default())
  }
  pub fn status_mut(&mut self) -> crate::xds::generated::google::rpc::status::StatusMut<'_> {
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
  pub fn set_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::google::rpc::status::Status>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // denied_response: optional message envoy.service.auth.v3.DeniedHttpResponse
  pub fn has_denied_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_denied_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn denied_response_opt(&self) -> ::std::option::Option<super::DeniedHttpResponseView<'_>> {
    self.has_denied_response().then(|| self.denied_response())
  }
  pub fn denied_response(&self) -> super::DeniedHttpResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DeniedHttpResponseView::default())
  }
  pub fn denied_response_mut(&mut self) -> super::DeniedHttpResponseMut<'_> {
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
  pub fn set_denied_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::DeniedHttpResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // ok_response: optional message envoy.service.auth.v3.OkHttpResponse
  pub fn has_ok_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_ok_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn ok_response_opt(&self) -> ::std::option::Option<super::OkHttpResponseView<'_>> {
    self.has_ok_response().then(|| self.ok_response())
  }
  pub fn ok_response(&self) -> super::OkHttpResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::OkHttpResponseView::default())
  }
  pub fn ok_response_mut(&mut self) -> super::OkHttpResponseMut<'_> {
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
  pub fn set_ok_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::OkHttpResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // error_response: optional message envoy.service.auth.v3.DeniedHttpResponse
  pub fn has_error_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_error_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn error_response_opt(&self) -> ::std::option::Option<super::DeniedHttpResponseView<'_>> {
    self.has_error_response().then(|| self.error_response())
  }
  pub fn error_response(&self) -> super::DeniedHttpResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DeniedHttpResponseView::default())
  }
  pub fn error_response_mut(&mut self) -> super::DeniedHttpResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_error_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::DeniedHttpResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // dynamic_metadata: optional message google.protobuf.Struct
  pub fn has_dynamic_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_dynamic_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn dynamic_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_dynamic_metadata().then(|| self.dynamic_metadata())
  }
  pub fn dynamic_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn dynamic_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dynamic_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn http_response(&self) -> super::check_response::HttpResponseOneof<'_> {
    match &self.http_response_case() {
      super::check_response::HttpResponseCase::DeniedResponse =>
          super::check_response::HttpResponseOneof::DeniedResponse(self.denied_response()),
      super::check_response::HttpResponseCase::OkResponse =>
          super::check_response::HttpResponseOneof::OkResponse(self.ok_response()),
      super::check_response::HttpResponseCase::ErrorResponse =>
          super::check_response::HttpResponseOneof::ErrorResponse(self.error_response()),
      _ => super::check_response::HttpResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn http_response_case(&self) -> super::check_response::HttpResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::check_response::HttpResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CheckResponse

impl ::std::ops::Drop for CheckResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CheckResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CheckResponse {
  type Proxied = Self;
  fn as_view(&self) -> CheckResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CheckResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CheckResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CheckResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__auth__v3__CheckResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33333^#|$|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__auth__v3__CheckResponse_msg_init.0, &[<crate::xds::generated::google::rpc::status::Status as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DeniedHttpResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::OkHttpResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DeniedHttpResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__auth__v3__CheckResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CheckResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CheckResponse {
  type Msg = CheckResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckResponse {
  type Msg = CheckResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CheckResponseMut<'_> {
  type Msg = CheckResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckResponseMut<'_> {
  type Msg = CheckResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CheckResponseView<'_> {
  type Msg = CheckResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CheckResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CheckResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod check_response {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum HttpResponseOneof<'msg> {
  DeniedResponse(::protobuf::View<'msg, super::super::DeniedHttpResponse>) = 2,
  OkResponse(::protobuf::View<'msg, super::super::OkHttpResponse>) = 3,
  ErrorResponse(::protobuf::View<'msg, super::super::DeniedHttpResponse>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum HttpResponseCase {
  DeniedResponse = 2,
  OkResponse = 3,
  ErrorResponse = 5,

  not_set = 0
}

impl HttpResponseCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<HttpResponseCase> {
    match v {
      0 => Some(HttpResponseCase::not_set),
      2 => Some(HttpResponseCase::DeniedResponse),
      3 => Some(HttpResponseCase::OkResponse),
      5 => Some(HttpResponseCase::ErrorResponse),
      _ => None
    }
  }
}
}  // pub mod check_response


