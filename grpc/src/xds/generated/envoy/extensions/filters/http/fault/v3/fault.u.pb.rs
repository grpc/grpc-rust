const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__fault__v3__FaultAbort_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FaultAbort {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FaultAbort>
}

impl ::protobuf::Message for FaultAbort {
  type MessageView<'msg> = FaultAbortView<'msg>;
  type MessageMut<'msg> = FaultAbortMut<'msg>;
}

impl ::std::default::Default for FaultAbort {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FaultAbort {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FaultAbort` is `Sync` because it does not implement interior mutability.
//    Neither does `FaultAbortMut`.
unsafe impl ::std::marker::Sync for FaultAbort {}

// SAFETY:
// - `FaultAbort` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FaultAbort {}

impl ::protobuf::Proxied for FaultAbort {
  type View<'msg> = FaultAbortView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FaultAbort {}

impl ::protobuf::MutProxied for FaultAbort {
  type Mut<'msg> = FaultAbortMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FaultAbortView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FaultAbort>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FaultAbortView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FaultAbortView<'msg> {
  type Message = FaultAbort;
}

impl ::std::fmt::Debug for FaultAbortView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FaultAbortView<'_> {
  fn default() -> FaultAbortView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FaultAbort>> for FaultAbortView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FaultAbort>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FaultAbortView<'msg> {

  pub fn to_owned(&self) -> FaultAbort {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // http_status: optional uint32
  pub fn has_http_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn http_status_opt(self) -> ::std::option::Option<u32> {
    self.has_http_status().then(|| self.http_status())
  }
  pub fn http_status(self) -> u32 {
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

  // grpc_status: optional uint32
  pub fn has_grpc_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn grpc_status_opt(self) -> ::std::option::Option<u32> {
    self.has_grpc_status().then(|| self.grpc_status())
  }
  pub fn grpc_status(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // header_abort: optional message envoy.extensions.filters.http.fault.v3.FaultAbort.HeaderAbort
  pub fn has_header_abort(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn header_abort_opt(self) -> ::std::option::Option<super::fault_abort::HeaderAbortView<'msg>> {
    self.has_header_abort().then(|| self.header_abort())
  }
  pub fn header_abort(self) -> super::fault_abort::HeaderAbortView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_abort::HeaderAbortView::default())
  }

  // percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_percentage(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn percentage_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg>> {
    self.has_percentage().then(|| self.percentage())
  }
  pub fn percentage(self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }

  pub fn error_type(self) -> super::fault_abort::ErrorTypeOneof<'msg> {
    match self.error_type_case() {
      super::fault_abort::ErrorTypeCase::HttpStatus =>
          super::fault_abort::ErrorTypeOneof::HttpStatus(self.http_status()),
      super::fault_abort::ErrorTypeCase::GrpcStatus =>
          super::fault_abort::ErrorTypeOneof::GrpcStatus(self.grpc_status()),
      super::fault_abort::ErrorTypeCase::HeaderAbort =>
          super::fault_abort::ErrorTypeOneof::HeaderAbort(self.header_abort()),
      _ => super::fault_abort::ErrorTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn error_type_case(self) -> super::fault_abort::ErrorTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::fault_abort::ErrorTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FaultAbortView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FaultAbortView<'_> {}

// SAFETY:
// - `FaultAbortView` is `Send` because while its alive a `FaultAbortMut` cannot.
// - `FaultAbortView` does not use thread-local data.
unsafe impl ::std::marker::Send for FaultAbortView<'_> {}

impl<'msg> ::protobuf::AsView for FaultAbortView<'msg> {
  type Proxied = FaultAbort;
  fn as_view(&self) -> ::protobuf::View<'msg, FaultAbort> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FaultAbortView<'msg> {
  fn into_view<'shorter>(self) -> FaultAbortView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FaultAbort> for FaultAbortView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FaultAbort {
    let mut dst = FaultAbort::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FaultAbort> for FaultAbortMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FaultAbort {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FaultAbort {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FaultAbortView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FaultAbortMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FaultAbortMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FaultAbort>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FaultAbortMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FaultAbortMut<'msg> {
  type Message = FaultAbort;
}

impl ::std::fmt::Debug for FaultAbortMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FaultAbort>> for FaultAbortMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FaultAbort>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FaultAbortMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FaultAbort> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FaultAbort {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // http_status: optional uint32
  pub fn has_http_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_status_opt(&self) -> ::std::option::Option<u32> {
    self.has_http_status().then(|| self.http_status())
  }
  pub fn http_status(&self) -> u32 {
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
  pub fn set_http_status(&mut self, val: u32) {
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

  // grpc_status: optional uint32
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
  pub fn grpc_status_opt(&self) -> ::std::option::Option<u32> {
    self.has_grpc_status().then(|| self.grpc_status())
  }
  pub fn grpc_status(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_grpc_status(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // header_abort: optional message envoy.extensions.filters.http.fault.v3.FaultAbort.HeaderAbort
  pub fn has_header_abort(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_header_abort(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn header_abort_opt(&self) -> ::std::option::Option<super::fault_abort::HeaderAbortView<'_>> {
    self.has_header_abort().then(|| self.header_abort())
  }
  pub fn header_abort(&self) -> super::fault_abort::HeaderAbortView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_abort::HeaderAbortView::default())
  }
  pub fn header_abort_mut(&mut self) -> super::fault_abort::HeaderAbortMut<'_> {
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
  pub fn set_header_abort(&mut self,
    val: impl ::protobuf::IntoProxied<super::fault_abort::HeaderAbort>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn percentage_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_percentage().then(|| self.percentage())
  }
  pub fn percentage(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn percentage_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_percentage(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn error_type(&self) -> super::fault_abort::ErrorTypeOneof<'_> {
    match &self.error_type_case() {
      super::fault_abort::ErrorTypeCase::HttpStatus =>
          super::fault_abort::ErrorTypeOneof::HttpStatus(self.http_status()),
      super::fault_abort::ErrorTypeCase::GrpcStatus =>
          super::fault_abort::ErrorTypeOneof::GrpcStatus(self.grpc_status()),
      super::fault_abort::ErrorTypeCase::HeaderAbort =>
          super::fault_abort::ErrorTypeOneof::HeaderAbort(self.header_abort()),
      _ => super::fault_abort::ErrorTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn error_type_case(&self) -> super::fault_abort::ErrorTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::fault_abort::ErrorTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FaultAbortMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FaultAbortMut<'_> {}

// SAFETY:
// - `FaultAbortMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FaultAbortMut<'_> {}

impl<'msg> ::protobuf::AsView for FaultAbortMut<'msg> {
  type Proxied = FaultAbort;
  fn as_view(&self) -> ::protobuf::View<'_, FaultAbort> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FaultAbortMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FaultAbort>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FaultAbortMut<'msg> {
  type MutProxied = FaultAbort;
  fn as_mut(&mut self) -> FaultAbortMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FaultAbortMut<'msg> {
  fn into_mut<'shorter>(self) -> FaultAbortMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FaultAbort {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FaultAbort> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FaultAbortView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FaultAbortMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // http_status: optional uint32
  pub fn has_http_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_status_opt(&self) -> ::std::option::Option<u32> {
    self.has_http_status().then(|| self.http_status())
  }
  pub fn http_status(&self) -> u32 {
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
  pub fn set_http_status(&mut self, val: u32) {
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

  // grpc_status: optional uint32
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
  pub fn grpc_status_opt(&self) -> ::std::option::Option<u32> {
    self.has_grpc_status().then(|| self.grpc_status())
  }
  pub fn grpc_status(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_grpc_status(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

  // header_abort: optional message envoy.extensions.filters.http.fault.v3.FaultAbort.HeaderAbort
  pub fn has_header_abort(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_header_abort(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn header_abort_opt(&self) -> ::std::option::Option<super::fault_abort::HeaderAbortView<'_>> {
    self.has_header_abort().then(|| self.header_abort())
  }
  pub fn header_abort(&self) -> super::fault_abort::HeaderAbortView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::fault_abort::HeaderAbortView::default())
  }
  pub fn header_abort_mut(&mut self) -> super::fault_abort::HeaderAbortMut<'_> {
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
  pub fn set_header_abort(&mut self,
    val: impl ::protobuf::IntoProxied<super::fault_abort::HeaderAbort>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn percentage_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_percentage().then(|| self.percentage())
  }
  pub fn percentage(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn percentage_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_percentage(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn error_type(&self) -> super::fault_abort::ErrorTypeOneof<'_> {
    match &self.error_type_case() {
      super::fault_abort::ErrorTypeCase::HttpStatus =>
          super::fault_abort::ErrorTypeOneof::HttpStatus(self.http_status()),
      super::fault_abort::ErrorTypeCase::GrpcStatus =>
          super::fault_abort::ErrorTypeOneof::GrpcStatus(self.grpc_status()),
      super::fault_abort::ErrorTypeCase::HeaderAbort =>
          super::fault_abort::ErrorTypeOneof::HeaderAbort(self.header_abort()),
      _ => super::fault_abort::ErrorTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn error_type_case(&self) -> super::fault_abort::ErrorTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::fault_abort::ErrorTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl FaultAbort

impl ::std::ops::Drop for FaultAbort {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FaultAbort {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FaultAbort {
  type Proxied = Self;
  fn as_view(&self) -> FaultAbortView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FaultAbort {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FaultAbortMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FaultAbort {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__fault__v3__FaultAbort_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$a)33)^#|&|%");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__fault__v3__FaultAbort_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::fault_abort::HeaderAbort as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__fault__v3__FaultAbort_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FaultAbort {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FaultAbort {
  type Msg = FaultAbort;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultAbort> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FaultAbort {
  type Msg = FaultAbort;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultAbort> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FaultAbortMut<'_> {
  type Msg = FaultAbort;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultAbort> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FaultAbortMut<'_> {
  type Msg = FaultAbort;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultAbort> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FaultAbortView<'_> {
  type Msg = FaultAbort;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FaultAbort> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FaultAbortMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod fault_abort {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__fault__v3__FaultAbort__HeaderAbort_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderAbort {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderAbort>
}

impl ::protobuf::Message for HeaderAbort {
  type MessageView<'msg> = HeaderAbortView<'msg>;
  type MessageMut<'msg> = HeaderAbortMut<'msg>;
}

impl ::std::default::Default for HeaderAbort {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderAbort {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderAbort` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderAbortMut`.
unsafe impl ::std::marker::Sync for HeaderAbort {}

// SAFETY:
// - `HeaderAbort` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderAbort {}

impl ::protobuf::Proxied for HeaderAbort {
  type View<'msg> = HeaderAbortView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderAbort {}

impl ::protobuf::MutProxied for HeaderAbort {
  type Mut<'msg> = HeaderAbortMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderAbortView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderAbort>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderAbortView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderAbortView<'msg> {
  type Message = HeaderAbort;
}

impl ::std::fmt::Debug for HeaderAbortView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderAbortView<'_> {
  fn default() -> HeaderAbortView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderAbort>> for HeaderAbortView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderAbort>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderAbortView<'msg> {

  pub fn to_owned(&self) -> HeaderAbort {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `HeaderAbortView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderAbortView<'_> {}

// SAFETY:
// - `HeaderAbortView` is `Send` because while its alive a `HeaderAbortMut` cannot.
// - `HeaderAbortView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderAbortView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderAbortView<'msg> {
  type Proxied = HeaderAbort;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderAbort> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderAbortView<'msg> {
  fn into_view<'shorter>(self) -> HeaderAbortView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderAbort> for HeaderAbortView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderAbort {
    let mut dst = HeaderAbort::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderAbort> for HeaderAbortMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderAbort {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderAbort {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderAbortView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderAbortMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderAbortMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderAbort>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderAbortMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderAbortMut<'msg> {
  type Message = HeaderAbort;
}

impl ::std::fmt::Debug for HeaderAbortMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderAbort>> for HeaderAbortMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderAbort>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderAbortMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderAbort> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderAbort {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `HeaderAbortMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderAbortMut<'_> {}

// SAFETY:
// - `HeaderAbortMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderAbortMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderAbortMut<'msg> {
  type Proxied = HeaderAbort;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderAbort> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderAbortMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderAbort>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderAbortMut<'msg> {
  type MutProxied = HeaderAbort;
  fn as_mut(&mut self) -> HeaderAbortMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderAbortMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderAbortMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderAbort {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderAbort> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderAbortView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderAbortMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl HeaderAbort

impl ::std::ops::Drop for HeaderAbort {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderAbort {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderAbort {
  type Proxied = Self;
  fn as_view(&self) -> HeaderAbortView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderAbort {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderAbortMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderAbort {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::fault_abort::envoy__extensions__filters__http__fault__v3__FaultAbort__HeaderAbort_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::fault_abort::envoy__extensions__filters__http__fault__v3__FaultAbort__HeaderAbort_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::fault_abort::envoy__extensions__filters__http__fault__v3__FaultAbort__HeaderAbort_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderAbort {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderAbort {
  type Msg = HeaderAbort;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderAbort> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderAbort {
  type Msg = HeaderAbort;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderAbort> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderAbortMut<'_> {
  type Msg = HeaderAbort;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderAbort> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderAbortMut<'_> {
  type Msg = HeaderAbort;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderAbort> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderAbortView<'_> {
  type Msg = HeaderAbort;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderAbort> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderAbortMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ErrorTypeOneof<'msg> {
  HttpStatus(u32) = 2,
  GrpcStatus(u32) = 5,
  HeaderAbort(::protobuf::View<'msg, super::super::fault_abort::HeaderAbort>) = 4,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ErrorTypeCase {
  HttpStatus = 2,
  GrpcStatus = 5,
  HeaderAbort = 4,

  not_set = 0
}

impl ErrorTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ErrorTypeCase> {
    match v {
      0 => Some(ErrorTypeCase::not_set),
      2 => Some(ErrorTypeCase::HttpStatus),
      5 => Some(ErrorTypeCase::GrpcStatus),
      4 => Some(ErrorTypeCase::HeaderAbort),
      _ => None
    }
  }
}
}  // pub mod fault_abort


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__fault__v3__HTTPFault_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HTTPFault {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HTTPFault>
}

impl ::protobuf::Message for HTTPFault {
  type MessageView<'msg> = HTTPFaultView<'msg>;
  type MessageMut<'msg> = HTTPFaultMut<'msg>;
}

impl ::std::default::Default for HTTPFault {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HTTPFault {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HTTPFault` is `Sync` because it does not implement interior mutability.
//    Neither does `HTTPFaultMut`.
unsafe impl ::std::marker::Sync for HTTPFault {}

// SAFETY:
// - `HTTPFault` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HTTPFault {}

impl ::protobuf::Proxied for HTTPFault {
  type View<'msg> = HTTPFaultView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HTTPFault {}

impl ::protobuf::MutProxied for HTTPFault {
  type Mut<'msg> = HTTPFaultMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HTTPFaultView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPFault>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HTTPFaultView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HTTPFaultView<'msg> {
  type Message = HTTPFault;
}

impl ::std::fmt::Debug for HTTPFaultView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HTTPFaultView<'_> {
  fn default() -> HTTPFaultView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPFault>> for HTTPFaultView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPFault>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HTTPFaultView<'msg> {

  pub fn to_owned(&self) -> HTTPFault {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // delay: optional message envoy.extensions.filters.common.fault.v3.FaultDelay
  pub fn has_delay(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn delay_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayView<'msg>> {
    self.has_delay().then(|| self.delay())
  }
  pub fn delay(self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayView::default())
  }

  // abort: optional message envoy.extensions.filters.http.fault.v3.FaultAbort
  pub fn has_abort(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn abort_opt(self) -> ::std::option::Option<super::FaultAbortView<'msg>> {
    self.has_abort().then(|| self.abort())
  }
  pub fn abort(self) -> super::FaultAbortView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FaultAbortView::default())
  }

  // upstream_cluster: optional string
  pub fn upstream_cluster(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // headers: repeated message envoy.config.route.v3.HeaderMatcher
  pub fn headers(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // downstream_nodes: repeated string
  pub fn downstream_nodes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // max_active_faults: optional message google.protobuf.UInt32Value
  pub fn has_max_active_faults(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn max_active_faults_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_active_faults().then(|| self.max_active_faults())
  }
  pub fn max_active_faults(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // response_rate_limit: optional message envoy.extensions.filters.common.fault.v3.FaultRateLimit
  pub fn has_response_rate_limit(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn response_rate_limit_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitView<'msg>> {
    self.has_response_rate_limit().then(|| self.response_rate_limit())
  }
  pub fn response_rate_limit(self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitView::default())
  }

  // delay_percent_runtime: optional string
  pub fn delay_percent_runtime(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // abort_percent_runtime: optional string
  pub fn abort_percent_runtime(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // delay_duration_runtime: optional string
  pub fn delay_duration_runtime(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // abort_http_status_runtime: optional string
  pub fn abort_http_status_runtime(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // max_active_faults_runtime: optional string
  pub fn max_active_faults_runtime(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // response_rate_limit_percent_runtime: optional string
  pub fn response_rate_limit_percent_runtime(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // abort_grpc_status_runtime: optional string
  pub fn abort_grpc_status_runtime(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        13, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // disable_downstream_cluster_stats: optional bool
  pub fn disable_downstream_cluster_stats(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }

  // filter_metadata: optional message google.protobuf.Struct
  pub fn has_filter_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn filter_metadata_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_filter_metadata().then(|| self.filter_metadata())
  }
  pub fn filter_metadata(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

}

// SAFETY:
// - `HTTPFaultView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HTTPFaultView<'_> {}

// SAFETY:
// - `HTTPFaultView` is `Send` because while its alive a `HTTPFaultMut` cannot.
// - `HTTPFaultView` does not use thread-local data.
unsafe impl ::std::marker::Send for HTTPFaultView<'_> {}

impl<'msg> ::protobuf::AsView for HTTPFaultView<'msg> {
  type Proxied = HTTPFault;
  fn as_view(&self) -> ::protobuf::View<'msg, HTTPFault> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HTTPFaultView<'msg> {
  fn into_view<'shorter>(self) -> HTTPFaultView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HTTPFault> for HTTPFaultView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HTTPFault {
    let mut dst = HTTPFault::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HTTPFault> for HTTPFaultMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HTTPFault {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HTTPFault {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HTTPFaultView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HTTPFaultMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HTTPFaultMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPFault>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HTTPFaultMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HTTPFaultMut<'msg> {
  type Message = HTTPFault;
}

impl ::std::fmt::Debug for HTTPFaultMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPFault>> for HTTPFaultMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPFault>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HTTPFaultMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPFault> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HTTPFault {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // delay: optional message envoy.extensions.filters.common.fault.v3.FaultDelay
  pub fn has_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn delay_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayView<'_>> {
    self.has_delay().then(|| self.delay())
  }
  pub fn delay(&self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayView::default())
  }
  pub fn delay_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayMut<'_> {
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
  pub fn set_delay(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelay>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // abort: optional message envoy.extensions.filters.http.fault.v3.FaultAbort
  pub fn has_abort(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_abort(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn abort_opt(&self) -> ::std::option::Option<super::FaultAbortView<'_>> {
    self.has_abort().then(|| self.abort())
  }
  pub fn abort(&self) -> super::FaultAbortView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FaultAbortView::default())
  }
  pub fn abort_mut(&mut self) -> super::FaultAbortMut<'_> {
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
  pub fn set_abort(&mut self,
    val: impl ::protobuf::IntoProxied<super::FaultAbort>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // upstream_cluster: optional string
  pub fn upstream_cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upstream_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // headers: repeated message envoy.config.route.v3.HeaderMatcher
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // downstream_nodes: repeated string
  pub fn downstream_nodes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn downstream_nodes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_downstream_nodes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // max_active_faults: optional message google.protobuf.UInt32Value
  pub fn has_max_active_faults(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_max_active_faults(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn max_active_faults_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_active_faults().then(|| self.max_active_faults())
  }
  pub fn max_active_faults(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_active_faults_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_active_faults(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // response_rate_limit: optional message envoy.extensions.filters.common.fault.v3.FaultRateLimit
  pub fn has_response_rate_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_response_rate_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn response_rate_limit_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitView<'_>> {
    self.has_response_rate_limit().then(|| self.response_rate_limit())
  }
  pub fn response_rate_limit(&self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitView::default())
  }
  pub fn response_rate_limit_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitMut<'_> {
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
  pub fn set_response_rate_limit(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // delay_percent_runtime: optional string
  pub fn delay_percent_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_delay_percent_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // abort_percent_runtime: optional string
  pub fn abort_percent_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_abort_percent_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // delay_duration_runtime: optional string
  pub fn delay_duration_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_delay_duration_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // abort_http_status_runtime: optional string
  pub fn abort_http_status_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_abort_http_status_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // max_active_faults_runtime: optional string
  pub fn max_active_faults_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_max_active_faults_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

  // response_rate_limit_percent_runtime: optional string
  pub fn response_rate_limit_percent_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_rate_limit_percent_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // abort_grpc_status_runtime: optional string
  pub fn abort_grpc_status_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        13, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_abort_grpc_status_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val);
    }
  }

  // disable_downstream_cluster_stats: optional bool
  pub fn disable_downstream_cluster_stats(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_downstream_cluster_stats(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        14, val.into()
      )
    }
  }

  // filter_metadata: optional message google.protobuf.Struct
  pub fn has_filter_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_filter_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn filter_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_filter_metadata().then(|| self.filter_metadata())
  }
  pub fn filter_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn filter_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

}

// SAFETY:
// - `HTTPFaultMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HTTPFaultMut<'_> {}

// SAFETY:
// - `HTTPFaultMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HTTPFaultMut<'_> {}

impl<'msg> ::protobuf::AsView for HTTPFaultMut<'msg> {
  type Proxied = HTTPFault;
  fn as_view(&self) -> ::protobuf::View<'_, HTTPFault> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HTTPFaultMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HTTPFault>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HTTPFaultMut<'msg> {
  type MutProxied = HTTPFault;
  fn as_mut(&mut self) -> HTTPFaultMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HTTPFaultMut<'msg> {
  fn into_mut<'shorter>(self) -> HTTPFaultMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HTTPFault {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HTTPFault> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HTTPFaultView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HTTPFaultMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // delay: optional message envoy.extensions.filters.common.fault.v3.FaultDelay
  pub fn has_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn delay_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayView<'_>> {
    self.has_delay().then(|| self.delay())
  }
  pub fn delay(&self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayView::default())
  }
  pub fn delay_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelayMut<'_> {
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
  pub fn set_delay(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelay>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // abort: optional message envoy.extensions.filters.http.fault.v3.FaultAbort
  pub fn has_abort(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_abort(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn abort_opt(&self) -> ::std::option::Option<super::FaultAbortView<'_>> {
    self.has_abort().then(|| self.abort())
  }
  pub fn abort(&self) -> super::FaultAbortView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FaultAbortView::default())
  }
  pub fn abort_mut(&mut self) -> super::FaultAbortMut<'_> {
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
  pub fn set_abort(&mut self,
    val: impl ::protobuf::IntoProxied<super::FaultAbort>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // upstream_cluster: optional string
  pub fn upstream_cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upstream_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // headers: repeated message envoy.config.route.v3.HeaderMatcher
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // downstream_nodes: repeated string
  pub fn downstream_nodes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn downstream_nodes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_downstream_nodes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // max_active_faults: optional message google.protobuf.UInt32Value
  pub fn has_max_active_faults(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_max_active_faults(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn max_active_faults_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_active_faults().then(|| self.max_active_faults())
  }
  pub fn max_active_faults(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_active_faults_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_active_faults(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // response_rate_limit: optional message envoy.extensions.filters.common.fault.v3.FaultRateLimit
  pub fn has_response_rate_limit(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_response_rate_limit(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn response_rate_limit_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitView<'_>> {
    self.has_response_rate_limit().then(|| self.response_rate_limit())
  }
  pub fn response_rate_limit(&self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitView::default())
  }
  pub fn response_rate_limit_mut(&mut self) -> crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimitMut<'_> {
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
  pub fn set_response_rate_limit(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimit>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // delay_percent_runtime: optional string
  pub fn delay_percent_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_delay_percent_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // abort_percent_runtime: optional string
  pub fn abort_percent_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_abort_percent_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // delay_duration_runtime: optional string
  pub fn delay_duration_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_delay_duration_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // abort_http_status_runtime: optional string
  pub fn abort_http_status_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_abort_http_status_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // max_active_faults_runtime: optional string
  pub fn max_active_faults_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_max_active_faults_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

  // response_rate_limit_percent_runtime: optional string
  pub fn response_rate_limit_percent_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_rate_limit_percent_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // abort_grpc_status_runtime: optional string
  pub fn abort_grpc_status_runtime(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        13, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_abort_grpc_status_runtime(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val);
    }
  }

  // disable_downstream_cluster_stats: optional bool
  pub fn disable_downstream_cluster_stats(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_downstream_cluster_stats(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        14, val.into()
      )
    }
  }

  // filter_metadata: optional message google.protobuf.Struct
  pub fn has_filter_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_filter_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn filter_metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_filter_metadata().then(|| self.filter_metadata())
  }
  pub fn filter_metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn filter_metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_filter_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

}  // impl HTTPFault

impl ::std::ops::Drop for HTTPFault {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HTTPFault {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HTTPFault {
  type Proxied = Self;
  fn as_view(&self) -> HTTPFaultView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HTTPFault {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HTTPFaultMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HTTPFault {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__fault__v3__HTTPFault_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$331XGET331X1X1X1X1X1X1X/P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__fault__v3__HTTPFault_msg_init.0, &[<crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultDelay as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::FaultAbort as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::filters::common::fault::v3::fault::FaultRateLimit as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__fault__v3__HTTPFault_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HTTPFault {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HTTPFault {
  type Msg = HTTPFault;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPFault> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPFault {
  type Msg = HTTPFault;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPFault> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HTTPFaultMut<'_> {
  type Msg = HTTPFault;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPFault> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPFaultMut<'_> {
  type Msg = HTTPFault;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPFault> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPFaultView<'_> {
  type Msg = HTTPFault;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPFault> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HTTPFaultMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



