const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__service__orca__v3__OrcaLoadReportRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OrcaLoadReportRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OrcaLoadReportRequest>
}

impl ::protobuf::Message for OrcaLoadReportRequest {
  type MessageView<'msg> = OrcaLoadReportRequestView<'msg>;
  type MessageMut<'msg> = OrcaLoadReportRequestMut<'msg>;
}

impl ::std::default::Default for OrcaLoadReportRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OrcaLoadReportRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OrcaLoadReportRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `OrcaLoadReportRequestMut`.
unsafe impl ::std::marker::Sync for OrcaLoadReportRequest {}

// SAFETY:
// - `OrcaLoadReportRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OrcaLoadReportRequest {}

impl ::protobuf::Proxied for OrcaLoadReportRequest {
  type View<'msg> = OrcaLoadReportRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OrcaLoadReportRequest {}

impl ::protobuf::MutProxied for OrcaLoadReportRequest {
  type Mut<'msg> = OrcaLoadReportRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OrcaLoadReportRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OrcaLoadReportRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OrcaLoadReportRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OrcaLoadReportRequestView<'msg> {
  type Message = OrcaLoadReportRequest;
}

impl ::std::fmt::Debug for OrcaLoadReportRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OrcaLoadReportRequestView<'_> {
  fn default() -> OrcaLoadReportRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OrcaLoadReportRequest>> for OrcaLoadReportRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OrcaLoadReportRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OrcaLoadReportRequestView<'msg> {

  pub fn to_owned(&self) -> OrcaLoadReportRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // report_interval: optional message google.protobuf.Duration
  pub fn has_report_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn report_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_report_interval().then(|| self.report_interval())
  }
  pub fn report_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // request_cost_names: repeated string
  pub fn request_cost_names(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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
// - `OrcaLoadReportRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OrcaLoadReportRequestView<'_> {}

// SAFETY:
// - `OrcaLoadReportRequestView` is `Send` because while its alive a `OrcaLoadReportRequestMut` cannot.
// - `OrcaLoadReportRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for OrcaLoadReportRequestView<'_> {}

impl<'msg> ::protobuf::AsView for OrcaLoadReportRequestView<'msg> {
  type Proxied = OrcaLoadReportRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, OrcaLoadReportRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OrcaLoadReportRequestView<'msg> {
  fn into_view<'shorter>(self) -> OrcaLoadReportRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OrcaLoadReportRequest> for OrcaLoadReportRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OrcaLoadReportRequest {
    let mut dst = OrcaLoadReportRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OrcaLoadReportRequest> for OrcaLoadReportRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OrcaLoadReportRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OrcaLoadReportRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OrcaLoadReportRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OrcaLoadReportRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OrcaLoadReportRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OrcaLoadReportRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OrcaLoadReportRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OrcaLoadReportRequestMut<'msg> {
  type Message = OrcaLoadReportRequest;
}

impl ::std::fmt::Debug for OrcaLoadReportRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OrcaLoadReportRequest>> for OrcaLoadReportRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OrcaLoadReportRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OrcaLoadReportRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OrcaLoadReportRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OrcaLoadReportRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // report_interval: optional message google.protobuf.Duration
  pub fn has_report_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_report_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn report_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_report_interval().then(|| self.report_interval())
  }
  pub fn report_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn report_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_report_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // request_cost_names: repeated string
  pub fn request_cost_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn request_cost_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_request_cost_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `OrcaLoadReportRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OrcaLoadReportRequestMut<'_> {}

// SAFETY:
// - `OrcaLoadReportRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OrcaLoadReportRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for OrcaLoadReportRequestMut<'msg> {
  type Proxied = OrcaLoadReportRequest;
  fn as_view(&self) -> ::protobuf::View<'_, OrcaLoadReportRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OrcaLoadReportRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OrcaLoadReportRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OrcaLoadReportRequestMut<'msg> {
  type MutProxied = OrcaLoadReportRequest;
  fn as_mut(&mut self) -> OrcaLoadReportRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OrcaLoadReportRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> OrcaLoadReportRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OrcaLoadReportRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OrcaLoadReportRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OrcaLoadReportRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OrcaLoadReportRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // report_interval: optional message google.protobuf.Duration
  pub fn has_report_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_report_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn report_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_report_interval().then(|| self.report_interval())
  }
  pub fn report_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn report_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_report_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // request_cost_names: repeated string
  pub fn request_cost_names(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn request_cost_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_request_cost_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl OrcaLoadReportRequest

impl ::std::ops::Drop for OrcaLoadReportRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OrcaLoadReportRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OrcaLoadReportRequest {
  type Proxied = Self;
  fn as_view(&self) -> OrcaLoadReportRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OrcaLoadReportRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OrcaLoadReportRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OrcaLoadReportRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::xds__service__orca__v3__OrcaLoadReportRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3ET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__service__orca__v3__OrcaLoadReportRequest_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__service__orca__v3__OrcaLoadReportRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OrcaLoadReportRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OrcaLoadReportRequest {
  type Msg = OrcaLoadReportRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReportRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrcaLoadReportRequest {
  type Msg = OrcaLoadReportRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReportRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OrcaLoadReportRequestMut<'_> {
  type Msg = OrcaLoadReportRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReportRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrcaLoadReportRequestMut<'_> {
  type Msg = OrcaLoadReportRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReportRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrcaLoadReportRequestView<'_> {
  type Msg = OrcaLoadReportRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrcaLoadReportRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OrcaLoadReportRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



