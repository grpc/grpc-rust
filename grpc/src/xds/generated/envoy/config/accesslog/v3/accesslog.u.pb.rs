const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__AccessLog_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AccessLog {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AccessLog>
}

impl ::protobuf::Message for AccessLog {
  type MessageView<'msg> = AccessLogView<'msg>;
  type MessageMut<'msg> = AccessLogMut<'msg>;
}

impl ::std::default::Default for AccessLog {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AccessLog {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AccessLog` is `Sync` because it does not implement interior mutability.
//    Neither does `AccessLogMut`.
unsafe impl ::std::marker::Sync for AccessLog {}

// SAFETY:
// - `AccessLog` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AccessLog {}

impl ::protobuf::Proxied for AccessLog {
  type View<'msg> = AccessLogView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AccessLog {}

impl ::protobuf::MutProxied for AccessLog {
  type Mut<'msg> = AccessLogMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AccessLogView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AccessLog>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AccessLogView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AccessLogView<'msg> {
  type Message = AccessLog;
}

impl ::std::fmt::Debug for AccessLogView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AccessLogView<'_> {
  fn default() -> AccessLogView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AccessLog>> for AccessLogView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AccessLog>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AccessLogView<'msg> {

  pub fn to_owned(&self) -> AccessLog {
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

  // filter: optional message envoy.config.accesslog.v3.AccessLogFilter
  pub fn has_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn filter_opt(self) -> ::std::option::Option<super::AccessLogFilterView<'msg>> {
    self.has_filter().then(|| self.filter())
  }
  pub fn filter(self) -> super::AccessLogFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AccessLogFilterView::default())
  }

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn typed_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  pub fn config_type(self) -> super::access_log::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::access_log::ConfigTypeCase::TypedConfig =>
          super::access_log::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::access_log::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::access_log::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::access_log::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AccessLogView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AccessLogView<'_> {}

// SAFETY:
// - `AccessLogView` is `Send` because while its alive a `AccessLogMut` cannot.
// - `AccessLogView` does not use thread-local data.
unsafe impl ::std::marker::Send for AccessLogView<'_> {}

impl<'msg> ::protobuf::AsView for AccessLogView<'msg> {
  type Proxied = AccessLog;
  fn as_view(&self) -> ::protobuf::View<'msg, AccessLog> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AccessLogView<'msg> {
  fn into_view<'shorter>(self) -> AccessLogView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AccessLog> for AccessLogView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AccessLog {
    let mut dst = AccessLog::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AccessLog> for AccessLogMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AccessLog {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AccessLog {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AccessLogView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AccessLogMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AccessLogMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLog>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AccessLogMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AccessLogMut<'msg> {
  type Message = AccessLog;
}

impl ::std::fmt::Debug for AccessLogMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLog>> for AccessLogMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLog>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AccessLogMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLog> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AccessLog {
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

  // filter: optional message envoy.config.accesslog.v3.AccessLogFilter
  pub fn has_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn filter_opt(&self) -> ::std::option::Option<super::AccessLogFilterView<'_>> {
    self.has_filter().then(|| self.filter())
  }
  pub fn filter(&self) -> super::AccessLogFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AccessLogFilterView::default())
  }
  pub fn filter_mut(&mut self) -> super::AccessLogFilterMut<'_> {
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
  pub fn set_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::AccessLogFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::access_log::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::access_log::ConfigTypeCase::TypedConfig =>
          super::access_log::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::access_log::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::access_log::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::access_log::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AccessLogMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AccessLogMut<'_> {}

// SAFETY:
// - `AccessLogMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AccessLogMut<'_> {}

impl<'msg> ::protobuf::AsView for AccessLogMut<'msg> {
  type Proxied = AccessLog;
  fn as_view(&self) -> ::protobuf::View<'_, AccessLog> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AccessLogMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AccessLog>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AccessLogMut<'msg> {
  type MutProxied = AccessLog;
  fn as_mut(&mut self) -> AccessLogMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AccessLogMut<'msg> {
  fn into_mut<'shorter>(self) -> AccessLogMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AccessLog {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AccessLog> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AccessLogView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AccessLogMut<'_> {
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

  // filter: optional message envoy.config.accesslog.v3.AccessLogFilter
  pub fn has_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn filter_opt(&self) -> ::std::option::Option<super::AccessLogFilterView<'_>> {
    self.has_filter().then(|| self.filter())
  }
  pub fn filter(&self) -> super::AccessLogFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AccessLogFilterView::default())
  }
  pub fn filter_mut(&mut self) -> super::AccessLogFilterMut<'_> {
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
  pub fn set_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::AccessLogFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // typed_config: optional message google.protobuf.Any
  pub fn has_typed_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_typed_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn typed_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn typed_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_typed_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn config_type(&self) -> super::access_log::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::access_log::ConfigTypeCase::TypedConfig =>
          super::access_log::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::access_log::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::access_log::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::access_log::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl AccessLog

impl ::std::ops::Drop for AccessLog {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AccessLog {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AccessLog {
  type Proxied = Self;
  fn as_view(&self) -> AccessLogView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AccessLog {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AccessLogMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AccessLog {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__AccessLog_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3a3^%");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__AccessLog_msg_init.0, &[<super::AccessLogFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__AccessLog_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AccessLog {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AccessLog {
  type Msg = AccessLog;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLog> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessLog {
  type Msg = AccessLog;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLog> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AccessLogMut<'_> {
  type Msg = AccessLog;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLog> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessLogMut<'_> {
  type Msg = AccessLog;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLog> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessLogView<'_> {
  type Msg = AccessLog;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLog> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AccessLogMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod access_log {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigTypeOneof<'msg> {
  TypedConfig(::protobuf::View<'msg, ::protobuf_well_known_types::Any>) = 4,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigTypeCase {
  TypedConfig = 4,

  not_set = 0
}

impl ConfigTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigTypeCase> {
    match v {
      0 => Some(ConfigTypeCase::not_set),
      4 => Some(ConfigTypeCase::TypedConfig),
      _ => None
    }
  }
}
}  // pub mod access_log


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__AccessLogFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AccessLogFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AccessLogFilter>
}

impl ::protobuf::Message for AccessLogFilter {
  type MessageView<'msg> = AccessLogFilterView<'msg>;
  type MessageMut<'msg> = AccessLogFilterMut<'msg>;
}

impl ::std::default::Default for AccessLogFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AccessLogFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AccessLogFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `AccessLogFilterMut`.
unsafe impl ::std::marker::Sync for AccessLogFilter {}

// SAFETY:
// - `AccessLogFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AccessLogFilter {}

impl ::protobuf::Proxied for AccessLogFilter {
  type View<'msg> = AccessLogFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AccessLogFilter {}

impl ::protobuf::MutProxied for AccessLogFilter {
  type Mut<'msg> = AccessLogFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AccessLogFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AccessLogFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AccessLogFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AccessLogFilterView<'msg> {
  type Message = AccessLogFilter;
}

impl ::std::fmt::Debug for AccessLogFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AccessLogFilterView<'_> {
  fn default() -> AccessLogFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AccessLogFilter>> for AccessLogFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AccessLogFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AccessLogFilterView<'msg> {

  pub fn to_owned(&self) -> AccessLogFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // status_code_filter: optional message envoy.config.accesslog.v3.StatusCodeFilter
  pub fn has_status_code_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn status_code_filter_opt(self) -> ::std::option::Option<super::StatusCodeFilterView<'msg>> {
    self.has_status_code_filter().then(|| self.status_code_filter())
  }
  pub fn status_code_filter(self) -> super::StatusCodeFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StatusCodeFilterView::default())
  }

  // duration_filter: optional message envoy.config.accesslog.v3.DurationFilter
  pub fn has_duration_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn duration_filter_opt(self) -> ::std::option::Option<super::DurationFilterView<'msg>> {
    self.has_duration_filter().then(|| self.duration_filter())
  }
  pub fn duration_filter(self) -> super::DurationFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DurationFilterView::default())
  }

  // not_health_check_filter: optional message envoy.config.accesslog.v3.NotHealthCheckFilter
  pub fn has_not_health_check_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn not_health_check_filter_opt(self) -> ::std::option::Option<super::NotHealthCheckFilterView<'msg>> {
    self.has_not_health_check_filter().then(|| self.not_health_check_filter())
  }
  pub fn not_health_check_filter(self) -> super::NotHealthCheckFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::NotHealthCheckFilterView::default())
  }

  // traceable_filter: optional message envoy.config.accesslog.v3.TraceableFilter
  pub fn has_traceable_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn traceable_filter_opt(self) -> ::std::option::Option<super::TraceableFilterView<'msg>> {
    self.has_traceable_filter().then(|| self.traceable_filter())
  }
  pub fn traceable_filter(self) -> super::TraceableFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TraceableFilterView::default())
  }

  // runtime_filter: optional message envoy.config.accesslog.v3.RuntimeFilter
  pub fn has_runtime_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn runtime_filter_opt(self) -> ::std::option::Option<super::RuntimeFilterView<'msg>> {
    self.has_runtime_filter().then(|| self.runtime_filter())
  }
  pub fn runtime_filter(self) -> super::RuntimeFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RuntimeFilterView::default())
  }

  // and_filter: optional message envoy.config.accesslog.v3.AndFilter
  pub fn has_and_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn and_filter_opt(self) -> ::std::option::Option<super::AndFilterView<'msg>> {
    self.has_and_filter().then(|| self.and_filter())
  }
  pub fn and_filter(self) -> super::AndFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AndFilterView::default())
  }

  // or_filter: optional message envoy.config.accesslog.v3.OrFilter
  pub fn has_or_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn or_filter_opt(self) -> ::std::option::Option<super::OrFilterView<'msg>> {
    self.has_or_filter().then(|| self.or_filter())
  }
  pub fn or_filter(self) -> super::OrFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::OrFilterView::default())
  }

  // header_filter: optional message envoy.config.accesslog.v3.HeaderFilter
  pub fn has_header_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn header_filter_opt(self) -> ::std::option::Option<super::HeaderFilterView<'msg>> {
    self.has_header_filter().then(|| self.header_filter())
  }
  pub fn header_filter(self) -> super::HeaderFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderFilterView::default())
  }

  // response_flag_filter: optional message envoy.config.accesslog.v3.ResponseFlagFilter
  pub fn has_response_flag_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn response_flag_filter_opt(self) -> ::std::option::Option<super::ResponseFlagFilterView<'msg>> {
    self.has_response_flag_filter().then(|| self.response_flag_filter())
  }
  pub fn response_flag_filter(self) -> super::ResponseFlagFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResponseFlagFilterView::default())
  }

  // grpc_status_filter: optional message envoy.config.accesslog.v3.GrpcStatusFilter
  pub fn has_grpc_status_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn grpc_status_filter_opt(self) -> ::std::option::Option<super::GrpcStatusFilterView<'msg>> {
    self.has_grpc_status_filter().then(|| self.grpc_status_filter())
  }
  pub fn grpc_status_filter(self) -> super::GrpcStatusFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GrpcStatusFilterView::default())
  }

  // extension_filter: optional message envoy.config.accesslog.v3.ExtensionFilter
  pub fn has_extension_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn extension_filter_opt(self) -> ::std::option::Option<super::ExtensionFilterView<'msg>> {
    self.has_extension_filter().then(|| self.extension_filter())
  }
  pub fn extension_filter(self) -> super::ExtensionFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExtensionFilterView::default())
  }

  // metadata_filter: optional message envoy.config.accesslog.v3.MetadataFilter
  pub fn has_metadata_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn metadata_filter_opt(self) -> ::std::option::Option<super::MetadataFilterView<'msg>> {
    self.has_metadata_filter().then(|| self.metadata_filter())
  }
  pub fn metadata_filter(self) -> super::MetadataFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetadataFilterView::default())
  }

  // log_type_filter: optional message envoy.config.accesslog.v3.LogTypeFilter
  pub fn has_log_type_filter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn log_type_filter_opt(self) -> ::std::option::Option<super::LogTypeFilterView<'msg>> {
    self.has_log_type_filter().then(|| self.log_type_filter())
  }
  pub fn log_type_filter(self) -> super::LogTypeFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LogTypeFilterView::default())
  }

  pub fn filter_specifier(self) -> super::access_log_filter::FilterSpecifierOneof<'msg> {
    match self.filter_specifier_case() {
      super::access_log_filter::FilterSpecifierCase::StatusCodeFilter =>
          super::access_log_filter::FilterSpecifierOneof::StatusCodeFilter(self.status_code_filter()),
      super::access_log_filter::FilterSpecifierCase::DurationFilter =>
          super::access_log_filter::FilterSpecifierOneof::DurationFilter(self.duration_filter()),
      super::access_log_filter::FilterSpecifierCase::NotHealthCheckFilter =>
          super::access_log_filter::FilterSpecifierOneof::NotHealthCheckFilter(self.not_health_check_filter()),
      super::access_log_filter::FilterSpecifierCase::TraceableFilter =>
          super::access_log_filter::FilterSpecifierOneof::TraceableFilter(self.traceable_filter()),
      super::access_log_filter::FilterSpecifierCase::RuntimeFilter =>
          super::access_log_filter::FilterSpecifierOneof::RuntimeFilter(self.runtime_filter()),
      super::access_log_filter::FilterSpecifierCase::AndFilter =>
          super::access_log_filter::FilterSpecifierOneof::AndFilter(self.and_filter()),
      super::access_log_filter::FilterSpecifierCase::OrFilter =>
          super::access_log_filter::FilterSpecifierOneof::OrFilter(self.or_filter()),
      super::access_log_filter::FilterSpecifierCase::HeaderFilter =>
          super::access_log_filter::FilterSpecifierOneof::HeaderFilter(self.header_filter()),
      super::access_log_filter::FilterSpecifierCase::ResponseFlagFilter =>
          super::access_log_filter::FilterSpecifierOneof::ResponseFlagFilter(self.response_flag_filter()),
      super::access_log_filter::FilterSpecifierCase::GrpcStatusFilter =>
          super::access_log_filter::FilterSpecifierOneof::GrpcStatusFilter(self.grpc_status_filter()),
      super::access_log_filter::FilterSpecifierCase::ExtensionFilter =>
          super::access_log_filter::FilterSpecifierOneof::ExtensionFilter(self.extension_filter()),
      super::access_log_filter::FilterSpecifierCase::MetadataFilter =>
          super::access_log_filter::FilterSpecifierOneof::MetadataFilter(self.metadata_filter()),
      super::access_log_filter::FilterSpecifierCase::LogTypeFilter =>
          super::access_log_filter::FilterSpecifierOneof::LogTypeFilter(self.log_type_filter()),
      _ => super::access_log_filter::FilterSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn filter_specifier_case(self) -> super::access_log_filter::FilterSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::access_log_filter::FilterSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AccessLogFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AccessLogFilterView<'_> {}

// SAFETY:
// - `AccessLogFilterView` is `Send` because while its alive a `AccessLogFilterMut` cannot.
// - `AccessLogFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for AccessLogFilterView<'_> {}

impl<'msg> ::protobuf::AsView for AccessLogFilterView<'msg> {
  type Proxied = AccessLogFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, AccessLogFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AccessLogFilterView<'msg> {
  fn into_view<'shorter>(self) -> AccessLogFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AccessLogFilter> for AccessLogFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AccessLogFilter {
    let mut dst = AccessLogFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AccessLogFilter> for AccessLogFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AccessLogFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AccessLogFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AccessLogFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AccessLogFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AccessLogFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLogFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AccessLogFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AccessLogFilterMut<'msg> {
  type Message = AccessLogFilter;
}

impl ::std::fmt::Debug for AccessLogFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLogFilter>> for AccessLogFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLogFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AccessLogFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AccessLogFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AccessLogFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // status_code_filter: optional message envoy.config.accesslog.v3.StatusCodeFilter
  pub fn has_status_code_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_status_code_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn status_code_filter_opt(&self) -> ::std::option::Option<super::StatusCodeFilterView<'_>> {
    self.has_status_code_filter().then(|| self.status_code_filter())
  }
  pub fn status_code_filter(&self) -> super::StatusCodeFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StatusCodeFilterView::default())
  }
  pub fn status_code_filter_mut(&mut self) -> super::StatusCodeFilterMut<'_> {
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
  pub fn set_status_code_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::StatusCodeFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // duration_filter: optional message envoy.config.accesslog.v3.DurationFilter
  pub fn has_duration_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_duration_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn duration_filter_opt(&self) -> ::std::option::Option<super::DurationFilterView<'_>> {
    self.has_duration_filter().then(|| self.duration_filter())
  }
  pub fn duration_filter(&self) -> super::DurationFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DurationFilterView::default())
  }
  pub fn duration_filter_mut(&mut self) -> super::DurationFilterMut<'_> {
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
  pub fn set_duration_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::DurationFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // not_health_check_filter: optional message envoy.config.accesslog.v3.NotHealthCheckFilter
  pub fn has_not_health_check_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_not_health_check_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn not_health_check_filter_opt(&self) -> ::std::option::Option<super::NotHealthCheckFilterView<'_>> {
    self.has_not_health_check_filter().then(|| self.not_health_check_filter())
  }
  pub fn not_health_check_filter(&self) -> super::NotHealthCheckFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::NotHealthCheckFilterView::default())
  }
  pub fn not_health_check_filter_mut(&mut self) -> super::NotHealthCheckFilterMut<'_> {
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
  pub fn set_not_health_check_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::NotHealthCheckFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // traceable_filter: optional message envoy.config.accesslog.v3.TraceableFilter
  pub fn has_traceable_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_traceable_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn traceable_filter_opt(&self) -> ::std::option::Option<super::TraceableFilterView<'_>> {
    self.has_traceable_filter().then(|| self.traceable_filter())
  }
  pub fn traceable_filter(&self) -> super::TraceableFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TraceableFilterView::default())
  }
  pub fn traceable_filter_mut(&mut self) -> super::TraceableFilterMut<'_> {
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
  pub fn set_traceable_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::TraceableFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // runtime_filter: optional message envoy.config.accesslog.v3.RuntimeFilter
  pub fn has_runtime_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_runtime_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn runtime_filter_opt(&self) -> ::std::option::Option<super::RuntimeFilterView<'_>> {
    self.has_runtime_filter().then(|| self.runtime_filter())
  }
  pub fn runtime_filter(&self) -> super::RuntimeFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RuntimeFilterView::default())
  }
  pub fn runtime_filter_mut(&mut self) -> super::RuntimeFilterMut<'_> {
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
  pub fn set_runtime_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::RuntimeFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // and_filter: optional message envoy.config.accesslog.v3.AndFilter
  pub fn has_and_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_and_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn and_filter_opt(&self) -> ::std::option::Option<super::AndFilterView<'_>> {
    self.has_and_filter().then(|| self.and_filter())
  }
  pub fn and_filter(&self) -> super::AndFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AndFilterView::default())
  }
  pub fn and_filter_mut(&mut self) -> super::AndFilterMut<'_> {
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
  pub fn set_and_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::AndFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // or_filter: optional message envoy.config.accesslog.v3.OrFilter
  pub fn has_or_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_or_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn or_filter_opt(&self) -> ::std::option::Option<super::OrFilterView<'_>> {
    self.has_or_filter().then(|| self.or_filter())
  }
  pub fn or_filter(&self) -> super::OrFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::OrFilterView::default())
  }
  pub fn or_filter_mut(&mut self) -> super::OrFilterMut<'_> {
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
  pub fn set_or_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::OrFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // header_filter: optional message envoy.config.accesslog.v3.HeaderFilter
  pub fn has_header_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_header_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn header_filter_opt(&self) -> ::std::option::Option<super::HeaderFilterView<'_>> {
    self.has_header_filter().then(|| self.header_filter())
  }
  pub fn header_filter(&self) -> super::HeaderFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderFilterView::default())
  }
  pub fn header_filter_mut(&mut self) -> super::HeaderFilterMut<'_> {
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
  pub fn set_header_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // response_flag_filter: optional message envoy.config.accesslog.v3.ResponseFlagFilter
  pub fn has_response_flag_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_response_flag_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn response_flag_filter_opt(&self) -> ::std::option::Option<super::ResponseFlagFilterView<'_>> {
    self.has_response_flag_filter().then(|| self.response_flag_filter())
  }
  pub fn response_flag_filter(&self) -> super::ResponseFlagFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResponseFlagFilterView::default())
  }
  pub fn response_flag_filter_mut(&mut self) -> super::ResponseFlagFilterMut<'_> {
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
  pub fn set_response_flag_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResponseFlagFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // grpc_status_filter: optional message envoy.config.accesslog.v3.GrpcStatusFilter
  pub fn has_grpc_status_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_grpc_status_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn grpc_status_filter_opt(&self) -> ::std::option::Option<super::GrpcStatusFilterView<'_>> {
    self.has_grpc_status_filter().then(|| self.grpc_status_filter())
  }
  pub fn grpc_status_filter(&self) -> super::GrpcStatusFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GrpcStatusFilterView::default())
  }
  pub fn grpc_status_filter_mut(&mut self) -> super::GrpcStatusFilterMut<'_> {
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
  pub fn set_grpc_status_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::GrpcStatusFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // extension_filter: optional message envoy.config.accesslog.v3.ExtensionFilter
  pub fn has_extension_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_extension_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn extension_filter_opt(&self) -> ::std::option::Option<super::ExtensionFilterView<'_>> {
    self.has_extension_filter().then(|| self.extension_filter())
  }
  pub fn extension_filter(&self) -> super::ExtensionFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExtensionFilterView::default())
  }
  pub fn extension_filter_mut(&mut self) -> super::ExtensionFilterMut<'_> {
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
  pub fn set_extension_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::ExtensionFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // metadata_filter: optional message envoy.config.accesslog.v3.MetadataFilter
  pub fn has_metadata_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_metadata_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn metadata_filter_opt(&self) -> ::std::option::Option<super::MetadataFilterView<'_>> {
    self.has_metadata_filter().then(|| self.metadata_filter())
  }
  pub fn metadata_filter(&self) -> super::MetadataFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetadataFilterView::default())
  }
  pub fn metadata_filter_mut(&mut self) -> super::MetadataFilterMut<'_> {
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
  pub fn set_metadata_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::MetadataFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // log_type_filter: optional message envoy.config.accesslog.v3.LogTypeFilter
  pub fn has_log_type_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_log_type_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn log_type_filter_opt(&self) -> ::std::option::Option<super::LogTypeFilterView<'_>> {
    self.has_log_type_filter().then(|| self.log_type_filter())
  }
  pub fn log_type_filter(&self) -> super::LogTypeFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LogTypeFilterView::default())
  }
  pub fn log_type_filter_mut(&mut self) -> super::LogTypeFilterMut<'_> {
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
  pub fn set_log_type_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::LogTypeFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  pub fn filter_specifier(&self) -> super::access_log_filter::FilterSpecifierOneof<'_> {
    match &self.filter_specifier_case() {
      super::access_log_filter::FilterSpecifierCase::StatusCodeFilter =>
          super::access_log_filter::FilterSpecifierOneof::StatusCodeFilter(self.status_code_filter()),
      super::access_log_filter::FilterSpecifierCase::DurationFilter =>
          super::access_log_filter::FilterSpecifierOneof::DurationFilter(self.duration_filter()),
      super::access_log_filter::FilterSpecifierCase::NotHealthCheckFilter =>
          super::access_log_filter::FilterSpecifierOneof::NotHealthCheckFilter(self.not_health_check_filter()),
      super::access_log_filter::FilterSpecifierCase::TraceableFilter =>
          super::access_log_filter::FilterSpecifierOneof::TraceableFilter(self.traceable_filter()),
      super::access_log_filter::FilterSpecifierCase::RuntimeFilter =>
          super::access_log_filter::FilterSpecifierOneof::RuntimeFilter(self.runtime_filter()),
      super::access_log_filter::FilterSpecifierCase::AndFilter =>
          super::access_log_filter::FilterSpecifierOneof::AndFilter(self.and_filter()),
      super::access_log_filter::FilterSpecifierCase::OrFilter =>
          super::access_log_filter::FilterSpecifierOneof::OrFilter(self.or_filter()),
      super::access_log_filter::FilterSpecifierCase::HeaderFilter =>
          super::access_log_filter::FilterSpecifierOneof::HeaderFilter(self.header_filter()),
      super::access_log_filter::FilterSpecifierCase::ResponseFlagFilter =>
          super::access_log_filter::FilterSpecifierOneof::ResponseFlagFilter(self.response_flag_filter()),
      super::access_log_filter::FilterSpecifierCase::GrpcStatusFilter =>
          super::access_log_filter::FilterSpecifierOneof::GrpcStatusFilter(self.grpc_status_filter()),
      super::access_log_filter::FilterSpecifierCase::ExtensionFilter =>
          super::access_log_filter::FilterSpecifierOneof::ExtensionFilter(self.extension_filter()),
      super::access_log_filter::FilterSpecifierCase::MetadataFilter =>
          super::access_log_filter::FilterSpecifierOneof::MetadataFilter(self.metadata_filter()),
      super::access_log_filter::FilterSpecifierCase::LogTypeFilter =>
          super::access_log_filter::FilterSpecifierOneof::LogTypeFilter(self.log_type_filter()),
      _ => super::access_log_filter::FilterSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn filter_specifier_case(&self) -> super::access_log_filter::FilterSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::access_log_filter::FilterSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AccessLogFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AccessLogFilterMut<'_> {}

// SAFETY:
// - `AccessLogFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AccessLogFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for AccessLogFilterMut<'msg> {
  type Proxied = AccessLogFilter;
  fn as_view(&self) -> ::protobuf::View<'_, AccessLogFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AccessLogFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AccessLogFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AccessLogFilterMut<'msg> {
  type MutProxied = AccessLogFilter;
  fn as_mut(&mut self) -> AccessLogFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AccessLogFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> AccessLogFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AccessLogFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AccessLogFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AccessLogFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AccessLogFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // status_code_filter: optional message envoy.config.accesslog.v3.StatusCodeFilter
  pub fn has_status_code_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_status_code_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn status_code_filter_opt(&self) -> ::std::option::Option<super::StatusCodeFilterView<'_>> {
    self.has_status_code_filter().then(|| self.status_code_filter())
  }
  pub fn status_code_filter(&self) -> super::StatusCodeFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StatusCodeFilterView::default())
  }
  pub fn status_code_filter_mut(&mut self) -> super::StatusCodeFilterMut<'_> {
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
  pub fn set_status_code_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::StatusCodeFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // duration_filter: optional message envoy.config.accesslog.v3.DurationFilter
  pub fn has_duration_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_duration_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn duration_filter_opt(&self) -> ::std::option::Option<super::DurationFilterView<'_>> {
    self.has_duration_filter().then(|| self.duration_filter())
  }
  pub fn duration_filter(&self) -> super::DurationFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DurationFilterView::default())
  }
  pub fn duration_filter_mut(&mut self) -> super::DurationFilterMut<'_> {
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
  pub fn set_duration_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::DurationFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // not_health_check_filter: optional message envoy.config.accesslog.v3.NotHealthCheckFilter
  pub fn has_not_health_check_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_not_health_check_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn not_health_check_filter_opt(&self) -> ::std::option::Option<super::NotHealthCheckFilterView<'_>> {
    self.has_not_health_check_filter().then(|| self.not_health_check_filter())
  }
  pub fn not_health_check_filter(&self) -> super::NotHealthCheckFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::NotHealthCheckFilterView::default())
  }
  pub fn not_health_check_filter_mut(&mut self) -> super::NotHealthCheckFilterMut<'_> {
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
  pub fn set_not_health_check_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::NotHealthCheckFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // traceable_filter: optional message envoy.config.accesslog.v3.TraceableFilter
  pub fn has_traceable_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_traceable_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn traceable_filter_opt(&self) -> ::std::option::Option<super::TraceableFilterView<'_>> {
    self.has_traceable_filter().then(|| self.traceable_filter())
  }
  pub fn traceable_filter(&self) -> super::TraceableFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TraceableFilterView::default())
  }
  pub fn traceable_filter_mut(&mut self) -> super::TraceableFilterMut<'_> {
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
  pub fn set_traceable_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::TraceableFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // runtime_filter: optional message envoy.config.accesslog.v3.RuntimeFilter
  pub fn has_runtime_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_runtime_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn runtime_filter_opt(&self) -> ::std::option::Option<super::RuntimeFilterView<'_>> {
    self.has_runtime_filter().then(|| self.runtime_filter())
  }
  pub fn runtime_filter(&self) -> super::RuntimeFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RuntimeFilterView::default())
  }
  pub fn runtime_filter_mut(&mut self) -> super::RuntimeFilterMut<'_> {
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
  pub fn set_runtime_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::RuntimeFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // and_filter: optional message envoy.config.accesslog.v3.AndFilter
  pub fn has_and_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_and_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn and_filter_opt(&self) -> ::std::option::Option<super::AndFilterView<'_>> {
    self.has_and_filter().then(|| self.and_filter())
  }
  pub fn and_filter(&self) -> super::AndFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AndFilterView::default())
  }
  pub fn and_filter_mut(&mut self) -> super::AndFilterMut<'_> {
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
  pub fn set_and_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::AndFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // or_filter: optional message envoy.config.accesslog.v3.OrFilter
  pub fn has_or_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_or_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn or_filter_opt(&self) -> ::std::option::Option<super::OrFilterView<'_>> {
    self.has_or_filter().then(|| self.or_filter())
  }
  pub fn or_filter(&self) -> super::OrFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::OrFilterView::default())
  }
  pub fn or_filter_mut(&mut self) -> super::OrFilterMut<'_> {
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
  pub fn set_or_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::OrFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // header_filter: optional message envoy.config.accesslog.v3.HeaderFilter
  pub fn has_header_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_header_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn header_filter_opt(&self) -> ::std::option::Option<super::HeaderFilterView<'_>> {
    self.has_header_filter().then(|| self.header_filter())
  }
  pub fn header_filter(&self) -> super::HeaderFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderFilterView::default())
  }
  pub fn header_filter_mut(&mut self) -> super::HeaderFilterMut<'_> {
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
  pub fn set_header_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // response_flag_filter: optional message envoy.config.accesslog.v3.ResponseFlagFilter
  pub fn has_response_flag_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_response_flag_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn response_flag_filter_opt(&self) -> ::std::option::Option<super::ResponseFlagFilterView<'_>> {
    self.has_response_flag_filter().then(|| self.response_flag_filter())
  }
  pub fn response_flag_filter(&self) -> super::ResponseFlagFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ResponseFlagFilterView::default())
  }
  pub fn response_flag_filter_mut(&mut self) -> super::ResponseFlagFilterMut<'_> {
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
  pub fn set_response_flag_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::ResponseFlagFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // grpc_status_filter: optional message envoy.config.accesslog.v3.GrpcStatusFilter
  pub fn has_grpc_status_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_grpc_status_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn grpc_status_filter_opt(&self) -> ::std::option::Option<super::GrpcStatusFilterView<'_>> {
    self.has_grpc_status_filter().then(|| self.grpc_status_filter())
  }
  pub fn grpc_status_filter(&self) -> super::GrpcStatusFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GrpcStatusFilterView::default())
  }
  pub fn grpc_status_filter_mut(&mut self) -> super::GrpcStatusFilterMut<'_> {
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
  pub fn set_grpc_status_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::GrpcStatusFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // extension_filter: optional message envoy.config.accesslog.v3.ExtensionFilter
  pub fn has_extension_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_extension_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn extension_filter_opt(&self) -> ::std::option::Option<super::ExtensionFilterView<'_>> {
    self.has_extension_filter().then(|| self.extension_filter())
  }
  pub fn extension_filter(&self) -> super::ExtensionFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExtensionFilterView::default())
  }
  pub fn extension_filter_mut(&mut self) -> super::ExtensionFilterMut<'_> {
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
  pub fn set_extension_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::ExtensionFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // metadata_filter: optional message envoy.config.accesslog.v3.MetadataFilter
  pub fn has_metadata_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_metadata_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn metadata_filter_opt(&self) -> ::std::option::Option<super::MetadataFilterView<'_>> {
    self.has_metadata_filter().then(|| self.metadata_filter())
  }
  pub fn metadata_filter(&self) -> super::MetadataFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MetadataFilterView::default())
  }
  pub fn metadata_filter_mut(&mut self) -> super::MetadataFilterMut<'_> {
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
  pub fn set_metadata_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::MetadataFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // log_type_filter: optional message envoy.config.accesslog.v3.LogTypeFilter
  pub fn has_log_type_filter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_log_type_filter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn log_type_filter_opt(&self) -> ::std::option::Option<super::LogTypeFilterView<'_>> {
    self.has_log_type_filter().then(|| self.log_type_filter())
  }
  pub fn log_type_filter(&self) -> super::LogTypeFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LogTypeFilterView::default())
  }
  pub fn log_type_filter_mut(&mut self) -> super::LogTypeFilterMut<'_> {
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
  pub fn set_log_type_filter(&mut self,
    val: impl ::protobuf::IntoProxied<super::LogTypeFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  pub fn filter_specifier(&self) -> super::access_log_filter::FilterSpecifierOneof<'_> {
    match &self.filter_specifier_case() {
      super::access_log_filter::FilterSpecifierCase::StatusCodeFilter =>
          super::access_log_filter::FilterSpecifierOneof::StatusCodeFilter(self.status_code_filter()),
      super::access_log_filter::FilterSpecifierCase::DurationFilter =>
          super::access_log_filter::FilterSpecifierOneof::DurationFilter(self.duration_filter()),
      super::access_log_filter::FilterSpecifierCase::NotHealthCheckFilter =>
          super::access_log_filter::FilterSpecifierOneof::NotHealthCheckFilter(self.not_health_check_filter()),
      super::access_log_filter::FilterSpecifierCase::TraceableFilter =>
          super::access_log_filter::FilterSpecifierOneof::TraceableFilter(self.traceable_filter()),
      super::access_log_filter::FilterSpecifierCase::RuntimeFilter =>
          super::access_log_filter::FilterSpecifierOneof::RuntimeFilter(self.runtime_filter()),
      super::access_log_filter::FilterSpecifierCase::AndFilter =>
          super::access_log_filter::FilterSpecifierOneof::AndFilter(self.and_filter()),
      super::access_log_filter::FilterSpecifierCase::OrFilter =>
          super::access_log_filter::FilterSpecifierOneof::OrFilter(self.or_filter()),
      super::access_log_filter::FilterSpecifierCase::HeaderFilter =>
          super::access_log_filter::FilterSpecifierOneof::HeaderFilter(self.header_filter()),
      super::access_log_filter::FilterSpecifierCase::ResponseFlagFilter =>
          super::access_log_filter::FilterSpecifierOneof::ResponseFlagFilter(self.response_flag_filter()),
      super::access_log_filter::FilterSpecifierCase::GrpcStatusFilter =>
          super::access_log_filter::FilterSpecifierOneof::GrpcStatusFilter(self.grpc_status_filter()),
      super::access_log_filter::FilterSpecifierCase::ExtensionFilter =>
          super::access_log_filter::FilterSpecifierOneof::ExtensionFilter(self.extension_filter()),
      super::access_log_filter::FilterSpecifierCase::MetadataFilter =>
          super::access_log_filter::FilterSpecifierOneof::MetadataFilter(self.metadata_filter()),
      super::access_log_filter::FilterSpecifierCase::LogTypeFilter =>
          super::access_log_filter::FilterSpecifierOneof::LogTypeFilter(self.log_type_filter()),
      _ => super::access_log_filter::FilterSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn filter_specifier_case(&self) -> super::access_log_filter::FilterSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::access_log_filter::FilterSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl AccessLogFilter

impl ::std::ops::Drop for AccessLogFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AccessLogFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AccessLogFilter {
  type Proxied = Self;
  fn as_view(&self) -> AccessLogFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AccessLogFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AccessLogFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AccessLogFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__AccessLogFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333333333333^!|#|$|%|&|(|)|*|+|,|-|.|/");
        super::envoy__config__accesslog__v3__AndFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        super::envoy__config__accesslog__v3__OrFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$aG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__AccessLogFilter_msg_init.0, &[<super::StatusCodeFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DurationFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::NotHealthCheckFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TraceableFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::RuntimeFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::envoy__config__accesslog__v3__AndFilter_msg_init.0,
            super::envoy__config__accesslog__v3__OrFilter_msg_init.0,
            <super::HeaderFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResponseFlagFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::GrpcStatusFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ExtensionFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::MetadataFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::LogTypeFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__AndFilter_msg_init.0, &[super::envoy__config__accesslog__v3__AccessLogFilter_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__OrFilter_msg_init.0, &[super::envoy__config__accesslog__v3__AccessLogFilter_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__AccessLogFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AccessLogFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AccessLogFilter {
  type Msg = AccessLogFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessLogFilter {
  type Msg = AccessLogFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AccessLogFilterMut<'_> {
  type Msg = AccessLogFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessLogFilterMut<'_> {
  type Msg = AccessLogFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AccessLogFilterView<'_> {
  type Msg = AccessLogFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AccessLogFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AccessLogFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod access_log_filter {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum FilterSpecifierOneof<'msg> {
  StatusCodeFilter(::protobuf::View<'msg, super::super::StatusCodeFilter>) = 1,
  DurationFilter(::protobuf::View<'msg, super::super::DurationFilter>) = 2,
  NotHealthCheckFilter(::protobuf::View<'msg, super::super::NotHealthCheckFilter>) = 3,
  TraceableFilter(::protobuf::View<'msg, super::super::TraceableFilter>) = 4,
  RuntimeFilter(::protobuf::View<'msg, super::super::RuntimeFilter>) = 5,
  AndFilter(::protobuf::View<'msg, super::super::AndFilter>) = 6,
  OrFilter(::protobuf::View<'msg, super::super::OrFilter>) = 7,
  HeaderFilter(::protobuf::View<'msg, super::super::HeaderFilter>) = 8,
  ResponseFlagFilter(::protobuf::View<'msg, super::super::ResponseFlagFilter>) = 9,
  GrpcStatusFilter(::protobuf::View<'msg, super::super::GrpcStatusFilter>) = 10,
  ExtensionFilter(::protobuf::View<'msg, super::super::ExtensionFilter>) = 11,
  MetadataFilter(::protobuf::View<'msg, super::super::MetadataFilter>) = 12,
  LogTypeFilter(::protobuf::View<'msg, super::super::LogTypeFilter>) = 13,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum FilterSpecifierCase {
  StatusCodeFilter = 1,
  DurationFilter = 2,
  NotHealthCheckFilter = 3,
  TraceableFilter = 4,
  RuntimeFilter = 5,
  AndFilter = 6,
  OrFilter = 7,
  HeaderFilter = 8,
  ResponseFlagFilter = 9,
  GrpcStatusFilter = 10,
  ExtensionFilter = 11,
  MetadataFilter = 12,
  LogTypeFilter = 13,

  not_set = 0
}

impl FilterSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<FilterSpecifierCase> {
    match v {
      0 => Some(FilterSpecifierCase::not_set),
      1 => Some(FilterSpecifierCase::StatusCodeFilter),
      2 => Some(FilterSpecifierCase::DurationFilter),
      3 => Some(FilterSpecifierCase::NotHealthCheckFilter),
      4 => Some(FilterSpecifierCase::TraceableFilter),
      5 => Some(FilterSpecifierCase::RuntimeFilter),
      6 => Some(FilterSpecifierCase::AndFilter),
      7 => Some(FilterSpecifierCase::OrFilter),
      8 => Some(FilterSpecifierCase::HeaderFilter),
      9 => Some(FilterSpecifierCase::ResponseFlagFilter),
      10 => Some(FilterSpecifierCase::GrpcStatusFilter),
      11 => Some(FilterSpecifierCase::ExtensionFilter),
      12 => Some(FilterSpecifierCase::MetadataFilter),
      13 => Some(FilterSpecifierCase::LogTypeFilter),
      _ => None
    }
  }
}
}  // pub mod access_log_filter


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__ComparisonFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ComparisonFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ComparisonFilter>
}

impl ::protobuf::Message for ComparisonFilter {
  type MessageView<'msg> = ComparisonFilterView<'msg>;
  type MessageMut<'msg> = ComparisonFilterMut<'msg>;
}

impl ::std::default::Default for ComparisonFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ComparisonFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ComparisonFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `ComparisonFilterMut`.
unsafe impl ::std::marker::Sync for ComparisonFilter {}

// SAFETY:
// - `ComparisonFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ComparisonFilter {}

impl ::protobuf::Proxied for ComparisonFilter {
  type View<'msg> = ComparisonFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ComparisonFilter {}

impl ::protobuf::MutProxied for ComparisonFilter {
  type Mut<'msg> = ComparisonFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ComparisonFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ComparisonFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ComparisonFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ComparisonFilterView<'msg> {
  type Message = ComparisonFilter;
}

impl ::std::fmt::Debug for ComparisonFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ComparisonFilterView<'_> {
  fn default() -> ComparisonFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ComparisonFilter>> for ComparisonFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ComparisonFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ComparisonFilterView<'msg> {

  pub fn to_owned(&self) -> ComparisonFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // op: optional enum envoy.config.accesslog.v3.ComparisonFilter.Op
  pub fn op(self) -> super::comparison_filter::Op {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::comparison_filter::Op::Eq).into()
      ).try_into().unwrap()
    }
  }

  // value: optional message envoy.config.core.v3.RuntimeUInt32
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32View<'msg>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32View::default())
  }

}

// SAFETY:
// - `ComparisonFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ComparisonFilterView<'_> {}

// SAFETY:
// - `ComparisonFilterView` is `Send` because while its alive a `ComparisonFilterMut` cannot.
// - `ComparisonFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for ComparisonFilterView<'_> {}

impl<'msg> ::protobuf::AsView for ComparisonFilterView<'msg> {
  type Proxied = ComparisonFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, ComparisonFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ComparisonFilterView<'msg> {
  fn into_view<'shorter>(self) -> ComparisonFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ComparisonFilter> for ComparisonFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ComparisonFilter {
    let mut dst = ComparisonFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ComparisonFilter> for ComparisonFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ComparisonFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ComparisonFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ComparisonFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ComparisonFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ComparisonFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ComparisonFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ComparisonFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ComparisonFilterMut<'msg> {
  type Message = ComparisonFilter;
}

impl ::std::fmt::Debug for ComparisonFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ComparisonFilter>> for ComparisonFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ComparisonFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ComparisonFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ComparisonFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ComparisonFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // op: optional enum envoy.config.accesslog.v3.ComparisonFilter.Op
  pub fn op(&self) -> super::comparison_filter::Op {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::comparison_filter::Op::Eq).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_op(&mut self, val: super::comparison_filter::Op) {
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

  // value: optional message envoy.config.core.v3.RuntimeUInt32
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32View<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32View::default())
  }
  pub fn value_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32Mut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32>) {

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
// - `ComparisonFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ComparisonFilterMut<'_> {}

// SAFETY:
// - `ComparisonFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ComparisonFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for ComparisonFilterMut<'msg> {
  type Proxied = ComparisonFilter;
  fn as_view(&self) -> ::protobuf::View<'_, ComparisonFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ComparisonFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ComparisonFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ComparisonFilterMut<'msg> {
  type MutProxied = ComparisonFilter;
  fn as_mut(&mut self) -> ComparisonFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ComparisonFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> ComparisonFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ComparisonFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ComparisonFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ComparisonFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ComparisonFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // op: optional enum envoy.config.accesslog.v3.ComparisonFilter.Op
  pub fn op(&self) -> super::comparison_filter::Op {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::comparison_filter::Op::Eq).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_op(&mut self, val: super::comparison_filter::Op) {
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

  // value: optional message envoy.config.core.v3.RuntimeUInt32
  pub fn has_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32View<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32View::default())
  }
  pub fn value_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32Mut<'_> {
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
  pub fn set_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ComparisonFilter

impl ::std::ops::Drop for ComparisonFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ComparisonFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ComparisonFilter {
  type Proxied = Self;
  fn as_view(&self) -> ComparisonFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ComparisonFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ComparisonFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ComparisonFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__ComparisonFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__ComparisonFilter_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::RuntimeUInt32 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__ComparisonFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ComparisonFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ComparisonFilter {
  type Msg = ComparisonFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ComparisonFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ComparisonFilter {
  type Msg = ComparisonFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ComparisonFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ComparisonFilterMut<'_> {
  type Msg = ComparisonFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ComparisonFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ComparisonFilterMut<'_> {
  type Msg = ComparisonFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ComparisonFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ComparisonFilterView<'_> {
  type Msg = ComparisonFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ComparisonFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ComparisonFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod comparison_filter {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Op(i32);

#[allow(non_upper_case_globals)]
impl Op {
  pub const Eq: Op = Op(0);
  pub const Ge: Op = Op(1);
  pub const Le: Op = Op(2);
  pub const Ne: Op = Op(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Eq",
      1 => "Ge",
      2 => "Le",
      3 => "Ne",
      _ => return None
    })
  }
}

impl ::std::convert::From<Op> for i32 {
  fn from(val: Op) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Op {
  fn from(val: i32) -> Op {
    Self(val)
  }
}

impl ::std::default::Default for Op {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Op {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Op::{}", constant_name)
    } else {
      write!(f, "Op::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Op {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Op {}

impl ::protobuf::Proxied for Op {
  type View<'a> = Op;
}

impl ::protobuf::AsView for Op {
  type Proxied = Op;

  fn as_view(&self) -> Op {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Op {
  fn into_view<'shorter>(self) -> Op where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Op {
  const NAME: &'static str = "Op";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for Op {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod comparison_filter


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__StatusCodeFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StatusCodeFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StatusCodeFilter>
}

impl ::protobuf::Message for StatusCodeFilter {
  type MessageView<'msg> = StatusCodeFilterView<'msg>;
  type MessageMut<'msg> = StatusCodeFilterMut<'msg>;
}

impl ::std::default::Default for StatusCodeFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StatusCodeFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StatusCodeFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `StatusCodeFilterMut`.
unsafe impl ::std::marker::Sync for StatusCodeFilter {}

// SAFETY:
// - `StatusCodeFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StatusCodeFilter {}

impl ::protobuf::Proxied for StatusCodeFilter {
  type View<'msg> = StatusCodeFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StatusCodeFilter {}

impl ::protobuf::MutProxied for StatusCodeFilter {
  type Mut<'msg> = StatusCodeFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StatusCodeFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatusCodeFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatusCodeFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StatusCodeFilterView<'msg> {
  type Message = StatusCodeFilter;
}

impl ::std::fmt::Debug for StatusCodeFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StatusCodeFilterView<'_> {
  fn default() -> StatusCodeFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StatusCodeFilter>> for StatusCodeFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatusCodeFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatusCodeFilterView<'msg> {

  pub fn to_owned(&self) -> StatusCodeFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // comparison: optional message envoy.config.accesslog.v3.ComparisonFilter
  pub fn has_comparison(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn comparison_opt(self) -> ::std::option::Option<super::ComparisonFilterView<'msg>> {
    self.has_comparison().then(|| self.comparison())
  }
  pub fn comparison(self) -> super::ComparisonFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComparisonFilterView::default())
  }

}

// SAFETY:
// - `StatusCodeFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StatusCodeFilterView<'_> {}

// SAFETY:
// - `StatusCodeFilterView` is `Send` because while its alive a `StatusCodeFilterMut` cannot.
// - `StatusCodeFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for StatusCodeFilterView<'_> {}

impl<'msg> ::protobuf::AsView for StatusCodeFilterView<'msg> {
  type Proxied = StatusCodeFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, StatusCodeFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatusCodeFilterView<'msg> {
  fn into_view<'shorter>(self) -> StatusCodeFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StatusCodeFilter> for StatusCodeFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatusCodeFilter {
    let mut dst = StatusCodeFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StatusCodeFilter> for StatusCodeFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatusCodeFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StatusCodeFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatusCodeFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatusCodeFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StatusCodeFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatusCodeFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatusCodeFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StatusCodeFilterMut<'msg> {
  type Message = StatusCodeFilter;
}

impl ::std::fmt::Debug for StatusCodeFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StatusCodeFilter>> for StatusCodeFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatusCodeFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatusCodeFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StatusCodeFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StatusCodeFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // comparison: optional message envoy.config.accesslog.v3.ComparisonFilter
  pub fn has_comparison(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_comparison(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn comparison_opt(&self) -> ::std::option::Option<super::ComparisonFilterView<'_>> {
    self.has_comparison().then(|| self.comparison())
  }
  pub fn comparison(&self) -> super::ComparisonFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComparisonFilterView::default())
  }
  pub fn comparison_mut(&mut self) -> super::ComparisonFilterMut<'_> {
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
  pub fn set_comparison(&mut self,
    val: impl ::protobuf::IntoProxied<super::ComparisonFilter>) {

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
// - `StatusCodeFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StatusCodeFilterMut<'_> {}

// SAFETY:
// - `StatusCodeFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StatusCodeFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for StatusCodeFilterMut<'msg> {
  type Proxied = StatusCodeFilter;
  fn as_view(&self) -> ::protobuf::View<'_, StatusCodeFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatusCodeFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StatusCodeFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StatusCodeFilterMut<'msg> {
  type MutProxied = StatusCodeFilter;
  fn as_mut(&mut self) -> StatusCodeFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StatusCodeFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> StatusCodeFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StatusCodeFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StatusCodeFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StatusCodeFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StatusCodeFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // comparison: optional message envoy.config.accesslog.v3.ComparisonFilter
  pub fn has_comparison(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_comparison(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn comparison_opt(&self) -> ::std::option::Option<super::ComparisonFilterView<'_>> {
    self.has_comparison().then(|| self.comparison())
  }
  pub fn comparison(&self) -> super::ComparisonFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComparisonFilterView::default())
  }
  pub fn comparison_mut(&mut self) -> super::ComparisonFilterMut<'_> {
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
  pub fn set_comparison(&mut self,
    val: impl ::protobuf::IntoProxied<super::ComparisonFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl StatusCodeFilter

impl ::std::ops::Drop for StatusCodeFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StatusCodeFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StatusCodeFilter {
  type Proxied = Self;
  fn as_view(&self) -> StatusCodeFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StatusCodeFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StatusCodeFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StatusCodeFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__StatusCodeFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__StatusCodeFilter_msg_init.0, &[<super::ComparisonFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__StatusCodeFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatusCodeFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatusCodeFilter {
  type Msg = StatusCodeFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusCodeFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatusCodeFilter {
  type Msg = StatusCodeFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusCodeFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatusCodeFilterMut<'_> {
  type Msg = StatusCodeFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusCodeFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatusCodeFilterMut<'_> {
  type Msg = StatusCodeFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusCodeFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatusCodeFilterView<'_> {
  type Msg = StatusCodeFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatusCodeFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatusCodeFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__DurationFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DurationFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DurationFilter>
}

impl ::protobuf::Message for DurationFilter {
  type MessageView<'msg> = DurationFilterView<'msg>;
  type MessageMut<'msg> = DurationFilterMut<'msg>;
}

impl ::std::default::Default for DurationFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DurationFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DurationFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `DurationFilterMut`.
unsafe impl ::std::marker::Sync for DurationFilter {}

// SAFETY:
// - `DurationFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DurationFilter {}

impl ::protobuf::Proxied for DurationFilter {
  type View<'msg> = DurationFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DurationFilter {}

impl ::protobuf::MutProxied for DurationFilter {
  type Mut<'msg> = DurationFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DurationFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DurationFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DurationFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DurationFilterView<'msg> {
  type Message = DurationFilter;
}

impl ::std::fmt::Debug for DurationFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DurationFilterView<'_> {
  fn default() -> DurationFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DurationFilter>> for DurationFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DurationFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DurationFilterView<'msg> {

  pub fn to_owned(&self) -> DurationFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // comparison: optional message envoy.config.accesslog.v3.ComparisonFilter
  pub fn has_comparison(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn comparison_opt(self) -> ::std::option::Option<super::ComparisonFilterView<'msg>> {
    self.has_comparison().then(|| self.comparison())
  }
  pub fn comparison(self) -> super::ComparisonFilterView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComparisonFilterView::default())
  }

}

// SAFETY:
// - `DurationFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DurationFilterView<'_> {}

// SAFETY:
// - `DurationFilterView` is `Send` because while its alive a `DurationFilterMut` cannot.
// - `DurationFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for DurationFilterView<'_> {}

impl<'msg> ::protobuf::AsView for DurationFilterView<'msg> {
  type Proxied = DurationFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, DurationFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DurationFilterView<'msg> {
  fn into_view<'shorter>(self) -> DurationFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DurationFilter> for DurationFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DurationFilter {
    let mut dst = DurationFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DurationFilter> for DurationFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DurationFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DurationFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DurationFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DurationFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DurationFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DurationFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DurationFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DurationFilterMut<'msg> {
  type Message = DurationFilter;
}

impl ::std::fmt::Debug for DurationFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DurationFilter>> for DurationFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DurationFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DurationFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DurationFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DurationFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // comparison: optional message envoy.config.accesslog.v3.ComparisonFilter
  pub fn has_comparison(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_comparison(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn comparison_opt(&self) -> ::std::option::Option<super::ComparisonFilterView<'_>> {
    self.has_comparison().then(|| self.comparison())
  }
  pub fn comparison(&self) -> super::ComparisonFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComparisonFilterView::default())
  }
  pub fn comparison_mut(&mut self) -> super::ComparisonFilterMut<'_> {
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
  pub fn set_comparison(&mut self,
    val: impl ::protobuf::IntoProxied<super::ComparisonFilter>) {

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
// - `DurationFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DurationFilterMut<'_> {}

// SAFETY:
// - `DurationFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DurationFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for DurationFilterMut<'msg> {
  type Proxied = DurationFilter;
  fn as_view(&self) -> ::protobuf::View<'_, DurationFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DurationFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DurationFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DurationFilterMut<'msg> {
  type MutProxied = DurationFilter;
  fn as_mut(&mut self) -> DurationFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DurationFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> DurationFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DurationFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DurationFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DurationFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DurationFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // comparison: optional message envoy.config.accesslog.v3.ComparisonFilter
  pub fn has_comparison(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_comparison(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn comparison_opt(&self) -> ::std::option::Option<super::ComparisonFilterView<'_>> {
    self.has_comparison().then(|| self.comparison())
  }
  pub fn comparison(&self) -> super::ComparisonFilterView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ComparisonFilterView::default())
  }
  pub fn comparison_mut(&mut self) -> super::ComparisonFilterMut<'_> {
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
  pub fn set_comparison(&mut self,
    val: impl ::protobuf::IntoProxied<super::ComparisonFilter>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl DurationFilter

impl ::std::ops::Drop for DurationFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DurationFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DurationFilter {
  type Proxied = Self;
  fn as_view(&self) -> DurationFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DurationFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DurationFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DurationFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__DurationFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__DurationFilter_msg_init.0, &[<super::ComparisonFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__DurationFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DurationFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DurationFilter {
  type Msg = DurationFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DurationFilter {
  type Msg = DurationFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DurationFilterMut<'_> {
  type Msg = DurationFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DurationFilterMut<'_> {
  type Msg = DurationFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DurationFilterView<'_> {
  type Msg = DurationFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DurationFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DurationFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__NotHealthCheckFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NotHealthCheckFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NotHealthCheckFilter>
}

impl ::protobuf::Message for NotHealthCheckFilter {
  type MessageView<'msg> = NotHealthCheckFilterView<'msg>;
  type MessageMut<'msg> = NotHealthCheckFilterMut<'msg>;
}

impl ::std::default::Default for NotHealthCheckFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NotHealthCheckFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NotHealthCheckFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `NotHealthCheckFilterMut`.
unsafe impl ::std::marker::Sync for NotHealthCheckFilter {}

// SAFETY:
// - `NotHealthCheckFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for NotHealthCheckFilter {}

impl ::protobuf::Proxied for NotHealthCheckFilter {
  type View<'msg> = NotHealthCheckFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NotHealthCheckFilter {}

impl ::protobuf::MutProxied for NotHealthCheckFilter {
  type Mut<'msg> = NotHealthCheckFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NotHealthCheckFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NotHealthCheckFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NotHealthCheckFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NotHealthCheckFilterView<'msg> {
  type Message = NotHealthCheckFilter;
}

impl ::std::fmt::Debug for NotHealthCheckFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NotHealthCheckFilterView<'_> {
  fn default() -> NotHealthCheckFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NotHealthCheckFilter>> for NotHealthCheckFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NotHealthCheckFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NotHealthCheckFilterView<'msg> {

  pub fn to_owned(&self) -> NotHealthCheckFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `NotHealthCheckFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for NotHealthCheckFilterView<'_> {}

// SAFETY:
// - `NotHealthCheckFilterView` is `Send` because while its alive a `NotHealthCheckFilterMut` cannot.
// - `NotHealthCheckFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for NotHealthCheckFilterView<'_> {}

impl<'msg> ::protobuf::AsView for NotHealthCheckFilterView<'msg> {
  type Proxied = NotHealthCheckFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, NotHealthCheckFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NotHealthCheckFilterView<'msg> {
  fn into_view<'shorter>(self) -> NotHealthCheckFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NotHealthCheckFilter> for NotHealthCheckFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NotHealthCheckFilter {
    let mut dst = NotHealthCheckFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NotHealthCheckFilter> for NotHealthCheckFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NotHealthCheckFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for NotHealthCheckFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NotHealthCheckFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NotHealthCheckFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NotHealthCheckFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NotHealthCheckFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NotHealthCheckFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NotHealthCheckFilterMut<'msg> {
  type Message = NotHealthCheckFilter;
}

impl ::std::fmt::Debug for NotHealthCheckFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NotHealthCheckFilter>> for NotHealthCheckFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NotHealthCheckFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NotHealthCheckFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NotHealthCheckFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> NotHealthCheckFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `NotHealthCheckFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for NotHealthCheckFilterMut<'_> {}

// SAFETY:
// - `NotHealthCheckFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for NotHealthCheckFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for NotHealthCheckFilterMut<'msg> {
  type Proxied = NotHealthCheckFilter;
  fn as_view(&self) -> ::protobuf::View<'_, NotHealthCheckFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NotHealthCheckFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NotHealthCheckFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for NotHealthCheckFilterMut<'msg> {
  type MutProxied = NotHealthCheckFilter;
  fn as_mut(&mut self) -> NotHealthCheckFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NotHealthCheckFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> NotHealthCheckFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NotHealthCheckFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NotHealthCheckFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NotHealthCheckFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NotHealthCheckFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl NotHealthCheckFilter

impl ::std::ops::Drop for NotHealthCheckFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NotHealthCheckFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NotHealthCheckFilter {
  type Proxied = Self;
  fn as_view(&self) -> NotHealthCheckFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NotHealthCheckFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NotHealthCheckFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NotHealthCheckFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__NotHealthCheckFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__NotHealthCheckFilter_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__NotHealthCheckFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NotHealthCheckFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NotHealthCheckFilter {
  type Msg = NotHealthCheckFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NotHealthCheckFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NotHealthCheckFilter {
  type Msg = NotHealthCheckFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NotHealthCheckFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NotHealthCheckFilterMut<'_> {
  type Msg = NotHealthCheckFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NotHealthCheckFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NotHealthCheckFilterMut<'_> {
  type Msg = NotHealthCheckFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NotHealthCheckFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NotHealthCheckFilterView<'_> {
  type Msg = NotHealthCheckFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NotHealthCheckFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NotHealthCheckFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__TraceableFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TraceableFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TraceableFilter>
}

impl ::protobuf::Message for TraceableFilter {
  type MessageView<'msg> = TraceableFilterView<'msg>;
  type MessageMut<'msg> = TraceableFilterMut<'msg>;
}

impl ::std::default::Default for TraceableFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TraceableFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TraceableFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `TraceableFilterMut`.
unsafe impl ::std::marker::Sync for TraceableFilter {}

// SAFETY:
// - `TraceableFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TraceableFilter {}

impl ::protobuf::Proxied for TraceableFilter {
  type View<'msg> = TraceableFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TraceableFilter {}

impl ::protobuf::MutProxied for TraceableFilter {
  type Mut<'msg> = TraceableFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TraceableFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TraceableFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TraceableFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TraceableFilterView<'msg> {
  type Message = TraceableFilter;
}

impl ::std::fmt::Debug for TraceableFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TraceableFilterView<'_> {
  fn default() -> TraceableFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TraceableFilter>> for TraceableFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TraceableFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TraceableFilterView<'msg> {

  pub fn to_owned(&self) -> TraceableFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `TraceableFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TraceableFilterView<'_> {}

// SAFETY:
// - `TraceableFilterView` is `Send` because while its alive a `TraceableFilterMut` cannot.
// - `TraceableFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for TraceableFilterView<'_> {}

impl<'msg> ::protobuf::AsView for TraceableFilterView<'msg> {
  type Proxied = TraceableFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, TraceableFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TraceableFilterView<'msg> {
  fn into_view<'shorter>(self) -> TraceableFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TraceableFilter> for TraceableFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TraceableFilter {
    let mut dst = TraceableFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TraceableFilter> for TraceableFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TraceableFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TraceableFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TraceableFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TraceableFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TraceableFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TraceableFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TraceableFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TraceableFilterMut<'msg> {
  type Message = TraceableFilter;
}

impl ::std::fmt::Debug for TraceableFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TraceableFilter>> for TraceableFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TraceableFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TraceableFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TraceableFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TraceableFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `TraceableFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TraceableFilterMut<'_> {}

// SAFETY:
// - `TraceableFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TraceableFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for TraceableFilterMut<'msg> {
  type Proxied = TraceableFilter;
  fn as_view(&self) -> ::protobuf::View<'_, TraceableFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TraceableFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TraceableFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TraceableFilterMut<'msg> {
  type MutProxied = TraceableFilter;
  fn as_mut(&mut self) -> TraceableFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TraceableFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> TraceableFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TraceableFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TraceableFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TraceableFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TraceableFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl TraceableFilter

impl ::std::ops::Drop for TraceableFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TraceableFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TraceableFilter {
  type Proxied = Self;
  fn as_view(&self) -> TraceableFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TraceableFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TraceableFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TraceableFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__TraceableFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__TraceableFilter_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__TraceableFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TraceableFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TraceableFilter {
  type Msg = TraceableFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceableFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TraceableFilter {
  type Msg = TraceableFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceableFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TraceableFilterMut<'_> {
  type Msg = TraceableFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceableFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TraceableFilterMut<'_> {
  type Msg = TraceableFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceableFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TraceableFilterView<'_> {
  type Msg = TraceableFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TraceableFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TraceableFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__RuntimeFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RuntimeFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RuntimeFilter>
}

impl ::protobuf::Message for RuntimeFilter {
  type MessageView<'msg> = RuntimeFilterView<'msg>;
  type MessageMut<'msg> = RuntimeFilterMut<'msg>;
}

impl ::std::default::Default for RuntimeFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RuntimeFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RuntimeFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `RuntimeFilterMut`.
unsafe impl ::std::marker::Sync for RuntimeFilter {}

// SAFETY:
// - `RuntimeFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeFilter {}

impl ::protobuf::Proxied for RuntimeFilter {
  type View<'msg> = RuntimeFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RuntimeFilter {}

impl ::protobuf::MutProxied for RuntimeFilter {
  type Mut<'msg> = RuntimeFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RuntimeFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RuntimeFilterView<'msg> {
  type Message = RuntimeFilter;
}

impl ::std::fmt::Debug for RuntimeFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RuntimeFilterView<'_> {
  fn default() -> RuntimeFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeFilter>> for RuntimeFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeFilterView<'msg> {

  pub fn to_owned(&self) -> RuntimeFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // runtime_key: optional string
  pub fn runtime_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // percent_sampled: optional message envoy.type.v3.FractionalPercent
  pub fn has_percent_sampled(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn percent_sampled_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg>> {
    self.has_percent_sampled().then(|| self.percent_sampled())
  }
  pub fn percent_sampled(self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }

  // use_independent_randomness: optional bool
  pub fn use_independent_randomness(self) -> bool {
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
// - `RuntimeFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RuntimeFilterView<'_> {}

// SAFETY:
// - `RuntimeFilterView` is `Send` because while its alive a `RuntimeFilterMut` cannot.
// - `RuntimeFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeFilterView<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeFilterView<'msg> {
  type Proxied = RuntimeFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, RuntimeFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeFilterView<'msg> {
  fn into_view<'shorter>(self) -> RuntimeFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeFilter> for RuntimeFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeFilter {
    let mut dst = RuntimeFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeFilter> for RuntimeFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RuntimeFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RuntimeFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RuntimeFilterMut<'msg> {
  type Message = RuntimeFilter;
}

impl ::std::fmt::Debug for RuntimeFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFilter>> for RuntimeFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RuntimeFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // percent_sampled: optional message envoy.type.v3.FractionalPercent
  pub fn has_percent_sampled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_percent_sampled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn percent_sampled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_percent_sampled().then(|| self.percent_sampled())
  }
  pub fn percent_sampled(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn percent_sampled_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_percent_sampled(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // use_independent_randomness: optional bool
  pub fn use_independent_randomness(&self) -> bool {
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
  pub fn set_use_independent_randomness(&mut self, val: bool) {
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
// - `RuntimeFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RuntimeFilterMut<'_> {}

// SAFETY:
// - `RuntimeFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RuntimeFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeFilterMut<'msg> {
  type Proxied = RuntimeFilter;
  fn as_view(&self) -> ::protobuf::View<'_, RuntimeFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RuntimeFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RuntimeFilterMut<'msg> {
  type MutProxied = RuntimeFilter;
  fn as_mut(&mut self) -> RuntimeFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RuntimeFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> RuntimeFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RuntimeFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RuntimeFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RuntimeFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RuntimeFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // percent_sampled: optional message envoy.type.v3.FractionalPercent
  pub fn has_percent_sampled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_percent_sampled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn percent_sampled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_percent_sampled().then(|| self.percent_sampled())
  }
  pub fn percent_sampled(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn percent_sampled_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_percent_sampled(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // use_independent_randomness: optional bool
  pub fn use_independent_randomness(&self) -> bool {
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
  pub fn set_use_independent_randomness(&mut self, val: bool) {
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

}  // impl RuntimeFilter

impl ::std::ops::Drop for RuntimeFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RuntimeFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RuntimeFilter {
  type Proxied = Self;
  fn as_view(&self) -> RuntimeFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RuntimeFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RuntimeFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RuntimeFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__RuntimeFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__RuntimeFilter_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__RuntimeFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeFilter {
  type Msg = RuntimeFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeFilter {
  type Msg = RuntimeFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeFilterMut<'_> {
  type Msg = RuntimeFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeFilterMut<'_> {
  type Msg = RuntimeFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeFilterView<'_> {
  type Msg = RuntimeFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__AndFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AndFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AndFilter>
}

impl ::protobuf::Message for AndFilter {
  type MessageView<'msg> = AndFilterView<'msg>;
  type MessageMut<'msg> = AndFilterMut<'msg>;
}

impl ::std::default::Default for AndFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AndFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AndFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `AndFilterMut`.
unsafe impl ::std::marker::Sync for AndFilter {}

// SAFETY:
// - `AndFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AndFilter {}

impl ::protobuf::Proxied for AndFilter {
  type View<'msg> = AndFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AndFilter {}

impl ::protobuf::MutProxied for AndFilter {
  type Mut<'msg> = AndFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AndFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AndFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AndFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AndFilterView<'msg> {
  type Message = AndFilter;
}

impl ::std::fmt::Debug for AndFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AndFilterView<'_> {
  fn default() -> AndFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AndFilter>> for AndFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AndFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AndFilterView<'msg> {

  pub fn to_owned(&self) -> AndFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // filters: repeated message envoy.config.accesslog.v3.AccessLogFilter
  pub fn filters(self) -> ::protobuf::RepeatedView<'msg, super::AccessLogFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::AccessLogFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `AndFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AndFilterView<'_> {}

// SAFETY:
// - `AndFilterView` is `Send` because while its alive a `AndFilterMut` cannot.
// - `AndFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for AndFilterView<'_> {}

impl<'msg> ::protobuf::AsView for AndFilterView<'msg> {
  type Proxied = AndFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, AndFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AndFilterView<'msg> {
  fn into_view<'shorter>(self) -> AndFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AndFilter> for AndFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AndFilter {
    let mut dst = AndFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AndFilter> for AndFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AndFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AndFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AndFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AndFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AndFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AndFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AndFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AndFilterMut<'msg> {
  type Message = AndFilter;
}

impl ::std::fmt::Debug for AndFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AndFilter>> for AndFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AndFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AndFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AndFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AndFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // filters: repeated message envoy.config.accesslog.v3.AccessLogFilter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, super::AccessLogFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::AccessLogFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::AccessLogFilter> {
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::AccessLogFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `AndFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AndFilterMut<'_> {}

// SAFETY:
// - `AndFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AndFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for AndFilterMut<'msg> {
  type Proxied = AndFilter;
  fn as_view(&self) -> ::protobuf::View<'_, AndFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AndFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AndFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AndFilterMut<'msg> {
  type MutProxied = AndFilter;
  fn as_mut(&mut self) -> AndFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AndFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> AndFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AndFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AndFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AndFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AndFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // filters: repeated message envoy.config.accesslog.v3.AccessLogFilter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, super::AccessLogFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::AccessLogFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::AccessLogFilter> {
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::AccessLogFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl AndFilter

impl ::std::ops::Drop for AndFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AndFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AndFilter {
  type Proxied = Self;
  fn as_view(&self) -> AndFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AndFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AndFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AndFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::AccessLogFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__AndFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AndFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AndFilter {
  type Msg = AndFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AndFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AndFilter {
  type Msg = AndFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AndFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AndFilterMut<'_> {
  type Msg = AndFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AndFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AndFilterMut<'_> {
  type Msg = AndFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AndFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AndFilterView<'_> {
  type Msg = AndFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AndFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AndFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__OrFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OrFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OrFilter>
}

impl ::protobuf::Message for OrFilter {
  type MessageView<'msg> = OrFilterView<'msg>;
  type MessageMut<'msg> = OrFilterMut<'msg>;
}

impl ::std::default::Default for OrFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OrFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OrFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `OrFilterMut`.
unsafe impl ::std::marker::Sync for OrFilter {}

// SAFETY:
// - `OrFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OrFilter {}

impl ::protobuf::Proxied for OrFilter {
  type View<'msg> = OrFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OrFilter {}

impl ::protobuf::MutProxied for OrFilter {
  type Mut<'msg> = OrFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OrFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OrFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OrFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OrFilterView<'msg> {
  type Message = OrFilter;
}

impl ::std::fmt::Debug for OrFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OrFilterView<'_> {
  fn default() -> OrFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OrFilter>> for OrFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OrFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OrFilterView<'msg> {

  pub fn to_owned(&self) -> OrFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // filters: repeated message envoy.config.accesslog.v3.AccessLogFilter
  pub fn filters(self) -> ::protobuf::RepeatedView<'msg, super::AccessLogFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::AccessLogFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `OrFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OrFilterView<'_> {}

// SAFETY:
// - `OrFilterView` is `Send` because while its alive a `OrFilterMut` cannot.
// - `OrFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for OrFilterView<'_> {}

impl<'msg> ::protobuf::AsView for OrFilterView<'msg> {
  type Proxied = OrFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, OrFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OrFilterView<'msg> {
  fn into_view<'shorter>(self) -> OrFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OrFilter> for OrFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OrFilter {
    let mut dst = OrFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OrFilter> for OrFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OrFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OrFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OrFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OrFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OrFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OrFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OrFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OrFilterMut<'msg> {
  type Message = OrFilter;
}

impl ::std::fmt::Debug for OrFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OrFilter>> for OrFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OrFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OrFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OrFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OrFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // filters: repeated message envoy.config.accesslog.v3.AccessLogFilter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, super::AccessLogFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::AccessLogFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::AccessLogFilter> {
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::AccessLogFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `OrFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OrFilterMut<'_> {}

// SAFETY:
// - `OrFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OrFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for OrFilterMut<'msg> {
  type Proxied = OrFilter;
  fn as_view(&self) -> ::protobuf::View<'_, OrFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OrFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OrFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OrFilterMut<'msg> {
  type MutProxied = OrFilter;
  fn as_mut(&mut self) -> OrFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OrFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> OrFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OrFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OrFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OrFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OrFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // filters: repeated message envoy.config.accesslog.v3.AccessLogFilter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, super::AccessLogFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::AccessLogFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::AccessLogFilter> {
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::AccessLogFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl OrFilter

impl ::std::ops::Drop for OrFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OrFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OrFilter {
  type Proxied = Self;
  fn as_view(&self) -> OrFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OrFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OrFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OrFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::AccessLogFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__OrFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OrFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OrFilter {
  type Msg = OrFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrFilter {
  type Msg = OrFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OrFilterMut<'_> {
  type Msg = OrFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrFilterMut<'_> {
  type Msg = OrFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OrFilterView<'_> {
  type Msg = OrFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OrFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OrFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__HeaderFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderFilter>
}

impl ::protobuf::Message for HeaderFilter {
  type MessageView<'msg> = HeaderFilterView<'msg>;
  type MessageMut<'msg> = HeaderFilterMut<'msg>;
}

impl ::std::default::Default for HeaderFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderFilterMut`.
unsafe impl ::std::marker::Sync for HeaderFilter {}

// SAFETY:
// - `HeaderFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderFilter {}

impl ::protobuf::Proxied for HeaderFilter {
  type View<'msg> = HeaderFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderFilter {}

impl ::protobuf::MutProxied for HeaderFilter {
  type Mut<'msg> = HeaderFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderFilterView<'msg> {
  type Message = HeaderFilter;
}

impl ::std::fmt::Debug for HeaderFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderFilterView<'_> {
  fn default() -> HeaderFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderFilter>> for HeaderFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderFilterView<'msg> {

  pub fn to_owned(&self) -> HeaderFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // header: optional message envoy.config.route.v3.HeaderMatcher
  pub fn has_header(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn header_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'msg>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView::default())
  }

}

// SAFETY:
// - `HeaderFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderFilterView<'_> {}

// SAFETY:
// - `HeaderFilterView` is `Send` because while its alive a `HeaderFilterMut` cannot.
// - `HeaderFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderFilterView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderFilterView<'msg> {
  type Proxied = HeaderFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderFilterView<'msg> {
  fn into_view<'shorter>(self) -> HeaderFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderFilter> for HeaderFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderFilter {
    let mut dst = HeaderFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderFilter> for HeaderFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderFilterMut<'msg> {
  type Message = HeaderFilter;
}

impl ::std::fmt::Debug for HeaderFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderFilter>> for HeaderFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // header: optional message envoy.config.route.v3.HeaderMatcher
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView::default())
  }
  pub fn header_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherMut<'_> {
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
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>) {

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
// - `HeaderFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderFilterMut<'_> {}

// SAFETY:
// - `HeaderFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderFilterMut<'msg> {
  type Proxied = HeaderFilter;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderFilterMut<'msg> {
  type MutProxied = HeaderFilter;
  fn as_mut(&mut self) -> HeaderFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // header: optional message envoy.config.route.v3.HeaderMatcher
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherView::default())
  }
  pub fn header_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcherMut<'_> {
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
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl HeaderFilter

impl ::std::ops::Drop for HeaderFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderFilter {
  type Proxied = Self;
  fn as_view(&self) -> HeaderFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__HeaderFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__HeaderFilter_msg_init.0, &[<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__HeaderFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderFilter {
  type Msg = HeaderFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderFilter {
  type Msg = HeaderFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderFilterMut<'_> {
  type Msg = HeaderFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderFilterMut<'_> {
  type Msg = HeaderFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderFilterView<'_> {
  type Msg = HeaderFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__ResponseFlagFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResponseFlagFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResponseFlagFilter>
}

impl ::protobuf::Message for ResponseFlagFilter {
  type MessageView<'msg> = ResponseFlagFilterView<'msg>;
  type MessageMut<'msg> = ResponseFlagFilterMut<'msg>;
}

impl ::std::default::Default for ResponseFlagFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResponseFlagFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResponseFlagFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `ResponseFlagFilterMut`.
unsafe impl ::std::marker::Sync for ResponseFlagFilter {}

// SAFETY:
// - `ResponseFlagFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ResponseFlagFilter {}

impl ::protobuf::Proxied for ResponseFlagFilter {
  type View<'msg> = ResponseFlagFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResponseFlagFilter {}

impl ::protobuf::MutProxied for ResponseFlagFilter {
  type Mut<'msg> = ResponseFlagFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResponseFlagFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResponseFlagFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResponseFlagFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResponseFlagFilterView<'msg> {
  type Message = ResponseFlagFilter;
}

impl ::std::fmt::Debug for ResponseFlagFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResponseFlagFilterView<'_> {
  fn default() -> ResponseFlagFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResponseFlagFilter>> for ResponseFlagFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResponseFlagFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResponseFlagFilterView<'msg> {

  pub fn to_owned(&self) -> ResponseFlagFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // flags: repeated string
  pub fn flags(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
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
// - `ResponseFlagFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResponseFlagFilterView<'_> {}

// SAFETY:
// - `ResponseFlagFilterView` is `Send` because while its alive a `ResponseFlagFilterMut` cannot.
// - `ResponseFlagFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResponseFlagFilterView<'_> {}

impl<'msg> ::protobuf::AsView for ResponseFlagFilterView<'msg> {
  type Proxied = ResponseFlagFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, ResponseFlagFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResponseFlagFilterView<'msg> {
  fn into_view<'shorter>(self) -> ResponseFlagFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResponseFlagFilter> for ResponseFlagFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResponseFlagFilter {
    let mut dst = ResponseFlagFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResponseFlagFilter> for ResponseFlagFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResponseFlagFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ResponseFlagFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResponseFlagFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResponseFlagFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResponseFlagFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseFlagFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResponseFlagFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResponseFlagFilterMut<'msg> {
  type Message = ResponseFlagFilter;
}

impl ::std::fmt::Debug for ResponseFlagFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseFlagFilter>> for ResponseFlagFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseFlagFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResponseFlagFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResponseFlagFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ResponseFlagFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // flags: repeated string
  pub fn flags(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn flags_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_flags(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ResponseFlagFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResponseFlagFilterMut<'_> {}

// SAFETY:
// - `ResponseFlagFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResponseFlagFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for ResponseFlagFilterMut<'msg> {
  type Proxied = ResponseFlagFilter;
  fn as_view(&self) -> ::protobuf::View<'_, ResponseFlagFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResponseFlagFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResponseFlagFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResponseFlagFilterMut<'msg> {
  type MutProxied = ResponseFlagFilter;
  fn as_mut(&mut self) -> ResponseFlagFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResponseFlagFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> ResponseFlagFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResponseFlagFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResponseFlagFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResponseFlagFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResponseFlagFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // flags: repeated string
  pub fn flags(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn flags_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_flags(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ResponseFlagFilter

impl ::std::ops::Drop for ResponseFlagFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResponseFlagFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResponseFlagFilter {
  type Proxied = Self;
  fn as_view(&self) -> ResponseFlagFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResponseFlagFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResponseFlagFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResponseFlagFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__ResponseFlagFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ME");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__ResponseFlagFilter_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__ResponseFlagFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResponseFlagFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResponseFlagFilter {
  type Msg = ResponseFlagFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlagFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResponseFlagFilter {
  type Msg = ResponseFlagFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlagFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResponseFlagFilterMut<'_> {
  type Msg = ResponseFlagFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlagFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResponseFlagFilterMut<'_> {
  type Msg = ResponseFlagFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlagFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResponseFlagFilterView<'_> {
  type Msg = ResponseFlagFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResponseFlagFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResponseFlagFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__GrpcStatusFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GrpcStatusFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GrpcStatusFilter>
}

impl ::protobuf::Message for GrpcStatusFilter {
  type MessageView<'msg> = GrpcStatusFilterView<'msg>;
  type MessageMut<'msg> = GrpcStatusFilterMut<'msg>;
}

impl ::std::default::Default for GrpcStatusFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GrpcStatusFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GrpcStatusFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `GrpcStatusFilterMut`.
unsafe impl ::std::marker::Sync for GrpcStatusFilter {}

// SAFETY:
// - `GrpcStatusFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GrpcStatusFilter {}

impl ::protobuf::Proxied for GrpcStatusFilter {
  type View<'msg> = GrpcStatusFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GrpcStatusFilter {}

impl ::protobuf::MutProxied for GrpcStatusFilter {
  type Mut<'msg> = GrpcStatusFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GrpcStatusFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcStatusFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcStatusFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GrpcStatusFilterView<'msg> {
  type Message = GrpcStatusFilter;
}

impl ::std::fmt::Debug for GrpcStatusFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GrpcStatusFilterView<'_> {
  fn default() -> GrpcStatusFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcStatusFilter>> for GrpcStatusFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcStatusFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcStatusFilterView<'msg> {

  pub fn to_owned(&self) -> GrpcStatusFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // statuses: repeated enum envoy.config.accesslog.v3.GrpcStatusFilter.Status
  pub fn statuses(self) -> ::protobuf::RepeatedView<'msg, super::grpc_status_filter::Status> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::grpc_status_filter::Status>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // exclude: optional bool
  pub fn exclude(self) -> bool {
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

}

// SAFETY:
// - `GrpcStatusFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GrpcStatusFilterView<'_> {}

// SAFETY:
// - `GrpcStatusFilterView` is `Send` because while its alive a `GrpcStatusFilterMut` cannot.
// - `GrpcStatusFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for GrpcStatusFilterView<'_> {}

impl<'msg> ::protobuf::AsView for GrpcStatusFilterView<'msg> {
  type Proxied = GrpcStatusFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, GrpcStatusFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcStatusFilterView<'msg> {
  fn into_view<'shorter>(self) -> GrpcStatusFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcStatusFilter> for GrpcStatusFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcStatusFilter {
    let mut dst = GrpcStatusFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcStatusFilter> for GrpcStatusFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcStatusFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GrpcStatusFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcStatusFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcStatusFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GrpcStatusFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcStatusFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcStatusFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GrpcStatusFilterMut<'msg> {
  type Message = GrpcStatusFilter;
}

impl ::std::fmt::Debug for GrpcStatusFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcStatusFilter>> for GrpcStatusFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcStatusFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcStatusFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcStatusFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GrpcStatusFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // statuses: repeated enum envoy.config.accesslog.v3.GrpcStatusFilter.Status
  pub fn statuses(&self) -> ::protobuf::RepeatedView<'_, super::grpc_status_filter::Status> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::grpc_status_filter::Status>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn statuses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::grpc_status_filter::Status> {
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
  pub fn set_statuses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::grpc_status_filter::Status>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // exclude: optional bool
  pub fn exclude(&self) -> bool {
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
  pub fn set_exclude(&mut self, val: bool) {
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

}

// SAFETY:
// - `GrpcStatusFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GrpcStatusFilterMut<'_> {}

// SAFETY:
// - `GrpcStatusFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GrpcStatusFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for GrpcStatusFilterMut<'msg> {
  type Proxied = GrpcStatusFilter;
  fn as_view(&self) -> ::protobuf::View<'_, GrpcStatusFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcStatusFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GrpcStatusFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GrpcStatusFilterMut<'msg> {
  type MutProxied = GrpcStatusFilter;
  fn as_mut(&mut self) -> GrpcStatusFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GrpcStatusFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> GrpcStatusFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GrpcStatusFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GrpcStatusFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GrpcStatusFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GrpcStatusFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // statuses: repeated enum envoy.config.accesslog.v3.GrpcStatusFilter.Status
  pub fn statuses(&self) -> ::protobuf::RepeatedView<'_, super::grpc_status_filter::Status> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::grpc_status_filter::Status>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn statuses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::grpc_status_filter::Status> {
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
  pub fn set_statuses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::grpc_status_filter::Status>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // exclude: optional bool
  pub fn exclude(&self) -> bool {
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
  pub fn set_exclude(&mut self, val: bool) {
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

}  // impl GrpcStatusFilter

impl ::std::ops::Drop for GrpcStatusFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GrpcStatusFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GrpcStatusFilter {
  type Proxied = Self;
  fn as_view(&self) -> GrpcStatusFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GrpcStatusFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GrpcStatusFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GrpcStatusFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__GrpcStatusFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$NB/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__GrpcStatusFilter_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__GrpcStatusFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcStatusFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcStatusFilter {
  type Msg = GrpcStatusFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatusFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcStatusFilter {
  type Msg = GrpcStatusFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatusFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcStatusFilterMut<'_> {
  type Msg = GrpcStatusFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatusFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcStatusFilterMut<'_> {
  type Msg = GrpcStatusFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatusFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcStatusFilterView<'_> {
  type Msg = GrpcStatusFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcStatusFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcStatusFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod grpc_status_filter {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Status(i32);

#[allow(non_upper_case_globals)]
impl Status {
  pub const Ok: Status = Status(0);
  pub const Canceled: Status = Status(1);
  pub const Unknown: Status = Status(2);
  pub const InvalidArgument: Status = Status(3);
  pub const DeadlineExceeded: Status = Status(4);
  pub const NotFound: Status = Status(5);
  pub const AlreadyExists: Status = Status(6);
  pub const PermissionDenied: Status = Status(7);
  pub const ResourceExhausted: Status = Status(8);
  pub const FailedPrecondition: Status = Status(9);
  pub const Aborted: Status = Status(10);
  pub const OutOfRange: Status = Status(11);
  pub const Unimplemented: Status = Status(12);
  pub const Internal: Status = Status(13);
  pub const Unavailable: Status = Status(14);
  pub const DataLoss: Status = Status(15);
  pub const Unauthenticated: Status = Status(16);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Ok",
      1 => "Canceled",
      2 => "Unknown",
      3 => "InvalidArgument",
      4 => "DeadlineExceeded",
      5 => "NotFound",
      6 => "AlreadyExists",
      7 => "PermissionDenied",
      8 => "ResourceExhausted",
      9 => "FailedPrecondition",
      10 => "Aborted",
      11 => "OutOfRange",
      12 => "Unimplemented",
      13 => "Internal",
      14 => "Unavailable",
      15 => "DataLoss",
      16 => "Unauthenticated",
      _ => return None
    })
  }
}

impl ::std::convert::From<Status> for i32 {
  fn from(val: Status) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Status {
  fn from(val: i32) -> Status {
    Self(val)
  }
}

impl ::std::default::Default for Status {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Status {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Status::{}", constant_name)
    } else {
      write!(f, "Status::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Status {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Status {}

impl ::protobuf::Proxied for Status {
  type View<'a> = Status;
}

impl ::protobuf::AsView for Status {
  type Proxied = Status;

  fn as_view(&self) -> Status {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Status {
  fn into_view<'shorter>(self) -> Status where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Status {
  const NAME: &'static str = "Status";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16)
  }
}

impl ::protobuf::__internal::EntityType for Status {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod grpc_status_filter


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__MetadataFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MetadataFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MetadataFilter>
}

impl ::protobuf::Message for MetadataFilter {
  type MessageView<'msg> = MetadataFilterView<'msg>;
  type MessageMut<'msg> = MetadataFilterMut<'msg>;
}

impl ::std::default::Default for MetadataFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MetadataFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MetadataFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `MetadataFilterMut`.
unsafe impl ::std::marker::Sync for MetadataFilter {}

// SAFETY:
// - `MetadataFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MetadataFilter {}

impl ::protobuf::Proxied for MetadataFilter {
  type View<'msg> = MetadataFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MetadataFilter {}

impl ::protobuf::MutProxied for MetadataFilter {
  type Mut<'msg> = MetadataFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetadataFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetadataFilterView<'msg> {
  type Message = MetadataFilter;
}

impl ::std::fmt::Debug for MetadataFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetadataFilterView<'_> {
  fn default() -> MetadataFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataFilter>> for MetadataFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MetadataFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataFilterView<'msg> {

  pub fn to_owned(&self) -> MetadataFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // matcher: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn matcher_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }

  // match_if_key_not_found: optional message google.protobuf.BoolValue
  pub fn has_match_if_key_not_found(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn match_if_key_not_found_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_match_if_key_not_found().then(|| self.match_if_key_not_found())
  }
  pub fn match_if_key_not_found(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

}

// SAFETY:
// - `MetadataFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MetadataFilterView<'_> {}

// SAFETY:
// - `MetadataFilterView` is `Send` because while its alive a `MetadataFilterMut` cannot.
// - `MetadataFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for MetadataFilterView<'_> {}

impl<'msg> ::protobuf::AsView for MetadataFilterView<'msg> {
  type Proxied = MetadataFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, MetadataFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataFilterView<'msg> {
  fn into_view<'shorter>(self) -> MetadataFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataFilter> for MetadataFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataFilter {
    let mut dst = MetadataFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MetadataFilter> for MetadataFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MetadataFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MetadataFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetadataFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetadataFilterMut<'msg> {
  type Message = MetadataFilter;
}

impl ::std::fmt::Debug for MetadataFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataFilter>> for MetadataFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MetadataFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MetadataFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // matcher: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // match_if_key_not_found: optional message google.protobuf.BoolValue
  pub fn has_match_if_key_not_found(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_match_if_key_not_found(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn match_if_key_not_found_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_match_if_key_not_found().then(|| self.match_if_key_not_found())
  }
  pub fn match_if_key_not_found(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn match_if_key_not_found_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_match_if_key_not_found(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

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
// - `MetadataFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MetadataFilterMut<'_> {}

// SAFETY:
// - `MetadataFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MetadataFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for MetadataFilterMut<'msg> {
  type Proxied = MetadataFilter;
  fn as_view(&self) -> ::protobuf::View<'_, MetadataFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MetadataFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MetadataFilterMut<'msg> {
  type MutProxied = MetadataFilter;
  fn as_mut(&mut self) -> MetadataFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetadataFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> MetadataFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MetadataFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MetadataFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetadataFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetadataFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // matcher: optional message envoy.type.matcher.v3.MetadataMatcher
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcherMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // match_if_key_not_found: optional message google.protobuf.BoolValue
  pub fn has_match_if_key_not_found(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_match_if_key_not_found(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn match_if_key_not_found_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_match_if_key_not_found().then(|| self.match_if_key_not_found())
  }
  pub fn match_if_key_not_found(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn match_if_key_not_found_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_match_if_key_not_found(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl MetadataFilter

impl ::std::ops::Drop for MetadataFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MetadataFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MetadataFilter {
  type Proxied = Self;
  fn as_view(&self) -> MetadataFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MetadataFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetadataFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MetadataFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__MetadataFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__MetadataFilter_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::metadata::MetadataMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__MetadataFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataFilter {
  type Msg = MetadataFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataFilter {
  type Msg = MetadataFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataFilterMut<'_> {
  type Msg = MetadataFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataFilterMut<'_> {
  type Msg = MetadataFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataFilterView<'_> {
  type Msg = MetadataFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MetadataFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__LogTypeFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LogTypeFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LogTypeFilter>
}

impl ::protobuf::Message for LogTypeFilter {
  type MessageView<'msg> = LogTypeFilterView<'msg>;
  type MessageMut<'msg> = LogTypeFilterMut<'msg>;
}

impl ::std::default::Default for LogTypeFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LogTypeFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LogTypeFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `LogTypeFilterMut`.
unsafe impl ::std::marker::Sync for LogTypeFilter {}

// SAFETY:
// - `LogTypeFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LogTypeFilter {}

impl ::protobuf::Proxied for LogTypeFilter {
  type View<'msg> = LogTypeFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LogTypeFilter {}

impl ::protobuf::MutProxied for LogTypeFilter {
  type Mut<'msg> = LogTypeFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LogTypeFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogTypeFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogTypeFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LogTypeFilterView<'msg> {
  type Message = LogTypeFilter;
}

impl ::std::fmt::Debug for LogTypeFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LogTypeFilterView<'_> {
  fn default() -> LogTypeFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LogTypeFilter>> for LogTypeFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogTypeFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogTypeFilterView<'msg> {

  pub fn to_owned(&self) -> LogTypeFilter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // types: repeated enum envoy.data.accesslog.v3.AccessLogType
  pub fn types(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // exclude: optional bool
  pub fn exclude(self) -> bool {
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

}

// SAFETY:
// - `LogTypeFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LogTypeFilterView<'_> {}

// SAFETY:
// - `LogTypeFilterView` is `Send` because while its alive a `LogTypeFilterMut` cannot.
// - `LogTypeFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for LogTypeFilterView<'_> {}

impl<'msg> ::protobuf::AsView for LogTypeFilterView<'msg> {
  type Proxied = LogTypeFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, LogTypeFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogTypeFilterView<'msg> {
  fn into_view<'shorter>(self) -> LogTypeFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LogTypeFilter> for LogTypeFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogTypeFilter {
    let mut dst = LogTypeFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LogTypeFilter> for LogTypeFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogTypeFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LogTypeFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LogTypeFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LogTypeFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LogTypeFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogTypeFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogTypeFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LogTypeFilterMut<'msg> {
  type Message = LogTypeFilter;
}

impl ::std::fmt::Debug for LogTypeFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LogTypeFilter>> for LogTypeFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogTypeFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogTypeFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LogTypeFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LogTypeFilter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // types: repeated enum envoy.data.accesslog.v3.AccessLogType
  pub fn types(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn types_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType> {
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
  pub fn set_types(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // exclude: optional bool
  pub fn exclude(&self) -> bool {
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
  pub fn set_exclude(&mut self, val: bool) {
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

}

// SAFETY:
// - `LogTypeFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LogTypeFilterMut<'_> {}

// SAFETY:
// - `LogTypeFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LogTypeFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for LogTypeFilterMut<'msg> {
  type Proxied = LogTypeFilter;
  fn as_view(&self) -> ::protobuf::View<'_, LogTypeFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogTypeFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LogTypeFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LogTypeFilterMut<'msg> {
  type MutProxied = LogTypeFilter;
  fn as_mut(&mut self) -> LogTypeFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LogTypeFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> LogTypeFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LogTypeFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LogTypeFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LogTypeFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LogTypeFilterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // types: repeated enum envoy.data.accesslog.v3.AccessLogType
  pub fn types(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn types_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType> {
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
  pub fn set_types(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::data::accesslog::v3::accesslog::AccessLogType>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // exclude: optional bool
  pub fn exclude(&self) -> bool {
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
  pub fn set_exclude(&mut self, val: bool) {
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

}  // impl LogTypeFilter

impl ::std::ops::Drop for LogTypeFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LogTypeFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LogTypeFilter {
  type Proxied = Self;
  fn as_view(&self) -> LogTypeFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LogTypeFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LogTypeFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LogTypeFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__LogTypeFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$NB/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__LogTypeFilter_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__LogTypeFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogTypeFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogTypeFilter {
  type Msg = LogTypeFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogTypeFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogTypeFilter {
  type Msg = LogTypeFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogTypeFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogTypeFilterMut<'_> {
  type Msg = LogTypeFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogTypeFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogTypeFilterMut<'_> {
  type Msg = LogTypeFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogTypeFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogTypeFilterView<'_> {
  type Msg = LogTypeFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogTypeFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogTypeFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__accesslog__v3__ExtensionFilter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtensionFilter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtensionFilter>
}

impl ::protobuf::Message for ExtensionFilter {
  type MessageView<'msg> = ExtensionFilterView<'msg>;
  type MessageMut<'msg> = ExtensionFilterMut<'msg>;
}

impl ::std::default::Default for ExtensionFilter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtensionFilter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtensionFilter` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtensionFilterMut`.
unsafe impl ::std::marker::Sync for ExtensionFilter {}

// SAFETY:
// - `ExtensionFilter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionFilter {}

impl ::protobuf::Proxied for ExtensionFilter {
  type View<'msg> = ExtensionFilterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtensionFilter {}

impl ::protobuf::MutProxied for ExtensionFilter {
  type Mut<'msg> = ExtensionFilterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtensionFilterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionFilterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtensionFilterView<'msg> {
  type Message = ExtensionFilter;
}

impl ::std::fmt::Debug for ExtensionFilterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtensionFilterView<'_> {
  fn default() -> ExtensionFilterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionFilter>> for ExtensionFilterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionFilterView<'msg> {

  pub fn to_owned(&self) -> ExtensionFilter {
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

  pub fn config_type(self) -> super::extension_filter::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::extension_filter::ConfigTypeCase::TypedConfig =>
          super::extension_filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::extension_filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::extension_filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::extension_filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExtensionFilterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtensionFilterView<'_> {}

// SAFETY:
// - `ExtensionFilterView` is `Send` because while its alive a `ExtensionFilterMut` cannot.
// - `ExtensionFilterView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionFilterView<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionFilterView<'msg> {
  type Proxied = ExtensionFilter;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtensionFilter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionFilterView<'msg> {
  fn into_view<'shorter>(self) -> ExtensionFilterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtensionFilter> for ExtensionFilterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtensionFilter {
    let mut dst = ExtensionFilter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtensionFilter> for ExtensionFilterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtensionFilter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtensionFilter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionFilterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionFilterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtensionFilterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionFilter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionFilterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtensionFilterMut<'msg> {
  type Message = ExtensionFilter;
}

impl ::std::fmt::Debug for ExtensionFilterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionFilter>> for ExtensionFilterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionFilter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionFilterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionFilter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtensionFilter {
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

  pub fn config_type(&self) -> super::extension_filter::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::extension_filter::ConfigTypeCase::TypedConfig =>
          super::extension_filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::extension_filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::extension_filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::extension_filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ExtensionFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtensionFilterMut<'_> {}

// SAFETY:
// - `ExtensionFilterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtensionFilterMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionFilterMut<'msg> {
  type Proxied = ExtensionFilter;
  fn as_view(&self) -> ::protobuf::View<'_, ExtensionFilter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionFilterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtensionFilter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtensionFilterMut<'msg> {
  type MutProxied = ExtensionFilter;
  fn as_mut(&mut self) -> ExtensionFilterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtensionFilterMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtensionFilterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtensionFilter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtensionFilter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtensionFilterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtensionFilterMut<'_> {
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

  pub fn config_type(&self) -> super::extension_filter::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::extension_filter::ConfigTypeCase::TypedConfig =>
          super::extension_filter::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::extension_filter::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::extension_filter::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::extension_filter::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ExtensionFilter

impl ::std::ops::Drop for ExtensionFilter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtensionFilter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtensionFilter {
  type Proxied = Self;
  fn as_view(&self) -> ExtensionFilterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtensionFilter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtensionFilterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtensionFilter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__accesslog__v3__ExtensionFilter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa3^$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__accesslog__v3__ExtensionFilter_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__accesslog__v3__ExtensionFilter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionFilter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionFilter {
  type Msg = ExtensionFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionFilter {
  type Msg = ExtensionFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionFilterMut<'_> {
  type Msg = ExtensionFilter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionFilter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionFilterMut<'_> {
  type Msg = ExtensionFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionFilter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionFilterView<'_> {
  type Msg = ExtensionFilter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionFilter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionFilterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod extension_filter {

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
}  // pub mod extension_filter


