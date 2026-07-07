const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__TCPAccessLogEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TCPAccessLogEntry {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TCPAccessLogEntry>
}

impl ::protobuf::Message for TCPAccessLogEntry {
  type MessageView<'msg> = TCPAccessLogEntryView<'msg>;
  type MessageMut<'msg> = TCPAccessLogEntryMut<'msg>;
}

impl ::std::default::Default for TCPAccessLogEntry {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TCPAccessLogEntry {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TCPAccessLogEntry` is `Sync` because it does not implement interior mutability.
//    Neither does `TCPAccessLogEntryMut`.
unsafe impl ::std::marker::Sync for TCPAccessLogEntry {}

// SAFETY:
// - `TCPAccessLogEntry` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TCPAccessLogEntry {}

impl ::protobuf::Proxied for TCPAccessLogEntry {
  type View<'msg> = TCPAccessLogEntryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TCPAccessLogEntry {}

impl ::protobuf::MutProxied for TCPAccessLogEntry {
  type Mut<'msg> = TCPAccessLogEntryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TCPAccessLogEntryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TCPAccessLogEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TCPAccessLogEntryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TCPAccessLogEntryView<'msg> {
  type Message = TCPAccessLogEntry;
}

impl ::std::fmt::Debug for TCPAccessLogEntryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TCPAccessLogEntryView<'_> {
  fn default() -> TCPAccessLogEntryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TCPAccessLogEntry>> for TCPAccessLogEntryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TCPAccessLogEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TCPAccessLogEntryView<'msg> {

  pub fn to_owned(&self) -> TCPAccessLogEntry {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // common_properties: optional message envoy.data.accesslog.v3.AccessLogCommon
  pub fn has_common_properties(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn common_properties_opt(self) -> ::std::option::Option<super::AccessLogCommonView<'msg>> {
    self.has_common_properties().then(|| self.common_properties())
  }
  pub fn common_properties(self) -> super::AccessLogCommonView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AccessLogCommonView::default())
  }

  // connection_properties: optional message envoy.data.accesslog.v3.ConnectionProperties
  pub fn has_connection_properties(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn connection_properties_opt(self) -> ::std::option::Option<super::ConnectionPropertiesView<'msg>> {
    self.has_connection_properties().then(|| self.connection_properties())
  }
  pub fn connection_properties(self) -> super::ConnectionPropertiesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConnectionPropertiesView::default())
  }

}

// SAFETY:
// - `TCPAccessLogEntryView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TCPAccessLogEntryView<'_> {}

// SAFETY:
// - `TCPAccessLogEntryView` is `Send` because while its alive a `TCPAccessLogEntryMut` cannot.
// - `TCPAccessLogEntryView` does not use thread-local data.
unsafe impl ::std::marker::Send for TCPAccessLogEntryView<'_> {}

impl<'msg> ::protobuf::AsView for TCPAccessLogEntryView<'msg> {
  type Proxied = TCPAccessLogEntry;
  fn as_view(&self) -> ::protobuf::View<'msg, TCPAccessLogEntry> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TCPAccessLogEntryView<'msg> {
  fn into_view<'shorter>(self) -> TCPAccessLogEntryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TCPAccessLogEntry> for TCPAccessLogEntryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TCPAccessLogEntry {
    let mut dst = TCPAccessLogEntry::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TCPAccessLogEntry> for TCPAccessLogEntryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TCPAccessLogEntry {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TCPAccessLogEntry {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TCPAccessLogEntryView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TCPAccessLogEntryMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TCPAccessLogEntryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TCPAccessLogEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TCPAccessLogEntryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TCPAccessLogEntryMut<'msg> {
  type Message = TCPAccessLogEntry;
}

impl ::std::fmt::Debug for TCPAccessLogEntryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TCPAccessLogEntry>> for TCPAccessLogEntryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TCPAccessLogEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TCPAccessLogEntryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TCPAccessLogEntry> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TCPAccessLogEntry {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // common_properties: optional message envoy.data.accesslog.v3.AccessLogCommon
  pub fn has_common_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_common_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn common_properties_opt(&self) -> ::std::option::Option<super::AccessLogCommonView<'_>> {
    self.has_common_properties().then(|| self.common_properties())
  }
  pub fn common_properties(&self) -> super::AccessLogCommonView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AccessLogCommonView::default())
  }
  pub fn common_properties_mut(&mut self) -> super::AccessLogCommonMut<'_> {
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
  pub fn set_common_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::AccessLogCommon>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // connection_properties: optional message envoy.data.accesslog.v3.ConnectionProperties
  pub fn has_connection_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_connection_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn connection_properties_opt(&self) -> ::std::option::Option<super::ConnectionPropertiesView<'_>> {
    self.has_connection_properties().then(|| self.connection_properties())
  }
  pub fn connection_properties(&self) -> super::ConnectionPropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConnectionPropertiesView::default())
  }
  pub fn connection_properties_mut(&mut self) -> super::ConnectionPropertiesMut<'_> {
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
  pub fn set_connection_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::ConnectionProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `TCPAccessLogEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TCPAccessLogEntryMut<'_> {}

// SAFETY:
// - `TCPAccessLogEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TCPAccessLogEntryMut<'_> {}

impl<'msg> ::protobuf::AsView for TCPAccessLogEntryMut<'msg> {
  type Proxied = TCPAccessLogEntry;
  fn as_view(&self) -> ::protobuf::View<'_, TCPAccessLogEntry> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TCPAccessLogEntryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TCPAccessLogEntry>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TCPAccessLogEntryMut<'msg> {
  type MutProxied = TCPAccessLogEntry;
  fn as_mut(&mut self) -> TCPAccessLogEntryMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TCPAccessLogEntryMut<'msg> {
  fn into_mut<'shorter>(self) -> TCPAccessLogEntryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TCPAccessLogEntry {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TCPAccessLogEntry> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TCPAccessLogEntryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TCPAccessLogEntryMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // common_properties: optional message envoy.data.accesslog.v3.AccessLogCommon
  pub fn has_common_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_common_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn common_properties_opt(&self) -> ::std::option::Option<super::AccessLogCommonView<'_>> {
    self.has_common_properties().then(|| self.common_properties())
  }
  pub fn common_properties(&self) -> super::AccessLogCommonView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AccessLogCommonView::default())
  }
  pub fn common_properties_mut(&mut self) -> super::AccessLogCommonMut<'_> {
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
  pub fn set_common_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::AccessLogCommon>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // connection_properties: optional message envoy.data.accesslog.v3.ConnectionProperties
  pub fn has_connection_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_connection_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn connection_properties_opt(&self) -> ::std::option::Option<super::ConnectionPropertiesView<'_>> {
    self.has_connection_properties().then(|| self.connection_properties())
  }
  pub fn connection_properties(&self) -> super::ConnectionPropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ConnectionPropertiesView::default())
  }
  pub fn connection_properties_mut(&mut self) -> super::ConnectionPropertiesMut<'_> {
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
  pub fn set_connection_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::ConnectionProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl TCPAccessLogEntry

impl ::std::ops::Drop for TCPAccessLogEntry {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TCPAccessLogEntry {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TCPAccessLogEntry {
  type Proxied = Self;
  fn as_view(&self) -> TCPAccessLogEntryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TCPAccessLogEntry {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TCPAccessLogEntryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TCPAccessLogEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__data__accesslog__v3__TCPAccessLogEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__data__accesslog__v3__TCPAccessLogEntry_msg_init.0, &[<super::AccessLogCommon as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ConnectionProperties as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__data__accesslog__v3__TCPAccessLogEntry_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TCPAccessLogEntry {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TCPAccessLogEntry {
  type Msg = TCPAccessLogEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TCPAccessLogEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TCPAccessLogEntry {
  type Msg = TCPAccessLogEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TCPAccessLogEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TCPAccessLogEntryMut<'_> {
  type Msg = TCPAccessLogEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TCPAccessLogEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TCPAccessLogEntryMut<'_> {
  type Msg = TCPAccessLogEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TCPAccessLogEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TCPAccessLogEntryView<'_> {
  type Msg = TCPAccessLogEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TCPAccessLogEntry> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TCPAccessLogEntryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__HTTPAccessLogEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HTTPAccessLogEntry {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HTTPAccessLogEntry>
}

impl ::protobuf::Message for HTTPAccessLogEntry {
  type MessageView<'msg> = HTTPAccessLogEntryView<'msg>;
  type MessageMut<'msg> = HTTPAccessLogEntryMut<'msg>;
}

impl ::std::default::Default for HTTPAccessLogEntry {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HTTPAccessLogEntry {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HTTPAccessLogEntry` is `Sync` because it does not implement interior mutability.
//    Neither does `HTTPAccessLogEntryMut`.
unsafe impl ::std::marker::Sync for HTTPAccessLogEntry {}

// SAFETY:
// - `HTTPAccessLogEntry` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HTTPAccessLogEntry {}

impl ::protobuf::Proxied for HTTPAccessLogEntry {
  type View<'msg> = HTTPAccessLogEntryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HTTPAccessLogEntry {}

impl ::protobuf::MutProxied for HTTPAccessLogEntry {
  type Mut<'msg> = HTTPAccessLogEntryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HTTPAccessLogEntryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPAccessLogEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HTTPAccessLogEntryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HTTPAccessLogEntryView<'msg> {
  type Message = HTTPAccessLogEntry;
}

impl ::std::fmt::Debug for HTTPAccessLogEntryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HTTPAccessLogEntryView<'_> {
  fn default() -> HTTPAccessLogEntryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPAccessLogEntry>> for HTTPAccessLogEntryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPAccessLogEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HTTPAccessLogEntryView<'msg> {

  pub fn to_owned(&self) -> HTTPAccessLogEntry {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // common_properties: optional message envoy.data.accesslog.v3.AccessLogCommon
  pub fn has_common_properties(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn common_properties_opt(self) -> ::std::option::Option<super::AccessLogCommonView<'msg>> {
    self.has_common_properties().then(|| self.common_properties())
  }
  pub fn common_properties(self) -> super::AccessLogCommonView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AccessLogCommonView::default())
  }

  // protocol_version: optional enum envoy.data.accesslog.v3.HTTPAccessLogEntry.HTTPVersion
  pub fn protocol_version(self) -> super::h_t_t_p_access_log_entry::HTTPVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::h_t_t_p_access_log_entry::HTTPVersion::ProtocolUnspecified).into()
      ).try_into().unwrap()
    }
  }

  // request: optional message envoy.data.accesslog.v3.HTTPRequestProperties
  pub fn has_request(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn request_opt(self) -> ::std::option::Option<super::HTTPRequestPropertiesView<'msg>> {
    self.has_request().then(|| self.request())
  }
  pub fn request(self) -> super::HTTPRequestPropertiesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HTTPRequestPropertiesView::default())
  }

  // response: optional message envoy.data.accesslog.v3.HTTPResponseProperties
  pub fn has_response(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn response_opt(self) -> ::std::option::Option<super::HTTPResponsePropertiesView<'msg>> {
    self.has_response().then(|| self.response())
  }
  pub fn response(self) -> super::HTTPResponsePropertiesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HTTPResponsePropertiesView::default())
  }

}

// SAFETY:
// - `HTTPAccessLogEntryView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HTTPAccessLogEntryView<'_> {}

// SAFETY:
// - `HTTPAccessLogEntryView` is `Send` because while its alive a `HTTPAccessLogEntryMut` cannot.
// - `HTTPAccessLogEntryView` does not use thread-local data.
unsafe impl ::std::marker::Send for HTTPAccessLogEntryView<'_> {}

impl<'msg> ::protobuf::AsView for HTTPAccessLogEntryView<'msg> {
  type Proxied = HTTPAccessLogEntry;
  fn as_view(&self) -> ::protobuf::View<'msg, HTTPAccessLogEntry> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HTTPAccessLogEntryView<'msg> {
  fn into_view<'shorter>(self) -> HTTPAccessLogEntryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HTTPAccessLogEntry> for HTTPAccessLogEntryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HTTPAccessLogEntry {
    let mut dst = HTTPAccessLogEntry::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HTTPAccessLogEntry> for HTTPAccessLogEntryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HTTPAccessLogEntry {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HTTPAccessLogEntry {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HTTPAccessLogEntryView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HTTPAccessLogEntryMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HTTPAccessLogEntryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPAccessLogEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HTTPAccessLogEntryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HTTPAccessLogEntryMut<'msg> {
  type Message = HTTPAccessLogEntry;
}

impl ::std::fmt::Debug for HTTPAccessLogEntryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPAccessLogEntry>> for HTTPAccessLogEntryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPAccessLogEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HTTPAccessLogEntryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPAccessLogEntry> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HTTPAccessLogEntry {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // common_properties: optional message envoy.data.accesslog.v3.AccessLogCommon
  pub fn has_common_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_common_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn common_properties_opt(&self) -> ::std::option::Option<super::AccessLogCommonView<'_>> {
    self.has_common_properties().then(|| self.common_properties())
  }
  pub fn common_properties(&self) -> super::AccessLogCommonView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AccessLogCommonView::default())
  }
  pub fn common_properties_mut(&mut self) -> super::AccessLogCommonMut<'_> {
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
  pub fn set_common_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::AccessLogCommon>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // protocol_version: optional enum envoy.data.accesslog.v3.HTTPAccessLogEntry.HTTPVersion
  pub fn protocol_version(&self) -> super::h_t_t_p_access_log_entry::HTTPVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::h_t_t_p_access_log_entry::HTTPVersion::ProtocolUnspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_protocol_version(&mut self, val: super::h_t_t_p_access_log_entry::HTTPVersion) {
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

  // request: optional message envoy.data.accesslog.v3.HTTPRequestProperties
  pub fn has_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn request_opt(&self) -> ::std::option::Option<super::HTTPRequestPropertiesView<'_>> {
    self.has_request().then(|| self.request())
  }
  pub fn request(&self) -> super::HTTPRequestPropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HTTPRequestPropertiesView::default())
  }
  pub fn request_mut(&mut self) -> super::HTTPRequestPropertiesMut<'_> {
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
  pub fn set_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::HTTPRequestProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // response: optional message envoy.data.accesslog.v3.HTTPResponseProperties
  pub fn has_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn response_opt(&self) -> ::std::option::Option<super::HTTPResponsePropertiesView<'_>> {
    self.has_response().then(|| self.response())
  }
  pub fn response(&self) -> super::HTTPResponsePropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HTTPResponsePropertiesView::default())
  }
  pub fn response_mut(&mut self) -> super::HTTPResponsePropertiesMut<'_> {
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
  pub fn set_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::HTTPResponseProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}

// SAFETY:
// - `HTTPAccessLogEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HTTPAccessLogEntryMut<'_> {}

// SAFETY:
// - `HTTPAccessLogEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HTTPAccessLogEntryMut<'_> {}

impl<'msg> ::protobuf::AsView for HTTPAccessLogEntryMut<'msg> {
  type Proxied = HTTPAccessLogEntry;
  fn as_view(&self) -> ::protobuf::View<'_, HTTPAccessLogEntry> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HTTPAccessLogEntryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HTTPAccessLogEntry>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HTTPAccessLogEntryMut<'msg> {
  type MutProxied = HTTPAccessLogEntry;
  fn as_mut(&mut self) -> HTTPAccessLogEntryMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HTTPAccessLogEntryMut<'msg> {
  fn into_mut<'shorter>(self) -> HTTPAccessLogEntryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HTTPAccessLogEntry {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HTTPAccessLogEntry> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HTTPAccessLogEntryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HTTPAccessLogEntryMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // common_properties: optional message envoy.data.accesslog.v3.AccessLogCommon
  pub fn has_common_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_common_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn common_properties_opt(&self) -> ::std::option::Option<super::AccessLogCommonView<'_>> {
    self.has_common_properties().then(|| self.common_properties())
  }
  pub fn common_properties(&self) -> super::AccessLogCommonView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AccessLogCommonView::default())
  }
  pub fn common_properties_mut(&mut self) -> super::AccessLogCommonMut<'_> {
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
  pub fn set_common_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::AccessLogCommon>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // protocol_version: optional enum envoy.data.accesslog.v3.HTTPAccessLogEntry.HTTPVersion
  pub fn protocol_version(&self) -> super::h_t_t_p_access_log_entry::HTTPVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::h_t_t_p_access_log_entry::HTTPVersion::ProtocolUnspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_protocol_version(&mut self, val: super::h_t_t_p_access_log_entry::HTTPVersion) {
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

  // request: optional message envoy.data.accesslog.v3.HTTPRequestProperties
  pub fn has_request(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_request(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn request_opt(&self) -> ::std::option::Option<super::HTTPRequestPropertiesView<'_>> {
    self.has_request().then(|| self.request())
  }
  pub fn request(&self) -> super::HTTPRequestPropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HTTPRequestPropertiesView::default())
  }
  pub fn request_mut(&mut self) -> super::HTTPRequestPropertiesMut<'_> {
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
  pub fn set_request(&mut self,
    val: impl ::protobuf::IntoProxied<super::HTTPRequestProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // response: optional message envoy.data.accesslog.v3.HTTPResponseProperties
  pub fn has_response(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_response(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn response_opt(&self) -> ::std::option::Option<super::HTTPResponsePropertiesView<'_>> {
    self.has_response().then(|| self.response())
  }
  pub fn response(&self) -> super::HTTPResponsePropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HTTPResponsePropertiesView::default())
  }
  pub fn response_mut(&mut self) -> super::HTTPResponsePropertiesMut<'_> {
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
  pub fn set_response(&mut self,
    val: impl ::protobuf::IntoProxied<super::HTTPResponseProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl HTTPAccessLogEntry

impl ::std::ops::Drop for HTTPAccessLogEntry {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HTTPAccessLogEntry {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HTTPAccessLogEntry {
  type Proxied = Self;
  fn as_view(&self) -> HTTPAccessLogEntryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HTTPAccessLogEntry {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HTTPAccessLogEntryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HTTPAccessLogEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__data__accesslog__v3__HTTPAccessLogEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3.P33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__data__accesslog__v3__HTTPAccessLogEntry_msg_init.0, &[<super::AccessLogCommon as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HTTPRequestProperties as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HTTPResponseProperties as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__data__accesslog__v3__HTTPAccessLogEntry_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HTTPAccessLogEntry {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HTTPAccessLogEntry {
  type Msg = HTTPAccessLogEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPAccessLogEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPAccessLogEntry {
  type Msg = HTTPAccessLogEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPAccessLogEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HTTPAccessLogEntryMut<'_> {
  type Msg = HTTPAccessLogEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPAccessLogEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPAccessLogEntryMut<'_> {
  type Msg = HTTPAccessLogEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPAccessLogEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPAccessLogEntryView<'_> {
  type Msg = HTTPAccessLogEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPAccessLogEntry> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HTTPAccessLogEntryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod h_t_t_p_access_log_entry {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HTTPVersion(i32);

#[allow(non_upper_case_globals)]
impl HTTPVersion {
  pub const ProtocolUnspecified: HTTPVersion = HTTPVersion(0);
  pub const Http10: HTTPVersion = HTTPVersion(1);
  pub const Http11: HTTPVersion = HTTPVersion(2);
  pub const Http2: HTTPVersion = HTTPVersion(3);
  pub const Http3: HTTPVersion = HTTPVersion(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "ProtocolUnspecified",
      1 => "Http10",
      2 => "Http11",
      3 => "Http2",
      4 => "Http3",
      _ => return None
    })
  }
}

impl ::std::convert::From<HTTPVersion> for i32 {
  fn from(val: HTTPVersion) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for HTTPVersion {
  fn from(val: i32) -> HTTPVersion {
    Self(val)
  }
}

impl ::std::default::Default for HTTPVersion {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for HTTPVersion {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "HTTPVersion::{}", constant_name)
    } else {
      write!(f, "HTTPVersion::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for HTTPVersion {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for HTTPVersion {}

impl ::protobuf::Proxied for HTTPVersion {
  type View<'a> = HTTPVersion;
}

impl ::protobuf::AsView for HTTPVersion {
  type Proxied = HTTPVersion;

  fn as_view(&self) -> HTTPVersion {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HTTPVersion {
  fn into_view<'shorter>(self) -> HTTPVersion where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for HTTPVersion {
  const NAME: &'static str = "HTTPVersion";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for HTTPVersion {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod h_t_t_p_access_log_entry


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__ConnectionProperties_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConnectionProperties {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConnectionProperties>
}

impl ::protobuf::Message for ConnectionProperties {
  type MessageView<'msg> = ConnectionPropertiesView<'msg>;
  type MessageMut<'msg> = ConnectionPropertiesMut<'msg>;
}

impl ::std::default::Default for ConnectionProperties {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConnectionProperties {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConnectionProperties` is `Sync` because it does not implement interior mutability.
//    Neither does `ConnectionPropertiesMut`.
unsafe impl ::std::marker::Sync for ConnectionProperties {}

// SAFETY:
// - `ConnectionProperties` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ConnectionProperties {}

impl ::protobuf::Proxied for ConnectionProperties {
  type View<'msg> = ConnectionPropertiesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConnectionProperties {}

impl ::protobuf::MutProxied for ConnectionProperties {
  type Mut<'msg> = ConnectionPropertiesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConnectionPropertiesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionPropertiesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConnectionPropertiesView<'msg> {
  type Message = ConnectionProperties;
}

impl ::std::fmt::Debug for ConnectionPropertiesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConnectionPropertiesView<'_> {
  fn default() -> ConnectionPropertiesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionProperties>> for ConnectionPropertiesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionPropertiesView<'msg> {

  pub fn to_owned(&self) -> ConnectionProperties {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // received_bytes: optional uint64
  pub fn received_bytes(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // sent_bytes: optional uint64
  pub fn sent_bytes(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ConnectionPropertiesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ConnectionPropertiesView<'_> {}

// SAFETY:
// - `ConnectionPropertiesView` is `Send` because while its alive a `ConnectionPropertiesMut` cannot.
// - `ConnectionPropertiesView` does not use thread-local data.
unsafe impl ::std::marker::Send for ConnectionPropertiesView<'_> {}

impl<'msg> ::protobuf::AsView for ConnectionPropertiesView<'msg> {
  type Proxied = ConnectionProperties;
  fn as_view(&self) -> ::protobuf::View<'msg, ConnectionProperties> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionPropertiesView<'msg> {
  fn into_view<'shorter>(self) -> ConnectionPropertiesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionProperties> for ConnectionPropertiesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionProperties {
    let mut dst = ConnectionProperties::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionProperties> for ConnectionPropertiesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionProperties {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ConnectionProperties {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConnectionPropertiesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConnectionPropertiesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConnectionPropertiesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionPropertiesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConnectionPropertiesMut<'msg> {
  type Message = ConnectionProperties;
}

impl ::std::fmt::Debug for ConnectionPropertiesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionProperties>> for ConnectionPropertiesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionPropertiesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionProperties> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ConnectionProperties {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // received_bytes: optional uint64
  pub fn received_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_received_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // sent_bytes: optional uint64
  pub fn sent_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sent_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `ConnectionPropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ConnectionPropertiesMut<'_> {}

// SAFETY:
// - `ConnectionPropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ConnectionPropertiesMut<'_> {}

impl<'msg> ::protobuf::AsView for ConnectionPropertiesMut<'msg> {
  type Proxied = ConnectionProperties;
  fn as_view(&self) -> ::protobuf::View<'_, ConnectionProperties> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionPropertiesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConnectionProperties>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ConnectionPropertiesMut<'msg> {
  type MutProxied = ConnectionProperties;
  fn as_mut(&mut self) -> ConnectionPropertiesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConnectionPropertiesMut<'msg> {
  fn into_mut<'shorter>(self) -> ConnectionPropertiesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConnectionProperties {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConnectionProperties> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConnectionPropertiesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConnectionPropertiesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // received_bytes: optional uint64
  pub fn received_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_received_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // sent_bytes: optional uint64
  pub fn sent_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sent_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}  // impl ConnectionProperties

impl ::std::ops::Drop for ConnectionProperties {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConnectionProperties {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConnectionProperties {
  type Proxied = Self;
  fn as_view(&self) -> ConnectionPropertiesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConnectionProperties {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConnectionPropertiesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConnectionProperties {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__data__accesslog__v3__ConnectionProperties_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,P,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__data__accesslog__v3__ConnectionProperties_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__data__accesslog__v3__ConnectionProperties_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionProperties {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionProperties {
  type Msg = ConnectionProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionProperties {
  type Msg = ConnectionProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionPropertiesMut<'_> {
  type Msg = ConnectionProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionPropertiesMut<'_> {
  type Msg = ConnectionProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionPropertiesView<'_> {
  type Msg = ConnectionProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionProperties> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionPropertiesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__AccessLogCommon_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AccessLogCommon {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AccessLogCommon>
}

impl ::protobuf::Message for AccessLogCommon {
  type MessageView<'msg> = AccessLogCommonView<'msg>;
  type MessageMut<'msg> = AccessLogCommonMut<'msg>;
}

impl ::std::default::Default for AccessLogCommon {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AccessLogCommon {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AccessLogCommon` is `Sync` because it does not implement interior mutability.
//    Neither does `AccessLogCommonMut`.
unsafe impl ::std::marker::Sync for AccessLogCommon {}

// SAFETY:
// - `AccessLogCommon` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AccessLogCommon {}

impl ::protobuf::Proxied for AccessLogCommon {
  type View<'msg> = AccessLogCommonView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AccessLogCommon {}

impl ::protobuf::MutProxied for AccessLogCommon {
  type Mut<'msg> = AccessLogCommonMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AccessLogCommonView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AccessLogCommon>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AccessLogCommonView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AccessLogCommonView<'msg> {
  type Message = AccessLogCommon;
}

impl ::std::fmt::Debug for AccessLogCommonView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AccessLogCommonView<'_> {
  fn default() -> AccessLogCommonView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AccessLogCommon>> for AccessLogCommonView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AccessLogCommon>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AccessLogCommonView<'msg> {

  pub fn to_owned(&self) -> AccessLogCommon {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // sample_rate: optional double
  pub fn sample_rate(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // downstream_remote_address: optional message envoy.config.core.v3.Address
  pub fn has_downstream_remote_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn downstream_remote_address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_downstream_remote_address().then(|| self.downstream_remote_address())
  }
  pub fn downstream_remote_address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // downstream_local_address: optional message envoy.config.core.v3.Address
  pub fn has_downstream_local_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn downstream_local_address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_downstream_local_address().then(|| self.downstream_local_address())
  }
  pub fn downstream_local_address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // tls_properties: optional message envoy.data.accesslog.v3.TLSProperties
  pub fn has_tls_properties(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn tls_properties_opt(self) -> ::std::option::Option<super::TLSPropertiesView<'msg>> {
    self.has_tls_properties().then(|| self.tls_properties())
  }
  pub fn tls_properties(self) -> super::TLSPropertiesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TLSPropertiesView::default())
  }

  // start_time: optional message google.protobuf.Timestamp
  pub fn has_start_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn start_time_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // time_to_last_rx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_rx_byte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn time_to_last_rx_byte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_time_to_last_rx_byte().then(|| self.time_to_last_rx_byte())
  }
  pub fn time_to_last_rx_byte(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // time_to_first_upstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_first_upstream_tx_byte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn time_to_first_upstream_tx_byte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_time_to_first_upstream_tx_byte().then(|| self.time_to_first_upstream_tx_byte())
  }
  pub fn time_to_first_upstream_tx_byte(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // time_to_last_upstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_upstream_tx_byte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn time_to_last_upstream_tx_byte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_time_to_last_upstream_tx_byte().then(|| self.time_to_last_upstream_tx_byte())
  }
  pub fn time_to_last_upstream_tx_byte(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // time_to_first_upstream_rx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_first_upstream_rx_byte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn time_to_first_upstream_rx_byte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_time_to_first_upstream_rx_byte().then(|| self.time_to_first_upstream_rx_byte())
  }
  pub fn time_to_first_upstream_rx_byte(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // time_to_last_upstream_rx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_upstream_rx_byte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn time_to_last_upstream_rx_byte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_time_to_last_upstream_rx_byte().then(|| self.time_to_last_upstream_rx_byte())
  }
  pub fn time_to_last_upstream_rx_byte(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // time_to_first_downstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_first_downstream_tx_byte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn time_to_first_downstream_tx_byte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_time_to_first_downstream_tx_byte().then(|| self.time_to_first_downstream_tx_byte())
  }
  pub fn time_to_first_downstream_tx_byte(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // time_to_last_downstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_downstream_tx_byte(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn time_to_last_downstream_tx_byte_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_time_to_last_downstream_tx_byte().then(|| self.time_to_last_downstream_tx_byte())
  }
  pub fn time_to_last_downstream_tx_byte(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // upstream_remote_address: optional message envoy.config.core.v3.Address
  pub fn has_upstream_remote_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn upstream_remote_address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_upstream_remote_address().then(|| self.upstream_remote_address())
  }
  pub fn upstream_remote_address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // upstream_local_address: optional message envoy.config.core.v3.Address
  pub fn has_upstream_local_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn upstream_local_address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_upstream_local_address().then(|| self.upstream_local_address())
  }
  pub fn upstream_local_address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // upstream_cluster: optional string
  pub fn upstream_cluster(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        14, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // response_flags: optional message envoy.data.accesslog.v3.ResponseFlags
  pub fn has_response_flags(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn response_flags_opt(self) -> ::std::option::Option<super::ResponseFlagsView<'msg>> {
    self.has_response_flags().then(|| self.response_flags())
  }
  pub fn response_flags(self) -> super::ResponseFlagsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResponseFlagsView::default())
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

  // upstream_transport_failure_reason: optional string
  pub fn upstream_transport_failure_reason(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        17, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // route_name: optional string
  pub fn route_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        18, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // downstream_direct_remote_address: optional message envoy.config.core.v3.Address
  pub fn has_downstream_direct_remote_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn downstream_direct_remote_address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_downstream_direct_remote_address().then(|| self.downstream_direct_remote_address())
  }
  pub fn downstream_direct_remote_address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // filter_state_objects: repeated message envoy.data.accesslog.v3.AccessLogCommon.FilterStateObjectsEntry
  pub fn filter_state_objects(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(20)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // custom_tags: repeated message envoy.data.accesslog.v3.AccessLogCommon.CustomTagsEntry
  pub fn custom_tags(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(21)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // duration: optional message google.protobuf.Duration
  pub fn has_duration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn duration_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_duration().then(|| self.duration())
  }
  pub fn duration(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // upstream_request_attempt_count: optional uint32
  pub fn upstream_request_attempt_count(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        23, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // connection_termination_details: optional string
  pub fn connection_termination_details(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        24, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // stream_id: optional string
  pub fn stream_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        25, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // intermediate_log_entry: optional bool
  pub fn intermediate_log_entry(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }

  // downstream_transport_failure_reason: optional string
  pub fn downstream_transport_failure_reason(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        27, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // downstream_wire_bytes_sent: optional uint64
  pub fn downstream_wire_bytes_sent(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        28, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // downstream_wire_bytes_received: optional uint64
  pub fn downstream_wire_bytes_received(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        29, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // upstream_wire_bytes_sent: optional uint64
  pub fn upstream_wire_bytes_sent(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        30, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // upstream_wire_bytes_received: optional uint64
  pub fn upstream_wire_bytes_received(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        31, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // access_log_type: optional enum envoy.data.accesslog.v3.AccessLogType
  pub fn access_log_type(self) -> super::AccessLogType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        32, (super::AccessLogType::Notset).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `AccessLogCommonView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AccessLogCommonView<'_> {}

// SAFETY:
// - `AccessLogCommonView` is `Send` because while its alive a `AccessLogCommonMut` cannot.
// - `AccessLogCommonView` does not use thread-local data.
unsafe impl ::std::marker::Send for AccessLogCommonView<'_> {}

impl<'msg> ::protobuf::AsView for AccessLogCommonView<'msg> {
  type Proxied = AccessLogCommon;
  fn as_view(&self) -> ::protobuf::View<'msg, AccessLogCommon> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AccessLogCommonView<'msg> {
  fn into_view<'shorter>(self) -> AccessLogCommonView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AccessLogCommon> for AccessLogCommonView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AccessLogCommon {
    let mut dst = AccessLogCommon::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AccessLogCommon> for AccessLogCommonMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AccessLogCommon {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AccessLogCommon {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AccessLogCommonView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AccessLogCommonMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AccessLogCommonMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLogCommon>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AccessLogCommonMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AccessLogCommonMut<'msg> {
  type Message = AccessLogCommon;
}

impl ::std::fmt::Debug for AccessLogCommonMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLogCommon>> for AccessLogCommonMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLogCommon>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AccessLogCommonMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLogCommon> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AccessLogCommon {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // sample_rate: optional double
  pub fn sample_rate(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sample_rate(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // downstream_remote_address: optional message envoy.config.core.v3.Address
  pub fn has_downstream_remote_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_downstream_remote_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn downstream_remote_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_downstream_remote_address().then(|| self.downstream_remote_address())
  }
  pub fn downstream_remote_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn downstream_remote_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_downstream_remote_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // downstream_local_address: optional message envoy.config.core.v3.Address
  pub fn has_downstream_local_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_downstream_local_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn downstream_local_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_downstream_local_address().then(|| self.downstream_local_address())
  }
  pub fn downstream_local_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn downstream_local_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_downstream_local_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // tls_properties: optional message envoy.data.accesslog.v3.TLSProperties
  pub fn has_tls_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_tls_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn tls_properties_opt(&self) -> ::std::option::Option<super::TLSPropertiesView<'_>> {
    self.has_tls_properties().then(|| self.tls_properties())
  }
  pub fn tls_properties(&self) -> super::TLSPropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TLSPropertiesView::default())
  }
  pub fn tls_properties_mut(&mut self) -> super::TLSPropertiesMut<'_> {
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
  pub fn set_tls_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::TLSProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // start_time: optional message google.protobuf.Timestamp
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn start_time_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn start_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_start_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // time_to_last_rx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_rx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_time_to_last_rx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn time_to_last_rx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_last_rx_byte().then(|| self.time_to_last_rx_byte())
  }
  pub fn time_to_last_rx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_last_rx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_last_rx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // time_to_first_upstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_first_upstream_tx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_time_to_first_upstream_tx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn time_to_first_upstream_tx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_first_upstream_tx_byte().then(|| self.time_to_first_upstream_tx_byte())
  }
  pub fn time_to_first_upstream_tx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_first_upstream_tx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_first_upstream_tx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // time_to_last_upstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_upstream_tx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_time_to_last_upstream_tx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn time_to_last_upstream_tx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_last_upstream_tx_byte().then(|| self.time_to_last_upstream_tx_byte())
  }
  pub fn time_to_last_upstream_tx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_last_upstream_tx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_last_upstream_tx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // time_to_first_upstream_rx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_first_upstream_rx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_time_to_first_upstream_rx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn time_to_first_upstream_rx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_first_upstream_rx_byte().then(|| self.time_to_first_upstream_rx_byte())
  }
  pub fn time_to_first_upstream_rx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_first_upstream_rx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_first_upstream_rx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // time_to_last_upstream_rx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_upstream_rx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_time_to_last_upstream_rx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn time_to_last_upstream_rx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_last_upstream_rx_byte().then(|| self.time_to_last_upstream_rx_byte())
  }
  pub fn time_to_last_upstream_rx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_last_upstream_rx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_last_upstream_rx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // time_to_first_downstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_first_downstream_tx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_time_to_first_downstream_tx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn time_to_first_downstream_tx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_first_downstream_tx_byte().then(|| self.time_to_first_downstream_tx_byte())
  }
  pub fn time_to_first_downstream_tx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_first_downstream_tx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_first_downstream_tx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // time_to_last_downstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_downstream_tx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_time_to_last_downstream_tx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn time_to_last_downstream_tx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_last_downstream_tx_byte().then(|| self.time_to_last_downstream_tx_byte())
  }
  pub fn time_to_last_downstream_tx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_last_downstream_tx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_time_to_last_downstream_tx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // upstream_remote_address: optional message envoy.config.core.v3.Address
  pub fn has_upstream_remote_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_upstream_remote_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn upstream_remote_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_upstream_remote_address().then(|| self.upstream_remote_address())
  }
  pub fn upstream_remote_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn upstream_remote_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_remote_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // upstream_local_address: optional message envoy.config.core.v3.Address
  pub fn has_upstream_local_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_upstream_local_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn upstream_local_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_upstream_local_address().then(|| self.upstream_local_address())
  }
  pub fn upstream_local_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn upstream_local_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_local_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // upstream_cluster: optional string
  pub fn upstream_cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        14, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upstream_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val);
    }
  }

  // response_flags: optional message envoy.data.accesslog.v3.ResponseFlags
  pub fn has_response_flags(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_response_flags(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn response_flags_opt(&self) -> ::std::option::Option<super::ResponseFlagsView<'_>> {
    self.has_response_flags().then(|| self.response_flags())
  }
  pub fn response_flags(&self) -> super::ResponseFlagsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResponseFlagsView::default())
  }
  pub fn response_flags_mut(&mut self) -> super::ResponseFlagsMut<'_> {
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
  pub fn set_response_flags(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResponseFlags>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // upstream_transport_failure_reason: optional string
  pub fn upstream_transport_failure_reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        17, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upstream_transport_failure_reason(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val);
    }
  }

  // route_name: optional string
  pub fn route_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        18, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val);
    }
  }

  // downstream_direct_remote_address: optional message envoy.config.core.v3.Address
  pub fn has_downstream_direct_remote_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_downstream_direct_remote_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn downstream_direct_remote_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_downstream_direct_remote_address().then(|| self.downstream_direct_remote_address())
  }
  pub fn downstream_direct_remote_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn downstream_direct_remote_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         19, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_downstream_direct_remote_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        val
      );
    }
  }

  // filter_state_objects: repeated message envoy.data.accesslog.v3.AccessLogCommon.FilterStateObjectsEntry
  pub fn filter_state_objects(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(20)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn filter_state_objects_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          20, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_filter_state_objects(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // custom_tags: repeated message envoy.data.accesslog.v3.AccessLogCommon.CustomTagsEntry
  pub fn custom_tags(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(21)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn custom_tags_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          21, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_custom_tags(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        src);
    }
  }

  // duration: optional message google.protobuf.Duration
  pub fn has_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn duration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_duration().then(|| self.duration())
  }
  pub fn duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         22, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // upstream_request_attempt_count: optional uint32
  pub fn upstream_request_attempt_count(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        23, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_request_attempt_count(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        23, val.into()
      )
    }
  }

  // connection_termination_details: optional string
  pub fn connection_termination_details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        24, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_connection_termination_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val);
    }
  }

  // stream_id: optional string
  pub fn stream_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        25, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stream_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val);
    }
  }

  // intermediate_log_entry: optional bool
  pub fn intermediate_log_entry(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_intermediate_log_entry(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // downstream_transport_failure_reason: optional string
  pub fn downstream_transport_failure_reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        27, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_downstream_transport_failure_reason(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        27,
        val);
    }
  }

  // downstream_wire_bytes_sent: optional uint64
  pub fn downstream_wire_bytes_sent(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        28, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_wire_bytes_sent(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        28, val.into()
      )
    }
  }

  // downstream_wire_bytes_received: optional uint64
  pub fn downstream_wire_bytes_received(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        29, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_wire_bytes_received(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        29, val.into()
      )
    }
  }

  // upstream_wire_bytes_sent: optional uint64
  pub fn upstream_wire_bytes_sent(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        30, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_wire_bytes_sent(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        30, val.into()
      )
    }
  }

  // upstream_wire_bytes_received: optional uint64
  pub fn upstream_wire_bytes_received(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        31, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_wire_bytes_received(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        31, val.into()
      )
    }
  }

  // access_log_type: optional enum envoy.data.accesslog.v3.AccessLogType
  pub fn access_log_type(&self) -> super::AccessLogType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        32, (super::AccessLogType::Notset).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_access_log_type(&mut self, val: super::AccessLogType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        32, val.into()
      )
    }
  }

}

// SAFETY:
// - `AccessLogCommonMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AccessLogCommonMut<'_> {}

// SAFETY:
// - `AccessLogCommonMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AccessLogCommonMut<'_> {}

impl<'msg> ::protobuf::AsView for AccessLogCommonMut<'msg> {
  type Proxied = AccessLogCommon;
  fn as_view(&self) -> ::protobuf::View<'_, AccessLogCommon> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AccessLogCommonMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AccessLogCommon>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AccessLogCommonMut<'msg> {
  type MutProxied = AccessLogCommon;
  fn as_mut(&mut self) -> AccessLogCommonMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AccessLogCommonMut<'msg> {
  fn into_mut<'shorter>(self) -> AccessLogCommonMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AccessLogCommon {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AccessLogCommon> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AccessLogCommonView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AccessLogCommonMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // sample_rate: optional double
  pub fn sample_rate(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sample_rate(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // downstream_remote_address: optional message envoy.config.core.v3.Address
  pub fn has_downstream_remote_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_downstream_remote_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn downstream_remote_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_downstream_remote_address().then(|| self.downstream_remote_address())
  }
  pub fn downstream_remote_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn downstream_remote_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_downstream_remote_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // downstream_local_address: optional message envoy.config.core.v3.Address
  pub fn has_downstream_local_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_downstream_local_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn downstream_local_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_downstream_local_address().then(|| self.downstream_local_address())
  }
  pub fn downstream_local_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn downstream_local_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_downstream_local_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // tls_properties: optional message envoy.data.accesslog.v3.TLSProperties
  pub fn has_tls_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_tls_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn tls_properties_opt(&self) -> ::std::option::Option<super::TLSPropertiesView<'_>> {
    self.has_tls_properties().then(|| self.tls_properties())
  }
  pub fn tls_properties(&self) -> super::TLSPropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TLSPropertiesView::default())
  }
  pub fn tls_properties_mut(&mut self) -> super::TLSPropertiesMut<'_> {
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
  pub fn set_tls_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::TLSProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // start_time: optional message google.protobuf.Timestamp
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn start_time_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn start_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_start_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // time_to_last_rx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_rx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_time_to_last_rx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn time_to_last_rx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_last_rx_byte().then(|| self.time_to_last_rx_byte())
  }
  pub fn time_to_last_rx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_last_rx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_last_rx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // time_to_first_upstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_first_upstream_tx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_time_to_first_upstream_tx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn time_to_first_upstream_tx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_first_upstream_tx_byte().then(|| self.time_to_first_upstream_tx_byte())
  }
  pub fn time_to_first_upstream_tx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_first_upstream_tx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_first_upstream_tx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // time_to_last_upstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_upstream_tx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_time_to_last_upstream_tx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn time_to_last_upstream_tx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_last_upstream_tx_byte().then(|| self.time_to_last_upstream_tx_byte())
  }
  pub fn time_to_last_upstream_tx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_last_upstream_tx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_last_upstream_tx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // time_to_first_upstream_rx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_first_upstream_rx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_time_to_first_upstream_rx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn time_to_first_upstream_rx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_first_upstream_rx_byte().then(|| self.time_to_first_upstream_rx_byte())
  }
  pub fn time_to_first_upstream_rx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_first_upstream_rx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_first_upstream_rx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // time_to_last_upstream_rx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_upstream_rx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_time_to_last_upstream_rx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn time_to_last_upstream_rx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_last_upstream_rx_byte().then(|| self.time_to_last_upstream_rx_byte())
  }
  pub fn time_to_last_upstream_rx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_last_upstream_rx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_last_upstream_rx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // time_to_first_downstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_first_downstream_tx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_time_to_first_downstream_tx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn time_to_first_downstream_tx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_first_downstream_tx_byte().then(|| self.time_to_first_downstream_tx_byte())
  }
  pub fn time_to_first_downstream_tx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_first_downstream_tx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_time_to_first_downstream_tx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // time_to_last_downstream_tx_byte: optional message google.protobuf.Duration
  pub fn has_time_to_last_downstream_tx_byte(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_time_to_last_downstream_tx_byte(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn time_to_last_downstream_tx_byte_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_time_to_last_downstream_tx_byte().then(|| self.time_to_last_downstream_tx_byte())
  }
  pub fn time_to_last_downstream_tx_byte(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn time_to_last_downstream_tx_byte_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_time_to_last_downstream_tx_byte(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // upstream_remote_address: optional message envoy.config.core.v3.Address
  pub fn has_upstream_remote_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_upstream_remote_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn upstream_remote_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_upstream_remote_address().then(|| self.upstream_remote_address())
  }
  pub fn upstream_remote_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn upstream_remote_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_remote_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // upstream_local_address: optional message envoy.config.core.v3.Address
  pub fn has_upstream_local_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_upstream_local_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn upstream_local_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_upstream_local_address().then(|| self.upstream_local_address())
  }
  pub fn upstream_local_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn upstream_local_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_local_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // upstream_cluster: optional string
  pub fn upstream_cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        14, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upstream_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val);
    }
  }

  // response_flags: optional message envoy.data.accesslog.v3.ResponseFlags
  pub fn has_response_flags(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_response_flags(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn response_flags_opt(&self) -> ::std::option::Option<super::ResponseFlagsView<'_>> {
    self.has_response_flags().then(|| self.response_flags())
  }
  pub fn response_flags(&self) -> super::ResponseFlagsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResponseFlagsView::default())
  }
  pub fn response_flags_mut(&mut self) -> super::ResponseFlagsMut<'_> {
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
  pub fn set_response_flags(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResponseFlags>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // upstream_transport_failure_reason: optional string
  pub fn upstream_transport_failure_reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        17, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_upstream_transport_failure_reason(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val);
    }
  }

  // route_name: optional string
  pub fn route_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        18, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val);
    }
  }

  // downstream_direct_remote_address: optional message envoy.config.core.v3.Address
  pub fn has_downstream_direct_remote_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_downstream_direct_remote_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn downstream_direct_remote_address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_downstream_direct_remote_address().then(|| self.downstream_direct_remote_address())
  }
  pub fn downstream_direct_remote_address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn downstream_direct_remote_address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         19, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_downstream_direct_remote_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        val
      );
    }
  }

  // filter_state_objects: repeated message envoy.data.accesslog.v3.AccessLogCommon.FilterStateObjectsEntry
  pub fn filter_state_objects(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(20)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn filter_state_objects_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          20, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_filter_state_objects(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // custom_tags: repeated message envoy.data.accesslog.v3.AccessLogCommon.CustomTagsEntry
  pub fn custom_tags(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(21)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn custom_tags_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          21, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_custom_tags(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        src);
    }
  }

  // duration: optional message google.protobuf.Duration
  pub fn has_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn duration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_duration().then(|| self.duration())
  }
  pub fn duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         22, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // upstream_request_attempt_count: optional uint32
  pub fn upstream_request_attempt_count(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        23, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_request_attempt_count(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        23, val.into()
      )
    }
  }

  // connection_termination_details: optional string
  pub fn connection_termination_details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        24, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_connection_termination_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val);
    }
  }

  // stream_id: optional string
  pub fn stream_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        25, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stream_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val);
    }
  }

  // intermediate_log_entry: optional bool
  pub fn intermediate_log_entry(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_intermediate_log_entry(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // downstream_transport_failure_reason: optional string
  pub fn downstream_transport_failure_reason(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        27, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_downstream_transport_failure_reason(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        27,
        val);
    }
  }

  // downstream_wire_bytes_sent: optional uint64
  pub fn downstream_wire_bytes_sent(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        28, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_wire_bytes_sent(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        28, val.into()
      )
    }
  }

  // downstream_wire_bytes_received: optional uint64
  pub fn downstream_wire_bytes_received(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        29, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_wire_bytes_received(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        29, val.into()
      )
    }
  }

  // upstream_wire_bytes_sent: optional uint64
  pub fn upstream_wire_bytes_sent(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        30, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_wire_bytes_sent(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        30, val.into()
      )
    }
  }

  // upstream_wire_bytes_received: optional uint64
  pub fn upstream_wire_bytes_received(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        31, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_wire_bytes_received(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        31, val.into()
      )
    }
  }

  // access_log_type: optional enum envoy.data.accesslog.v3.AccessLogType
  pub fn access_log_type(&self) -> super::AccessLogType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        32, (super::AccessLogType::Notset).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_access_log_type(&mut self, val: super::AccessLogType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        32, val.into()
      )
    }
  }

}  // impl AccessLogCommon

impl ::std::ops::Drop for AccessLogCommon {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AccessLogCommon {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AccessLogCommon {
  type Proxied = Self;
  fn as_view(&self) -> AccessLogCommonView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AccessLogCommon {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AccessLogCommonMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AccessLogCommon {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__data__accesslog__v3__AccessLogCommon_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ P33333333333331X331X1X3GG3)P1X1X/P1X,P,P,P,P.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__data__accesslog__v3__AccessLogCommon_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TLSProperties as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResponseFlags as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::access_log_common::FilterStateObjectsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::access_log_common::CustomTagsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__data__accesslog__v3__AccessLogCommon_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AccessLogCommon {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AccessLogCommon {
  type Msg = AccessLogCommon;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogCommon> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessLogCommon {
  type Msg = AccessLogCommon;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogCommon> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AccessLogCommonMut<'_> {
  type Msg = AccessLogCommon;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogCommon> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessLogCommonMut<'_> {
  type Msg = AccessLogCommon;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogCommon> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessLogCommonView<'_> {
  type Msg = AccessLogCommon;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogCommon> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AccessLogCommonMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod access_log_common {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__AccessLogCommon__FilterStateObjectsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct FilterStateObjectsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FilterStateObjectsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::access_log_common::envoy__data__accesslog__v3__AccessLogCommon__FilterStateObjectsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::access_log_common::envoy__data__accesslog__v3__AccessLogCommon__FilterStateObjectsEntry_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::access_log_common::envoy__data__accesslog__v3__AccessLogCommon__FilterStateObjectsEntry_msg_init.0)
      }).0
    }
  }
}
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__AccessLogCommon__CustomTagsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct CustomTagsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CustomTagsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::access_log_common::envoy__data__accesslog__v3__AccessLogCommon__CustomTagsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::access_log_common::envoy__data__accesslog__v3__AccessLogCommon__CustomTagsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::access_log_common::envoy__data__accesslog__v3__AccessLogCommon__CustomTagsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod access_log_common


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__ResponseFlags_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResponseFlags {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResponseFlags>
}

impl ::protobuf::Message for ResponseFlags {
  type MessageView<'msg> = ResponseFlagsView<'msg>;
  type MessageMut<'msg> = ResponseFlagsMut<'msg>;
}

impl ::std::default::Default for ResponseFlags {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResponseFlags {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResponseFlags` is `Sync` because it does not implement interior mutability.
//    Neither does `ResponseFlagsMut`.
unsafe impl ::std::marker::Sync for ResponseFlags {}

// SAFETY:
// - `ResponseFlags` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ResponseFlags {}

impl ::protobuf::Proxied for ResponseFlags {
  type View<'msg> = ResponseFlagsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResponseFlags {}

impl ::protobuf::MutProxied for ResponseFlags {
  type Mut<'msg> = ResponseFlagsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResponseFlagsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResponseFlags>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResponseFlagsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResponseFlagsView<'msg> {
  type Message = ResponseFlags;
}

impl ::std::fmt::Debug for ResponseFlagsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResponseFlagsView<'_> {
  fn default() -> ResponseFlagsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResponseFlags>> for ResponseFlagsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResponseFlags>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResponseFlagsView<'msg> {

  pub fn to_owned(&self) -> ResponseFlags {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // failed_local_healthcheck: optional bool
  pub fn failed_local_healthcheck(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

  // no_healthy_upstream: optional bool
  pub fn no_healthy_upstream(self) -> bool {
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

  // upstream_request_timeout: optional bool
  pub fn upstream_request_timeout(self) -> bool {
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

  // local_reset: optional bool
  pub fn local_reset(self) -> bool {
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

  // upstream_remote_reset: optional bool
  pub fn upstream_remote_reset(self) -> bool {
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

  // upstream_connection_failure: optional bool
  pub fn upstream_connection_failure(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }

  // upstream_connection_termination: optional bool
  pub fn upstream_connection_termination(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }

  // upstream_overflow: optional bool
  pub fn upstream_overflow(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

  // no_route_found: optional bool
  pub fn no_route_found(self) -> bool {
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

  // delay_injected: optional bool
  pub fn delay_injected(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }

  // fault_injected: optional bool
  pub fn fault_injected(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }

  // rate_limited: optional bool
  pub fn rate_limited(self) -> bool {
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

  // unauthorized_details: optional message envoy.data.accesslog.v3.ResponseFlags.Unauthorized
  pub fn has_unauthorized_details(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn unauthorized_details_opt(self) -> ::std::option::Option<super::response_flags::UnauthorizedView<'msg>> {
    self.has_unauthorized_details().then(|| self.unauthorized_details())
  }
  pub fn unauthorized_details(self) -> super::response_flags::UnauthorizedView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::response_flags::UnauthorizedView::default())
  }

  // rate_limit_service_error: optional bool
  pub fn rate_limit_service_error(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }

  // downstream_connection_termination: optional bool
  pub fn downstream_connection_termination(self) -> bool {
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

  // upstream_retry_limit_exceeded: optional bool
  pub fn upstream_retry_limit_exceeded(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }

  // stream_idle_timeout: optional bool
  pub fn stream_idle_timeout(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        16, (false).into()
      ).try_into().unwrap()
    }
  }

  // invalid_envoy_request_headers: optional bool
  pub fn invalid_envoy_request_headers(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }

  // downstream_protocol_error: optional bool
  pub fn downstream_protocol_error(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        18, (false).into()
      ).try_into().unwrap()
    }
  }

  // upstream_max_stream_duration_reached: optional bool
  pub fn upstream_max_stream_duration_reached(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        19, (false).into()
      ).try_into().unwrap()
    }
  }

  // response_from_cache_filter: optional bool
  pub fn response_from_cache_filter(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        20, (false).into()
      ).try_into().unwrap()
    }
  }

  // no_filter_config_found: optional bool
  pub fn no_filter_config_found(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        21, (false).into()
      ).try_into().unwrap()
    }
  }

  // duration_timeout: optional bool
  pub fn duration_timeout(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        22, (false).into()
      ).try_into().unwrap()
    }
  }

  // upstream_protocol_error: optional bool
  pub fn upstream_protocol_error(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        23, (false).into()
      ).try_into().unwrap()
    }
  }

  // no_cluster_found: optional bool
  pub fn no_cluster_found(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        24, (false).into()
      ).try_into().unwrap()
    }
  }

  // overload_manager: optional bool
  pub fn overload_manager(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        25, (false).into()
      ).try_into().unwrap()
    }
  }

  // dns_resolution_failure: optional bool
  pub fn dns_resolution_failure(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }

  // downstream_remote_reset: optional bool
  pub fn downstream_remote_reset(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        27, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ResponseFlagsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResponseFlagsView<'_> {}

// SAFETY:
// - `ResponseFlagsView` is `Send` because while its alive a `ResponseFlagsMut` cannot.
// - `ResponseFlagsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResponseFlagsView<'_> {}

impl<'msg> ::protobuf::AsView for ResponseFlagsView<'msg> {
  type Proxied = ResponseFlags;
  fn as_view(&self) -> ::protobuf::View<'msg, ResponseFlags> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResponseFlagsView<'msg> {
  fn into_view<'shorter>(self) -> ResponseFlagsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResponseFlags> for ResponseFlagsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResponseFlags {
    let mut dst = ResponseFlags::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResponseFlags> for ResponseFlagsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResponseFlags {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ResponseFlags {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResponseFlagsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResponseFlagsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResponseFlagsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseFlags>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResponseFlagsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResponseFlagsMut<'msg> {
  type Message = ResponseFlags;
}

impl ::std::fmt::Debug for ResponseFlagsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseFlags>> for ResponseFlagsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseFlags>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResponseFlagsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseFlags> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ResponseFlags {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // failed_local_healthcheck: optional bool
  pub fn failed_local_healthcheck(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_failed_local_healthcheck(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // no_healthy_upstream: optional bool
  pub fn no_healthy_upstream(&self) -> bool {
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
  pub fn set_no_healthy_upstream(&mut self, val: bool) {
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

  // upstream_request_timeout: optional bool
  pub fn upstream_request_timeout(&self) -> bool {
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
  pub fn set_upstream_request_timeout(&mut self, val: bool) {
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

  // local_reset: optional bool
  pub fn local_reset(&self) -> bool {
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
  pub fn set_local_reset(&mut self, val: bool) {
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

  // upstream_remote_reset: optional bool
  pub fn upstream_remote_reset(&self) -> bool {
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
  pub fn set_upstream_remote_reset(&mut self, val: bool) {
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

  // upstream_connection_failure: optional bool
  pub fn upstream_connection_failure(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_connection_failure(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // upstream_connection_termination: optional bool
  pub fn upstream_connection_termination(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_connection_termination(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // upstream_overflow: optional bool
  pub fn upstream_overflow(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_overflow(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // no_route_found: optional bool
  pub fn no_route_found(&self) -> bool {
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
  pub fn set_no_route_found(&mut self, val: bool) {
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

  // delay_injected: optional bool
  pub fn delay_injected(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delay_injected(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

  // fault_injected: optional bool
  pub fn fault_injected(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fault_injected(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        10, val.into()
      )
    }
  }

  // rate_limited: optional bool
  pub fn rate_limited(&self) -> bool {
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
  pub fn set_rate_limited(&mut self, val: bool) {
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

  // unauthorized_details: optional message envoy.data.accesslog.v3.ResponseFlags.Unauthorized
  pub fn has_unauthorized_details(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_unauthorized_details(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn unauthorized_details_opt(&self) -> ::std::option::Option<super::response_flags::UnauthorizedView<'_>> {
    self.has_unauthorized_details().then(|| self.unauthorized_details())
  }
  pub fn unauthorized_details(&self) -> super::response_flags::UnauthorizedView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::response_flags::UnauthorizedView::default())
  }
  pub fn unauthorized_details_mut(&mut self) -> super::response_flags::UnauthorizedMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_unauthorized_details(&mut self,
    val: impl ::protobuf::IntoProxied<super::response_flags::Unauthorized>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // rate_limit_service_error: optional bool
  pub fn rate_limit_service_error(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_rate_limit_service_error(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // downstream_connection_termination: optional bool
  pub fn downstream_connection_termination(&self) -> bool {
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
  pub fn set_downstream_connection_termination(&mut self, val: bool) {
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

  // upstream_retry_limit_exceeded: optional bool
  pub fn upstream_retry_limit_exceeded(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_retry_limit_exceeded(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        15, val.into()
      )
    }
  }

  // stream_idle_timeout: optional bool
  pub fn stream_idle_timeout(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        16, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stream_idle_timeout(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        16, val.into()
      )
    }
  }

  // invalid_envoy_request_headers: optional bool
  pub fn invalid_envoy_request_headers(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_invalid_envoy_request_headers(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // downstream_protocol_error: optional bool
  pub fn downstream_protocol_error(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        18, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_protocol_error(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        18, val.into()
      )
    }
  }

  // upstream_max_stream_duration_reached: optional bool
  pub fn upstream_max_stream_duration_reached(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        19, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_max_stream_duration_reached(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        19, val.into()
      )
    }
  }

  // response_from_cache_filter: optional bool
  pub fn response_from_cache_filter(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        20, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_from_cache_filter(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        20, val.into()
      )
    }
  }

  // no_filter_config_found: optional bool
  pub fn no_filter_config_found(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        21, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_no_filter_config_found(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        21, val.into()
      )
    }
  }

  // duration_timeout: optional bool
  pub fn duration_timeout(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        22, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_duration_timeout(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        22, val.into()
      )
    }
  }

  // upstream_protocol_error: optional bool
  pub fn upstream_protocol_error(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        23, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_protocol_error(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        23, val.into()
      )
    }
  }

  // no_cluster_found: optional bool
  pub fn no_cluster_found(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        24, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_no_cluster_found(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        24, val.into()
      )
    }
  }

  // overload_manager: optional bool
  pub fn overload_manager(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        25, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_overload_manager(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        25, val.into()
      )
    }
  }

  // dns_resolution_failure: optional bool
  pub fn dns_resolution_failure(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_dns_resolution_failure(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // downstream_remote_reset: optional bool
  pub fn downstream_remote_reset(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        27, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_remote_reset(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        27, val.into()
      )
    }
  }

}

// SAFETY:
// - `ResponseFlagsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResponseFlagsMut<'_> {}

// SAFETY:
// - `ResponseFlagsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResponseFlagsMut<'_> {}

impl<'msg> ::protobuf::AsView for ResponseFlagsMut<'msg> {
  type Proxied = ResponseFlags;
  fn as_view(&self) -> ::protobuf::View<'_, ResponseFlags> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResponseFlagsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResponseFlags>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResponseFlagsMut<'msg> {
  type MutProxied = ResponseFlags;
  fn as_mut(&mut self) -> ResponseFlagsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResponseFlagsMut<'msg> {
  fn into_mut<'shorter>(self) -> ResponseFlagsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResponseFlags {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResponseFlags> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResponseFlagsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResponseFlagsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // failed_local_healthcheck: optional bool
  pub fn failed_local_healthcheck(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_failed_local_healthcheck(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

  // no_healthy_upstream: optional bool
  pub fn no_healthy_upstream(&self) -> bool {
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
  pub fn set_no_healthy_upstream(&mut self, val: bool) {
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

  // upstream_request_timeout: optional bool
  pub fn upstream_request_timeout(&self) -> bool {
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
  pub fn set_upstream_request_timeout(&mut self, val: bool) {
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

  // local_reset: optional bool
  pub fn local_reset(&self) -> bool {
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
  pub fn set_local_reset(&mut self, val: bool) {
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

  // upstream_remote_reset: optional bool
  pub fn upstream_remote_reset(&self) -> bool {
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
  pub fn set_upstream_remote_reset(&mut self, val: bool) {
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

  // upstream_connection_failure: optional bool
  pub fn upstream_connection_failure(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_connection_failure(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // upstream_connection_termination: optional bool
  pub fn upstream_connection_termination(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_connection_termination(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // upstream_overflow: optional bool
  pub fn upstream_overflow(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_overflow(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // no_route_found: optional bool
  pub fn no_route_found(&self) -> bool {
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
  pub fn set_no_route_found(&mut self, val: bool) {
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

  // delay_injected: optional bool
  pub fn delay_injected(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delay_injected(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

  // fault_injected: optional bool
  pub fn fault_injected(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fault_injected(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        10, val.into()
      )
    }
  }

  // rate_limited: optional bool
  pub fn rate_limited(&self) -> bool {
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
  pub fn set_rate_limited(&mut self, val: bool) {
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

  // unauthorized_details: optional message envoy.data.accesslog.v3.ResponseFlags.Unauthorized
  pub fn has_unauthorized_details(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_unauthorized_details(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn unauthorized_details_opt(&self) -> ::std::option::Option<super::response_flags::UnauthorizedView<'_>> {
    self.has_unauthorized_details().then(|| self.unauthorized_details())
  }
  pub fn unauthorized_details(&self) -> super::response_flags::UnauthorizedView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::response_flags::UnauthorizedView::default())
  }
  pub fn unauthorized_details_mut(&mut self) -> super::response_flags::UnauthorizedMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_unauthorized_details(&mut self,
    val: impl ::protobuf::IntoProxied<super::response_flags::Unauthorized>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // rate_limit_service_error: optional bool
  pub fn rate_limit_service_error(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_rate_limit_service_error(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // downstream_connection_termination: optional bool
  pub fn downstream_connection_termination(&self) -> bool {
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
  pub fn set_downstream_connection_termination(&mut self, val: bool) {
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

  // upstream_retry_limit_exceeded: optional bool
  pub fn upstream_retry_limit_exceeded(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_retry_limit_exceeded(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        15, val.into()
      )
    }
  }

  // stream_idle_timeout: optional bool
  pub fn stream_idle_timeout(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        16, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stream_idle_timeout(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        16, val.into()
      )
    }
  }

  // invalid_envoy_request_headers: optional bool
  pub fn invalid_envoy_request_headers(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_invalid_envoy_request_headers(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // downstream_protocol_error: optional bool
  pub fn downstream_protocol_error(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        18, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_protocol_error(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        18, val.into()
      )
    }
  }

  // upstream_max_stream_duration_reached: optional bool
  pub fn upstream_max_stream_duration_reached(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        19, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_max_stream_duration_reached(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        19, val.into()
      )
    }
  }

  // response_from_cache_filter: optional bool
  pub fn response_from_cache_filter(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        20, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_from_cache_filter(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        20, val.into()
      )
    }
  }

  // no_filter_config_found: optional bool
  pub fn no_filter_config_found(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        21, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_no_filter_config_found(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        21, val.into()
      )
    }
  }

  // duration_timeout: optional bool
  pub fn duration_timeout(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        22, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_duration_timeout(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        22, val.into()
      )
    }
  }

  // upstream_protocol_error: optional bool
  pub fn upstream_protocol_error(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        23, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_protocol_error(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        23, val.into()
      )
    }
  }

  // no_cluster_found: optional bool
  pub fn no_cluster_found(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        24, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_no_cluster_found(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        24, val.into()
      )
    }
  }

  // overload_manager: optional bool
  pub fn overload_manager(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        25, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_overload_manager(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        25, val.into()
      )
    }
  }

  // dns_resolution_failure: optional bool
  pub fn dns_resolution_failure(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_dns_resolution_failure(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // downstream_remote_reset: optional bool
  pub fn downstream_remote_reset(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        27, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_remote_reset(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        27, val.into()
      )
    }
  }

}  // impl ResponseFlags

impl ::std::ops::Drop for ResponseFlags {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResponseFlags {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResponseFlags {
  type Proxied = Self;
  fn as_view(&self) -> ResponseFlagsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResponseFlags {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResponseFlagsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResponseFlags {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__data__accesslog__v3__ResponseFlags_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P/P/P/P/P/P/P/P/P/P/P/P3/P/P/P/P/P/P/P/P/P/P/P/P/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__data__accesslog__v3__ResponseFlags_msg_init.0, &[<super::response_flags::Unauthorized as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__data__accesslog__v3__ResponseFlags_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResponseFlags {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResponseFlags {
  type Msg = ResponseFlags;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlags> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResponseFlags {
  type Msg = ResponseFlags;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlags> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResponseFlagsMut<'_> {
  type Msg = ResponseFlags;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlags> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResponseFlagsMut<'_> {
  type Msg = ResponseFlags;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlags> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResponseFlagsView<'_> {
  type Msg = ResponseFlags;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlags> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResponseFlagsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod response_flags {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__ResponseFlags__Unauthorized_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Unauthorized {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Unauthorized>
}

impl ::protobuf::Message for Unauthorized {
  type MessageView<'msg> = UnauthorizedView<'msg>;
  type MessageMut<'msg> = UnauthorizedMut<'msg>;
}

impl ::std::default::Default for Unauthorized {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Unauthorized {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Unauthorized` is `Sync` because it does not implement interior mutability.
//    Neither does `UnauthorizedMut`.
unsafe impl ::std::marker::Sync for Unauthorized {}

// SAFETY:
// - `Unauthorized` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Unauthorized {}

impl ::protobuf::Proxied for Unauthorized {
  type View<'msg> = UnauthorizedView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Unauthorized {}

impl ::protobuf::MutProxied for Unauthorized {
  type Mut<'msg> = UnauthorizedMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UnauthorizedView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Unauthorized>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnauthorizedView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UnauthorizedView<'msg> {
  type Message = Unauthorized;
}

impl ::std::fmt::Debug for UnauthorizedView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UnauthorizedView<'_> {
  fn default() -> UnauthorizedView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Unauthorized>> for UnauthorizedView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Unauthorized>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UnauthorizedView<'msg> {

  pub fn to_owned(&self) -> Unauthorized {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // reason: optional enum envoy.data.accesslog.v3.ResponseFlags.Unauthorized.Reason
  pub fn reason(self) -> super::super::response_flags::unauthorized::Reason {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::response_flags::unauthorized::Reason::Unspecified).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `UnauthorizedView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UnauthorizedView<'_> {}

// SAFETY:
// - `UnauthorizedView` is `Send` because while its alive a `UnauthorizedMut` cannot.
// - `UnauthorizedView` does not use thread-local data.
unsafe impl ::std::marker::Send for UnauthorizedView<'_> {}

impl<'msg> ::protobuf::AsView for UnauthorizedView<'msg> {
  type Proxied = Unauthorized;
  fn as_view(&self) -> ::protobuf::View<'msg, Unauthorized> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnauthorizedView<'msg> {
  fn into_view<'shorter>(self) -> UnauthorizedView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Unauthorized> for UnauthorizedView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Unauthorized {
    let mut dst = Unauthorized::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Unauthorized> for UnauthorizedMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Unauthorized {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Unauthorized {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UnauthorizedView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UnauthorizedMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UnauthorizedMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Unauthorized>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UnauthorizedMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UnauthorizedMut<'msg> {
  type Message = Unauthorized;
}

impl ::std::fmt::Debug for UnauthorizedMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Unauthorized>> for UnauthorizedMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Unauthorized>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UnauthorizedMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Unauthorized> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Unauthorized {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // reason: optional enum envoy.data.accesslog.v3.ResponseFlags.Unauthorized.Reason
  pub fn reason(&self) -> super::super::response_flags::unauthorized::Reason {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::response_flags::unauthorized::Reason::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_reason(&mut self, val: super::super::response_flags::unauthorized::Reason) {
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

}

// SAFETY:
// - `UnauthorizedMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UnauthorizedMut<'_> {}

// SAFETY:
// - `UnauthorizedMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UnauthorizedMut<'_> {}

impl<'msg> ::protobuf::AsView for UnauthorizedMut<'msg> {
  type Proxied = Unauthorized;
  fn as_view(&self) -> ::protobuf::View<'_, Unauthorized> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UnauthorizedMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Unauthorized>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UnauthorizedMut<'msg> {
  type MutProxied = Unauthorized;
  fn as_mut(&mut self) -> UnauthorizedMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UnauthorizedMut<'msg> {
  fn into_mut<'shorter>(self) -> UnauthorizedMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Unauthorized {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Unauthorized> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UnauthorizedView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UnauthorizedMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // reason: optional enum envoy.data.accesslog.v3.ResponseFlags.Unauthorized.Reason
  pub fn reason(&self) -> super::super::response_flags::unauthorized::Reason {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::response_flags::unauthorized::Reason::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_reason(&mut self, val: super::super::response_flags::unauthorized::Reason) {
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

}  // impl Unauthorized

impl ::std::ops::Drop for Unauthorized {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Unauthorized {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Unauthorized {
  type Proxied = Self;
  fn as_view(&self) -> UnauthorizedView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Unauthorized {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UnauthorizedMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Unauthorized {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::response_flags::envoy__data__accesslog__v3__ResponseFlags__Unauthorized_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::response_flags::envoy__data__accesslog__v3__ResponseFlags__Unauthorized_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::response_flags::envoy__data__accesslog__v3__ResponseFlags__Unauthorized_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Unauthorized {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Unauthorized {
  type Msg = Unauthorized;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Unauthorized> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Unauthorized {
  type Msg = Unauthorized;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Unauthorized> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UnauthorizedMut<'_> {
  type Msg = Unauthorized;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Unauthorized> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UnauthorizedMut<'_> {
  type Msg = Unauthorized;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Unauthorized> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UnauthorizedView<'_> {
  type Msg = Unauthorized;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Unauthorized> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UnauthorizedMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod unauthorized {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Reason(i32);

#[allow(non_upper_case_globals)]
impl Reason {
  pub const Unspecified: Reason = Reason(0);
  pub const ExternalService: Reason = Reason(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "ExternalService",
      _ => return None
    })
  }
}

impl ::std::convert::From<Reason> for i32 {
  fn from(val: Reason) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Reason {
  fn from(val: i32) -> Reason {
    Self(val)
  }
}

impl ::std::default::Default for Reason {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Reason {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Reason::{}", constant_name)
    } else {
      write!(f, "Reason::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Reason {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Reason {}

impl ::protobuf::Proxied for Reason {
  type View<'a> = Reason;
}

impl ::protobuf::AsView for Reason {
  type Proxied = Reason;

  fn as_view(&self) -> Reason {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Reason {
  fn into_view<'shorter>(self) -> Reason where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Reason {
  const NAME: &'static str = "Reason";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for Reason {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod unauthorized


}  // pub mod response_flags


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__TLSProperties_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TLSProperties {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TLSProperties>
}

impl ::protobuf::Message for TLSProperties {
  type MessageView<'msg> = TLSPropertiesView<'msg>;
  type MessageMut<'msg> = TLSPropertiesMut<'msg>;
}

impl ::std::default::Default for TLSProperties {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TLSProperties {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TLSProperties` is `Sync` because it does not implement interior mutability.
//    Neither does `TLSPropertiesMut`.
unsafe impl ::std::marker::Sync for TLSProperties {}

// SAFETY:
// - `TLSProperties` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TLSProperties {}

impl ::protobuf::Proxied for TLSProperties {
  type View<'msg> = TLSPropertiesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TLSProperties {}

impl ::protobuf::MutProxied for TLSProperties {
  type Mut<'msg> = TLSPropertiesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TLSPropertiesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TLSProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TLSPropertiesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TLSPropertiesView<'msg> {
  type Message = TLSProperties;
}

impl ::std::fmt::Debug for TLSPropertiesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TLSPropertiesView<'_> {
  fn default() -> TLSPropertiesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TLSProperties>> for TLSPropertiesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TLSProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TLSPropertiesView<'msg> {

  pub fn to_owned(&self) -> TLSProperties {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // tls_version: optional enum envoy.data.accesslog.v3.TLSProperties.TLSVersion
  pub fn tls_version(self) -> super::t_l_s_properties::TLSVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::t_l_s_properties::TLSVersion::VersionUnspecified).into()
      ).try_into().unwrap()
    }
  }

  // tls_cipher_suite: optional message google.protobuf.UInt32Value
  pub fn has_tls_cipher_suite(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn tls_cipher_suite_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_tls_cipher_suite().then(|| self.tls_cipher_suite())
  }
  pub fn tls_cipher_suite(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // tls_sni_hostname: optional string
  pub fn tls_sni_hostname(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // local_certificate_properties: optional message envoy.data.accesslog.v3.TLSProperties.CertificateProperties
  pub fn has_local_certificate_properties(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn local_certificate_properties_opt(self) -> ::std::option::Option<super::t_l_s_properties::CertificatePropertiesView<'msg>> {
    self.has_local_certificate_properties().then(|| self.local_certificate_properties())
  }
  pub fn local_certificate_properties(self) -> super::t_l_s_properties::CertificatePropertiesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::t_l_s_properties::CertificatePropertiesView::default())
  }

  // peer_certificate_properties: optional message envoy.data.accesslog.v3.TLSProperties.CertificateProperties
  pub fn has_peer_certificate_properties(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn peer_certificate_properties_opt(self) -> ::std::option::Option<super::t_l_s_properties::CertificatePropertiesView<'msg>> {
    self.has_peer_certificate_properties().then(|| self.peer_certificate_properties())
  }
  pub fn peer_certificate_properties(self) -> super::t_l_s_properties::CertificatePropertiesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::t_l_s_properties::CertificatePropertiesView::default())
  }

  // tls_session_id: optional string
  pub fn tls_session_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // ja3_fingerprint: optional string
  pub fn ja3_fingerprint(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `TLSPropertiesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TLSPropertiesView<'_> {}

// SAFETY:
// - `TLSPropertiesView` is `Send` because while its alive a `TLSPropertiesMut` cannot.
// - `TLSPropertiesView` does not use thread-local data.
unsafe impl ::std::marker::Send for TLSPropertiesView<'_> {}

impl<'msg> ::protobuf::AsView for TLSPropertiesView<'msg> {
  type Proxied = TLSProperties;
  fn as_view(&self) -> ::protobuf::View<'msg, TLSProperties> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TLSPropertiesView<'msg> {
  fn into_view<'shorter>(self) -> TLSPropertiesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TLSProperties> for TLSPropertiesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TLSProperties {
    let mut dst = TLSProperties::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TLSProperties> for TLSPropertiesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TLSProperties {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TLSProperties {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TLSPropertiesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TLSPropertiesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TLSPropertiesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TLSProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TLSPropertiesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TLSPropertiesMut<'msg> {
  type Message = TLSProperties;
}

impl ::std::fmt::Debug for TLSPropertiesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TLSProperties>> for TLSPropertiesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TLSProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TLSPropertiesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TLSProperties> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TLSProperties {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // tls_version: optional enum envoy.data.accesslog.v3.TLSProperties.TLSVersion
  pub fn tls_version(&self) -> super::t_l_s_properties::TLSVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::t_l_s_properties::TLSVersion::VersionUnspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_tls_version(&mut self, val: super::t_l_s_properties::TLSVersion) {
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

  // tls_cipher_suite: optional message google.protobuf.UInt32Value
  pub fn has_tls_cipher_suite(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_tls_cipher_suite(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn tls_cipher_suite_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_tls_cipher_suite().then(|| self.tls_cipher_suite())
  }
  pub fn tls_cipher_suite(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn tls_cipher_suite_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_tls_cipher_suite(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // tls_sni_hostname: optional string
  pub fn tls_sni_hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tls_sni_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // local_certificate_properties: optional message envoy.data.accesslog.v3.TLSProperties.CertificateProperties
  pub fn has_local_certificate_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_local_certificate_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn local_certificate_properties_opt(&self) -> ::std::option::Option<super::t_l_s_properties::CertificatePropertiesView<'_>> {
    self.has_local_certificate_properties().then(|| self.local_certificate_properties())
  }
  pub fn local_certificate_properties(&self) -> super::t_l_s_properties::CertificatePropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::t_l_s_properties::CertificatePropertiesView::default())
  }
  pub fn local_certificate_properties_mut(&mut self) -> super::t_l_s_properties::CertificatePropertiesMut<'_> {
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
  pub fn set_local_certificate_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::t_l_s_properties::CertificateProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // peer_certificate_properties: optional message envoy.data.accesslog.v3.TLSProperties.CertificateProperties
  pub fn has_peer_certificate_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_peer_certificate_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn peer_certificate_properties_opt(&self) -> ::std::option::Option<super::t_l_s_properties::CertificatePropertiesView<'_>> {
    self.has_peer_certificate_properties().then(|| self.peer_certificate_properties())
  }
  pub fn peer_certificate_properties(&self) -> super::t_l_s_properties::CertificatePropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::t_l_s_properties::CertificatePropertiesView::default())
  }
  pub fn peer_certificate_properties_mut(&mut self) -> super::t_l_s_properties::CertificatePropertiesMut<'_> {
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
  pub fn set_peer_certificate_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::t_l_s_properties::CertificateProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // tls_session_id: optional string
  pub fn tls_session_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tls_session_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // ja3_fingerprint: optional string
  pub fn ja3_fingerprint(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_ja3_fingerprint(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

}

// SAFETY:
// - `TLSPropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TLSPropertiesMut<'_> {}

// SAFETY:
// - `TLSPropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TLSPropertiesMut<'_> {}

impl<'msg> ::protobuf::AsView for TLSPropertiesMut<'msg> {
  type Proxied = TLSProperties;
  fn as_view(&self) -> ::protobuf::View<'_, TLSProperties> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TLSPropertiesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TLSProperties>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TLSPropertiesMut<'msg> {
  type MutProxied = TLSProperties;
  fn as_mut(&mut self) -> TLSPropertiesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TLSPropertiesMut<'msg> {
  fn into_mut<'shorter>(self) -> TLSPropertiesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TLSProperties {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TLSProperties> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TLSPropertiesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TLSPropertiesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // tls_version: optional enum envoy.data.accesslog.v3.TLSProperties.TLSVersion
  pub fn tls_version(&self) -> super::t_l_s_properties::TLSVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::t_l_s_properties::TLSVersion::VersionUnspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_tls_version(&mut self, val: super::t_l_s_properties::TLSVersion) {
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

  // tls_cipher_suite: optional message google.protobuf.UInt32Value
  pub fn has_tls_cipher_suite(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_tls_cipher_suite(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn tls_cipher_suite_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_tls_cipher_suite().then(|| self.tls_cipher_suite())
  }
  pub fn tls_cipher_suite(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn tls_cipher_suite_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_tls_cipher_suite(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // tls_sni_hostname: optional string
  pub fn tls_sni_hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tls_sni_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // local_certificate_properties: optional message envoy.data.accesslog.v3.TLSProperties.CertificateProperties
  pub fn has_local_certificate_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_local_certificate_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn local_certificate_properties_opt(&self) -> ::std::option::Option<super::t_l_s_properties::CertificatePropertiesView<'_>> {
    self.has_local_certificate_properties().then(|| self.local_certificate_properties())
  }
  pub fn local_certificate_properties(&self) -> super::t_l_s_properties::CertificatePropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::t_l_s_properties::CertificatePropertiesView::default())
  }
  pub fn local_certificate_properties_mut(&mut self) -> super::t_l_s_properties::CertificatePropertiesMut<'_> {
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
  pub fn set_local_certificate_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::t_l_s_properties::CertificateProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // peer_certificate_properties: optional message envoy.data.accesslog.v3.TLSProperties.CertificateProperties
  pub fn has_peer_certificate_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_peer_certificate_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn peer_certificate_properties_opt(&self) -> ::std::option::Option<super::t_l_s_properties::CertificatePropertiesView<'_>> {
    self.has_peer_certificate_properties().then(|| self.peer_certificate_properties())
  }
  pub fn peer_certificate_properties(&self) -> super::t_l_s_properties::CertificatePropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::t_l_s_properties::CertificatePropertiesView::default())
  }
  pub fn peer_certificate_properties_mut(&mut self) -> super::t_l_s_properties::CertificatePropertiesMut<'_> {
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
  pub fn set_peer_certificate_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::t_l_s_properties::CertificateProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // tls_session_id: optional string
  pub fn tls_session_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tls_session_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // ja3_fingerprint: optional string
  pub fn ja3_fingerprint(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_ja3_fingerprint(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

}  // impl TLSProperties

impl ::std::ops::Drop for TLSProperties {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TLSProperties {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TLSProperties {
  type Proxied = Self;
  fn as_view(&self) -> TLSPropertiesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TLSProperties {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TLSPropertiesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TLSProperties {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__data__accesslog__v3__TLSProperties_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P31X331X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__data__accesslog__v3__TLSProperties_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::t_l_s_properties::CertificateProperties as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::t_l_s_properties::CertificateProperties as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__data__accesslog__v3__TLSProperties_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TLSProperties {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TLSProperties {
  type Msg = TLSProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TLSProperties {
  type Msg = TLSProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TLSPropertiesMut<'_> {
  type Msg = TLSProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TLSPropertiesMut<'_> {
  type Msg = TLSProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TLSPropertiesView<'_> {
  type Msg = TLSProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TLSProperties> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TLSPropertiesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod t_l_s_properties {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__TLSProperties__CertificateProperties_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CertificateProperties {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CertificateProperties>
}

impl ::protobuf::Message for CertificateProperties {
  type MessageView<'msg> = CertificatePropertiesView<'msg>;
  type MessageMut<'msg> = CertificatePropertiesMut<'msg>;
}

impl ::std::default::Default for CertificateProperties {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CertificateProperties {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CertificateProperties` is `Sync` because it does not implement interior mutability.
//    Neither does `CertificatePropertiesMut`.
unsafe impl ::std::marker::Sync for CertificateProperties {}

// SAFETY:
// - `CertificateProperties` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CertificateProperties {}

impl ::protobuf::Proxied for CertificateProperties {
  type View<'msg> = CertificatePropertiesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CertificateProperties {}

impl ::protobuf::MutProxied for CertificateProperties {
  type Mut<'msg> = CertificatePropertiesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CertificatePropertiesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificatePropertiesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CertificatePropertiesView<'msg> {
  type Message = CertificateProperties;
}

impl ::std::fmt::Debug for CertificatePropertiesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CertificatePropertiesView<'_> {
  fn default() -> CertificatePropertiesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProperties>> for CertificatePropertiesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificatePropertiesView<'msg> {

  pub fn to_owned(&self) -> CertificateProperties {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subject_alt_name: repeated message envoy.data.accesslog.v3.TLSProperties.CertificateProperties.SubjectAltName
  pub fn subject_alt_name(self) -> ::protobuf::RepeatedView<'msg, super::super::t_l_s_properties::certificate_properties::SubjectAltName> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::t_l_s_properties::certificate_properties::SubjectAltName>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // subject: optional string
  pub fn subject(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // issuer: optional string
  pub fn issuer(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `CertificatePropertiesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CertificatePropertiesView<'_> {}

// SAFETY:
// - `CertificatePropertiesView` is `Send` because while its alive a `CertificatePropertiesMut` cannot.
// - `CertificatePropertiesView` does not use thread-local data.
unsafe impl ::std::marker::Send for CertificatePropertiesView<'_> {}

impl<'msg> ::protobuf::AsView for CertificatePropertiesView<'msg> {
  type Proxied = CertificateProperties;
  fn as_view(&self) -> ::protobuf::View<'msg, CertificateProperties> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificatePropertiesView<'msg> {
  fn into_view<'shorter>(self) -> CertificatePropertiesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateProperties> for CertificatePropertiesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateProperties {
    let mut dst = CertificateProperties::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateProperties> for CertificatePropertiesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateProperties {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CertificateProperties {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificatePropertiesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificatePropertiesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CertificatePropertiesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificatePropertiesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CertificatePropertiesMut<'msg> {
  type Message = CertificateProperties;
}

impl ::std::fmt::Debug for CertificatePropertiesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProperties>> for CertificatePropertiesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificatePropertiesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProperties> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CertificateProperties {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subject_alt_name: repeated message envoy.data.accesslog.v3.TLSProperties.CertificateProperties.SubjectAltName
  pub fn subject_alt_name(&self) -> ::protobuf::RepeatedView<'_, super::super::t_l_s_properties::certificate_properties::SubjectAltName> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::t_l_s_properties::certificate_properties::SubjectAltName>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn subject_alt_name_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::t_l_s_properties::certificate_properties::SubjectAltName> {
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
  pub fn set_subject_alt_name(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::t_l_s_properties::certificate_properties::SubjectAltName>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // subject: optional string
  pub fn subject(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subject(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // issuer: optional string
  pub fn issuer(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_issuer(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `CertificatePropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CertificatePropertiesMut<'_> {}

// SAFETY:
// - `CertificatePropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CertificatePropertiesMut<'_> {}

impl<'msg> ::protobuf::AsView for CertificatePropertiesMut<'msg> {
  type Proxied = CertificateProperties;
  fn as_view(&self) -> ::protobuf::View<'_, CertificateProperties> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificatePropertiesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CertificateProperties>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CertificatePropertiesMut<'msg> {
  type MutProxied = CertificateProperties;
  fn as_mut(&mut self) -> CertificatePropertiesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CertificatePropertiesMut<'msg> {
  fn into_mut<'shorter>(self) -> CertificatePropertiesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CertificateProperties {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CertificateProperties> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CertificatePropertiesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CertificatePropertiesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subject_alt_name: repeated message envoy.data.accesslog.v3.TLSProperties.CertificateProperties.SubjectAltName
  pub fn subject_alt_name(&self) -> ::protobuf::RepeatedView<'_, super::super::t_l_s_properties::certificate_properties::SubjectAltName> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::t_l_s_properties::certificate_properties::SubjectAltName>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn subject_alt_name_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::t_l_s_properties::certificate_properties::SubjectAltName> {
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
  pub fn set_subject_alt_name(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::t_l_s_properties::certificate_properties::SubjectAltName>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // subject: optional string
  pub fn subject(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subject(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // issuer: optional string
  pub fn issuer(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_issuer(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl CertificateProperties

impl ::std::ops::Drop for CertificateProperties {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CertificateProperties {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CertificateProperties {
  type Proxied = Self;
  fn as_view(&self) -> CertificatePropertiesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CertificateProperties {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CertificatePropertiesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CertificateProperties {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::t_l_s_properties::envoy__data__accesslog__v3__TLSProperties__CertificateProperties_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::t_l_s_properties::envoy__data__accesslog__v3__TLSProperties__CertificateProperties_msg_init.0, &[<super::super::t_l_s_properties::certificate_properties::SubjectAltName as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::t_l_s_properties::envoy__data__accesslog__v3__TLSProperties__CertificateProperties_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificateProperties {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificateProperties {
  type Msg = CertificateProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProperties {
  type Msg = CertificateProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificatePropertiesMut<'_> {
  type Msg = CertificateProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificatePropertiesMut<'_> {
  type Msg = CertificateProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificatePropertiesView<'_> {
  type Msg = CertificateProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProperties> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificatePropertiesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod certificate_properties {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__TLSProperties__CertificateProperties__SubjectAltName_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SubjectAltName {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SubjectAltName>
}

impl ::protobuf::Message for SubjectAltName {
  type MessageView<'msg> = SubjectAltNameView<'msg>;
  type MessageMut<'msg> = SubjectAltNameMut<'msg>;
}

impl ::std::default::Default for SubjectAltName {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SubjectAltName {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SubjectAltName` is `Sync` because it does not implement interior mutability.
//    Neither does `SubjectAltNameMut`.
unsafe impl ::std::marker::Sync for SubjectAltName {}

// SAFETY:
// - `SubjectAltName` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SubjectAltName {}

impl ::protobuf::Proxied for SubjectAltName {
  type View<'msg> = SubjectAltNameView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SubjectAltName {}

impl ::protobuf::MutProxied for SubjectAltName {
  type Mut<'msg> = SubjectAltNameMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SubjectAltNameView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SubjectAltName>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubjectAltNameView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SubjectAltNameView<'msg> {
  type Message = SubjectAltName;
}

impl ::std::fmt::Debug for SubjectAltNameView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SubjectAltNameView<'_> {
  fn default() -> SubjectAltNameView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SubjectAltName>> for SubjectAltNameView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SubjectAltName>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubjectAltNameView<'msg> {

  pub fn to_owned(&self) -> SubjectAltName {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // uri: optional string
  pub fn has_uri(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn uri_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_uri().then(|| self.uri())
  }
  pub fn uri(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // dns: optional string
  pub fn has_dns(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn dns_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_dns().then(|| self.dns())
  }
  pub fn dns(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn san(self) -> super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof<'msg> {
    match self.san_case() {
      super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase::Uri =>
          super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof::Uri(self.uri()),
      super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase::Dns =>
          super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof::Dns(self.dns()),
      _ => super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn san_case(self) -> super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SubjectAltNameView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SubjectAltNameView<'_> {}

// SAFETY:
// - `SubjectAltNameView` is `Send` because while its alive a `SubjectAltNameMut` cannot.
// - `SubjectAltNameView` does not use thread-local data.
unsafe impl ::std::marker::Send for SubjectAltNameView<'_> {}

impl<'msg> ::protobuf::AsView for SubjectAltNameView<'msg> {
  type Proxied = SubjectAltName;
  fn as_view(&self) -> ::protobuf::View<'msg, SubjectAltName> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubjectAltNameView<'msg> {
  fn into_view<'shorter>(self) -> SubjectAltNameView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SubjectAltName> for SubjectAltNameView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SubjectAltName {
    let mut dst = SubjectAltName::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SubjectAltName> for SubjectAltNameMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SubjectAltName {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SubjectAltName {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SubjectAltNameView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SubjectAltNameMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SubjectAltNameMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectAltName>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubjectAltNameMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SubjectAltNameMut<'msg> {
  type Message = SubjectAltName;
}

impl ::std::fmt::Debug for SubjectAltNameMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectAltName>> for SubjectAltNameMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectAltName>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubjectAltNameMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectAltName> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SubjectAltName {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // uri: optional string
  pub fn has_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn uri_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_uri().then(|| self.uri())
  }
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

  // dns: optional string
  pub fn has_dns(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dns(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dns_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_dns().then(|| self.dns())
  }
  pub fn dns(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_dns(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn san(&self) -> super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof<'_> {
    match &self.san_case() {
      super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase::Uri =>
          super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof::Uri(self.uri()),
      super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase::Dns =>
          super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof::Dns(self.dns()),
      _ => super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn san_case(&self) -> super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SubjectAltNameMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SubjectAltNameMut<'_> {}

// SAFETY:
// - `SubjectAltNameMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SubjectAltNameMut<'_> {}

impl<'msg> ::protobuf::AsView for SubjectAltNameMut<'msg> {
  type Proxied = SubjectAltName;
  fn as_view(&self) -> ::protobuf::View<'_, SubjectAltName> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubjectAltNameMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SubjectAltName>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SubjectAltNameMut<'msg> {
  type MutProxied = SubjectAltName;
  fn as_mut(&mut self) -> SubjectAltNameMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SubjectAltNameMut<'msg> {
  fn into_mut<'shorter>(self) -> SubjectAltNameMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SubjectAltName {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SubjectAltName> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SubjectAltNameView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SubjectAltNameMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // uri: optional string
  pub fn has_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn uri_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_uri().then(|| self.uri())
  }
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

  // dns: optional string
  pub fn has_dns(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dns(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dns_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_dns().then(|| self.dns())
  }
  pub fn dns(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_dns(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn san(&self) -> super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof<'_> {
    match &self.san_case() {
      super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase::Uri =>
          super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof::Uri(self.uri()),
      super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase::Dns =>
          super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof::Dns(self.dns()),
      _ => super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn san_case(&self) -> super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::t_l_s_properties::certificate_properties::subject_alt_name::SanCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl SubjectAltName

impl ::std::ops::Drop for SubjectAltName {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SubjectAltName {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SubjectAltName {
  type Proxied = Self;
  fn as_view(&self) -> SubjectAltNameView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SubjectAltName {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SubjectAltNameMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SubjectAltName {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::t_l_s_properties::certificate_properties::envoy__data__accesslog__v3__TLSProperties__CertificateProperties__SubjectAltName_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M11^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::t_l_s_properties::certificate_properties::envoy__data__accesslog__v3__TLSProperties__CertificateProperties__SubjectAltName_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::t_l_s_properties::certificate_properties::envoy__data__accesslog__v3__TLSProperties__CertificateProperties__SubjectAltName_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubjectAltName {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubjectAltName {
  type Msg = SubjectAltName;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltName> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectAltName {
  type Msg = SubjectAltName;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltName> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubjectAltNameMut<'_> {
  type Msg = SubjectAltName;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltName> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectAltNameMut<'_> {
  type Msg = SubjectAltName;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltName> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectAltNameView<'_> {
  type Msg = SubjectAltName;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltName> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubjectAltNameMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod subject_alt_name {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum SanOneof<'msg> {
  Uri(&'msg ::protobuf::ProtoStr) = 1,
  Dns(&'msg ::protobuf::ProtoStr) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum SanCase {
  Uri = 1,
  Dns = 2,

  not_set = 0
}

impl SanCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<SanCase> {
    match v {
      0 => Some(SanCase::not_set),
      1 => Some(SanCase::Uri),
      2 => Some(SanCase::Dns),
      _ => None
    }
  }
}
}  // pub mod subject_alt_name


}  // pub mod certificate_properties

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TLSVersion(i32);

#[allow(non_upper_case_globals)]
impl TLSVersion {
  pub const VersionUnspecified: TLSVersion = TLSVersion(0);
  pub const Tlsv1: TLSVersion = TLSVersion(1);
  pub const Tlsv11: TLSVersion = TLSVersion(2);
  pub const Tlsv12: TLSVersion = TLSVersion(3);
  pub const Tlsv13: TLSVersion = TLSVersion(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "VersionUnspecified",
      1 => "Tlsv1",
      2 => "Tlsv11",
      3 => "Tlsv12",
      4 => "Tlsv13",
      _ => return None
    })
  }
}

impl ::std::convert::From<TLSVersion> for i32 {
  fn from(val: TLSVersion) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for TLSVersion {
  fn from(val: i32) -> TLSVersion {
    Self(val)
  }
}

impl ::std::default::Default for TLSVersion {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for TLSVersion {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "TLSVersion::{}", constant_name)
    } else {
      write!(f, "TLSVersion::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for TLSVersion {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for TLSVersion {}

impl ::protobuf::Proxied for TLSVersion {
  type View<'a> = TLSVersion;
}

impl ::protobuf::AsView for TLSVersion {
  type Proxied = TLSVersion;

  fn as_view(&self) -> TLSVersion {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TLSVersion {
  fn into_view<'shorter>(self) -> TLSVersion where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for TLSVersion {
  const NAME: &'static str = "TLSVersion";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for TLSVersion {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod t_l_s_properties


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__HTTPRequestProperties_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HTTPRequestProperties {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HTTPRequestProperties>
}

impl ::protobuf::Message for HTTPRequestProperties {
  type MessageView<'msg> = HTTPRequestPropertiesView<'msg>;
  type MessageMut<'msg> = HTTPRequestPropertiesMut<'msg>;
}

impl ::std::default::Default for HTTPRequestProperties {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HTTPRequestProperties {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HTTPRequestProperties` is `Sync` because it does not implement interior mutability.
//    Neither does `HTTPRequestPropertiesMut`.
unsafe impl ::std::marker::Sync for HTTPRequestProperties {}

// SAFETY:
// - `HTTPRequestProperties` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HTTPRequestProperties {}

impl ::protobuf::Proxied for HTTPRequestProperties {
  type View<'msg> = HTTPRequestPropertiesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HTTPRequestProperties {}

impl ::protobuf::MutProxied for HTTPRequestProperties {
  type Mut<'msg> = HTTPRequestPropertiesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HTTPRequestPropertiesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPRequestProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HTTPRequestPropertiesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HTTPRequestPropertiesView<'msg> {
  type Message = HTTPRequestProperties;
}

impl ::std::fmt::Debug for HTTPRequestPropertiesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HTTPRequestPropertiesView<'_> {
  fn default() -> HTTPRequestPropertiesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPRequestProperties>> for HTTPRequestPropertiesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPRequestProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HTTPRequestPropertiesView<'msg> {

  pub fn to_owned(&self) -> HTTPRequestProperties {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // request_method: optional enum envoy.config.core.v3.RequestMethod
  pub fn request_method(self) -> crate::xds::generated::envoy::config::core::v3::base::RequestMethod {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (crate::xds::generated::envoy::config::core::v3::base::RequestMethod::MethodUnspecified).into()
      ).try_into().unwrap()
    }
  }

  // scheme: optional string
  pub fn scheme(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // authority: optional string
  pub fn authority(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // port: optional message google.protobuf.UInt32Value
  pub fn has_port(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn port_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_port().then(|| self.port())
  }
  pub fn port(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // path: optional string
  pub fn path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // user_agent: optional string
  pub fn user_agent(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // referer: optional string
  pub fn referer(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // forwarded_for: optional string
  pub fn forwarded_for(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // request_id: optional string
  pub fn request_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // original_path: optional string
  pub fn original_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // request_headers_bytes: optional uint64
  pub fn request_headers_bytes(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        10, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // request_body_bytes: optional uint64
  pub fn request_body_bytes(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        11, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // request_headers: repeated message envoy.data.accesslog.v3.HTTPRequestProperties.RequestHeadersEntry
  pub fn request_headers(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(12)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // upstream_header_bytes_sent: optional uint64
  pub fn upstream_header_bytes_sent(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        13, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // downstream_header_bytes_received: optional uint64
  pub fn downstream_header_bytes_received(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        14, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `HTTPRequestPropertiesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HTTPRequestPropertiesView<'_> {}

// SAFETY:
// - `HTTPRequestPropertiesView` is `Send` because while its alive a `HTTPRequestPropertiesMut` cannot.
// - `HTTPRequestPropertiesView` does not use thread-local data.
unsafe impl ::std::marker::Send for HTTPRequestPropertiesView<'_> {}

impl<'msg> ::protobuf::AsView for HTTPRequestPropertiesView<'msg> {
  type Proxied = HTTPRequestProperties;
  fn as_view(&self) -> ::protobuf::View<'msg, HTTPRequestProperties> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HTTPRequestPropertiesView<'msg> {
  fn into_view<'shorter>(self) -> HTTPRequestPropertiesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HTTPRequestProperties> for HTTPRequestPropertiesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HTTPRequestProperties {
    let mut dst = HTTPRequestProperties::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HTTPRequestProperties> for HTTPRequestPropertiesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HTTPRequestProperties {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HTTPRequestProperties {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HTTPRequestPropertiesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HTTPRequestPropertiesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HTTPRequestPropertiesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPRequestProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HTTPRequestPropertiesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HTTPRequestPropertiesMut<'msg> {
  type Message = HTTPRequestProperties;
}

impl ::std::fmt::Debug for HTTPRequestPropertiesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPRequestProperties>> for HTTPRequestPropertiesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPRequestProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HTTPRequestPropertiesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPRequestProperties> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HTTPRequestProperties {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // request_method: optional enum envoy.config.core.v3.RequestMethod
  pub fn request_method(&self) -> crate::xds::generated::envoy::config::core::v3::base::RequestMethod {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (crate::xds::generated::envoy::config::core::v3::base::RequestMethod::MethodUnspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_method(&mut self, val: crate::xds::generated::envoy::config::core::v3::base::RequestMethod) {
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

  // scheme: optional string
  pub fn scheme(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scheme(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // authority: optional string
  pub fn authority(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // port: optional message google.protobuf.UInt32Value
  pub fn has_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_port().then(|| self.port())
  }
  pub fn port(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn port_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // user_agent: optional string
  pub fn user_agent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_user_agent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // referer: optional string
  pub fn referer(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_referer(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // forwarded_for: optional string
  pub fn forwarded_for(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_forwarded_for(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // request_id: optional string
  pub fn request_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_request_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // original_path: optional string
  pub fn original_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_original_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // request_headers_bytes: optional uint64
  pub fn request_headers_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        10, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_headers_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        10, val.into()
      )
    }
  }

  // request_body_bytes: optional uint64
  pub fn request_body_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        11, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_body_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        11, val.into()
      )
    }
  }

  // request_headers: repeated message envoy.data.accesslog.v3.HTTPRequestProperties.RequestHeadersEntry
  pub fn request_headers(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(12)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn request_headers_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          12, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_request_headers(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // upstream_header_bytes_sent: optional uint64
  pub fn upstream_header_bytes_sent(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        13, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_header_bytes_sent(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        13, val.into()
      )
    }
  }

  // downstream_header_bytes_received: optional uint64
  pub fn downstream_header_bytes_received(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        14, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_header_bytes_received(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        14, val.into()
      )
    }
  }

}

// SAFETY:
// - `HTTPRequestPropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HTTPRequestPropertiesMut<'_> {}

// SAFETY:
// - `HTTPRequestPropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HTTPRequestPropertiesMut<'_> {}

impl<'msg> ::protobuf::AsView for HTTPRequestPropertiesMut<'msg> {
  type Proxied = HTTPRequestProperties;
  fn as_view(&self) -> ::protobuf::View<'_, HTTPRequestProperties> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HTTPRequestPropertiesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HTTPRequestProperties>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HTTPRequestPropertiesMut<'msg> {
  type MutProxied = HTTPRequestProperties;
  fn as_mut(&mut self) -> HTTPRequestPropertiesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HTTPRequestPropertiesMut<'msg> {
  fn into_mut<'shorter>(self) -> HTTPRequestPropertiesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HTTPRequestProperties {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HTTPRequestProperties> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HTTPRequestPropertiesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HTTPRequestPropertiesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // request_method: optional enum envoy.config.core.v3.RequestMethod
  pub fn request_method(&self) -> crate::xds::generated::envoy::config::core::v3::base::RequestMethod {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (crate::xds::generated::envoy::config::core::v3::base::RequestMethod::MethodUnspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_method(&mut self, val: crate::xds::generated::envoy::config::core::v3::base::RequestMethod) {
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

  // scheme: optional string
  pub fn scheme(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scheme(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // authority: optional string
  pub fn authority(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authority(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // port: optional message google.protobuf.UInt32Value
  pub fn has_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn port_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_port().then(|| self.port())
  }
  pub fn port(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn port_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_port(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // user_agent: optional string
  pub fn user_agent(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_user_agent(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // referer: optional string
  pub fn referer(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_referer(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // forwarded_for: optional string
  pub fn forwarded_for(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_forwarded_for(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // request_id: optional string
  pub fn request_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_request_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // original_path: optional string
  pub fn original_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_original_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // request_headers_bytes: optional uint64
  pub fn request_headers_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        10, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_headers_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        10, val.into()
      )
    }
  }

  // request_body_bytes: optional uint64
  pub fn request_body_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        11, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_request_body_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        11, val.into()
      )
    }
  }

  // request_headers: repeated message envoy.data.accesslog.v3.HTTPRequestProperties.RequestHeadersEntry
  pub fn request_headers(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(12)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn request_headers_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          12, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_request_headers(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // upstream_header_bytes_sent: optional uint64
  pub fn upstream_header_bytes_sent(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        13, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_header_bytes_sent(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        13, val.into()
      )
    }
  }

  // downstream_header_bytes_received: optional uint64
  pub fn downstream_header_bytes_received(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        14, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_header_bytes_received(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        14, val.into()
      )
    }
  }

}  // impl HTTPRequestProperties

impl ::std::ops::Drop for HTTPRequestProperties {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HTTPRequestProperties {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HTTPRequestProperties {
  type Proxied = Self;
  fn as_view(&self) -> HTTPRequestPropertiesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HTTPRequestProperties {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HTTPRequestPropertiesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HTTPRequestProperties {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__data__accesslog__v3__HTTPRequestProperties_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P1X1X31X1X1X1X1X1X,P,PG,P,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__data__accesslog__v3__HTTPRequestProperties_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::h_t_t_p_request_properties::RequestHeadersEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__data__accesslog__v3__HTTPRequestProperties_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HTTPRequestProperties {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HTTPRequestProperties {
  type Msg = HTTPRequestProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPRequestProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPRequestProperties {
  type Msg = HTTPRequestProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPRequestProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HTTPRequestPropertiesMut<'_> {
  type Msg = HTTPRequestProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPRequestProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPRequestPropertiesMut<'_> {
  type Msg = HTTPRequestProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPRequestProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPRequestPropertiesView<'_> {
  type Msg = HTTPRequestProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPRequestProperties> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HTTPRequestPropertiesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod h_t_t_p_request_properties {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__HTTPRequestProperties__RequestHeadersEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct RequestHeadersEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RequestHeadersEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::h_t_t_p_request_properties::envoy__data__accesslog__v3__HTTPRequestProperties__RequestHeadersEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::h_t_t_p_request_properties::envoy__data__accesslog__v3__HTTPRequestProperties__RequestHeadersEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::h_t_t_p_request_properties::envoy__data__accesslog__v3__HTTPRequestProperties__RequestHeadersEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod h_t_t_p_request_properties


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__HTTPResponseProperties_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HTTPResponseProperties {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HTTPResponseProperties>
}

impl ::protobuf::Message for HTTPResponseProperties {
  type MessageView<'msg> = HTTPResponsePropertiesView<'msg>;
  type MessageMut<'msg> = HTTPResponsePropertiesMut<'msg>;
}

impl ::std::default::Default for HTTPResponseProperties {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HTTPResponseProperties {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HTTPResponseProperties` is `Sync` because it does not implement interior mutability.
//    Neither does `HTTPResponsePropertiesMut`.
unsafe impl ::std::marker::Sync for HTTPResponseProperties {}

// SAFETY:
// - `HTTPResponseProperties` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HTTPResponseProperties {}

impl ::protobuf::Proxied for HTTPResponseProperties {
  type View<'msg> = HTTPResponsePropertiesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HTTPResponseProperties {}

impl ::protobuf::MutProxied for HTTPResponseProperties {
  type Mut<'msg> = HTTPResponsePropertiesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HTTPResponsePropertiesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPResponseProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HTTPResponsePropertiesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HTTPResponsePropertiesView<'msg> {
  type Message = HTTPResponseProperties;
}

impl ::std::fmt::Debug for HTTPResponsePropertiesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HTTPResponsePropertiesView<'_> {
  fn default() -> HTTPResponsePropertiesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPResponseProperties>> for HTTPResponsePropertiesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HTTPResponseProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HTTPResponsePropertiesView<'msg> {

  pub fn to_owned(&self) -> HTTPResponseProperties {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // response_code: optional message google.protobuf.UInt32Value
  pub fn has_response_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn response_code_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_response_code().then(|| self.response_code())
  }
  pub fn response_code(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // response_headers_bytes: optional uint64
  pub fn response_headers_bytes(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // response_body_bytes: optional uint64
  pub fn response_body_bytes(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // response_headers: repeated message envoy.data.accesslog.v3.HTTPResponseProperties.ResponseHeadersEntry
  pub fn response_headers(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // response_trailers: repeated message envoy.data.accesslog.v3.HTTPResponseProperties.ResponseTrailersEntry
  pub fn response_trailers(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // response_code_details: optional string
  pub fn response_code_details(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // upstream_header_bytes_received: optional uint64
  pub fn upstream_header_bytes_received(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        6, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // downstream_header_bytes_sent: optional uint64
  pub fn downstream_header_bytes_sent(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        7, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `HTTPResponsePropertiesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HTTPResponsePropertiesView<'_> {}

// SAFETY:
// - `HTTPResponsePropertiesView` is `Send` because while its alive a `HTTPResponsePropertiesMut` cannot.
// - `HTTPResponsePropertiesView` does not use thread-local data.
unsafe impl ::std::marker::Send for HTTPResponsePropertiesView<'_> {}

impl<'msg> ::protobuf::AsView for HTTPResponsePropertiesView<'msg> {
  type Proxied = HTTPResponseProperties;
  fn as_view(&self) -> ::protobuf::View<'msg, HTTPResponseProperties> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HTTPResponsePropertiesView<'msg> {
  fn into_view<'shorter>(self) -> HTTPResponsePropertiesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HTTPResponseProperties> for HTTPResponsePropertiesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HTTPResponseProperties {
    let mut dst = HTTPResponseProperties::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HTTPResponseProperties> for HTTPResponsePropertiesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HTTPResponseProperties {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HTTPResponseProperties {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HTTPResponsePropertiesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HTTPResponsePropertiesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HTTPResponsePropertiesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPResponseProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HTTPResponsePropertiesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HTTPResponsePropertiesMut<'msg> {
  type Message = HTTPResponseProperties;
}

impl ::std::fmt::Debug for HTTPResponsePropertiesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPResponseProperties>> for HTTPResponsePropertiesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPResponseProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HTTPResponsePropertiesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HTTPResponseProperties> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HTTPResponseProperties {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // response_code: optional message google.protobuf.UInt32Value
  pub fn has_response_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_response_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn response_code_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_response_code().then(|| self.response_code())
  }
  pub fn response_code(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn response_code_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_response_code(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // response_headers_bytes: optional uint64
  pub fn response_headers_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_headers_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // response_body_bytes: optional uint64
  pub fn response_body_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_body_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // response_headers: repeated message envoy.data.accesslog.v3.HTTPResponseProperties.ResponseHeadersEntry
  pub fn response_headers(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn response_headers_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_response_headers(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // response_trailers: repeated message envoy.data.accesslog.v3.HTTPResponseProperties.ResponseTrailersEntry
  pub fn response_trailers(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn response_trailers_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          4, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_response_trailers(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // response_code_details: optional string
  pub fn response_code_details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_code_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // upstream_header_bytes_received: optional uint64
  pub fn upstream_header_bytes_received(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        6, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_header_bytes_received(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        6, val.into()
      )
    }
  }

  // downstream_header_bytes_sent: optional uint64
  pub fn downstream_header_bytes_sent(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        7, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_header_bytes_sent(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `HTTPResponsePropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HTTPResponsePropertiesMut<'_> {}

// SAFETY:
// - `HTTPResponsePropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HTTPResponsePropertiesMut<'_> {}

impl<'msg> ::protobuf::AsView for HTTPResponsePropertiesMut<'msg> {
  type Proxied = HTTPResponseProperties;
  fn as_view(&self) -> ::protobuf::View<'_, HTTPResponseProperties> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HTTPResponsePropertiesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HTTPResponseProperties>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HTTPResponsePropertiesMut<'msg> {
  type MutProxied = HTTPResponseProperties;
  fn as_mut(&mut self) -> HTTPResponsePropertiesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HTTPResponsePropertiesMut<'msg> {
  fn into_mut<'shorter>(self) -> HTTPResponsePropertiesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HTTPResponseProperties {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HTTPResponseProperties> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HTTPResponsePropertiesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HTTPResponsePropertiesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // response_code: optional message google.protobuf.UInt32Value
  pub fn has_response_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_response_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn response_code_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_response_code().then(|| self.response_code())
  }
  pub fn response_code(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn response_code_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_response_code(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // response_headers_bytes: optional uint64
  pub fn response_headers_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_headers_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

  // response_body_bytes: optional uint64
  pub fn response_body_bytes(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_response_body_bytes(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // response_headers: repeated message envoy.data.accesslog.v3.HTTPResponseProperties.ResponseHeadersEntry
  pub fn response_headers(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn response_headers_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_response_headers(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // response_trailers: repeated message envoy.data.accesslog.v3.HTTPResponseProperties.ResponseTrailersEntry
  pub fn response_trailers(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(4)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn response_trailers_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          4, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_response_trailers(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // response_code_details: optional string
  pub fn response_code_details(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_response_code_details(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // upstream_header_bytes_received: optional uint64
  pub fn upstream_header_bytes_received(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        6, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_upstream_header_bytes_received(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        6, val.into()
      )
    }
  }

  // downstream_header_bytes_sent: optional uint64
  pub fn downstream_header_bytes_sent(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        7, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_downstream_header_bytes_sent(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        7, val.into()
      )
    }
  }

}  // impl HTTPResponseProperties

impl ::std::ops::Drop for HTTPResponseProperties {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HTTPResponseProperties {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HTTPResponseProperties {
  type Proxied = Self;
  fn as_view(&self) -> HTTPResponsePropertiesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HTTPResponseProperties {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HTTPResponsePropertiesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HTTPResponseProperties {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__data__accesslog__v3__HTTPResponseProperties_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3,P,PGG1X,P,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__data__accesslog__v3__HTTPResponseProperties_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::h_t_t_p_response_properties::ResponseHeadersEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::h_t_t_p_response_properties::ResponseTrailersEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__data__accesslog__v3__HTTPResponseProperties_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HTTPResponseProperties {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HTTPResponseProperties {
  type Msg = HTTPResponseProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPResponseProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPResponseProperties {
  type Msg = HTTPResponseProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPResponseProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HTTPResponsePropertiesMut<'_> {
  type Msg = HTTPResponseProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPResponseProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPResponsePropertiesMut<'_> {
  type Msg = HTTPResponseProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPResponseProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HTTPResponsePropertiesView<'_> {
  type Msg = HTTPResponseProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HTTPResponseProperties> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HTTPResponsePropertiesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod h_t_t_p_response_properties {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__HTTPResponseProperties__ResponseHeadersEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct ResponseHeadersEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResponseHeadersEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::h_t_t_p_response_properties::envoy__data__accesslog__v3__HTTPResponseProperties__ResponseHeadersEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::h_t_t_p_response_properties::envoy__data__accesslog__v3__HTTPResponseProperties__ResponseHeadersEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::h_t_t_p_response_properties::envoy__data__accesslog__v3__HTTPResponseProperties__ResponseHeadersEntry_msg_init.0)
      }).0
    }
  }
}
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__data__accesslog__v3__HTTPResponseProperties__ResponseTrailersEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct ResponseTrailersEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResponseTrailersEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::h_t_t_p_response_properties::envoy__data__accesslog__v3__HTTPResponseProperties__ResponseTrailersEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::h_t_t_p_response_properties::envoy__data__accesslog__v3__HTTPResponseProperties__ResponseTrailersEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::h_t_t_p_response_properties::envoy__data__accesslog__v3__HTTPResponseProperties__ResponseTrailersEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod h_t_t_p_response_properties


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccessLogType(i32);

#[allow(non_upper_case_globals)]
impl AccessLogType {
  pub const Notset: AccessLogType = AccessLogType(0);
  pub const Tcpupstreamconnected: AccessLogType = AccessLogType(1);
  pub const Tcpperiodic: AccessLogType = AccessLogType(2);
  pub const Tcpconnectionend: AccessLogType = AccessLogType(3);
  pub const Downstreamstart: AccessLogType = AccessLogType(4);
  pub const Downstreamperiodic: AccessLogType = AccessLogType(5);
  pub const Downstreamend: AccessLogType = AccessLogType(6);
  pub const Upstreampoolready: AccessLogType = AccessLogType(7);
  pub const Upstreamperiodic: AccessLogType = AccessLogType(8);
  pub const Upstreamend: AccessLogType = AccessLogType(9);
  pub const Downstreamtunnelsuccessfullyestablished: AccessLogType = AccessLogType(10);
  pub const Udptunnelupstreamconnected: AccessLogType = AccessLogType(11);
  pub const Udpperiodic: AccessLogType = AccessLogType(12);
  pub const Udpsessionend: AccessLogType = AccessLogType(13);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Notset",
      1 => "Tcpupstreamconnected",
      2 => "Tcpperiodic",
      3 => "Tcpconnectionend",
      4 => "Downstreamstart",
      5 => "Downstreamperiodic",
      6 => "Downstreamend",
      7 => "Upstreampoolready",
      8 => "Upstreamperiodic",
      9 => "Upstreamend",
      10 => "Downstreamtunnelsuccessfullyestablished",
      11 => "Udptunnelupstreamconnected",
      12 => "Udpperiodic",
      13 => "Udpsessionend",
      _ => return None
    })
  }
}

impl ::std::convert::From<AccessLogType> for i32 {
  fn from(val: AccessLogType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for AccessLogType {
  fn from(val: i32) -> AccessLogType {
    Self(val)
  }
}

impl ::std::default::Default for AccessLogType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for AccessLogType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "AccessLogType::{}", constant_name)
    } else {
      write!(f, "AccessLogType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for AccessLogType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for AccessLogType {}

impl ::protobuf::Proxied for AccessLogType {
  type View<'a> = AccessLogType;
}

impl ::protobuf::AsView for AccessLogType {
  type Proxied = AccessLogType;

  fn as_view(&self) -> AccessLogType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AccessLogType {
  fn into_view<'shorter>(self) -> AccessLogType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for AccessLogType {
  const NAME: &'static str = "AccessLogType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9|10|11|12|13)
  }
}

impl ::protobuf::__internal::EntityType for AccessLogType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


