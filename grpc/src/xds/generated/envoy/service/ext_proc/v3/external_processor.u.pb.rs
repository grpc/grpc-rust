const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__ProtocolConfiguration_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProtocolConfiguration {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProtocolConfiguration>
}

impl ::protobuf::Message for ProtocolConfiguration {
  type MessageView<'msg> = ProtocolConfigurationView<'msg>;
  type MessageMut<'msg> = ProtocolConfigurationMut<'msg>;
}

impl ::std::default::Default for ProtocolConfiguration {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProtocolConfiguration {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProtocolConfiguration` is `Sync` because it does not implement interior mutability.
//    Neither does `ProtocolConfigurationMut`.
unsafe impl ::std::marker::Sync for ProtocolConfiguration {}

// SAFETY:
// - `ProtocolConfiguration` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ProtocolConfiguration {}

impl ::protobuf::Proxied for ProtocolConfiguration {
  type View<'msg> = ProtocolConfigurationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProtocolConfiguration {}

impl ::protobuf::MutProxied for ProtocolConfiguration {
  type Mut<'msg> = ProtocolConfigurationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProtocolConfigurationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProtocolConfiguration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProtocolConfigurationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProtocolConfigurationView<'msg> {
  type Message = ProtocolConfiguration;
}

impl ::std::fmt::Debug for ProtocolConfigurationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProtocolConfigurationView<'_> {
  fn default() -> ProtocolConfigurationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProtocolConfiguration>> for ProtocolConfigurationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProtocolConfiguration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProtocolConfigurationView<'msg> {

  pub fn to_owned(&self) -> ProtocolConfiguration {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // request_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn request_body_mode(self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }

  // response_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn response_body_mode(self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }

  // send_body_without_waiting_for_header_response: optional bool
  pub fn send_body_without_waiting_for_header_response(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ProtocolConfigurationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ProtocolConfigurationView<'_> {}

// SAFETY:
// - `ProtocolConfigurationView` is `Send` because while its alive a `ProtocolConfigurationMut` cannot.
// - `ProtocolConfigurationView` does not use thread-local data.
unsafe impl ::std::marker::Send for ProtocolConfigurationView<'_> {}

impl<'msg> ::protobuf::AsView for ProtocolConfigurationView<'msg> {
  type Proxied = ProtocolConfiguration;
  fn as_view(&self) -> ::protobuf::View<'msg, ProtocolConfiguration> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProtocolConfigurationView<'msg> {
  fn into_view<'shorter>(self) -> ProtocolConfigurationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProtocolConfiguration> for ProtocolConfigurationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProtocolConfiguration {
    let mut dst = ProtocolConfiguration::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProtocolConfiguration> for ProtocolConfigurationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProtocolConfiguration {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ProtocolConfiguration {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProtocolConfigurationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProtocolConfigurationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProtocolConfigurationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProtocolConfiguration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProtocolConfigurationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProtocolConfigurationMut<'msg> {
  type Message = ProtocolConfiguration;
}

impl ::std::fmt::Debug for ProtocolConfigurationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProtocolConfiguration>> for ProtocolConfigurationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProtocolConfiguration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProtocolConfigurationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProtocolConfiguration> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ProtocolConfiguration {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // request_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn request_body_mode(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_body_mode(&mut self, val: crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // response_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn response_body_mode(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_body_mode(&mut self, val: crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // send_body_without_waiting_for_header_response: optional bool
  pub fn send_body_without_waiting_for_header_response(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_send_body_without_waiting_for_header_response(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `ProtocolConfigurationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ProtocolConfigurationMut<'_> {}

// SAFETY:
// - `ProtocolConfigurationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ProtocolConfigurationMut<'_> {}

impl<'msg> ::protobuf::AsView for ProtocolConfigurationMut<'msg> {
  type Proxied = ProtocolConfiguration;
  fn as_view(&self) -> ::protobuf::View<'_, ProtocolConfiguration> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProtocolConfigurationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProtocolConfiguration>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ProtocolConfigurationMut<'msg> {
  type MutProxied = ProtocolConfiguration;
  fn as_mut(&mut self) -> ProtocolConfigurationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProtocolConfigurationMut<'msg> {
  fn into_mut<'shorter>(self) -> ProtocolConfigurationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProtocolConfiguration {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProtocolConfiguration> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProtocolConfigurationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProtocolConfigurationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // request_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn request_body_mode(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_body_mode(&mut self, val: crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // response_body_mode: optional enum envoy.extensions.filters.http.ext_proc.v3.ProcessingMode.BodySendMode
  pub fn response_body_mode(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode::None).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_body_mode(&mut self, val: crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::processing_mode::BodySendMode) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // send_body_without_waiting_for_header_response: optional bool
  pub fn send_body_without_waiting_for_header_response(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_send_body_without_waiting_for_header_response(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

}  // impl ProtocolConfiguration

impl ::std::ops::Drop for ProtocolConfiguration {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProtocolConfiguration {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProtocolConfiguration {
  type Proxied = Self;
  fn as_view(&self) -> ProtocolConfigurationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProtocolConfiguration {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProtocolConfigurationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProtocolConfiguration {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__ProtocolConfiguration_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P.P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__ProtocolConfiguration_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__ProtocolConfiguration_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProtocolConfiguration {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProtocolConfiguration {
  type Msg = ProtocolConfiguration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProtocolConfiguration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProtocolConfiguration {
  type Msg = ProtocolConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProtocolConfiguration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProtocolConfigurationMut<'_> {
  type Msg = ProtocolConfiguration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProtocolConfiguration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProtocolConfigurationMut<'_> {
  type Msg = ProtocolConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProtocolConfiguration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProtocolConfigurationView<'_> {
  type Msg = ProtocolConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProtocolConfiguration> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProtocolConfigurationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__ProcessingRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProcessingRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProcessingRequest>
}

impl ::protobuf::Message for ProcessingRequest {
  type MessageView<'msg> = ProcessingRequestView<'msg>;
  type MessageMut<'msg> = ProcessingRequestMut<'msg>;
}

impl ::std::default::Default for ProcessingRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProcessingRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProcessingRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ProcessingRequestMut`.
unsafe impl ::std::marker::Sync for ProcessingRequest {}

// SAFETY:
// - `ProcessingRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ProcessingRequest {}

impl ::protobuf::Proxied for ProcessingRequest {
  type View<'msg> = ProcessingRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProcessingRequest {}

impl ::protobuf::MutProxied for ProcessingRequest {
  type Mut<'msg> = ProcessingRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProcessingRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProcessingRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProcessingRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProcessingRequestView<'msg> {
  type Message = ProcessingRequest;
}

impl ::std::fmt::Debug for ProcessingRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProcessingRequestView<'_> {
  fn default() -> ProcessingRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProcessingRequest>> for ProcessingRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProcessingRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProcessingRequestView<'msg> {

  pub fn to_owned(&self) -> ProcessingRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // request_headers: optional message envoy.service.ext_proc.v3.HttpHeaders
  pub fn has_request_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn request_headers_opt(self) -> ::std::option::Option<super::HttpHeadersView<'msg>> {
    self.has_request_headers().then(|| self.request_headers())
  }
  pub fn request_headers(self) -> super::HttpHeadersView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersView::default())
  }

  // response_headers: optional message envoy.service.ext_proc.v3.HttpHeaders
  pub fn has_response_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn response_headers_opt(self) -> ::std::option::Option<super::HttpHeadersView<'msg>> {
    self.has_response_headers().then(|| self.response_headers())
  }
  pub fn response_headers(self) -> super::HttpHeadersView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersView::default())
  }

  // request_body: optional message envoy.service.ext_proc.v3.HttpBody
  pub fn has_request_body(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn request_body_opt(self) -> ::std::option::Option<super::HttpBodyView<'msg>> {
    self.has_request_body().then(|| self.request_body())
  }
  pub fn request_body(self) -> super::HttpBodyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpBodyView::default())
  }

  // response_body: optional message envoy.service.ext_proc.v3.HttpBody
  pub fn has_response_body(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn response_body_opt(self) -> ::std::option::Option<super::HttpBodyView<'msg>> {
    self.has_response_body().then(|| self.response_body())
  }
  pub fn response_body(self) -> super::HttpBodyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpBodyView::default())
  }

  // request_trailers: optional message envoy.service.ext_proc.v3.HttpTrailers
  pub fn has_request_trailers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn request_trailers_opt(self) -> ::std::option::Option<super::HttpTrailersView<'msg>> {
    self.has_request_trailers().then(|| self.request_trailers())
  }
  pub fn request_trailers(self) -> super::HttpTrailersView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpTrailersView::default())
  }

  // response_trailers: optional message envoy.service.ext_proc.v3.HttpTrailers
  pub fn has_response_trailers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn response_trailers_opt(self) -> ::std::option::Option<super::HttpTrailersView<'msg>> {
    self.has_response_trailers().then(|| self.response_trailers())
  }
  pub fn response_trailers(self) -> super::HttpTrailersView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpTrailersView::default())
  }

  // metadata_context: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn metadata_context_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata_context().then(|| self.metadata_context())
  }
  pub fn metadata_context(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

  // attributes: repeated message envoy.service.ext_proc.v3.ProcessingRequest.AttributesEntry
  pub fn attributes(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(7)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // observability_mode: optional bool
  pub fn observability_mode(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }

  // protocol_config: optional message envoy.service.ext_proc.v3.ProtocolConfiguration
  pub fn has_protocol_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn protocol_config_opt(self) -> ::std::option::Option<super::ProtocolConfigurationView<'msg>> {
    self.has_protocol_config().then(|| self.protocol_config())
  }
  pub fn protocol_config(self) -> super::ProtocolConfigurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ProtocolConfigurationView::default())
  }

  pub fn request(self) -> super::processing_request::RequestOneof<'msg> {
    match self.request_case() {
      super::processing_request::RequestCase::RequestHeaders =>
          super::processing_request::RequestOneof::RequestHeaders(self.request_headers()),
      super::processing_request::RequestCase::ResponseHeaders =>
          super::processing_request::RequestOneof::ResponseHeaders(self.response_headers()),
      super::processing_request::RequestCase::RequestBody =>
          super::processing_request::RequestOneof::RequestBody(self.request_body()),
      super::processing_request::RequestCase::ResponseBody =>
          super::processing_request::RequestOneof::ResponseBody(self.response_body()),
      super::processing_request::RequestCase::RequestTrailers =>
          super::processing_request::RequestOneof::RequestTrailers(self.request_trailers()),
      super::processing_request::RequestCase::ResponseTrailers =>
          super::processing_request::RequestOneof::ResponseTrailers(self.response_trailers()),
      _ => super::processing_request::RequestOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn request_case(self) -> super::processing_request::RequestCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::processing_request::RequestCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ProcessingRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ProcessingRequestView<'_> {}

// SAFETY:
// - `ProcessingRequestView` is `Send` because while its alive a `ProcessingRequestMut` cannot.
// - `ProcessingRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for ProcessingRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ProcessingRequestView<'msg> {
  type Proxied = ProcessingRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ProcessingRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProcessingRequestView<'msg> {
  fn into_view<'shorter>(self) -> ProcessingRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProcessingRequest> for ProcessingRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProcessingRequest {
    let mut dst = ProcessingRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProcessingRequest> for ProcessingRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProcessingRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ProcessingRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProcessingRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProcessingRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProcessingRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProcessingRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProcessingRequestMut<'msg> {
  type Message = ProcessingRequest;
}

impl ::std::fmt::Debug for ProcessingRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingRequest>> for ProcessingRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProcessingRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ProcessingRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // request_headers: optional message envoy.service.ext_proc.v3.HttpHeaders
  pub fn has_request_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_request_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn request_headers_opt(&self) -> ::std::option::Option<super::HttpHeadersView<'_>> {
    self.has_request_headers().then(|| self.request_headers())
  }
  pub fn request_headers(&self) -> super::HttpHeadersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersView::default())
  }
  pub fn request_headers_mut(&mut self) -> super::HttpHeadersMut<'_> {
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
  pub fn set_request_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeaders>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // response_headers: optional message envoy.service.ext_proc.v3.HttpHeaders
  pub fn has_response_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_response_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn response_headers_opt(&self) -> ::std::option::Option<super::HttpHeadersView<'_>> {
    self.has_response_headers().then(|| self.response_headers())
  }
  pub fn response_headers(&self) -> super::HttpHeadersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersView::default())
  }
  pub fn response_headers_mut(&mut self) -> super::HttpHeadersMut<'_> {
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
  pub fn set_response_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeaders>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // request_body: optional message envoy.service.ext_proc.v3.HttpBody
  pub fn has_request_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_request_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn request_body_opt(&self) -> ::std::option::Option<super::HttpBodyView<'_>> {
    self.has_request_body().then(|| self.request_body())
  }
  pub fn request_body(&self) -> super::HttpBodyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpBodyView::default())
  }
  pub fn request_body_mut(&mut self) -> super::HttpBodyMut<'_> {
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
  pub fn set_request_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpBody>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // response_body: optional message envoy.service.ext_proc.v3.HttpBody
  pub fn has_response_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_response_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn response_body_opt(&self) -> ::std::option::Option<super::HttpBodyView<'_>> {
    self.has_response_body().then(|| self.response_body())
  }
  pub fn response_body(&self) -> super::HttpBodyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpBodyView::default())
  }
  pub fn response_body_mut(&mut self) -> super::HttpBodyMut<'_> {
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
  pub fn set_response_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpBody>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // request_trailers: optional message envoy.service.ext_proc.v3.HttpTrailers
  pub fn has_request_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_request_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn request_trailers_opt(&self) -> ::std::option::Option<super::HttpTrailersView<'_>> {
    self.has_request_trailers().then(|| self.request_trailers())
  }
  pub fn request_trailers(&self) -> super::HttpTrailersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpTrailersView::default())
  }
  pub fn request_trailers_mut(&mut self) -> super::HttpTrailersMut<'_> {
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
  pub fn set_request_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpTrailers>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // response_trailers: optional message envoy.service.ext_proc.v3.HttpTrailers
  pub fn has_response_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_response_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn response_trailers_opt(&self) -> ::std::option::Option<super::HttpTrailersView<'_>> {
    self.has_response_trailers().then(|| self.response_trailers())
  }
  pub fn response_trailers(&self) -> super::HttpTrailersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpTrailersView::default())
  }
  pub fn response_trailers_mut(&mut self) -> super::HttpTrailersMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_response_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpTrailers>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // metadata_context: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_metadata_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn metadata_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata_context().then(|| self.metadata_context())
  }
  pub fn metadata_context(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_context_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // attributes: repeated message envoy.service.ext_proc.v3.ProcessingRequest.AttributesEntry
  pub fn attributes(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(7)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn attributes_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          7, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_attributes(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // observability_mode: optional bool
  pub fn observability_mode(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_observability_mode(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

  // protocol_config: optional message envoy.service.ext_proc.v3.ProtocolConfiguration
  pub fn has_protocol_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_protocol_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn protocol_config_opt(&self) -> ::std::option::Option<super::ProtocolConfigurationView<'_>> {
    self.has_protocol_config().then(|| self.protocol_config())
  }
  pub fn protocol_config(&self) -> super::ProtocolConfigurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ProtocolConfigurationView::default())
  }
  pub fn protocol_config_mut(&mut self) -> super::ProtocolConfigurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_protocol_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProtocolConfiguration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  pub fn request(&self) -> super::processing_request::RequestOneof<'_> {
    match &self.request_case() {
      super::processing_request::RequestCase::RequestHeaders =>
          super::processing_request::RequestOneof::RequestHeaders(self.request_headers()),
      super::processing_request::RequestCase::ResponseHeaders =>
          super::processing_request::RequestOneof::ResponseHeaders(self.response_headers()),
      super::processing_request::RequestCase::RequestBody =>
          super::processing_request::RequestOneof::RequestBody(self.request_body()),
      super::processing_request::RequestCase::ResponseBody =>
          super::processing_request::RequestOneof::ResponseBody(self.response_body()),
      super::processing_request::RequestCase::RequestTrailers =>
          super::processing_request::RequestOneof::RequestTrailers(self.request_trailers()),
      super::processing_request::RequestCase::ResponseTrailers =>
          super::processing_request::RequestOneof::ResponseTrailers(self.response_trailers()),
      _ => super::processing_request::RequestOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn request_case(&self) -> super::processing_request::RequestCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::processing_request::RequestCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ProcessingRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ProcessingRequestMut<'_> {}

// SAFETY:
// - `ProcessingRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ProcessingRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ProcessingRequestMut<'msg> {
  type Proxied = ProcessingRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ProcessingRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProcessingRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProcessingRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ProcessingRequestMut<'msg> {
  type MutProxied = ProcessingRequest;
  fn as_mut(&mut self) -> ProcessingRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProcessingRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ProcessingRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProcessingRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProcessingRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProcessingRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProcessingRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // request_headers: optional message envoy.service.ext_proc.v3.HttpHeaders
  pub fn has_request_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_request_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn request_headers_opt(&self) -> ::std::option::Option<super::HttpHeadersView<'_>> {
    self.has_request_headers().then(|| self.request_headers())
  }
  pub fn request_headers(&self) -> super::HttpHeadersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersView::default())
  }
  pub fn request_headers_mut(&mut self) -> super::HttpHeadersMut<'_> {
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
  pub fn set_request_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeaders>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // response_headers: optional message envoy.service.ext_proc.v3.HttpHeaders
  pub fn has_response_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_response_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn response_headers_opt(&self) -> ::std::option::Option<super::HttpHeadersView<'_>> {
    self.has_response_headers().then(|| self.response_headers())
  }
  pub fn response_headers(&self) -> super::HttpHeadersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersView::default())
  }
  pub fn response_headers_mut(&mut self) -> super::HttpHeadersMut<'_> {
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
  pub fn set_response_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeaders>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // request_body: optional message envoy.service.ext_proc.v3.HttpBody
  pub fn has_request_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_request_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn request_body_opt(&self) -> ::std::option::Option<super::HttpBodyView<'_>> {
    self.has_request_body().then(|| self.request_body())
  }
  pub fn request_body(&self) -> super::HttpBodyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpBodyView::default())
  }
  pub fn request_body_mut(&mut self) -> super::HttpBodyMut<'_> {
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
  pub fn set_request_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpBody>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // response_body: optional message envoy.service.ext_proc.v3.HttpBody
  pub fn has_response_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_response_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn response_body_opt(&self) -> ::std::option::Option<super::HttpBodyView<'_>> {
    self.has_response_body().then(|| self.response_body())
  }
  pub fn response_body(&self) -> super::HttpBodyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpBodyView::default())
  }
  pub fn response_body_mut(&mut self) -> super::HttpBodyMut<'_> {
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
  pub fn set_response_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpBody>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // request_trailers: optional message envoy.service.ext_proc.v3.HttpTrailers
  pub fn has_request_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_request_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn request_trailers_opt(&self) -> ::std::option::Option<super::HttpTrailersView<'_>> {
    self.has_request_trailers().then(|| self.request_trailers())
  }
  pub fn request_trailers(&self) -> super::HttpTrailersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpTrailersView::default())
  }
  pub fn request_trailers_mut(&mut self) -> super::HttpTrailersMut<'_> {
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
  pub fn set_request_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpTrailers>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // response_trailers: optional message envoy.service.ext_proc.v3.HttpTrailers
  pub fn has_response_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_response_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn response_trailers_opt(&self) -> ::std::option::Option<super::HttpTrailersView<'_>> {
    self.has_response_trailers().then(|| self.response_trailers())
  }
  pub fn response_trailers(&self) -> super::HttpTrailersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpTrailersView::default())
  }
  pub fn response_trailers_mut(&mut self) -> super::HttpTrailersMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_response_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpTrailers>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // metadata_context: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_metadata_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn metadata_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata_context().then(|| self.metadata_context())
  }
  pub fn metadata_context(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_context_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // attributes: repeated message envoy.service.ext_proc.v3.ProcessingRequest.AttributesEntry
  pub fn attributes(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(7)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn attributes_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          7, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_attributes(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // observability_mode: optional bool
  pub fn observability_mode(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_observability_mode(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

  // protocol_config: optional message envoy.service.ext_proc.v3.ProtocolConfiguration
  pub fn has_protocol_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_protocol_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn protocol_config_opt(&self) -> ::std::option::Option<super::ProtocolConfigurationView<'_>> {
    self.has_protocol_config().then(|| self.protocol_config())
  }
  pub fn protocol_config(&self) -> super::ProtocolConfigurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ProtocolConfigurationView::default())
  }
  pub fn protocol_config_mut(&mut self) -> super::ProtocolConfigurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_protocol_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::ProtocolConfiguration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  pub fn request(&self) -> super::processing_request::RequestOneof<'_> {
    match &self.request_case() {
      super::processing_request::RequestCase::RequestHeaders =>
          super::processing_request::RequestOneof::RequestHeaders(self.request_headers()),
      super::processing_request::RequestCase::ResponseHeaders =>
          super::processing_request::RequestOneof::ResponseHeaders(self.response_headers()),
      super::processing_request::RequestCase::RequestBody =>
          super::processing_request::RequestOneof::RequestBody(self.request_body()),
      super::processing_request::RequestCase::ResponseBody =>
          super::processing_request::RequestOneof::ResponseBody(self.response_body()),
      super::processing_request::RequestCase::RequestTrailers =>
          super::processing_request::RequestOneof::RequestTrailers(self.request_trailers()),
      super::processing_request::RequestCase::ResponseTrailers =>
          super::processing_request::RequestOneof::ResponseTrailers(self.response_trailers()),
      _ => super::processing_request::RequestOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn request_case(&self) -> super::processing_request::RequestCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::processing_request::RequestCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ProcessingRequest

impl ::std::ops::Drop for ProcessingRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProcessingRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProcessingRequest {
  type Proxied = Self;
  fn as_view(&self) -> ProcessingRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProcessingRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProcessingRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProcessingRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__ProcessingRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$a3333333G/P3^#|$|%|&|(|)");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__ProcessingRequest_msg_init.0, &[<super::HttpHeaders as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpHeaders as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpBody as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpBody as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpTrailers as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpTrailers as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::processing_request::AttributesEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ProtocolConfiguration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__ProcessingRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProcessingRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProcessingRequest {
  type Msg = ProcessingRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProcessingRequest {
  type Msg = ProcessingRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProcessingRequestMut<'_> {
  type Msg = ProcessingRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProcessingRequestMut<'_> {
  type Msg = ProcessingRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProcessingRequestView<'_> {
  type Msg = ProcessingRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProcessingRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod processing_request {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__ProcessingRequest__AttributesEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct AttributesEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AttributesEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::processing_request::envoy__service__ext_0proc__v3__ProcessingRequest__AttributesEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::processing_request::envoy__service__ext_0proc__v3__ProcessingRequest__AttributesEntry_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::processing_request::envoy__service__ext_0proc__v3__ProcessingRequest__AttributesEntry_msg_init.0)
      }).0
    }
  }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum RequestOneof<'msg> {
  RequestHeaders(::protobuf::View<'msg, super::super::HttpHeaders>) = 2,
  ResponseHeaders(::protobuf::View<'msg, super::super::HttpHeaders>) = 3,
  RequestBody(::protobuf::View<'msg, super::super::HttpBody>) = 4,
  ResponseBody(::protobuf::View<'msg, super::super::HttpBody>) = 5,
  RequestTrailers(::protobuf::View<'msg, super::super::HttpTrailers>) = 6,
  ResponseTrailers(::protobuf::View<'msg, super::super::HttpTrailers>) = 7,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum RequestCase {
  RequestHeaders = 2,
  ResponseHeaders = 3,
  RequestBody = 4,
  ResponseBody = 5,
  RequestTrailers = 6,
  ResponseTrailers = 7,

  not_set = 0
}

impl RequestCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<RequestCase> {
    match v {
      0 => Some(RequestCase::not_set),
      2 => Some(RequestCase::RequestHeaders),
      3 => Some(RequestCase::ResponseHeaders),
      4 => Some(RequestCase::RequestBody),
      5 => Some(RequestCase::ResponseBody),
      6 => Some(RequestCase::RequestTrailers),
      7 => Some(RequestCase::ResponseTrailers),
      _ => None
    }
  }
}
}  // pub mod processing_request


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__ProcessingResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProcessingResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProcessingResponse>
}

impl ::protobuf::Message for ProcessingResponse {
  type MessageView<'msg> = ProcessingResponseView<'msg>;
  type MessageMut<'msg> = ProcessingResponseMut<'msg>;
}

impl ::std::default::Default for ProcessingResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProcessingResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProcessingResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ProcessingResponseMut`.
unsafe impl ::std::marker::Sync for ProcessingResponse {}

// SAFETY:
// - `ProcessingResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ProcessingResponse {}

impl ::protobuf::Proxied for ProcessingResponse {
  type View<'msg> = ProcessingResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProcessingResponse {}

impl ::protobuf::MutProxied for ProcessingResponse {
  type Mut<'msg> = ProcessingResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProcessingResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProcessingResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProcessingResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProcessingResponseView<'msg> {
  type Message = ProcessingResponse;
}

impl ::std::fmt::Debug for ProcessingResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProcessingResponseView<'_> {
  fn default() -> ProcessingResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProcessingResponse>> for ProcessingResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProcessingResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProcessingResponseView<'msg> {

  pub fn to_owned(&self) -> ProcessingResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // request_headers: optional message envoy.service.ext_proc.v3.HeadersResponse
  pub fn has_request_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn request_headers_opt(self) -> ::std::option::Option<super::HeadersResponseView<'msg>> {
    self.has_request_headers().then(|| self.request_headers())
  }
  pub fn request_headers(self) -> super::HeadersResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeadersResponseView::default())
  }

  // response_headers: optional message envoy.service.ext_proc.v3.HeadersResponse
  pub fn has_response_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn response_headers_opt(self) -> ::std::option::Option<super::HeadersResponseView<'msg>> {
    self.has_response_headers().then(|| self.response_headers())
  }
  pub fn response_headers(self) -> super::HeadersResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeadersResponseView::default())
  }

  // request_body: optional message envoy.service.ext_proc.v3.BodyResponse
  pub fn has_request_body(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn request_body_opt(self) -> ::std::option::Option<super::BodyResponseView<'msg>> {
    self.has_request_body().then(|| self.request_body())
  }
  pub fn request_body(self) -> super::BodyResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BodyResponseView::default())
  }

  // response_body: optional message envoy.service.ext_proc.v3.BodyResponse
  pub fn has_response_body(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn response_body_opt(self) -> ::std::option::Option<super::BodyResponseView<'msg>> {
    self.has_response_body().then(|| self.response_body())
  }
  pub fn response_body(self) -> super::BodyResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BodyResponseView::default())
  }

  // request_trailers: optional message envoy.service.ext_proc.v3.TrailersResponse
  pub fn has_request_trailers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn request_trailers_opt(self) -> ::std::option::Option<super::TrailersResponseView<'msg>> {
    self.has_request_trailers().then(|| self.request_trailers())
  }
  pub fn request_trailers(self) -> super::TrailersResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TrailersResponseView::default())
  }

  // response_trailers: optional message envoy.service.ext_proc.v3.TrailersResponse
  pub fn has_response_trailers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn response_trailers_opt(self) -> ::std::option::Option<super::TrailersResponseView<'msg>> {
    self.has_response_trailers().then(|| self.response_trailers())
  }
  pub fn response_trailers(self) -> super::TrailersResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TrailersResponseView::default())
  }

  // immediate_response: optional message envoy.service.ext_proc.v3.ImmediateResponse
  pub fn has_immediate_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn immediate_response_opt(self) -> ::std::option::Option<super::ImmediateResponseView<'msg>> {
    self.has_immediate_response().then(|| self.immediate_response())
  }
  pub fn immediate_response(self) -> super::ImmediateResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ImmediateResponseView::default())
  }

  // streamed_immediate_response: optional message envoy.service.ext_proc.v3.StreamedImmediateResponse
  pub fn has_streamed_immediate_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn streamed_immediate_response_opt(self) -> ::std::option::Option<super::StreamedImmediateResponseView<'msg>> {
    self.has_streamed_immediate_response().then(|| self.streamed_immediate_response())
  }
  pub fn streamed_immediate_response(self) -> super::StreamedImmediateResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StreamedImmediateResponseView::default())
  }

  // dynamic_metadata: optional message google.protobuf.Struct
  pub fn has_dynamic_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn dynamic_metadata_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_dynamic_metadata().then(|| self.dynamic_metadata())
  }
  pub fn dynamic_metadata(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // mode_override: optional message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn has_mode_override(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn mode_override_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'msg>> {
    self.has_mode_override().then(|| self.mode_override())
  }
  pub fn mode_override(self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView::default())
  }

  // request_drain: optional bool
  pub fn request_drain(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        11, (false).into()
      ).try_into().unwrap()
    }
  }

  // override_message_timeout: optional message google.protobuf.Duration
  pub fn has_override_message_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn override_message_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_override_message_timeout().then(|| self.override_message_timeout())
  }
  pub fn override_message_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  pub fn response(self) -> super::processing_response::ResponseOneof<'msg> {
    match self.response_case() {
      super::processing_response::ResponseCase::RequestHeaders =>
          super::processing_response::ResponseOneof::RequestHeaders(self.request_headers()),
      super::processing_response::ResponseCase::ResponseHeaders =>
          super::processing_response::ResponseOneof::ResponseHeaders(self.response_headers()),
      super::processing_response::ResponseCase::RequestBody =>
          super::processing_response::ResponseOneof::RequestBody(self.request_body()),
      super::processing_response::ResponseCase::ResponseBody =>
          super::processing_response::ResponseOneof::ResponseBody(self.response_body()),
      super::processing_response::ResponseCase::RequestTrailers =>
          super::processing_response::ResponseOneof::RequestTrailers(self.request_trailers()),
      super::processing_response::ResponseCase::ResponseTrailers =>
          super::processing_response::ResponseOneof::ResponseTrailers(self.response_trailers()),
      super::processing_response::ResponseCase::ImmediateResponse =>
          super::processing_response::ResponseOneof::ImmediateResponse(self.immediate_response()),
      super::processing_response::ResponseCase::StreamedImmediateResponse =>
          super::processing_response::ResponseOneof::StreamedImmediateResponse(self.streamed_immediate_response()),
      _ => super::processing_response::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(self) -> super::processing_response::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::processing_response::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ProcessingResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ProcessingResponseView<'_> {}

// SAFETY:
// - `ProcessingResponseView` is `Send` because while its alive a `ProcessingResponseMut` cannot.
// - `ProcessingResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for ProcessingResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ProcessingResponseView<'msg> {
  type Proxied = ProcessingResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ProcessingResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProcessingResponseView<'msg> {
  fn into_view<'shorter>(self) -> ProcessingResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProcessingResponse> for ProcessingResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProcessingResponse {
    let mut dst = ProcessingResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProcessingResponse> for ProcessingResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProcessingResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ProcessingResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProcessingResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProcessingResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProcessingResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProcessingResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProcessingResponseMut<'msg> {
  type Message = ProcessingResponse;
}

impl ::std::fmt::Debug for ProcessingResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingResponse>> for ProcessingResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProcessingResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProcessingResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ProcessingResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // request_headers: optional message envoy.service.ext_proc.v3.HeadersResponse
  pub fn has_request_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_request_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn request_headers_opt(&self) -> ::std::option::Option<super::HeadersResponseView<'_>> {
    self.has_request_headers().then(|| self.request_headers())
  }
  pub fn request_headers(&self) -> super::HeadersResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeadersResponseView::default())
  }
  pub fn request_headers_mut(&mut self) -> super::HeadersResponseMut<'_> {
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
  pub fn set_request_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeadersResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // response_headers: optional message envoy.service.ext_proc.v3.HeadersResponse
  pub fn has_response_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_response_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn response_headers_opt(&self) -> ::std::option::Option<super::HeadersResponseView<'_>> {
    self.has_response_headers().then(|| self.response_headers())
  }
  pub fn response_headers(&self) -> super::HeadersResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeadersResponseView::default())
  }
  pub fn response_headers_mut(&mut self) -> super::HeadersResponseMut<'_> {
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
  pub fn set_response_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeadersResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // request_body: optional message envoy.service.ext_proc.v3.BodyResponse
  pub fn has_request_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_request_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn request_body_opt(&self) -> ::std::option::Option<super::BodyResponseView<'_>> {
    self.has_request_body().then(|| self.request_body())
  }
  pub fn request_body(&self) -> super::BodyResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BodyResponseView::default())
  }
  pub fn request_body_mut(&mut self) -> super::BodyResponseMut<'_> {
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
  pub fn set_request_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::BodyResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // response_body: optional message envoy.service.ext_proc.v3.BodyResponse
  pub fn has_response_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_response_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn response_body_opt(&self) -> ::std::option::Option<super::BodyResponseView<'_>> {
    self.has_response_body().then(|| self.response_body())
  }
  pub fn response_body(&self) -> super::BodyResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BodyResponseView::default())
  }
  pub fn response_body_mut(&mut self) -> super::BodyResponseMut<'_> {
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
  pub fn set_response_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::BodyResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // request_trailers: optional message envoy.service.ext_proc.v3.TrailersResponse
  pub fn has_request_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_request_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn request_trailers_opt(&self) -> ::std::option::Option<super::TrailersResponseView<'_>> {
    self.has_request_trailers().then(|| self.request_trailers())
  }
  pub fn request_trailers(&self) -> super::TrailersResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TrailersResponseView::default())
  }
  pub fn request_trailers_mut(&mut self) -> super::TrailersResponseMut<'_> {
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
  pub fn set_request_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<super::TrailersResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // response_trailers: optional message envoy.service.ext_proc.v3.TrailersResponse
  pub fn has_response_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_response_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn response_trailers_opt(&self) -> ::std::option::Option<super::TrailersResponseView<'_>> {
    self.has_response_trailers().then(|| self.response_trailers())
  }
  pub fn response_trailers(&self) -> super::TrailersResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TrailersResponseView::default())
  }
  pub fn response_trailers_mut(&mut self) -> super::TrailersResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_response_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<super::TrailersResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // immediate_response: optional message envoy.service.ext_proc.v3.ImmediateResponse
  pub fn has_immediate_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_immediate_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn immediate_response_opt(&self) -> ::std::option::Option<super::ImmediateResponseView<'_>> {
    self.has_immediate_response().then(|| self.immediate_response())
  }
  pub fn immediate_response(&self) -> super::ImmediateResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ImmediateResponseView::default())
  }
  pub fn immediate_response_mut(&mut self) -> super::ImmediateResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_immediate_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::ImmediateResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // streamed_immediate_response: optional message envoy.service.ext_proc.v3.StreamedImmediateResponse
  pub fn has_streamed_immediate_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_streamed_immediate_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn streamed_immediate_response_opt(&self) -> ::std::option::Option<super::StreamedImmediateResponseView<'_>> {
    self.has_streamed_immediate_response().then(|| self.streamed_immediate_response())
  }
  pub fn streamed_immediate_response(&self) -> super::StreamedImmediateResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StreamedImmediateResponseView::default())
  }
  pub fn streamed_immediate_response_mut(&mut self) -> super::StreamedImmediateResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_streamed_immediate_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::StreamedImmediateResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // dynamic_metadata: optional message google.protobuf.Struct
  pub fn has_dynamic_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_dynamic_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn dynamic_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_dynamic_metadata().then(|| self.dynamic_metadata())
  }
  pub fn dynamic_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn dynamic_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_dynamic_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // mode_override: optional message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn has_mode_override(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_mode_override(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn mode_override_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_>> {
    self.has_mode_override().then(|| self.mode_override())
  }
  pub fn mode_override(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView::default())
  }
  pub fn mode_override_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_mode_override(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // request_drain: optional bool
  pub fn request_drain(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        11, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_drain(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        11, val.into()
      )
    }
  }

  // override_message_timeout: optional message google.protobuf.Duration
  pub fn has_override_message_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_override_message_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn override_message_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_override_message_timeout().then(|| self.override_message_timeout())
  }
  pub fn override_message_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn override_message_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_override_message_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  pub fn response(&self) -> super::processing_response::ResponseOneof<'_> {
    match &self.response_case() {
      super::processing_response::ResponseCase::RequestHeaders =>
          super::processing_response::ResponseOneof::RequestHeaders(self.request_headers()),
      super::processing_response::ResponseCase::ResponseHeaders =>
          super::processing_response::ResponseOneof::ResponseHeaders(self.response_headers()),
      super::processing_response::ResponseCase::RequestBody =>
          super::processing_response::ResponseOneof::RequestBody(self.request_body()),
      super::processing_response::ResponseCase::ResponseBody =>
          super::processing_response::ResponseOneof::ResponseBody(self.response_body()),
      super::processing_response::ResponseCase::RequestTrailers =>
          super::processing_response::ResponseOneof::RequestTrailers(self.request_trailers()),
      super::processing_response::ResponseCase::ResponseTrailers =>
          super::processing_response::ResponseOneof::ResponseTrailers(self.response_trailers()),
      super::processing_response::ResponseCase::ImmediateResponse =>
          super::processing_response::ResponseOneof::ImmediateResponse(self.immediate_response()),
      super::processing_response::ResponseCase::StreamedImmediateResponse =>
          super::processing_response::ResponseOneof::StreamedImmediateResponse(self.streamed_immediate_response()),
      _ => super::processing_response::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(&self) -> super::processing_response::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::processing_response::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ProcessingResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ProcessingResponseMut<'_> {}

// SAFETY:
// - `ProcessingResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ProcessingResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ProcessingResponseMut<'msg> {
  type Proxied = ProcessingResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ProcessingResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProcessingResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProcessingResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ProcessingResponseMut<'msg> {
  type MutProxied = ProcessingResponse;
  fn as_mut(&mut self) -> ProcessingResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProcessingResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ProcessingResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProcessingResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProcessingResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProcessingResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProcessingResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // request_headers: optional message envoy.service.ext_proc.v3.HeadersResponse
  pub fn has_request_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_request_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn request_headers_opt(&self) -> ::std::option::Option<super::HeadersResponseView<'_>> {
    self.has_request_headers().then(|| self.request_headers())
  }
  pub fn request_headers(&self) -> super::HeadersResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeadersResponseView::default())
  }
  pub fn request_headers_mut(&mut self) -> super::HeadersResponseMut<'_> {
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
  pub fn set_request_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeadersResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // response_headers: optional message envoy.service.ext_proc.v3.HeadersResponse
  pub fn has_response_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_response_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn response_headers_opt(&self) -> ::std::option::Option<super::HeadersResponseView<'_>> {
    self.has_response_headers().then(|| self.response_headers())
  }
  pub fn response_headers(&self) -> super::HeadersResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeadersResponseView::default())
  }
  pub fn response_headers_mut(&mut self) -> super::HeadersResponseMut<'_> {
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
  pub fn set_response_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeadersResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // request_body: optional message envoy.service.ext_proc.v3.BodyResponse
  pub fn has_request_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_request_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn request_body_opt(&self) -> ::std::option::Option<super::BodyResponseView<'_>> {
    self.has_request_body().then(|| self.request_body())
  }
  pub fn request_body(&self) -> super::BodyResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BodyResponseView::default())
  }
  pub fn request_body_mut(&mut self) -> super::BodyResponseMut<'_> {
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
  pub fn set_request_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::BodyResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // response_body: optional message envoy.service.ext_proc.v3.BodyResponse
  pub fn has_response_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_response_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn response_body_opt(&self) -> ::std::option::Option<super::BodyResponseView<'_>> {
    self.has_response_body().then(|| self.response_body())
  }
  pub fn response_body(&self) -> super::BodyResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BodyResponseView::default())
  }
  pub fn response_body_mut(&mut self) -> super::BodyResponseMut<'_> {
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
  pub fn set_response_body(&mut self,
    val: impl ::protobuf::IntoProxied<super::BodyResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // request_trailers: optional message envoy.service.ext_proc.v3.TrailersResponse
  pub fn has_request_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_request_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn request_trailers_opt(&self) -> ::std::option::Option<super::TrailersResponseView<'_>> {
    self.has_request_trailers().then(|| self.request_trailers())
  }
  pub fn request_trailers(&self) -> super::TrailersResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TrailersResponseView::default())
  }
  pub fn request_trailers_mut(&mut self) -> super::TrailersResponseMut<'_> {
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
  pub fn set_request_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<super::TrailersResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // response_trailers: optional message envoy.service.ext_proc.v3.TrailersResponse
  pub fn has_response_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_response_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn response_trailers_opt(&self) -> ::std::option::Option<super::TrailersResponseView<'_>> {
    self.has_response_trailers().then(|| self.response_trailers())
  }
  pub fn response_trailers(&self) -> super::TrailersResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TrailersResponseView::default())
  }
  pub fn response_trailers_mut(&mut self) -> super::TrailersResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_response_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<super::TrailersResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // immediate_response: optional message envoy.service.ext_proc.v3.ImmediateResponse
  pub fn has_immediate_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_immediate_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn immediate_response_opt(&self) -> ::std::option::Option<super::ImmediateResponseView<'_>> {
    self.has_immediate_response().then(|| self.immediate_response())
  }
  pub fn immediate_response(&self) -> super::ImmediateResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ImmediateResponseView::default())
  }
  pub fn immediate_response_mut(&mut self) -> super::ImmediateResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_immediate_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::ImmediateResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // streamed_immediate_response: optional message envoy.service.ext_proc.v3.StreamedImmediateResponse
  pub fn has_streamed_immediate_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_streamed_immediate_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn streamed_immediate_response_opt(&self) -> ::std::option::Option<super::StreamedImmediateResponseView<'_>> {
    self.has_streamed_immediate_response().then(|| self.streamed_immediate_response())
  }
  pub fn streamed_immediate_response(&self) -> super::StreamedImmediateResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StreamedImmediateResponseView::default())
  }
  pub fn streamed_immediate_response_mut(&mut self) -> super::StreamedImmediateResponseMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_streamed_immediate_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::StreamedImmediateResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // dynamic_metadata: optional message google.protobuf.Struct
  pub fn has_dynamic_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_dynamic_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn dynamic_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_dynamic_metadata().then(|| self.dynamic_metadata())
  }
  pub fn dynamic_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn dynamic_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_dynamic_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // mode_override: optional message envoy.extensions.filters.http.ext_proc.v3.ProcessingMode
  pub fn has_mode_override(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_mode_override(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn mode_override_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_>> {
    self.has_mode_override().then(|| self.mode_override())
  }
  pub fn mode_override(&self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeView::default())
  }
  pub fn mode_override_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingModeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_mode_override(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // request_drain: optional bool
  pub fn request_drain(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        11, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_drain(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        11, val.into()
      )
    }
  }

  // override_message_timeout: optional message google.protobuf.Duration
  pub fn has_override_message_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_override_message_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn override_message_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_override_message_timeout().then(|| self.override_message_timeout())
  }
  pub fn override_message_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn override_message_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_override_message_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  pub fn response(&self) -> super::processing_response::ResponseOneof<'_> {
    match &self.response_case() {
      super::processing_response::ResponseCase::RequestHeaders =>
          super::processing_response::ResponseOneof::RequestHeaders(self.request_headers()),
      super::processing_response::ResponseCase::ResponseHeaders =>
          super::processing_response::ResponseOneof::ResponseHeaders(self.response_headers()),
      super::processing_response::ResponseCase::RequestBody =>
          super::processing_response::ResponseOneof::RequestBody(self.request_body()),
      super::processing_response::ResponseCase::ResponseBody =>
          super::processing_response::ResponseOneof::ResponseBody(self.response_body()),
      super::processing_response::ResponseCase::RequestTrailers =>
          super::processing_response::ResponseOneof::RequestTrailers(self.request_trailers()),
      super::processing_response::ResponseCase::ResponseTrailers =>
          super::processing_response::ResponseOneof::ResponseTrailers(self.response_trailers()),
      super::processing_response::ResponseCase::ImmediateResponse =>
          super::processing_response::ResponseOneof::ImmediateResponse(self.immediate_response()),
      super::processing_response::ResponseCase::StreamedImmediateResponse =>
          super::processing_response::ResponseOneof::StreamedImmediateResponse(self.streamed_immediate_response()),
      _ => super::processing_response::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(&self) -> super::processing_response::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::processing_response::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ProcessingResponse

impl ::std::ops::Drop for ProcessingResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProcessingResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProcessingResponse {
  type Proxied = Self;
  fn as_view(&self) -> ProcessingResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProcessingResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProcessingResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProcessingResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__ProcessingResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33333333333/P^!|#|$|%|&|(|)|-");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__ProcessingResponse_msg_init.0, &[<super::HeadersResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HeadersResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BodyResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BodyResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TrailersResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TrailersResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ImmediateResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::filters::http::ext_proc::v3::processing_mode::ProcessingMode as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::StreamedImmediateResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__ProcessingResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProcessingResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProcessingResponse {
  type Msg = ProcessingResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProcessingResponse {
  type Msg = ProcessingResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProcessingResponseMut<'_> {
  type Msg = ProcessingResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProcessingResponseMut<'_> {
  type Msg = ProcessingResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProcessingResponseView<'_> {
  type Msg = ProcessingResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProcessingResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProcessingResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod processing_response {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ResponseOneof<'msg> {
  RequestHeaders(::protobuf::View<'msg, super::super::HeadersResponse>) = 1,
  ResponseHeaders(::protobuf::View<'msg, super::super::HeadersResponse>) = 2,
  RequestBody(::protobuf::View<'msg, super::super::BodyResponse>) = 3,
  ResponseBody(::protobuf::View<'msg, super::super::BodyResponse>) = 4,
  RequestTrailers(::protobuf::View<'msg, super::super::TrailersResponse>) = 5,
  ResponseTrailers(::protobuf::View<'msg, super::super::TrailersResponse>) = 6,
  ImmediateResponse(::protobuf::View<'msg, super::super::ImmediateResponse>) = 7,
  StreamedImmediateResponse(::protobuf::View<'msg, super::super::StreamedImmediateResponse>) = 11,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ResponseCase {
  RequestHeaders = 1,
  ResponseHeaders = 2,
  RequestBody = 3,
  ResponseBody = 4,
  RequestTrailers = 5,
  ResponseTrailers = 6,
  ImmediateResponse = 7,
  StreamedImmediateResponse = 11,

  not_set = 0
}

impl ResponseCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ResponseCase> {
    match v {
      0 => Some(ResponseCase::not_set),
      1 => Some(ResponseCase::RequestHeaders),
      2 => Some(ResponseCase::ResponseHeaders),
      3 => Some(ResponseCase::RequestBody),
      4 => Some(ResponseCase::ResponseBody),
      5 => Some(ResponseCase::RequestTrailers),
      6 => Some(ResponseCase::ResponseTrailers),
      7 => Some(ResponseCase::ImmediateResponse),
      11 => Some(ResponseCase::StreamedImmediateResponse),
      _ => None
    }
  }
}
}  // pub mod processing_response


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__HttpHeaders_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpHeaders {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpHeaders>
}

impl ::protobuf::Message for HttpHeaders {
  type MessageView<'msg> = HttpHeadersView<'msg>;
  type MessageMut<'msg> = HttpHeadersMut<'msg>;
}

impl ::std::default::Default for HttpHeaders {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpHeaders {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpHeaders` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpHeadersMut`.
unsafe impl ::std::marker::Sync for HttpHeaders {}

// SAFETY:
// - `HttpHeaders` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpHeaders {}

impl ::protobuf::Proxied for HttpHeaders {
  type View<'msg> = HttpHeadersView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpHeaders {}

impl ::protobuf::MutProxied for HttpHeaders {
  type Mut<'msg> = HttpHeadersMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpHeadersView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpHeaders>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpHeadersView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpHeadersView<'msg> {
  type Message = HttpHeaders;
}

impl ::std::fmt::Debug for HttpHeadersView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpHeadersView<'_> {
  fn default() -> HttpHeadersView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpHeaders>> for HttpHeadersView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpHeaders>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpHeadersView<'msg> {

  pub fn to_owned(&self) -> HttpHeaders {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // headers: optional message envoy.config.core.v3.HeaderMap
  pub fn has_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn headers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg>> {
    self.has_headers().then(|| self.headers())
  }
  pub fn headers(self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }

  // attributes: repeated message envoy.service.ext_proc.v3.HttpHeaders.AttributesEntry
  pub fn attributes(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // end_of_stream: optional bool
  pub fn end_of_stream(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `HttpHeadersView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpHeadersView<'_> {}

// SAFETY:
// - `HttpHeadersView` is `Send` because while its alive a `HttpHeadersMut` cannot.
// - `HttpHeadersView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpHeadersView<'_> {}

impl<'msg> ::protobuf::AsView for HttpHeadersView<'msg> {
  type Proxied = HttpHeaders;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpHeaders> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpHeadersView<'msg> {
  fn into_view<'shorter>(self) -> HttpHeadersView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpHeaders> for HttpHeadersView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpHeaders {
    let mut dst = HttpHeaders::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpHeaders> for HttpHeadersMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpHeaders {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpHeaders {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpHeadersView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpHeadersMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpHeadersMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHeaders>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpHeadersMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpHeadersMut<'msg> {
  type Message = HttpHeaders;
}

impl ::std::fmt::Debug for HttpHeadersMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHeaders>> for HttpHeadersMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHeaders>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpHeadersMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHeaders> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpHeaders {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // headers: optional message envoy.config.core.v3.HeaderMap
  pub fn has_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_headers().then(|| self.headers())
  }
  pub fn headers(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn headers_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // attributes: repeated message envoy.service.ext_proc.v3.HttpHeaders.AttributesEntry
  pub fn attributes(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn attributes_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_attributes(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // end_of_stream: optional bool
  pub fn end_of_stream(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_of_stream(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `HttpHeadersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpHeadersMut<'_> {}

// SAFETY:
// - `HttpHeadersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpHeadersMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpHeadersMut<'msg> {
  type Proxied = HttpHeaders;
  fn as_view(&self) -> ::protobuf::View<'_, HttpHeaders> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpHeadersMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpHeaders>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpHeadersMut<'msg> {
  type MutProxied = HttpHeaders;
  fn as_mut(&mut self) -> HttpHeadersMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpHeadersMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpHeadersMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpHeaders {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpHeaders> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpHeadersView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpHeadersMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // headers: optional message envoy.config.core.v3.HeaderMap
  pub fn has_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn headers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_headers().then(|| self.headers())
  }
  pub fn headers(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn headers_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_headers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // attributes: repeated message envoy.service.ext_proc.v3.HttpHeaders.AttributesEntry
  pub fn attributes(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn attributes_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_attributes(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // end_of_stream: optional bool
  pub fn end_of_stream(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_of_stream(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

}  // impl HttpHeaders

impl ::std::ops::Drop for HttpHeaders {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpHeaders {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpHeaders {
  type Proxied = Self;
  fn as_view(&self) -> HttpHeadersView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpHeaders {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpHeadersMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpHeaders {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__HttpHeaders_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__HttpHeaders_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::HeaderMap as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http_headers::AttributesEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__HttpHeaders_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpHeaders {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpHeaders {
  type Msg = HttpHeaders;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeaders> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpHeaders {
  type Msg = HttpHeaders;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeaders> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpHeadersMut<'_> {
  type Msg = HttpHeaders;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeaders> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpHeadersMut<'_> {
  type Msg = HttpHeaders;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeaders> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpHeadersView<'_> {
  type Msg = HttpHeaders;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeaders> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpHeadersMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http_headers {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__HttpHeaders__AttributesEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct AttributesEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AttributesEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_headers::envoy__service__ext_0proc__v3__HttpHeaders__AttributesEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_headers::envoy__service__ext_0proc__v3__HttpHeaders__AttributesEntry_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_headers::envoy__service__ext_0proc__v3__HttpHeaders__AttributesEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod http_headers


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__HttpBody_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpBody {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpBody>
}

impl ::protobuf::Message for HttpBody {
  type MessageView<'msg> = HttpBodyView<'msg>;
  type MessageMut<'msg> = HttpBodyMut<'msg>;
}

impl ::std::default::Default for HttpBody {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpBody {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpBody` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpBodyMut`.
unsafe impl ::std::marker::Sync for HttpBody {}

// SAFETY:
// - `HttpBody` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpBody {}

impl ::protobuf::Proxied for HttpBody {
  type View<'msg> = HttpBodyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpBody {}

impl ::protobuf::MutProxied for HttpBody {
  type Mut<'msg> = HttpBodyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpBodyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpBody>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpBodyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpBodyView<'msg> {
  type Message = HttpBody;
}

impl ::std::fmt::Debug for HttpBodyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpBodyView<'_> {
  fn default() -> HttpBodyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpBody>> for HttpBodyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpBody>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpBodyView<'msg> {

  pub fn to_owned(&self) -> HttpBody {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // body: optional bytes
  pub fn body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // end_of_stream: optional bool
  pub fn end_of_stream(self) -> bool {
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

  // end_of_stream_without_message: optional bool
  pub fn end_of_stream_without_message(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }

  // grpc_message_compressed: optional bool
  pub fn grpc_message_compressed(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `HttpBodyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpBodyView<'_> {}

// SAFETY:
// - `HttpBodyView` is `Send` because while its alive a `HttpBodyMut` cannot.
// - `HttpBodyView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpBodyView<'_> {}

impl<'msg> ::protobuf::AsView for HttpBodyView<'msg> {
  type Proxied = HttpBody;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpBody> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpBodyView<'msg> {
  fn into_view<'shorter>(self) -> HttpBodyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpBody> for HttpBodyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpBody {
    let mut dst = HttpBody::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpBody> for HttpBodyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpBody {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpBody {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpBodyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpBodyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpBodyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpBody>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpBodyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpBodyMut<'msg> {
  type Message = HttpBody;
}

impl ::std::fmt::Debug for HttpBodyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpBody>> for HttpBodyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpBody>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpBodyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpBody> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpBody {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // body: optional bytes
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // end_of_stream: optional bool
  pub fn end_of_stream(&self) -> bool {
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
  pub fn set_end_of_stream(&mut self, val: bool) {
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

  // end_of_stream_without_message: optional bool
  pub fn end_of_stream_without_message(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_of_stream_without_message(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // grpc_message_compressed: optional bool
  pub fn grpc_message_compressed(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_grpc_message_compressed(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `HttpBodyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpBodyMut<'_> {}

// SAFETY:
// - `HttpBodyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpBodyMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpBodyMut<'msg> {
  type Proxied = HttpBody;
  fn as_view(&self) -> ::protobuf::View<'_, HttpBody> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpBodyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpBody>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpBodyMut<'msg> {
  type MutProxied = HttpBody;
  fn as_mut(&mut self) -> HttpBodyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpBodyMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpBodyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpBody {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpBody> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpBodyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpBodyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // body: optional bytes
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // end_of_stream: optional bool
  pub fn end_of_stream(&self) -> bool {
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
  pub fn set_end_of_stream(&mut self, val: bool) {
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

  // end_of_stream_without_message: optional bool
  pub fn end_of_stream_without_message(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_of_stream_without_message(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // grpc_message_compressed: optional bool
  pub fn grpc_message_compressed(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_grpc_message_compressed(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}  // impl HttpBody

impl ::std::ops::Drop for HttpBody {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpBody {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpBody {
  type Proxied = Self;
  fn as_view(&self) -> HttpBodyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpBody {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpBodyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpBody {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__HttpBody_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__HttpBody_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__HttpBody_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpBody {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpBody {
  type Msg = HttpBody;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpBody> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpBody {
  type Msg = HttpBody;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpBody> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpBodyMut<'_> {
  type Msg = HttpBody;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpBody> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpBodyMut<'_> {
  type Msg = HttpBody;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpBody> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpBodyView<'_> {
  type Msg = HttpBody;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpBody> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpBodyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__HttpTrailers_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpTrailers {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpTrailers>
}

impl ::protobuf::Message for HttpTrailers {
  type MessageView<'msg> = HttpTrailersView<'msg>;
  type MessageMut<'msg> = HttpTrailersMut<'msg>;
}

impl ::std::default::Default for HttpTrailers {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpTrailers {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpTrailers` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpTrailersMut`.
unsafe impl ::std::marker::Sync for HttpTrailers {}

// SAFETY:
// - `HttpTrailers` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpTrailers {}

impl ::protobuf::Proxied for HttpTrailers {
  type View<'msg> = HttpTrailersView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpTrailers {}

impl ::protobuf::MutProxied for HttpTrailers {
  type Mut<'msg> = HttpTrailersMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpTrailersView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpTrailers>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpTrailersView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpTrailersView<'msg> {
  type Message = HttpTrailers;
}

impl ::std::fmt::Debug for HttpTrailersView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpTrailersView<'_> {
  fn default() -> HttpTrailersView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpTrailers>> for HttpTrailersView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpTrailers>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpTrailersView<'msg> {

  pub fn to_owned(&self) -> HttpTrailers {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // trailers: optional message envoy.config.core.v3.HeaderMap
  pub fn has_trailers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn trailers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg>> {
    self.has_trailers().then(|| self.trailers())
  }
  pub fn trailers(self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }

}

// SAFETY:
// - `HttpTrailersView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpTrailersView<'_> {}

// SAFETY:
// - `HttpTrailersView` is `Send` because while its alive a `HttpTrailersMut` cannot.
// - `HttpTrailersView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpTrailersView<'_> {}

impl<'msg> ::protobuf::AsView for HttpTrailersView<'msg> {
  type Proxied = HttpTrailers;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpTrailers> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpTrailersView<'msg> {
  fn into_view<'shorter>(self) -> HttpTrailersView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpTrailers> for HttpTrailersView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpTrailers {
    let mut dst = HttpTrailers::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpTrailers> for HttpTrailersMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpTrailers {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpTrailers {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpTrailersView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpTrailersMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpTrailersMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpTrailers>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpTrailersMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpTrailersMut<'msg> {
  type Message = HttpTrailers;
}

impl ::std::fmt::Debug for HttpTrailersMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpTrailers>> for HttpTrailersMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpTrailers>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpTrailersMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpTrailers> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpTrailers {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // trailers: optional message envoy.config.core.v3.HeaderMap
  pub fn has_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trailers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_trailers().then(|| self.trailers())
  }
  pub fn trailers(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn trailers_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

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
// - `HttpTrailersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpTrailersMut<'_> {}

// SAFETY:
// - `HttpTrailersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpTrailersMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpTrailersMut<'msg> {
  type Proxied = HttpTrailers;
  fn as_view(&self) -> ::protobuf::View<'_, HttpTrailers> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpTrailersMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpTrailers>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpTrailersMut<'msg> {
  type MutProxied = HttpTrailers;
  fn as_mut(&mut self) -> HttpTrailersMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpTrailersMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpTrailersMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpTrailers {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpTrailers> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpTrailersView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpTrailersMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // trailers: optional message envoy.config.core.v3.HeaderMap
  pub fn has_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trailers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_trailers().then(|| self.trailers())
  }
  pub fn trailers(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn trailers_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl HttpTrailers

impl ::std::ops::Drop for HttpTrailers {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpTrailers {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpTrailers {
  type Proxied = Self;
  fn as_view(&self) -> HttpTrailersView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpTrailers {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpTrailersMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpTrailers {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__HttpTrailers_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__HttpTrailers_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::HeaderMap as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__HttpTrailers_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpTrailers {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpTrailers {
  type Msg = HttpTrailers;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpTrailers> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpTrailers {
  type Msg = HttpTrailers;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpTrailers> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpTrailersMut<'_> {
  type Msg = HttpTrailers;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpTrailers> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpTrailersMut<'_> {
  type Msg = HttpTrailers;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpTrailers> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpTrailersView<'_> {
  type Msg = HttpTrailers;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpTrailers> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpTrailersMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__HeadersResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeadersResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeadersResponse>
}

impl ::protobuf::Message for HeadersResponse {
  type MessageView<'msg> = HeadersResponseView<'msg>;
  type MessageMut<'msg> = HeadersResponseMut<'msg>;
}

impl ::std::default::Default for HeadersResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeadersResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeadersResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `HeadersResponseMut`.
unsafe impl ::std::marker::Sync for HeadersResponse {}

// SAFETY:
// - `HeadersResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeadersResponse {}

impl ::protobuf::Proxied for HeadersResponse {
  type View<'msg> = HeadersResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeadersResponse {}

impl ::protobuf::MutProxied for HeadersResponse {
  type Mut<'msg> = HeadersResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeadersResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeadersResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeadersResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeadersResponseView<'msg> {
  type Message = HeadersResponse;
}

impl ::std::fmt::Debug for HeadersResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeadersResponseView<'_> {
  fn default() -> HeadersResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeadersResponse>> for HeadersResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeadersResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeadersResponseView<'msg> {

  pub fn to_owned(&self) -> HeadersResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // response: optional message envoy.service.ext_proc.v3.CommonResponse
  pub fn has_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn response_opt(self) -> ::std::option::Option<super::CommonResponseView<'msg>> {
    self.has_response().then(|| self.response())
  }
  pub fn response(self) -> super::CommonResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonResponseView::default())
  }

}

// SAFETY:
// - `HeadersResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeadersResponseView<'_> {}

// SAFETY:
// - `HeadersResponseView` is `Send` because while its alive a `HeadersResponseMut` cannot.
// - `HeadersResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeadersResponseView<'_> {}

impl<'msg> ::protobuf::AsView for HeadersResponseView<'msg> {
  type Proxied = HeadersResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, HeadersResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeadersResponseView<'msg> {
  fn into_view<'shorter>(self) -> HeadersResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeadersResponse> for HeadersResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeadersResponse {
    let mut dst = HeadersResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeadersResponse> for HeadersResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeadersResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeadersResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeadersResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeadersResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeadersResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeadersResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeadersResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeadersResponseMut<'msg> {
  type Message = HeadersResponse;
}

impl ::std::fmt::Debug for HeadersResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeadersResponse>> for HeadersResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeadersResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeadersResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeadersResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeadersResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // response: optional message envoy.service.ext_proc.v3.CommonResponse
  pub fn has_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn response_opt(&self) -> ::std::option::Option<super::CommonResponseView<'_>> {
    self.has_response().then(|| self.response())
  }
  pub fn response(&self) -> super::CommonResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonResponseView::default())
  }
  pub fn response_mut(&mut self) -> super::CommonResponseMut<'_> {
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
  pub fn set_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::CommonResponse>) {

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
// - `HeadersResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeadersResponseMut<'_> {}

// SAFETY:
// - `HeadersResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeadersResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for HeadersResponseMut<'msg> {
  type Proxied = HeadersResponse;
  fn as_view(&self) -> ::protobuf::View<'_, HeadersResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeadersResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeadersResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeadersResponseMut<'msg> {
  type MutProxied = HeadersResponse;
  fn as_mut(&mut self) -> HeadersResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeadersResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> HeadersResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeadersResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeadersResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeadersResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeadersResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // response: optional message envoy.service.ext_proc.v3.CommonResponse
  pub fn has_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn response_opt(&self) -> ::std::option::Option<super::CommonResponseView<'_>> {
    self.has_response().then(|| self.response())
  }
  pub fn response(&self) -> super::CommonResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonResponseView::default())
  }
  pub fn response_mut(&mut self) -> super::CommonResponseMut<'_> {
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
  pub fn set_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::CommonResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl HeadersResponse

impl ::std::ops::Drop for HeadersResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeadersResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeadersResponse {
  type Proxied = Self;
  fn as_view(&self) -> HeadersResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeadersResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeadersResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeadersResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__HeadersResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__HeadersResponse_msg_init.0, &[<super::CommonResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__HeadersResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeadersResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeadersResponse {
  type Msg = HeadersResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeadersResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeadersResponse {
  type Msg = HeadersResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeadersResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeadersResponseMut<'_> {
  type Msg = HeadersResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeadersResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeadersResponseMut<'_> {
  type Msg = HeadersResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeadersResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeadersResponseView<'_> {
  type Msg = HeadersResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeadersResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeadersResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__BodyResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BodyResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BodyResponse>
}

impl ::protobuf::Message for BodyResponse {
  type MessageView<'msg> = BodyResponseView<'msg>;
  type MessageMut<'msg> = BodyResponseMut<'msg>;
}

impl ::std::default::Default for BodyResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BodyResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BodyResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `BodyResponseMut`.
unsafe impl ::std::marker::Sync for BodyResponse {}

// SAFETY:
// - `BodyResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BodyResponse {}

impl ::protobuf::Proxied for BodyResponse {
  type View<'msg> = BodyResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BodyResponse {}

impl ::protobuf::MutProxied for BodyResponse {
  type Mut<'msg> = BodyResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BodyResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BodyResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BodyResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BodyResponseView<'msg> {
  type Message = BodyResponse;
}

impl ::std::fmt::Debug for BodyResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BodyResponseView<'_> {
  fn default() -> BodyResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BodyResponse>> for BodyResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BodyResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BodyResponseView<'msg> {

  pub fn to_owned(&self) -> BodyResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // response: optional message envoy.service.ext_proc.v3.CommonResponse
  pub fn has_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn response_opt(self) -> ::std::option::Option<super::CommonResponseView<'msg>> {
    self.has_response().then(|| self.response())
  }
  pub fn response(self) -> super::CommonResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonResponseView::default())
  }

}

// SAFETY:
// - `BodyResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BodyResponseView<'_> {}

// SAFETY:
// - `BodyResponseView` is `Send` because while its alive a `BodyResponseMut` cannot.
// - `BodyResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for BodyResponseView<'_> {}

impl<'msg> ::protobuf::AsView for BodyResponseView<'msg> {
  type Proxied = BodyResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, BodyResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BodyResponseView<'msg> {
  fn into_view<'shorter>(self) -> BodyResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BodyResponse> for BodyResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BodyResponse {
    let mut dst = BodyResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BodyResponse> for BodyResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BodyResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BodyResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BodyResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BodyResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BodyResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BodyResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BodyResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BodyResponseMut<'msg> {
  type Message = BodyResponse;
}

impl ::std::fmt::Debug for BodyResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BodyResponse>> for BodyResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BodyResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BodyResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BodyResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BodyResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // response: optional message envoy.service.ext_proc.v3.CommonResponse
  pub fn has_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn response_opt(&self) -> ::std::option::Option<super::CommonResponseView<'_>> {
    self.has_response().then(|| self.response())
  }
  pub fn response(&self) -> super::CommonResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonResponseView::default())
  }
  pub fn response_mut(&mut self) -> super::CommonResponseMut<'_> {
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
  pub fn set_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::CommonResponse>) {

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
// - `BodyResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BodyResponseMut<'_> {}

// SAFETY:
// - `BodyResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BodyResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for BodyResponseMut<'msg> {
  type Proxied = BodyResponse;
  fn as_view(&self) -> ::protobuf::View<'_, BodyResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BodyResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BodyResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BodyResponseMut<'msg> {
  type MutProxied = BodyResponse;
  fn as_mut(&mut self) -> BodyResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BodyResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> BodyResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BodyResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BodyResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BodyResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BodyResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // response: optional message envoy.service.ext_proc.v3.CommonResponse
  pub fn has_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn response_opt(&self) -> ::std::option::Option<super::CommonResponseView<'_>> {
    self.has_response().then(|| self.response())
  }
  pub fn response(&self) -> super::CommonResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonResponseView::default())
  }
  pub fn response_mut(&mut self) -> super::CommonResponseMut<'_> {
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
  pub fn set_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::CommonResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl BodyResponse

impl ::std::ops::Drop for BodyResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BodyResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BodyResponse {
  type Proxied = Self;
  fn as_view(&self) -> BodyResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BodyResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BodyResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BodyResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__BodyResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__BodyResponse_msg_init.0, &[<super::CommonResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__BodyResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BodyResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BodyResponse {
  type Msg = BodyResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BodyResponse {
  type Msg = BodyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BodyResponseMut<'_> {
  type Msg = BodyResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BodyResponseMut<'_> {
  type Msg = BodyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BodyResponseView<'_> {
  type Msg = BodyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BodyResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__TrailersResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TrailersResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TrailersResponse>
}

impl ::protobuf::Message for TrailersResponse {
  type MessageView<'msg> = TrailersResponseView<'msg>;
  type MessageMut<'msg> = TrailersResponseMut<'msg>;
}

impl ::std::default::Default for TrailersResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TrailersResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TrailersResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `TrailersResponseMut`.
unsafe impl ::std::marker::Sync for TrailersResponse {}

// SAFETY:
// - `TrailersResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TrailersResponse {}

impl ::protobuf::Proxied for TrailersResponse {
  type View<'msg> = TrailersResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TrailersResponse {}

impl ::protobuf::MutProxied for TrailersResponse {
  type Mut<'msg> = TrailersResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TrailersResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TrailersResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TrailersResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TrailersResponseView<'msg> {
  type Message = TrailersResponse;
}

impl ::std::fmt::Debug for TrailersResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TrailersResponseView<'_> {
  fn default() -> TrailersResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TrailersResponse>> for TrailersResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TrailersResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TrailersResponseView<'msg> {

  pub fn to_owned(&self) -> TrailersResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // header_mutation: optional message envoy.service.ext_proc.v3.HeaderMutation
  pub fn has_header_mutation(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn header_mutation_opt(self) -> ::std::option::Option<super::HeaderMutationView<'msg>> {
    self.has_header_mutation().then(|| self.header_mutation())
  }
  pub fn header_mutation(self) -> super::HeaderMutationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderMutationView::default())
  }

}

// SAFETY:
// - `TrailersResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TrailersResponseView<'_> {}

// SAFETY:
// - `TrailersResponseView` is `Send` because while its alive a `TrailersResponseMut` cannot.
// - `TrailersResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for TrailersResponseView<'_> {}

impl<'msg> ::protobuf::AsView for TrailersResponseView<'msg> {
  type Proxied = TrailersResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, TrailersResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TrailersResponseView<'msg> {
  fn into_view<'shorter>(self) -> TrailersResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TrailersResponse> for TrailersResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TrailersResponse {
    let mut dst = TrailersResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TrailersResponse> for TrailersResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TrailersResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TrailersResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TrailersResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TrailersResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TrailersResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TrailersResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TrailersResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TrailersResponseMut<'msg> {
  type Message = TrailersResponse;
}

impl ::std::fmt::Debug for TrailersResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TrailersResponse>> for TrailersResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TrailersResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TrailersResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TrailersResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TrailersResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // header_mutation: optional message envoy.service.ext_proc.v3.HeaderMutation
  pub fn has_header_mutation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header_mutation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_mutation_opt(&self) -> ::std::option::Option<super::HeaderMutationView<'_>> {
    self.has_header_mutation().then(|| self.header_mutation())
  }
  pub fn header_mutation(&self) -> super::HeaderMutationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderMutationView::default())
  }
  pub fn header_mutation_mut(&mut self) -> super::HeaderMutationMut<'_> {
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
  pub fn set_header_mutation(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderMutation>) {

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
// - `TrailersResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TrailersResponseMut<'_> {}

// SAFETY:
// - `TrailersResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TrailersResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for TrailersResponseMut<'msg> {
  type Proxied = TrailersResponse;
  fn as_view(&self) -> ::protobuf::View<'_, TrailersResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TrailersResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TrailersResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TrailersResponseMut<'msg> {
  type MutProxied = TrailersResponse;
  fn as_mut(&mut self) -> TrailersResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TrailersResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> TrailersResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TrailersResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TrailersResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TrailersResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TrailersResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // header_mutation: optional message envoy.service.ext_proc.v3.HeaderMutation
  pub fn has_header_mutation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header_mutation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_mutation_opt(&self) -> ::std::option::Option<super::HeaderMutationView<'_>> {
    self.has_header_mutation().then(|| self.header_mutation())
  }
  pub fn header_mutation(&self) -> super::HeaderMutationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderMutationView::default())
  }
  pub fn header_mutation_mut(&mut self) -> super::HeaderMutationMut<'_> {
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
  pub fn set_header_mutation(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderMutation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl TrailersResponse

impl ::std::ops::Drop for TrailersResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TrailersResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TrailersResponse {
  type Proxied = Self;
  fn as_view(&self) -> TrailersResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TrailersResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TrailersResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TrailersResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__TrailersResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__TrailersResponse_msg_init.0, &[<super::HeaderMutation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__TrailersResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TrailersResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TrailersResponse {
  type Msg = TrailersResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrailersResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TrailersResponse {
  type Msg = TrailersResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrailersResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TrailersResponseMut<'_> {
  type Msg = TrailersResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrailersResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TrailersResponseMut<'_> {
  type Msg = TrailersResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrailersResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TrailersResponseView<'_> {
  type Msg = TrailersResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrailersResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TrailersResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__StreamedImmediateResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StreamedImmediateResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StreamedImmediateResponse>
}

impl ::protobuf::Message for StreamedImmediateResponse {
  type MessageView<'msg> = StreamedImmediateResponseView<'msg>;
  type MessageMut<'msg> = StreamedImmediateResponseMut<'msg>;
}

impl ::std::default::Default for StreamedImmediateResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StreamedImmediateResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StreamedImmediateResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `StreamedImmediateResponseMut`.
unsafe impl ::std::marker::Sync for StreamedImmediateResponse {}

// SAFETY:
// - `StreamedImmediateResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StreamedImmediateResponse {}

impl ::protobuf::Proxied for StreamedImmediateResponse {
  type View<'msg> = StreamedImmediateResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StreamedImmediateResponse {}

impl ::protobuf::MutProxied for StreamedImmediateResponse {
  type Mut<'msg> = StreamedImmediateResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StreamedImmediateResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StreamedImmediateResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamedImmediateResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StreamedImmediateResponseView<'msg> {
  type Message = StreamedImmediateResponse;
}

impl ::std::fmt::Debug for StreamedImmediateResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StreamedImmediateResponseView<'_> {
  fn default() -> StreamedImmediateResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StreamedImmediateResponse>> for StreamedImmediateResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StreamedImmediateResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamedImmediateResponseView<'msg> {

  pub fn to_owned(&self) -> StreamedImmediateResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // headers_response: optional message envoy.service.ext_proc.v3.HttpHeaders
  pub fn has_headers_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn headers_response_opt(self) -> ::std::option::Option<super::HttpHeadersView<'msg>> {
    self.has_headers_response().then(|| self.headers_response())
  }
  pub fn headers_response(self) -> super::HttpHeadersView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersView::default())
  }

  // body_response: optional message envoy.service.ext_proc.v3.StreamedBodyResponse
  pub fn has_body_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn body_response_opt(self) -> ::std::option::Option<super::StreamedBodyResponseView<'msg>> {
    self.has_body_response().then(|| self.body_response())
  }
  pub fn body_response(self) -> super::StreamedBodyResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StreamedBodyResponseView::default())
  }

  // trailers_response: optional message envoy.config.core.v3.HeaderMap
  pub fn has_trailers_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn trailers_response_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg>> {
    self.has_trailers_response().then(|| self.trailers_response())
  }
  pub fn trailers_response(self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }

  pub fn response(self) -> super::streamed_immediate_response::ResponseOneof<'msg> {
    match self.response_case() {
      super::streamed_immediate_response::ResponseCase::HeadersResponse =>
          super::streamed_immediate_response::ResponseOneof::HeadersResponse(self.headers_response()),
      super::streamed_immediate_response::ResponseCase::BodyResponse =>
          super::streamed_immediate_response::ResponseOneof::BodyResponse(self.body_response()),
      super::streamed_immediate_response::ResponseCase::TrailersResponse =>
          super::streamed_immediate_response::ResponseOneof::TrailersResponse(self.trailers_response()),
      _ => super::streamed_immediate_response::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(self) -> super::streamed_immediate_response::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::streamed_immediate_response::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StreamedImmediateResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StreamedImmediateResponseView<'_> {}

// SAFETY:
// - `StreamedImmediateResponseView` is `Send` because while its alive a `StreamedImmediateResponseMut` cannot.
// - `StreamedImmediateResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for StreamedImmediateResponseView<'_> {}

impl<'msg> ::protobuf::AsView for StreamedImmediateResponseView<'msg> {
  type Proxied = StreamedImmediateResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, StreamedImmediateResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamedImmediateResponseView<'msg> {
  fn into_view<'shorter>(self) -> StreamedImmediateResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StreamedImmediateResponse> for StreamedImmediateResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StreamedImmediateResponse {
    let mut dst = StreamedImmediateResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StreamedImmediateResponse> for StreamedImmediateResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StreamedImmediateResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StreamedImmediateResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StreamedImmediateResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StreamedImmediateResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StreamedImmediateResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamedImmediateResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamedImmediateResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StreamedImmediateResponseMut<'msg> {
  type Message = StreamedImmediateResponse;
}

impl ::std::fmt::Debug for StreamedImmediateResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StreamedImmediateResponse>> for StreamedImmediateResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamedImmediateResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamedImmediateResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamedImmediateResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StreamedImmediateResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // headers_response: optional message envoy.service.ext_proc.v3.HttpHeaders
  pub fn has_headers_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_headers_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn headers_response_opt(&self) -> ::std::option::Option<super::HttpHeadersView<'_>> {
    self.has_headers_response().then(|| self.headers_response())
  }
  pub fn headers_response(&self) -> super::HttpHeadersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersView::default())
  }
  pub fn headers_response_mut(&mut self) -> super::HttpHeadersMut<'_> {
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
  pub fn set_headers_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeaders>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // body_response: optional message envoy.service.ext_proc.v3.StreamedBodyResponse
  pub fn has_body_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_body_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn body_response_opt(&self) -> ::std::option::Option<super::StreamedBodyResponseView<'_>> {
    self.has_body_response().then(|| self.body_response())
  }
  pub fn body_response(&self) -> super::StreamedBodyResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StreamedBodyResponseView::default())
  }
  pub fn body_response_mut(&mut self) -> super::StreamedBodyResponseMut<'_> {
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
  pub fn set_body_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::StreamedBodyResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // trailers_response: optional message envoy.config.core.v3.HeaderMap
  pub fn has_trailers_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_trailers_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn trailers_response_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_trailers_response().then(|| self.trailers_response())
  }
  pub fn trailers_response(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn trailers_response_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_trailers_response(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn response(&self) -> super::streamed_immediate_response::ResponseOneof<'_> {
    match &self.response_case() {
      super::streamed_immediate_response::ResponseCase::HeadersResponse =>
          super::streamed_immediate_response::ResponseOneof::HeadersResponse(self.headers_response()),
      super::streamed_immediate_response::ResponseCase::BodyResponse =>
          super::streamed_immediate_response::ResponseOneof::BodyResponse(self.body_response()),
      super::streamed_immediate_response::ResponseCase::TrailersResponse =>
          super::streamed_immediate_response::ResponseOneof::TrailersResponse(self.trailers_response()),
      _ => super::streamed_immediate_response::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(&self) -> super::streamed_immediate_response::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::streamed_immediate_response::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StreamedImmediateResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StreamedImmediateResponseMut<'_> {}

// SAFETY:
// - `StreamedImmediateResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StreamedImmediateResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for StreamedImmediateResponseMut<'msg> {
  type Proxied = StreamedImmediateResponse;
  fn as_view(&self) -> ::protobuf::View<'_, StreamedImmediateResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamedImmediateResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StreamedImmediateResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StreamedImmediateResponseMut<'msg> {
  type MutProxied = StreamedImmediateResponse;
  fn as_mut(&mut self) -> StreamedImmediateResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StreamedImmediateResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> StreamedImmediateResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StreamedImmediateResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StreamedImmediateResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StreamedImmediateResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StreamedImmediateResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // headers_response: optional message envoy.service.ext_proc.v3.HttpHeaders
  pub fn has_headers_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_headers_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn headers_response_opt(&self) -> ::std::option::Option<super::HttpHeadersView<'_>> {
    self.has_headers_response().then(|| self.headers_response())
  }
  pub fn headers_response(&self) -> super::HttpHeadersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersView::default())
  }
  pub fn headers_response_mut(&mut self) -> super::HttpHeadersMut<'_> {
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
  pub fn set_headers_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeaders>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // body_response: optional message envoy.service.ext_proc.v3.StreamedBodyResponse
  pub fn has_body_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_body_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn body_response_opt(&self) -> ::std::option::Option<super::StreamedBodyResponseView<'_>> {
    self.has_body_response().then(|| self.body_response())
  }
  pub fn body_response(&self) -> super::StreamedBodyResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StreamedBodyResponseView::default())
  }
  pub fn body_response_mut(&mut self) -> super::StreamedBodyResponseMut<'_> {
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
  pub fn set_body_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::StreamedBodyResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // trailers_response: optional message envoy.config.core.v3.HeaderMap
  pub fn has_trailers_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_trailers_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn trailers_response_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_trailers_response().then(|| self.trailers_response())
  }
  pub fn trailers_response(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn trailers_response_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_trailers_response(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn response(&self) -> super::streamed_immediate_response::ResponseOneof<'_> {
    match &self.response_case() {
      super::streamed_immediate_response::ResponseCase::HeadersResponse =>
          super::streamed_immediate_response::ResponseOneof::HeadersResponse(self.headers_response()),
      super::streamed_immediate_response::ResponseCase::BodyResponse =>
          super::streamed_immediate_response::ResponseOneof::BodyResponse(self.body_response()),
      super::streamed_immediate_response::ResponseCase::TrailersResponse =>
          super::streamed_immediate_response::ResponseOneof::TrailersResponse(self.trailers_response()),
      _ => super::streamed_immediate_response::ResponseOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn response_case(&self) -> super::streamed_immediate_response::ResponseCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::streamed_immediate_response::ResponseCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl StreamedImmediateResponse

impl ::std::ops::Drop for StreamedImmediateResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StreamedImmediateResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StreamedImmediateResponse {
  type Proxied = Self;
  fn as_view(&self) -> StreamedImmediateResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StreamedImmediateResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StreamedImmediateResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StreamedImmediateResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__StreamedImmediateResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^!|#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__StreamedImmediateResponse_msg_init.0, &[<super::HttpHeaders as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::StreamedBodyResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderMap as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__StreamedImmediateResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StreamedImmediateResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StreamedImmediateResponse {
  type Msg = StreamedImmediateResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedImmediateResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamedImmediateResponse {
  type Msg = StreamedImmediateResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedImmediateResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StreamedImmediateResponseMut<'_> {
  type Msg = StreamedImmediateResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedImmediateResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamedImmediateResponseMut<'_> {
  type Msg = StreamedImmediateResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedImmediateResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamedImmediateResponseView<'_> {
  type Msg = StreamedImmediateResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedImmediateResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StreamedImmediateResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod streamed_immediate_response {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ResponseOneof<'msg> {
  HeadersResponse(::protobuf::View<'msg, super::super::HttpHeaders>) = 1,
  BodyResponse(::protobuf::View<'msg, super::super::StreamedBodyResponse>) = 2,
  TrailersResponse(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ResponseCase {
  HeadersResponse = 1,
  BodyResponse = 2,
  TrailersResponse = 3,

  not_set = 0
}

impl ResponseCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ResponseCase> {
    match v {
      0 => Some(ResponseCase::not_set),
      1 => Some(ResponseCase::HeadersResponse),
      2 => Some(ResponseCase::BodyResponse),
      3 => Some(ResponseCase::TrailersResponse),
      _ => None
    }
  }
}
}  // pub mod streamed_immediate_response


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__CommonResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CommonResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CommonResponse>
}

impl ::protobuf::Message for CommonResponse {
  type MessageView<'msg> = CommonResponseView<'msg>;
  type MessageMut<'msg> = CommonResponseMut<'msg>;
}

impl ::std::default::Default for CommonResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CommonResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CommonResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `CommonResponseMut`.
unsafe impl ::std::marker::Sync for CommonResponse {}

// SAFETY:
// - `CommonResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CommonResponse {}

impl ::protobuf::Proxied for CommonResponse {
  type View<'msg> = CommonResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CommonResponse {}

impl ::protobuf::MutProxied for CommonResponse {
  type Mut<'msg> = CommonResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CommonResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommonResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommonResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CommonResponseView<'msg> {
  type Message = CommonResponse;
}

impl ::std::fmt::Debug for CommonResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CommonResponseView<'_> {
  fn default() -> CommonResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CommonResponse>> for CommonResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommonResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommonResponseView<'msg> {

  pub fn to_owned(&self) -> CommonResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // status: optional enum envoy.service.ext_proc.v3.CommonResponse.ResponseStatus
  pub fn status(self) -> super::common_response::ResponseStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::common_response::ResponseStatus::Continue).into()
      ).try_into().unwrap()
    }
  }

  // header_mutation: optional message envoy.service.ext_proc.v3.HeaderMutation
  pub fn has_header_mutation(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn header_mutation_opt(self) -> ::std::option::Option<super::HeaderMutationView<'msg>> {
    self.has_header_mutation().then(|| self.header_mutation())
  }
  pub fn header_mutation(self) -> super::HeaderMutationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderMutationView::default())
  }

  // body_mutation: optional message envoy.service.ext_proc.v3.BodyMutation
  pub fn has_body_mutation(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn body_mutation_opt(self) -> ::std::option::Option<super::BodyMutationView<'msg>> {
    self.has_body_mutation().then(|| self.body_mutation())
  }
  pub fn body_mutation(self) -> super::BodyMutationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BodyMutationView::default())
  }

  // trailers: optional message envoy.config.core.v3.HeaderMap
  pub fn has_trailers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn trailers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg>> {
    self.has_trailers().then(|| self.trailers())
  }
  pub fn trailers(self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }

  // clear_route_cache: optional bool
  pub fn clear_route_cache(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `CommonResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CommonResponseView<'_> {}

// SAFETY:
// - `CommonResponseView` is `Send` because while its alive a `CommonResponseMut` cannot.
// - `CommonResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for CommonResponseView<'_> {}

impl<'msg> ::protobuf::AsView for CommonResponseView<'msg> {
  type Proxied = CommonResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, CommonResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommonResponseView<'msg> {
  fn into_view<'shorter>(self) -> CommonResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CommonResponse> for CommonResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommonResponse {
    let mut dst = CommonResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CommonResponse> for CommonResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommonResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CommonResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CommonResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CommonResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CommonResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommonResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommonResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CommonResponseMut<'msg> {
  type Message = CommonResponse;
}

impl ::std::fmt::Debug for CommonResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CommonResponse>> for CommonResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommonResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommonResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CommonResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CommonResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // status: optional enum envoy.service.ext_proc.v3.CommonResponse.ResponseStatus
  pub fn status(&self) -> super::common_response::ResponseStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::common_response::ResponseStatus::Continue).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: super::common_response::ResponseStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // header_mutation: optional message envoy.service.ext_proc.v3.HeaderMutation
  pub fn has_header_mutation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_header_mutation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn header_mutation_opt(&self) -> ::std::option::Option<super::HeaderMutationView<'_>> {
    self.has_header_mutation().then(|| self.header_mutation())
  }
  pub fn header_mutation(&self) -> super::HeaderMutationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderMutationView::default())
  }
  pub fn header_mutation_mut(&mut self) -> super::HeaderMutationMut<'_> {
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
  pub fn set_header_mutation(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderMutation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // body_mutation: optional message envoy.service.ext_proc.v3.BodyMutation
  pub fn has_body_mutation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_body_mutation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn body_mutation_opt(&self) -> ::std::option::Option<super::BodyMutationView<'_>> {
    self.has_body_mutation().then(|| self.body_mutation())
  }
  pub fn body_mutation(&self) -> super::BodyMutationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BodyMutationView::default())
  }
  pub fn body_mutation_mut(&mut self) -> super::BodyMutationMut<'_> {
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
  pub fn set_body_mutation(&mut self,
    val: impl ::protobuf::IntoProxied<super::BodyMutation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // trailers: optional message envoy.config.core.v3.HeaderMap
  pub fn has_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn trailers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_trailers().then(|| self.trailers())
  }
  pub fn trailers(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn trailers_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // clear_route_cache: optional bool
  pub fn clear_route_cache(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_clear_route_cache(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}

// SAFETY:
// - `CommonResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CommonResponseMut<'_> {}

// SAFETY:
// - `CommonResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CommonResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for CommonResponseMut<'msg> {
  type Proxied = CommonResponse;
  fn as_view(&self) -> ::protobuf::View<'_, CommonResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommonResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CommonResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CommonResponseMut<'msg> {
  type MutProxied = CommonResponse;
  fn as_mut(&mut self) -> CommonResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CommonResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> CommonResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CommonResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CommonResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CommonResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CommonResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // status: optional enum envoy.service.ext_proc.v3.CommonResponse.ResponseStatus
  pub fn status(&self) -> super::common_response::ResponseStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::common_response::ResponseStatus::Continue).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: super::common_response::ResponseStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // header_mutation: optional message envoy.service.ext_proc.v3.HeaderMutation
  pub fn has_header_mutation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_header_mutation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn header_mutation_opt(&self) -> ::std::option::Option<super::HeaderMutationView<'_>> {
    self.has_header_mutation().then(|| self.header_mutation())
  }
  pub fn header_mutation(&self) -> super::HeaderMutationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderMutationView::default())
  }
  pub fn header_mutation_mut(&mut self) -> super::HeaderMutationMut<'_> {
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
  pub fn set_header_mutation(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderMutation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // body_mutation: optional message envoy.service.ext_proc.v3.BodyMutation
  pub fn has_body_mutation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_body_mutation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn body_mutation_opt(&self) -> ::std::option::Option<super::BodyMutationView<'_>> {
    self.has_body_mutation().then(|| self.body_mutation())
  }
  pub fn body_mutation(&self) -> super::BodyMutationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BodyMutationView::default())
  }
  pub fn body_mutation_mut(&mut self) -> super::BodyMutationMut<'_> {
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
  pub fn set_body_mutation(&mut self,
    val: impl ::protobuf::IntoProxied<super::BodyMutation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // trailers: optional message envoy.config.core.v3.HeaderMap
  pub fn has_trailers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_trailers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn trailers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_>> {
    self.has_trailers().then(|| self.trailers())
  }
  pub fn trailers(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderMapView::default())
  }
  pub fn trailers_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderMapMut<'_> {
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
  pub fn set_trailers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // clear_route_cache: optional bool
  pub fn clear_route_cache(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_clear_route_cache(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}  // impl CommonResponse

impl ::std::ops::Drop for CommonResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CommonResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CommonResponse {
  type Proxied = Self;
  fn as_view(&self) -> CommonResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CommonResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CommonResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CommonResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__CommonResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P333/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__CommonResponse_msg_init.0, &[<super::HeaderMutation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BodyMutation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::HeaderMap as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__CommonResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommonResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommonResponse {
  type Msg = CommonResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommonResponse {
  type Msg = CommonResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommonResponseMut<'_> {
  type Msg = CommonResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommonResponseMut<'_> {
  type Msg = CommonResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommonResponseView<'_> {
  type Msg = CommonResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommonResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod common_response {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResponseStatus(i32);

#[allow(non_upper_case_globals)]
impl ResponseStatus {
  pub const Continue: ResponseStatus = ResponseStatus(0);
  pub const ContinueAndReplace: ResponseStatus = ResponseStatus(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Continue",
      1 => "ContinueAndReplace",
      _ => return None
    })
  }
}

impl ::std::convert::From<ResponseStatus> for i32 {
  fn from(val: ResponseStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ResponseStatus {
  fn from(val: i32) -> ResponseStatus {
    Self(val)
  }
}

impl ::std::default::Default for ResponseStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ResponseStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ResponseStatus::{}", constant_name)
    } else {
      write!(f, "ResponseStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ResponseStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ResponseStatus {}

impl ::protobuf::Proxied for ResponseStatus {
  type View<'a> = ResponseStatus;
}

impl ::protobuf::AsView for ResponseStatus {
  type Proxied = ResponseStatus;

  fn as_view(&self) -> ResponseStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResponseStatus {
  fn into_view<'shorter>(self) -> ResponseStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ResponseStatus {
  const NAME: &'static str = "ResponseStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for ResponseStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod common_response


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__ImmediateResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ImmediateResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ImmediateResponse>
}

impl ::protobuf::Message for ImmediateResponse {
  type MessageView<'msg> = ImmediateResponseView<'msg>;
  type MessageMut<'msg> = ImmediateResponseMut<'msg>;
}

impl ::std::default::Default for ImmediateResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ImmediateResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ImmediateResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ImmediateResponseMut`.
unsafe impl ::std::marker::Sync for ImmediateResponse {}

// SAFETY:
// - `ImmediateResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ImmediateResponse {}

impl ::protobuf::Proxied for ImmediateResponse {
  type View<'msg> = ImmediateResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ImmediateResponse {}

impl ::protobuf::MutProxied for ImmediateResponse {
  type Mut<'msg> = ImmediateResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ImmediateResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ImmediateResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ImmediateResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ImmediateResponseView<'msg> {
  type Message = ImmediateResponse;
}

impl ::std::fmt::Debug for ImmediateResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ImmediateResponseView<'_> {
  fn default() -> ImmediateResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ImmediateResponse>> for ImmediateResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ImmediateResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ImmediateResponseView<'msg> {

  pub fn to_owned(&self) -> ImmediateResponse {
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

  // headers: optional message envoy.service.ext_proc.v3.HeaderMutation
  pub fn has_headers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn headers_opt(self) -> ::std::option::Option<super::HeaderMutationView<'msg>> {
    self.has_headers().then(|| self.headers())
  }
  pub fn headers(self) -> super::HeaderMutationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderMutationView::default())
  }

  // body: optional bytes
  pub fn body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // grpc_status: optional message envoy.service.ext_proc.v3.GrpcStatus
  pub fn has_grpc_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn grpc_status_opt(self) -> ::std::option::Option<super::GrpcStatusView<'msg>> {
    self.has_grpc_status().then(|| self.grpc_status())
  }
  pub fn grpc_status(self) -> super::GrpcStatusView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GrpcStatusView::default())
  }

  // details: optional string
  pub fn details(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ImmediateResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ImmediateResponseView<'_> {}

// SAFETY:
// - `ImmediateResponseView` is `Send` because while its alive a `ImmediateResponseMut` cannot.
// - `ImmediateResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for ImmediateResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ImmediateResponseView<'msg> {
  type Proxied = ImmediateResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ImmediateResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ImmediateResponseView<'msg> {
  fn into_view<'shorter>(self) -> ImmediateResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ImmediateResponse> for ImmediateResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ImmediateResponse {
    let mut dst = ImmediateResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ImmediateResponse> for ImmediateResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ImmediateResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ImmediateResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ImmediateResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ImmediateResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ImmediateResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ImmediateResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ImmediateResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ImmediateResponseMut<'msg> {
  type Message = ImmediateResponse;
}

impl ::std::fmt::Debug for ImmediateResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ImmediateResponse>> for ImmediateResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ImmediateResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ImmediateResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ImmediateResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ImmediateResponse {
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

  // headers: optional message envoy.service.ext_proc.v3.HeaderMutation
  pub fn has_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn headers_opt(&self) -> ::std::option::Option<super::HeaderMutationView<'_>> {
    self.has_headers().then(|| self.headers())
  }
  pub fn headers(&self) -> super::HeaderMutationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderMutationView::default())
  }
  pub fn headers_mut(&mut self) -> super::HeaderMutationMut<'_> {
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
  pub fn set_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderMutation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // body: optional bytes
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // grpc_status: optional message envoy.service.ext_proc.v3.GrpcStatus
  pub fn has_grpc_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_grpc_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn grpc_status_opt(&self) -> ::std::option::Option<super::GrpcStatusView<'_>> {
    self.has_grpc_status().then(|| self.grpc_status())
  }
  pub fn grpc_status(&self) -> super::GrpcStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GrpcStatusView::default())
  }
  pub fn grpc_status_mut(&mut self) -> super::GrpcStatusMut<'_> {
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
  pub fn set_grpc_status(&mut self,
    val: impl ::protobuf::IntoProxied<super::GrpcStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // details: optional string
  pub fn details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

}

// SAFETY:
// - `ImmediateResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ImmediateResponseMut<'_> {}

// SAFETY:
// - `ImmediateResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ImmediateResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ImmediateResponseMut<'msg> {
  type Proxied = ImmediateResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ImmediateResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ImmediateResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ImmediateResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ImmediateResponseMut<'msg> {
  type MutProxied = ImmediateResponse;
  fn as_mut(&mut self) -> ImmediateResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ImmediateResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ImmediateResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ImmediateResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ImmediateResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ImmediateResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ImmediateResponseMut<'_> {
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

  // headers: optional message envoy.service.ext_proc.v3.HeaderMutation
  pub fn has_headers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_headers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn headers_opt(&self) -> ::std::option::Option<super::HeaderMutationView<'_>> {
    self.has_headers().then(|| self.headers())
  }
  pub fn headers(&self) -> super::HeaderMutationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderMutationView::default())
  }
  pub fn headers_mut(&mut self) -> super::HeaderMutationMut<'_> {
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
  pub fn set_headers(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderMutation>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // body: optional bytes
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // grpc_status: optional message envoy.service.ext_proc.v3.GrpcStatus
  pub fn has_grpc_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_grpc_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn grpc_status_opt(&self) -> ::std::option::Option<super::GrpcStatusView<'_>> {
    self.has_grpc_status().then(|| self.grpc_status())
  }
  pub fn grpc_status(&self) -> super::GrpcStatusView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GrpcStatusView::default())
  }
  pub fn grpc_status_mut(&mut self) -> super::GrpcStatusMut<'_> {
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
  pub fn set_grpc_status(&mut self,
    val: impl ::protobuf::IntoProxied<super::GrpcStatus>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // details: optional string
  pub fn details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

}  // impl ImmediateResponse

impl ::std::ops::Drop for ImmediateResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ImmediateResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ImmediateResponse {
  type Proxied = Self;
  fn as_view(&self) -> ImmediateResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ImmediateResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ImmediateResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ImmediateResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__ImmediateResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$330P31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__ImmediateResponse_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::http_status::HttpStatus as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HeaderMutation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::GrpcStatus as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__ImmediateResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ImmediateResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ImmediateResponse {
  type Msg = ImmediateResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ImmediateResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ImmediateResponse {
  type Msg = ImmediateResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ImmediateResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ImmediateResponseMut<'_> {
  type Msg = ImmediateResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ImmediateResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ImmediateResponseMut<'_> {
  type Msg = ImmediateResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ImmediateResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ImmediateResponseView<'_> {
  type Msg = ImmediateResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ImmediateResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ImmediateResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__GrpcStatus_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GrpcStatus {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GrpcStatus>
}

impl ::protobuf::Message for GrpcStatus {
  type MessageView<'msg> = GrpcStatusView<'msg>;
  type MessageMut<'msg> = GrpcStatusMut<'msg>;
}

impl ::std::default::Default for GrpcStatus {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GrpcStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GrpcStatus` is `Sync` because it does not implement interior mutability.
//    Neither does `GrpcStatusMut`.
unsafe impl ::std::marker::Sync for GrpcStatus {}

// SAFETY:
// - `GrpcStatus` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GrpcStatus {}

impl ::protobuf::Proxied for GrpcStatus {
  type View<'msg> = GrpcStatusView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GrpcStatus {}

impl ::protobuf::MutProxied for GrpcStatus {
  type Mut<'msg> = GrpcStatusMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GrpcStatusView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcStatus>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcStatusView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GrpcStatusView<'msg> {
  type Message = GrpcStatus;
}

impl ::std::fmt::Debug for GrpcStatusView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GrpcStatusView<'_> {
  fn default() -> GrpcStatusView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcStatus>> for GrpcStatusView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcStatus>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcStatusView<'msg> {

  pub fn to_owned(&self) -> GrpcStatus {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // status: optional uint32
  pub fn status(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `GrpcStatusView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GrpcStatusView<'_> {}

// SAFETY:
// - `GrpcStatusView` is `Send` because while its alive a `GrpcStatusMut` cannot.
// - `GrpcStatusView` does not use thread-local data.
unsafe impl ::std::marker::Send for GrpcStatusView<'_> {}

impl<'msg> ::protobuf::AsView for GrpcStatusView<'msg> {
  type Proxied = GrpcStatus;
  fn as_view(&self) -> ::protobuf::View<'msg, GrpcStatus> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcStatusView<'msg> {
  fn into_view<'shorter>(self) -> GrpcStatusView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcStatus> for GrpcStatusView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcStatus {
    let mut dst = GrpcStatus::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcStatus> for GrpcStatusMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcStatus {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GrpcStatus {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcStatusView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcStatusMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GrpcStatusMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcStatus>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcStatusMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GrpcStatusMut<'msg> {
  type Message = GrpcStatus;
}

impl ::std::fmt::Debug for GrpcStatusMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcStatus>> for GrpcStatusMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcStatus>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcStatusMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcStatus> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GrpcStatus {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // status: optional uint32
  pub fn status(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `GrpcStatusMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GrpcStatusMut<'_> {}

// SAFETY:
// - `GrpcStatusMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GrpcStatusMut<'_> {}

impl<'msg> ::protobuf::AsView for GrpcStatusMut<'msg> {
  type Proxied = GrpcStatus;
  fn as_view(&self) -> ::protobuf::View<'_, GrpcStatus> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcStatusMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GrpcStatus>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GrpcStatusMut<'msg> {
  type MutProxied = GrpcStatus;
  fn as_mut(&mut self) -> GrpcStatusMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GrpcStatusMut<'msg> {
  fn into_mut<'shorter>(self) -> GrpcStatusMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GrpcStatus {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GrpcStatus> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GrpcStatusView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GrpcStatusMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // status: optional uint32
  pub fn status(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

}  // impl GrpcStatus

impl ::std::ops::Drop for GrpcStatus {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GrpcStatus {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GrpcStatus {
  type Proxied = Self;
  fn as_view(&self) -> GrpcStatusView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GrpcStatus {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GrpcStatusMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GrpcStatus {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__GrpcStatus_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__GrpcStatus_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__GrpcStatus_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcStatus {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcStatus {
  type Msg = GrpcStatus;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatus> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcStatus {
  type Msg = GrpcStatus;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatus> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcStatusMut<'_> {
  type Msg = GrpcStatus;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatus> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcStatusMut<'_> {
  type Msg = GrpcStatus;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatus> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcStatusView<'_> {
  type Msg = GrpcStatus;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatus> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcStatusMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__HeaderMutation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderMutation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderMutation>
}

impl ::protobuf::Message for HeaderMutation {
  type MessageView<'msg> = HeaderMutationView<'msg>;
  type MessageMut<'msg> = HeaderMutationMut<'msg>;
}

impl ::std::default::Default for HeaderMutation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderMutation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderMutation` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderMutationMut`.
unsafe impl ::std::marker::Sync for HeaderMutation {}

// SAFETY:
// - `HeaderMutation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderMutation {}

impl ::protobuf::Proxied for HeaderMutation {
  type View<'msg> = HeaderMutationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderMutation {}

impl ::protobuf::MutProxied for HeaderMutation {
  type Mut<'msg> = HeaderMutationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderMutationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMutation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderMutationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderMutationView<'msg> {
  type Message = HeaderMutation;
}

impl ::std::fmt::Debug for HeaderMutationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderMutationView<'_> {
  fn default() -> HeaderMutationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMutation>> for HeaderMutationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMutation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderMutationView<'msg> {

  pub fn to_owned(&self) -> HeaderMutation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // set_headers: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn set_headers(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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

  // remove_headers: repeated string
  pub fn remove_headers(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
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
// - `HeaderMutationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderMutationView<'_> {}

// SAFETY:
// - `HeaderMutationView` is `Send` because while its alive a `HeaderMutationMut` cannot.
// - `HeaderMutationView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderMutationView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderMutationView<'msg> {
  type Proxied = HeaderMutation;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderMutation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderMutationView<'msg> {
  fn into_view<'shorter>(self) -> HeaderMutationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderMutation> for HeaderMutationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderMutation {
    let mut dst = HeaderMutation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderMutation> for HeaderMutationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderMutation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderMutation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderMutationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderMutationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderMutationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderMutationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderMutationMut<'msg> {
  type Message = HeaderMutation;
}

impl ::std::fmt::Debug for HeaderMutationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutation>> for HeaderMutationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderMutationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderMutation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // set_headers: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn set_headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // remove_headers: repeated string
  pub fn remove_headers(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn remove_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_remove_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `HeaderMutationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderMutationMut<'_> {}

// SAFETY:
// - `HeaderMutationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderMutationMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderMutationMut<'msg> {
  type Proxied = HeaderMutation;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderMutation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderMutationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderMutation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderMutationMut<'msg> {
  type MutProxied = HeaderMutation;
  fn as_mut(&mut self) -> HeaderMutationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderMutationMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderMutationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderMutation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderMutation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderMutationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderMutationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // set_headers: repeated message envoy.config.core.v3.HeaderValueOption
  pub fn set_headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption> {
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
  pub fn set_set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // remove_headers: repeated string
  pub fn remove_headers(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn remove_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_remove_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl HeaderMutation

impl ::std::ops::Drop for HeaderMutation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderMutation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderMutation {
  type Proxied = Self;
  fn as_view(&self) -> HeaderMutationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderMutation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderMutationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderMutation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__HeaderMutation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__HeaderMutation_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__HeaderMutation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderMutation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderMutation {
  type Msg = HeaderMutation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMutation {
  type Msg = HeaderMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderMutationMut<'_> {
  type Msg = HeaderMutation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMutationMut<'_> {
  type Msg = HeaderMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMutationView<'_> {
  type Msg = HeaderMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderMutationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__StreamedBodyResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StreamedBodyResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StreamedBodyResponse>
}

impl ::protobuf::Message for StreamedBodyResponse {
  type MessageView<'msg> = StreamedBodyResponseView<'msg>;
  type MessageMut<'msg> = StreamedBodyResponseMut<'msg>;
}

impl ::std::default::Default for StreamedBodyResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StreamedBodyResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StreamedBodyResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `StreamedBodyResponseMut`.
unsafe impl ::std::marker::Sync for StreamedBodyResponse {}

// SAFETY:
// - `StreamedBodyResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StreamedBodyResponse {}

impl ::protobuf::Proxied for StreamedBodyResponse {
  type View<'msg> = StreamedBodyResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StreamedBodyResponse {}

impl ::protobuf::MutProxied for StreamedBodyResponse {
  type Mut<'msg> = StreamedBodyResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StreamedBodyResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StreamedBodyResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamedBodyResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StreamedBodyResponseView<'msg> {
  type Message = StreamedBodyResponse;
}

impl ::std::fmt::Debug for StreamedBodyResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StreamedBodyResponseView<'_> {
  fn default() -> StreamedBodyResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StreamedBodyResponse>> for StreamedBodyResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StreamedBodyResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamedBodyResponseView<'msg> {

  pub fn to_owned(&self) -> StreamedBodyResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // body: optional bytes
  pub fn body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // end_of_stream: optional bool
  pub fn end_of_stream(self) -> bool {
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

  // end_of_stream_without_message: optional bool
  pub fn end_of_stream_without_message(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }

  // grpc_message_compressed: optional bool
  pub fn grpc_message_compressed(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `StreamedBodyResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StreamedBodyResponseView<'_> {}

// SAFETY:
// - `StreamedBodyResponseView` is `Send` because while its alive a `StreamedBodyResponseMut` cannot.
// - `StreamedBodyResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for StreamedBodyResponseView<'_> {}

impl<'msg> ::protobuf::AsView for StreamedBodyResponseView<'msg> {
  type Proxied = StreamedBodyResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, StreamedBodyResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamedBodyResponseView<'msg> {
  fn into_view<'shorter>(self) -> StreamedBodyResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StreamedBodyResponse> for StreamedBodyResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StreamedBodyResponse {
    let mut dst = StreamedBodyResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StreamedBodyResponse> for StreamedBodyResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StreamedBodyResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StreamedBodyResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StreamedBodyResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StreamedBodyResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StreamedBodyResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamedBodyResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamedBodyResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StreamedBodyResponseMut<'msg> {
  type Message = StreamedBodyResponse;
}

impl ::std::fmt::Debug for StreamedBodyResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StreamedBodyResponse>> for StreamedBodyResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamedBodyResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamedBodyResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamedBodyResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StreamedBodyResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // body: optional bytes
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // end_of_stream: optional bool
  pub fn end_of_stream(&self) -> bool {
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
  pub fn set_end_of_stream(&mut self, val: bool) {
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

  // end_of_stream_without_message: optional bool
  pub fn end_of_stream_without_message(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_of_stream_without_message(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // grpc_message_compressed: optional bool
  pub fn grpc_message_compressed(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_grpc_message_compressed(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `StreamedBodyResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StreamedBodyResponseMut<'_> {}

// SAFETY:
// - `StreamedBodyResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StreamedBodyResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for StreamedBodyResponseMut<'msg> {
  type Proxied = StreamedBodyResponse;
  fn as_view(&self) -> ::protobuf::View<'_, StreamedBodyResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamedBodyResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StreamedBodyResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StreamedBodyResponseMut<'msg> {
  type MutProxied = StreamedBodyResponse;
  fn as_mut(&mut self) -> StreamedBodyResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StreamedBodyResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> StreamedBodyResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StreamedBodyResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StreamedBodyResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StreamedBodyResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StreamedBodyResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // body: optional bytes
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // end_of_stream: optional bool
  pub fn end_of_stream(&self) -> bool {
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
  pub fn set_end_of_stream(&mut self, val: bool) {
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

  // end_of_stream_without_message: optional bool
  pub fn end_of_stream_without_message(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_of_stream_without_message(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // grpc_message_compressed: optional bool
  pub fn grpc_message_compressed(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_grpc_message_compressed(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

}  // impl StreamedBodyResponse

impl ::std::ops::Drop for StreamedBodyResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StreamedBodyResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StreamedBodyResponse {
  type Proxied = Self;
  fn as_view(&self) -> StreamedBodyResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StreamedBodyResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StreamedBodyResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StreamedBodyResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__StreamedBodyResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0P/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__StreamedBodyResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__StreamedBodyResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StreamedBodyResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StreamedBodyResponse {
  type Msg = StreamedBodyResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedBodyResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamedBodyResponse {
  type Msg = StreamedBodyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedBodyResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StreamedBodyResponseMut<'_> {
  type Msg = StreamedBodyResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedBodyResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamedBodyResponseMut<'_> {
  type Msg = StreamedBodyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedBodyResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamedBodyResponseView<'_> {
  type Msg = StreamedBodyResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamedBodyResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StreamedBodyResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__ext_0proc__v3__BodyMutation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BodyMutation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BodyMutation>
}

impl ::protobuf::Message for BodyMutation {
  type MessageView<'msg> = BodyMutationView<'msg>;
  type MessageMut<'msg> = BodyMutationMut<'msg>;
}

impl ::std::default::Default for BodyMutation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BodyMutation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BodyMutation` is `Sync` because it does not implement interior mutability.
//    Neither does `BodyMutationMut`.
unsafe impl ::std::marker::Sync for BodyMutation {}

// SAFETY:
// - `BodyMutation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BodyMutation {}

impl ::protobuf::Proxied for BodyMutation {
  type View<'msg> = BodyMutationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BodyMutation {}

impl ::protobuf::MutProxied for BodyMutation {
  type Mut<'msg> = BodyMutationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BodyMutationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BodyMutation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BodyMutationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BodyMutationView<'msg> {
  type Message = BodyMutation;
}

impl ::std::fmt::Debug for BodyMutationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BodyMutationView<'_> {
  fn default() -> BodyMutationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BodyMutation>> for BodyMutationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BodyMutation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BodyMutationView<'msg> {

  pub fn to_owned(&self) -> BodyMutation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // body: optional bytes
  pub fn has_body(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn body_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_body().then(|| self.body())
  }
  pub fn body(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // clear_body: optional bool
  pub fn has_clear_body_2(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_body_2_opt(self) -> ::std::option::Option<bool> {
    self.has_clear_body_2().then(|| self.clear_body_2())
  }
  pub fn clear_body_2(self) -> bool {
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

  // streamed_response: optional message envoy.service.ext_proc.v3.StreamedBodyResponse
  pub fn has_streamed_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn streamed_response_opt(self) -> ::std::option::Option<super::StreamedBodyResponseView<'msg>> {
    self.has_streamed_response().then(|| self.streamed_response())
  }
  pub fn streamed_response(self) -> super::StreamedBodyResponseView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StreamedBodyResponseView::default())
  }

  pub fn mutation(self) -> super::body_mutation::MutationOneof<'msg> {
    match self.mutation_case() {
      super::body_mutation::MutationCase::Body =>
          super::body_mutation::MutationOneof::Body(self.body()),
      super::body_mutation::MutationCase::ClearBody =>
          super::body_mutation::MutationOneof::ClearBody(self.clear_body_2()),
      super::body_mutation::MutationCase::StreamedResponse =>
          super::body_mutation::MutationOneof::StreamedResponse(self.streamed_response()),
      _ => super::body_mutation::MutationOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn mutation_case(self) -> super::body_mutation::MutationCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::body_mutation::MutationCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BodyMutationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BodyMutationView<'_> {}

// SAFETY:
// - `BodyMutationView` is `Send` because while its alive a `BodyMutationMut` cannot.
// - `BodyMutationView` does not use thread-local data.
unsafe impl ::std::marker::Send for BodyMutationView<'_> {}

impl<'msg> ::protobuf::AsView for BodyMutationView<'msg> {
  type Proxied = BodyMutation;
  fn as_view(&self) -> ::protobuf::View<'msg, BodyMutation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BodyMutationView<'msg> {
  fn into_view<'shorter>(self) -> BodyMutationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BodyMutation> for BodyMutationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BodyMutation {
    let mut dst = BodyMutation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BodyMutation> for BodyMutationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BodyMutation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BodyMutation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BodyMutationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BodyMutationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BodyMutationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BodyMutation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BodyMutationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BodyMutationMut<'msg> {
  type Message = BodyMutation;
}

impl ::std::fmt::Debug for BodyMutationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BodyMutation>> for BodyMutationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BodyMutation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BodyMutationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BodyMutation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BodyMutation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // body: optional bytes
  pub fn has_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn body_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_body().then(|| self.body())
  }
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // clear_body: optional bool
  pub fn has_clear_body_2(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_clear_body_2(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn clear_body_2_opt(&self) -> ::std::option::Option<bool> {
    self.has_clear_body_2().then(|| self.clear_body_2())
  }
  pub fn clear_body_2(&self) -> bool {
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
  pub fn set_clear_body_2(&mut self, val: bool) {
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

  // streamed_response: optional message envoy.service.ext_proc.v3.StreamedBodyResponse
  pub fn has_streamed_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_streamed_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn streamed_response_opt(&self) -> ::std::option::Option<super::StreamedBodyResponseView<'_>> {
    self.has_streamed_response().then(|| self.streamed_response())
  }
  pub fn streamed_response(&self) -> super::StreamedBodyResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StreamedBodyResponseView::default())
  }
  pub fn streamed_response_mut(&mut self) -> super::StreamedBodyResponseMut<'_> {
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
  pub fn set_streamed_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::StreamedBodyResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn mutation(&self) -> super::body_mutation::MutationOneof<'_> {
    match &self.mutation_case() {
      super::body_mutation::MutationCase::Body =>
          super::body_mutation::MutationOneof::Body(self.body()),
      super::body_mutation::MutationCase::ClearBody =>
          super::body_mutation::MutationOneof::ClearBody(self.clear_body_2()),
      super::body_mutation::MutationCase::StreamedResponse =>
          super::body_mutation::MutationOneof::StreamedResponse(self.streamed_response()),
      _ => super::body_mutation::MutationOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn mutation_case(&self) -> super::body_mutation::MutationCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::body_mutation::MutationCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BodyMutationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BodyMutationMut<'_> {}

// SAFETY:
// - `BodyMutationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BodyMutationMut<'_> {}

impl<'msg> ::protobuf::AsView for BodyMutationMut<'msg> {
  type Proxied = BodyMutation;
  fn as_view(&self) -> ::protobuf::View<'_, BodyMutation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BodyMutationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BodyMutation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BodyMutationMut<'msg> {
  type MutProxied = BodyMutation;
  fn as_mut(&mut self) -> BodyMutationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BodyMutationMut<'msg> {
  fn into_mut<'shorter>(self) -> BodyMutationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BodyMutation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BodyMutation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BodyMutationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BodyMutationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // body: optional bytes
  pub fn has_body(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_body(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn body_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_body().then(|| self.body())
  }
  pub fn body(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_body(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // clear_body: optional bool
  pub fn has_clear_body_2(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_clear_body_2(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn clear_body_2_opt(&self) -> ::std::option::Option<bool> {
    self.has_clear_body_2().then(|| self.clear_body_2())
  }
  pub fn clear_body_2(&self) -> bool {
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
  pub fn set_clear_body_2(&mut self, val: bool) {
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

  // streamed_response: optional message envoy.service.ext_proc.v3.StreamedBodyResponse
  pub fn has_streamed_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_streamed_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn streamed_response_opt(&self) -> ::std::option::Option<super::StreamedBodyResponseView<'_>> {
    self.has_streamed_response().then(|| self.streamed_response())
  }
  pub fn streamed_response(&self) -> super::StreamedBodyResponseView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StreamedBodyResponseView::default())
  }
  pub fn streamed_response_mut(&mut self) -> super::StreamedBodyResponseMut<'_> {
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
  pub fn set_streamed_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::StreamedBodyResponse>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn mutation(&self) -> super::body_mutation::MutationOneof<'_> {
    match &self.mutation_case() {
      super::body_mutation::MutationCase::Body =>
          super::body_mutation::MutationOneof::Body(self.body()),
      super::body_mutation::MutationCase::ClearBody =>
          super::body_mutation::MutationOneof::ClearBody(self.clear_body_2()),
      super::body_mutation::MutationCase::StreamedResponse =>
          super::body_mutation::MutationOneof::StreamedResponse(self.streamed_response()),
      _ => super::body_mutation::MutationOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn mutation_case(&self) -> super::body_mutation::MutationCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::body_mutation::MutationCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl BodyMutation

impl ::std::ops::Drop for BodyMutation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BodyMutation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BodyMutation {
  type Proxied = Self;
  fn as_view(&self) -> BodyMutationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BodyMutation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BodyMutationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BodyMutation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__ext_0proc__v3__BodyMutation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0/3^!|#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__ext_0proc__v3__BodyMutation_msg_init.0, &[<super::StreamedBodyResponse as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__ext_0proc__v3__BodyMutation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BodyMutation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BodyMutation {
  type Msg = BodyMutation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyMutation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BodyMutation {
  type Msg = BodyMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyMutation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BodyMutationMut<'_> {
  type Msg = BodyMutation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyMutation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BodyMutationMut<'_> {
  type Msg = BodyMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyMutation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BodyMutationView<'_> {
  type Msg = BodyMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BodyMutation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BodyMutationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod body_mutation {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MutationOneof<'msg> {
  Body(&'msg [u8]) = 1,
  ClearBody(bool) = 2,
  StreamedResponse(::protobuf::View<'msg, super::super::StreamedBodyResponse>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MutationCase {
  Body = 1,
  ClearBody = 2,
  StreamedResponse = 3,

  not_set = 0
}

impl MutationCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MutationCase> {
    match v {
      0 => Some(MutationCase::not_set),
      1 => Some(MutationCase::Body),
      2 => Some(MutationCase::ClearBody),
      3 => Some(MutationCase::StreamedResponse),
      _ => None
    }
  }
}
}  // pub mod body_mutation


