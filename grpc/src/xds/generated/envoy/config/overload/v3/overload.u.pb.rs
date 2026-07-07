const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__ResourceMonitor_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ResourceMonitor {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ResourceMonitor>
}

impl ::protobuf::Message for ResourceMonitor {
  type MessageView<'msg> = ResourceMonitorView<'msg>;
  type MessageMut<'msg> = ResourceMonitorMut<'msg>;
}

impl ::std::default::Default for ResourceMonitor {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ResourceMonitor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ResourceMonitor` is `Sync` because it does not implement interior mutability.
//    Neither does `ResourceMonitorMut`.
unsafe impl ::std::marker::Sync for ResourceMonitor {}

// SAFETY:
// - `ResourceMonitor` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ResourceMonitor {}

impl ::protobuf::Proxied for ResourceMonitor {
  type View<'msg> = ResourceMonitorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ResourceMonitor {}

impl ::protobuf::MutProxied for ResourceMonitor {
  type Mut<'msg> = ResourceMonitorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ResourceMonitorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceMonitor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceMonitorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ResourceMonitorView<'msg> {
  type Message = ResourceMonitor;
}

impl ::std::fmt::Debug for ResourceMonitorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ResourceMonitorView<'_> {
  fn default() -> ResourceMonitorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceMonitor>> for ResourceMonitorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ResourceMonitor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceMonitorView<'msg> {

  pub fn to_owned(&self) -> ResourceMonitor {
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

  pub fn config_type(self) -> super::resource_monitor::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::resource_monitor::ConfigTypeCase::TypedConfig =>
          super::resource_monitor::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::resource_monitor::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::resource_monitor::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::resource_monitor::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ResourceMonitorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ResourceMonitorView<'_> {}

// SAFETY:
// - `ResourceMonitorView` is `Send` because while its alive a `ResourceMonitorMut` cannot.
// - `ResourceMonitorView` does not use thread-local data.
unsafe impl ::std::marker::Send for ResourceMonitorView<'_> {}

impl<'msg> ::protobuf::AsView for ResourceMonitorView<'msg> {
  type Proxied = ResourceMonitor;
  fn as_view(&self) -> ::protobuf::View<'msg, ResourceMonitor> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceMonitorView<'msg> {
  fn into_view<'shorter>(self) -> ResourceMonitorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceMonitor> for ResourceMonitorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceMonitor {
    let mut dst = ResourceMonitor::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ResourceMonitor> for ResourceMonitorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ResourceMonitor {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ResourceMonitor {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceMonitorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ResourceMonitorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ResourceMonitorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceMonitor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ResourceMonitorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ResourceMonitorMut<'msg> {
  type Message = ResourceMonitor;
}

impl ::std::fmt::Debug for ResourceMonitorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceMonitor>> for ResourceMonitorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceMonitor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ResourceMonitorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ResourceMonitor> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ResourceMonitor {
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

  pub fn config_type(&self) -> super::resource_monitor::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::resource_monitor::ConfigTypeCase::TypedConfig =>
          super::resource_monitor::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::resource_monitor::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::resource_monitor::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::resource_monitor::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ResourceMonitorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ResourceMonitorMut<'_> {}

// SAFETY:
// - `ResourceMonitorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ResourceMonitorMut<'_> {}

impl<'msg> ::protobuf::AsView for ResourceMonitorMut<'msg> {
  type Proxied = ResourceMonitor;
  fn as_view(&self) -> ::protobuf::View<'_, ResourceMonitor> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ResourceMonitorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ResourceMonitor>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ResourceMonitorMut<'msg> {
  type MutProxied = ResourceMonitor;
  fn as_mut(&mut self) -> ResourceMonitorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ResourceMonitorMut<'msg> {
  fn into_mut<'shorter>(self) -> ResourceMonitorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ResourceMonitor {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ResourceMonitor> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ResourceMonitorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ResourceMonitorMut<'_> {
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

  pub fn config_type(&self) -> super::resource_monitor::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::resource_monitor::ConfigTypeCase::TypedConfig =>
          super::resource_monitor::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::resource_monitor::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::resource_monitor::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::resource_monitor::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ResourceMonitor

impl ::std::ops::Drop for ResourceMonitor {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ResourceMonitor {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ResourceMonitor {
  type Proxied = Self;
  fn as_view(&self) -> ResourceMonitorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ResourceMonitor {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ResourceMonitorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ResourceMonitor {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__overload__v3__ResourceMonitor_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa3^$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__overload__v3__ResourceMonitor_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__overload__v3__ResourceMonitor_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceMonitor {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceMonitor {
  type Msg = ResourceMonitor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceMonitor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceMonitor {
  type Msg = ResourceMonitor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceMonitor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ResourceMonitorMut<'_> {
  type Msg = ResourceMonitor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceMonitor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceMonitorMut<'_> {
  type Msg = ResourceMonitor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceMonitor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ResourceMonitorView<'_> {
  type Msg = ResourceMonitor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ResourceMonitor> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ResourceMonitorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod resource_monitor {

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
}  // pub mod resource_monitor


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__ThresholdTrigger_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ThresholdTrigger {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ThresholdTrigger>
}

impl ::protobuf::Message for ThresholdTrigger {
  type MessageView<'msg> = ThresholdTriggerView<'msg>;
  type MessageMut<'msg> = ThresholdTriggerMut<'msg>;
}

impl ::std::default::Default for ThresholdTrigger {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ThresholdTrigger {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ThresholdTrigger` is `Sync` because it does not implement interior mutability.
//    Neither does `ThresholdTriggerMut`.
unsafe impl ::std::marker::Sync for ThresholdTrigger {}

// SAFETY:
// - `ThresholdTrigger` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ThresholdTrigger {}

impl ::protobuf::Proxied for ThresholdTrigger {
  type View<'msg> = ThresholdTriggerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ThresholdTrigger {}

impl ::protobuf::MutProxied for ThresholdTrigger {
  type Mut<'msg> = ThresholdTriggerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ThresholdTriggerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ThresholdTrigger>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ThresholdTriggerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ThresholdTriggerView<'msg> {
  type Message = ThresholdTrigger;
}

impl ::std::fmt::Debug for ThresholdTriggerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ThresholdTriggerView<'_> {
  fn default() -> ThresholdTriggerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ThresholdTrigger>> for ThresholdTriggerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ThresholdTrigger>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ThresholdTriggerView<'msg> {

  pub fn to_owned(&self) -> ThresholdTrigger {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // value: optional double
  pub fn value(self) -> f64 {
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

}

// SAFETY:
// - `ThresholdTriggerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ThresholdTriggerView<'_> {}

// SAFETY:
// - `ThresholdTriggerView` is `Send` because while its alive a `ThresholdTriggerMut` cannot.
// - `ThresholdTriggerView` does not use thread-local data.
unsafe impl ::std::marker::Send for ThresholdTriggerView<'_> {}

impl<'msg> ::protobuf::AsView for ThresholdTriggerView<'msg> {
  type Proxied = ThresholdTrigger;
  fn as_view(&self) -> ::protobuf::View<'msg, ThresholdTrigger> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ThresholdTriggerView<'msg> {
  fn into_view<'shorter>(self) -> ThresholdTriggerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ThresholdTrigger> for ThresholdTriggerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ThresholdTrigger {
    let mut dst = ThresholdTrigger::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ThresholdTrigger> for ThresholdTriggerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ThresholdTrigger {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ThresholdTrigger {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ThresholdTriggerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ThresholdTriggerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ThresholdTriggerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ThresholdTrigger>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ThresholdTriggerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ThresholdTriggerMut<'msg> {
  type Message = ThresholdTrigger;
}

impl ::std::fmt::Debug for ThresholdTriggerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ThresholdTrigger>> for ThresholdTriggerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ThresholdTrigger>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ThresholdTriggerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ThresholdTrigger> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ThresholdTrigger {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // value: optional double
  pub fn value(&self) -> f64 {
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
  pub fn set_value(&mut self, val: f64) {
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

}

// SAFETY:
// - `ThresholdTriggerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ThresholdTriggerMut<'_> {}

// SAFETY:
// - `ThresholdTriggerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ThresholdTriggerMut<'_> {}

impl<'msg> ::protobuf::AsView for ThresholdTriggerMut<'msg> {
  type Proxied = ThresholdTrigger;
  fn as_view(&self) -> ::protobuf::View<'_, ThresholdTrigger> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ThresholdTriggerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ThresholdTrigger>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ThresholdTriggerMut<'msg> {
  type MutProxied = ThresholdTrigger;
  fn as_mut(&mut self) -> ThresholdTriggerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ThresholdTriggerMut<'msg> {
  fn into_mut<'shorter>(self) -> ThresholdTriggerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ThresholdTrigger {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ThresholdTrigger> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ThresholdTriggerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ThresholdTriggerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // value: optional double
  pub fn value(&self) -> f64 {
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
  pub fn set_value(&mut self, val: f64) {
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

}  // impl ThresholdTrigger

impl ::std::ops::Drop for ThresholdTrigger {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ThresholdTrigger {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ThresholdTrigger {
  type Proxied = Self;
  fn as_view(&self) -> ThresholdTriggerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ThresholdTrigger {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ThresholdTriggerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ThresholdTrigger {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__overload__v3__ThresholdTrigger_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__overload__v3__ThresholdTrigger_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__overload__v3__ThresholdTrigger_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ThresholdTrigger {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ThresholdTrigger {
  type Msg = ThresholdTrigger;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ThresholdTrigger> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ThresholdTrigger {
  type Msg = ThresholdTrigger;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ThresholdTrigger> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ThresholdTriggerMut<'_> {
  type Msg = ThresholdTrigger;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ThresholdTrigger> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ThresholdTriggerMut<'_> {
  type Msg = ThresholdTrigger;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ThresholdTrigger> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ThresholdTriggerView<'_> {
  type Msg = ThresholdTrigger;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ThresholdTrigger> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ThresholdTriggerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__ScaledTrigger_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScaledTrigger {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScaledTrigger>
}

impl ::protobuf::Message for ScaledTrigger {
  type MessageView<'msg> = ScaledTriggerView<'msg>;
  type MessageMut<'msg> = ScaledTriggerMut<'msg>;
}

impl ::std::default::Default for ScaledTrigger {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScaledTrigger {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScaledTrigger` is `Sync` because it does not implement interior mutability.
//    Neither does `ScaledTriggerMut`.
unsafe impl ::std::marker::Sync for ScaledTrigger {}

// SAFETY:
// - `ScaledTrigger` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScaledTrigger {}

impl ::protobuf::Proxied for ScaledTrigger {
  type View<'msg> = ScaledTriggerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScaledTrigger {}

impl ::protobuf::MutProxied for ScaledTrigger {
  type Mut<'msg> = ScaledTriggerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScaledTriggerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScaledTrigger>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScaledTriggerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScaledTriggerView<'msg> {
  type Message = ScaledTrigger;
}

impl ::std::fmt::Debug for ScaledTriggerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScaledTriggerView<'_> {
  fn default() -> ScaledTriggerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScaledTrigger>> for ScaledTriggerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScaledTrigger>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScaledTriggerView<'msg> {

  pub fn to_owned(&self) -> ScaledTrigger {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // scaling_threshold: optional double
  pub fn scaling_threshold(self) -> f64 {
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

  // saturation_threshold: optional double
  pub fn saturation_threshold(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ScaledTriggerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScaledTriggerView<'_> {}

// SAFETY:
// - `ScaledTriggerView` is `Send` because while its alive a `ScaledTriggerMut` cannot.
// - `ScaledTriggerView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScaledTriggerView<'_> {}

impl<'msg> ::protobuf::AsView for ScaledTriggerView<'msg> {
  type Proxied = ScaledTrigger;
  fn as_view(&self) -> ::protobuf::View<'msg, ScaledTrigger> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScaledTriggerView<'msg> {
  fn into_view<'shorter>(self) -> ScaledTriggerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScaledTrigger> for ScaledTriggerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScaledTrigger {
    let mut dst = ScaledTrigger::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScaledTrigger> for ScaledTriggerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScaledTrigger {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScaledTrigger {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScaledTriggerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScaledTriggerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScaledTriggerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScaledTrigger>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScaledTriggerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScaledTriggerMut<'msg> {
  type Message = ScaledTrigger;
}

impl ::std::fmt::Debug for ScaledTriggerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScaledTrigger>> for ScaledTriggerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScaledTrigger>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScaledTriggerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScaledTrigger> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScaledTrigger {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // scaling_threshold: optional double
  pub fn scaling_threshold(&self) -> f64 {
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
  pub fn set_scaling_threshold(&mut self, val: f64) {
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

  // saturation_threshold: optional double
  pub fn saturation_threshold(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_saturation_threshold(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `ScaledTriggerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScaledTriggerMut<'_> {}

// SAFETY:
// - `ScaledTriggerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScaledTriggerMut<'_> {}

impl<'msg> ::protobuf::AsView for ScaledTriggerMut<'msg> {
  type Proxied = ScaledTrigger;
  fn as_view(&self) -> ::protobuf::View<'_, ScaledTrigger> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScaledTriggerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScaledTrigger>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScaledTriggerMut<'msg> {
  type MutProxied = ScaledTrigger;
  fn as_mut(&mut self) -> ScaledTriggerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScaledTriggerMut<'msg> {
  fn into_mut<'shorter>(self) -> ScaledTriggerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScaledTrigger {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScaledTrigger> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScaledTriggerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScaledTriggerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // scaling_threshold: optional double
  pub fn scaling_threshold(&self) -> f64 {
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
  pub fn set_scaling_threshold(&mut self, val: f64) {
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

  // saturation_threshold: optional double
  pub fn saturation_threshold(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_saturation_threshold(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

}  // impl ScaledTrigger

impl ::std::ops::Drop for ScaledTrigger {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScaledTrigger {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScaledTrigger {
  type Proxied = Self;
  fn as_view(&self) -> ScaledTriggerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScaledTrigger {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScaledTriggerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScaledTrigger {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__overload__v3__ScaledTrigger_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ P P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__overload__v3__ScaledTrigger_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__overload__v3__ScaledTrigger_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScaledTrigger {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScaledTrigger {
  type Msg = ScaledTrigger;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaledTrigger> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScaledTrigger {
  type Msg = ScaledTrigger;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaledTrigger> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScaledTriggerMut<'_> {
  type Msg = ScaledTrigger;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaledTrigger> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScaledTriggerMut<'_> {
  type Msg = ScaledTrigger;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaledTrigger> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScaledTriggerView<'_> {
  type Msg = ScaledTrigger;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaledTrigger> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScaledTriggerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__Trigger_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Trigger {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Trigger>
}

impl ::protobuf::Message for Trigger {
  type MessageView<'msg> = TriggerView<'msg>;
  type MessageMut<'msg> = TriggerMut<'msg>;
}

impl ::std::default::Default for Trigger {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Trigger {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Trigger` is `Sync` because it does not implement interior mutability.
//    Neither does `TriggerMut`.
unsafe impl ::std::marker::Sync for Trigger {}

// SAFETY:
// - `Trigger` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Trigger {}

impl ::protobuf::Proxied for Trigger {
  type View<'msg> = TriggerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Trigger {}

impl ::protobuf::MutProxied for Trigger {
  type Mut<'msg> = TriggerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TriggerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Trigger>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TriggerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TriggerView<'msg> {
  type Message = Trigger;
}

impl ::std::fmt::Debug for TriggerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TriggerView<'_> {
  fn default() -> TriggerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Trigger>> for TriggerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Trigger>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TriggerView<'msg> {

  pub fn to_owned(&self) -> Trigger {
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

  // threshold: optional message envoy.config.overload.v3.ThresholdTrigger
  pub fn has_threshold(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn threshold_opt(self) -> ::std::option::Option<super::ThresholdTriggerView<'msg>> {
    self.has_threshold().then(|| self.threshold())
  }
  pub fn threshold(self) -> super::ThresholdTriggerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ThresholdTriggerView::default())
  }

  // scaled: optional message envoy.config.overload.v3.ScaledTrigger
  pub fn has_scaled(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn scaled_opt(self) -> ::std::option::Option<super::ScaledTriggerView<'msg>> {
    self.has_scaled().then(|| self.scaled())
  }
  pub fn scaled(self) -> super::ScaledTriggerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScaledTriggerView::default())
  }

  pub fn trigger_oneof(self) -> super::trigger::TriggerOneofOneof<'msg> {
    match self.trigger_oneof_case() {
      super::trigger::TriggerOneofCase::Threshold =>
          super::trigger::TriggerOneofOneof::Threshold(self.threshold()),
      super::trigger::TriggerOneofCase::Scaled =>
          super::trigger::TriggerOneofOneof::Scaled(self.scaled()),
      _ => super::trigger::TriggerOneofOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn trigger_oneof_case(self) -> super::trigger::TriggerOneofCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::trigger::TriggerOneofCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `TriggerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TriggerView<'_> {}

// SAFETY:
// - `TriggerView` is `Send` because while its alive a `TriggerMut` cannot.
// - `TriggerView` does not use thread-local data.
unsafe impl ::std::marker::Send for TriggerView<'_> {}

impl<'msg> ::protobuf::AsView for TriggerView<'msg> {
  type Proxied = Trigger;
  fn as_view(&self) -> ::protobuf::View<'msg, Trigger> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TriggerView<'msg> {
  fn into_view<'shorter>(self) -> TriggerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Trigger> for TriggerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Trigger {
    let mut dst = Trigger::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Trigger> for TriggerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Trigger {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Trigger {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TriggerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TriggerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TriggerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Trigger>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TriggerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TriggerMut<'msg> {
  type Message = Trigger;
}

impl ::std::fmt::Debug for TriggerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Trigger>> for TriggerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Trigger>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TriggerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Trigger> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Trigger {
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

  // threshold: optional message envoy.config.overload.v3.ThresholdTrigger
  pub fn has_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn threshold_opt(&self) -> ::std::option::Option<super::ThresholdTriggerView<'_>> {
    self.has_threshold().then(|| self.threshold())
  }
  pub fn threshold(&self) -> super::ThresholdTriggerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ThresholdTriggerView::default())
  }
  pub fn threshold_mut(&mut self) -> super::ThresholdTriggerMut<'_> {
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
  pub fn set_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<super::ThresholdTrigger>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // scaled: optional message envoy.config.overload.v3.ScaledTrigger
  pub fn has_scaled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_scaled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn scaled_opt(&self) -> ::std::option::Option<super::ScaledTriggerView<'_>> {
    self.has_scaled().then(|| self.scaled())
  }
  pub fn scaled(&self) -> super::ScaledTriggerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScaledTriggerView::default())
  }
  pub fn scaled_mut(&mut self) -> super::ScaledTriggerMut<'_> {
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
  pub fn set_scaled(&mut self,
    val: impl ::protobuf::IntoProxied<super::ScaledTrigger>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn trigger_oneof(&self) -> super::trigger::TriggerOneofOneof<'_> {
    match &self.trigger_oneof_case() {
      super::trigger::TriggerOneofCase::Threshold =>
          super::trigger::TriggerOneofOneof::Threshold(self.threshold()),
      super::trigger::TriggerOneofCase::Scaled =>
          super::trigger::TriggerOneofOneof::Scaled(self.scaled()),
      _ => super::trigger::TriggerOneofOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn trigger_oneof_case(&self) -> super::trigger::TriggerOneofCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::trigger::TriggerOneofCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `TriggerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TriggerMut<'_> {}

// SAFETY:
// - `TriggerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TriggerMut<'_> {}

impl<'msg> ::protobuf::AsView for TriggerMut<'msg> {
  type Proxied = Trigger;
  fn as_view(&self) -> ::protobuf::View<'_, Trigger> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TriggerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Trigger>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TriggerMut<'msg> {
  type MutProxied = Trigger;
  fn as_mut(&mut self) -> TriggerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TriggerMut<'msg> {
  fn into_mut<'shorter>(self) -> TriggerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Trigger {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Trigger> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TriggerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TriggerMut<'_> {
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

  // threshold: optional message envoy.config.overload.v3.ThresholdTrigger
  pub fn has_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn threshold_opt(&self) -> ::std::option::Option<super::ThresholdTriggerView<'_>> {
    self.has_threshold().then(|| self.threshold())
  }
  pub fn threshold(&self) -> super::ThresholdTriggerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ThresholdTriggerView::default())
  }
  pub fn threshold_mut(&mut self) -> super::ThresholdTriggerMut<'_> {
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
  pub fn set_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<super::ThresholdTrigger>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // scaled: optional message envoy.config.overload.v3.ScaledTrigger
  pub fn has_scaled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_scaled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn scaled_opt(&self) -> ::std::option::Option<super::ScaledTriggerView<'_>> {
    self.has_scaled().then(|| self.scaled())
  }
  pub fn scaled(&self) -> super::ScaledTriggerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ScaledTriggerView::default())
  }
  pub fn scaled_mut(&mut self) -> super::ScaledTriggerMut<'_> {
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
  pub fn set_scaled(&mut self,
    val: impl ::protobuf::IntoProxied<super::ScaledTrigger>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn trigger_oneof(&self) -> super::trigger::TriggerOneofOneof<'_> {
    match &self.trigger_oneof_case() {
      super::trigger::TriggerOneofCase::Threshold =>
          super::trigger::TriggerOneofOneof::Threshold(self.threshold()),
      super::trigger::TriggerOneofCase::Scaled =>
          super::trigger::TriggerOneofOneof::Scaled(self.scaled()),
      _ => super::trigger::TriggerOneofOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn trigger_oneof_case(&self) -> super::trigger::TriggerOneofCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::trigger::TriggerOneofCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Trigger

impl ::std::ops::Drop for Trigger {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Trigger {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Trigger {
  type Proxied = Self;
  fn as_view(&self) -> TriggerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Trigger {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TriggerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Trigger {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__overload__v3__Trigger_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X33^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__overload__v3__Trigger_msg_init.0, &[<super::ThresholdTrigger as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ScaledTrigger as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__overload__v3__Trigger_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Trigger {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Trigger {
  type Msg = Trigger;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Trigger> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Trigger {
  type Msg = Trigger;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Trigger> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TriggerMut<'_> {
  type Msg = Trigger;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Trigger> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TriggerMut<'_> {
  type Msg = Trigger;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Trigger> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TriggerView<'_> {
  type Msg = Trigger;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Trigger> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TriggerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod trigger {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TriggerOneofOneof<'msg> {
  Threshold(::protobuf::View<'msg, super::super::ThresholdTrigger>) = 2,
  Scaled(::protobuf::View<'msg, super::super::ScaledTrigger>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TriggerOneofCase {
  Threshold = 2,
  Scaled = 3,

  not_set = 0
}

impl TriggerOneofCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TriggerOneofCase> {
    match v {
      0 => Some(TriggerOneofCase::not_set),
      2 => Some(TriggerOneofCase::Threshold),
      3 => Some(TriggerOneofCase::Scaled),
      _ => None
    }
  }
}
}  // pub mod trigger


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__ScaleTimersOverloadActionConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScaleTimersOverloadActionConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScaleTimersOverloadActionConfig>
}

impl ::protobuf::Message for ScaleTimersOverloadActionConfig {
  type MessageView<'msg> = ScaleTimersOverloadActionConfigView<'msg>;
  type MessageMut<'msg> = ScaleTimersOverloadActionConfigMut<'msg>;
}

impl ::std::default::Default for ScaleTimersOverloadActionConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScaleTimersOverloadActionConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScaleTimersOverloadActionConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ScaleTimersOverloadActionConfigMut`.
unsafe impl ::std::marker::Sync for ScaleTimersOverloadActionConfig {}

// SAFETY:
// - `ScaleTimersOverloadActionConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScaleTimersOverloadActionConfig {}

impl ::protobuf::Proxied for ScaleTimersOverloadActionConfig {
  type View<'msg> = ScaleTimersOverloadActionConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScaleTimersOverloadActionConfig {}

impl ::protobuf::MutProxied for ScaleTimersOverloadActionConfig {
  type Mut<'msg> = ScaleTimersOverloadActionConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScaleTimersOverloadActionConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScaleTimersOverloadActionConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScaleTimersOverloadActionConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScaleTimersOverloadActionConfigView<'msg> {
  type Message = ScaleTimersOverloadActionConfig;
}

impl ::std::fmt::Debug for ScaleTimersOverloadActionConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScaleTimersOverloadActionConfigView<'_> {
  fn default() -> ScaleTimersOverloadActionConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScaleTimersOverloadActionConfig>> for ScaleTimersOverloadActionConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScaleTimersOverloadActionConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScaleTimersOverloadActionConfigView<'msg> {

  pub fn to_owned(&self) -> ScaleTimersOverloadActionConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // timer_scale_factors: repeated message envoy.config.overload.v3.ScaleTimersOverloadActionConfig.ScaleTimer
  pub fn timer_scale_factors(self) -> ::protobuf::RepeatedView<'msg, super::scale_timers_overload_action_config::ScaleTimer> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::scale_timers_overload_action_config::ScaleTimer>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ScaleTimersOverloadActionConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScaleTimersOverloadActionConfigView<'_> {}

// SAFETY:
// - `ScaleTimersOverloadActionConfigView` is `Send` because while its alive a `ScaleTimersOverloadActionConfigMut` cannot.
// - `ScaleTimersOverloadActionConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScaleTimersOverloadActionConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ScaleTimersOverloadActionConfigView<'msg> {
  type Proxied = ScaleTimersOverloadActionConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ScaleTimersOverloadActionConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScaleTimersOverloadActionConfigView<'msg> {
  fn into_view<'shorter>(self) -> ScaleTimersOverloadActionConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScaleTimersOverloadActionConfig> for ScaleTimersOverloadActionConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScaleTimersOverloadActionConfig {
    let mut dst = ScaleTimersOverloadActionConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScaleTimersOverloadActionConfig> for ScaleTimersOverloadActionConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScaleTimersOverloadActionConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScaleTimersOverloadActionConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScaleTimersOverloadActionConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScaleTimersOverloadActionConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScaleTimersOverloadActionConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScaleTimersOverloadActionConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScaleTimersOverloadActionConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScaleTimersOverloadActionConfigMut<'msg> {
  type Message = ScaleTimersOverloadActionConfig;
}

impl ::std::fmt::Debug for ScaleTimersOverloadActionConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScaleTimersOverloadActionConfig>> for ScaleTimersOverloadActionConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScaleTimersOverloadActionConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScaleTimersOverloadActionConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScaleTimersOverloadActionConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScaleTimersOverloadActionConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // timer_scale_factors: repeated message envoy.config.overload.v3.ScaleTimersOverloadActionConfig.ScaleTimer
  pub fn timer_scale_factors(&self) -> ::protobuf::RepeatedView<'_, super::scale_timers_overload_action_config::ScaleTimer> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::scale_timers_overload_action_config::ScaleTimer>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn timer_scale_factors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::scale_timers_overload_action_config::ScaleTimer> {
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
  pub fn set_timer_scale_factors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::scale_timers_overload_action_config::ScaleTimer>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ScaleTimersOverloadActionConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScaleTimersOverloadActionConfigMut<'_> {}

// SAFETY:
// - `ScaleTimersOverloadActionConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScaleTimersOverloadActionConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ScaleTimersOverloadActionConfigMut<'msg> {
  type Proxied = ScaleTimersOverloadActionConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ScaleTimersOverloadActionConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScaleTimersOverloadActionConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScaleTimersOverloadActionConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScaleTimersOverloadActionConfigMut<'msg> {
  type MutProxied = ScaleTimersOverloadActionConfig;
  fn as_mut(&mut self) -> ScaleTimersOverloadActionConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScaleTimersOverloadActionConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ScaleTimersOverloadActionConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScaleTimersOverloadActionConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScaleTimersOverloadActionConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScaleTimersOverloadActionConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScaleTimersOverloadActionConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // timer_scale_factors: repeated message envoy.config.overload.v3.ScaleTimersOverloadActionConfig.ScaleTimer
  pub fn timer_scale_factors(&self) -> ::protobuf::RepeatedView<'_, super::scale_timers_overload_action_config::ScaleTimer> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::scale_timers_overload_action_config::ScaleTimer>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn timer_scale_factors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::scale_timers_overload_action_config::ScaleTimer> {
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
  pub fn set_timer_scale_factors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::scale_timers_overload_action_config::ScaleTimer>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ScaleTimersOverloadActionConfig

impl ::std::ops::Drop for ScaleTimersOverloadActionConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScaleTimersOverloadActionConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScaleTimersOverloadActionConfig {
  type Proxied = Self;
  fn as_view(&self) -> ScaleTimersOverloadActionConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScaleTimersOverloadActionConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScaleTimersOverloadActionConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScaleTimersOverloadActionConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__overload__v3__ScaleTimersOverloadActionConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__overload__v3__ScaleTimersOverloadActionConfig_msg_init.0, &[<super::scale_timers_overload_action_config::ScaleTimer as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__overload__v3__ScaleTimersOverloadActionConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScaleTimersOverloadActionConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScaleTimersOverloadActionConfig {
  type Msg = ScaleTimersOverloadActionConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimersOverloadActionConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScaleTimersOverloadActionConfig {
  type Msg = ScaleTimersOverloadActionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimersOverloadActionConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScaleTimersOverloadActionConfigMut<'_> {
  type Msg = ScaleTimersOverloadActionConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimersOverloadActionConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScaleTimersOverloadActionConfigMut<'_> {
  type Msg = ScaleTimersOverloadActionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimersOverloadActionConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScaleTimersOverloadActionConfigView<'_> {
  type Msg = ScaleTimersOverloadActionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimersOverloadActionConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScaleTimersOverloadActionConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod scale_timers_overload_action_config {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__ScaleTimersOverloadActionConfig__ScaleTimer_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScaleTimer {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScaleTimer>
}

impl ::protobuf::Message for ScaleTimer {
  type MessageView<'msg> = ScaleTimerView<'msg>;
  type MessageMut<'msg> = ScaleTimerMut<'msg>;
}

impl ::std::default::Default for ScaleTimer {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScaleTimer {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScaleTimer` is `Sync` because it does not implement interior mutability.
//    Neither does `ScaleTimerMut`.
unsafe impl ::std::marker::Sync for ScaleTimer {}

// SAFETY:
// - `ScaleTimer` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScaleTimer {}

impl ::protobuf::Proxied for ScaleTimer {
  type View<'msg> = ScaleTimerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScaleTimer {}

impl ::protobuf::MutProxied for ScaleTimer {
  type Mut<'msg> = ScaleTimerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScaleTimerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScaleTimer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScaleTimerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScaleTimerView<'msg> {
  type Message = ScaleTimer;
}

impl ::std::fmt::Debug for ScaleTimerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScaleTimerView<'_> {
  fn default() -> ScaleTimerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScaleTimer>> for ScaleTimerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScaleTimer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScaleTimerView<'msg> {

  pub fn to_owned(&self) -> ScaleTimer {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // timer: optional enum envoy.config.overload.v3.ScaleTimersOverloadActionConfig.TimerType
  pub fn timer(self) -> super::super::scale_timers_overload_action_config::TimerType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::scale_timers_overload_action_config::TimerType::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // min_timeout: optional message google.protobuf.Duration
  pub fn has_min_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn min_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_min_timeout().then(|| self.min_timeout())
  }
  pub fn min_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // min_scale: optional message envoy.type.v3.Percent
  pub fn has_min_scale(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn min_scale_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_min_scale().then(|| self.min_scale())
  }
  pub fn min_scale(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  pub fn overload_adjust(self) -> super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof<'msg> {
    match self.overload_adjust_case() {
      super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase::MinTimeout =>
          super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof::MinTimeout(self.min_timeout()),
      super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase::MinScale =>
          super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof::MinScale(self.min_scale()),
      _ => super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn overload_adjust_case(self) -> super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ScaleTimerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScaleTimerView<'_> {}

// SAFETY:
// - `ScaleTimerView` is `Send` because while its alive a `ScaleTimerMut` cannot.
// - `ScaleTimerView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScaleTimerView<'_> {}

impl<'msg> ::protobuf::AsView for ScaleTimerView<'msg> {
  type Proxied = ScaleTimer;
  fn as_view(&self) -> ::protobuf::View<'msg, ScaleTimer> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScaleTimerView<'msg> {
  fn into_view<'shorter>(self) -> ScaleTimerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScaleTimer> for ScaleTimerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScaleTimer {
    let mut dst = ScaleTimer::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScaleTimer> for ScaleTimerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScaleTimer {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScaleTimer {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScaleTimerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScaleTimerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScaleTimerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScaleTimer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScaleTimerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScaleTimerMut<'msg> {
  type Message = ScaleTimer;
}

impl ::std::fmt::Debug for ScaleTimerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScaleTimer>> for ScaleTimerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScaleTimer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScaleTimerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScaleTimer> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScaleTimer {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // timer: optional enum envoy.config.overload.v3.ScaleTimersOverloadActionConfig.TimerType
  pub fn timer(&self) -> super::super::scale_timers_overload_action_config::TimerType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::scale_timers_overload_action_config::TimerType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timer(&mut self, val: super::super::scale_timers_overload_action_config::TimerType) {
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

  // min_timeout: optional message google.protobuf.Duration
  pub fn has_min_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_min_timeout().then(|| self.min_timeout())
  }
  pub fn min_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn min_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_min_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // min_scale: optional message envoy.type.v3.Percent
  pub fn has_min_scale(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_min_scale(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn min_scale_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_min_scale().then(|| self.min_scale())
  }
  pub fn min_scale(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn min_scale_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_min_scale(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn overload_adjust(&self) -> super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof<'_> {
    match &self.overload_adjust_case() {
      super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase::MinTimeout =>
          super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof::MinTimeout(self.min_timeout()),
      super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase::MinScale =>
          super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof::MinScale(self.min_scale()),
      _ => super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn overload_adjust_case(&self) -> super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ScaleTimerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScaleTimerMut<'_> {}

// SAFETY:
// - `ScaleTimerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScaleTimerMut<'_> {}

impl<'msg> ::protobuf::AsView for ScaleTimerMut<'msg> {
  type Proxied = ScaleTimer;
  fn as_view(&self) -> ::protobuf::View<'_, ScaleTimer> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScaleTimerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScaleTimer>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScaleTimerMut<'msg> {
  type MutProxied = ScaleTimer;
  fn as_mut(&mut self) -> ScaleTimerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScaleTimerMut<'msg> {
  fn into_mut<'shorter>(self) -> ScaleTimerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScaleTimer {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScaleTimer> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScaleTimerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScaleTimerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // timer: optional enum envoy.config.overload.v3.ScaleTimersOverloadActionConfig.TimerType
  pub fn timer(&self) -> super::super::scale_timers_overload_action_config::TimerType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::scale_timers_overload_action_config::TimerType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timer(&mut self, val: super::super::scale_timers_overload_action_config::TimerType) {
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

  // min_timeout: optional message google.protobuf.Duration
  pub fn has_min_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_min_timeout().then(|| self.min_timeout())
  }
  pub fn min_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn min_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_min_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // min_scale: optional message envoy.type.v3.Percent
  pub fn has_min_scale(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_min_scale(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn min_scale_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_min_scale().then(|| self.min_scale())
  }
  pub fn min_scale(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn min_scale_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_min_scale(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn overload_adjust(&self) -> super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof<'_> {
    match &self.overload_adjust_case() {
      super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase::MinTimeout =>
          super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof::MinTimeout(self.min_timeout()),
      super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase::MinScale =>
          super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof::MinScale(self.min_scale()),
      _ => super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn overload_adjust_case(&self) -> super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::scale_timers_overload_action_config::scale_timer::OverloadAdjustCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl ScaleTimer

impl ::std::ops::Drop for ScaleTimer {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScaleTimer {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScaleTimer {
  type Proxied = Self;
  fn as_view(&self) -> ScaleTimerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScaleTimer {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScaleTimerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScaleTimer {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::scale_timers_overload_action_config::envoy__config__overload__v3__ScaleTimersOverloadActionConfig__ScaleTimer_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P33^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::scale_timers_overload_action_config::envoy__config__overload__v3__ScaleTimersOverloadActionConfig__ScaleTimer_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::scale_timers_overload_action_config::envoy__config__overload__v3__ScaleTimersOverloadActionConfig__ScaleTimer_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScaleTimer {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScaleTimer {
  type Msg = ScaleTimer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScaleTimer {
  type Msg = ScaleTimer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScaleTimerMut<'_> {
  type Msg = ScaleTimer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScaleTimerMut<'_> {
  type Msg = ScaleTimer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScaleTimerView<'_> {
  type Msg = ScaleTimer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScaleTimer> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScaleTimerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod scale_timer {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum OverloadAdjustOneof<'msg> {
  MinTimeout(::protobuf::View<'msg, ::protobuf_well_known_types::Duration>) = 2,
  MinScale(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::v3::percent::Percent>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum OverloadAdjustCase {
  MinTimeout = 2,
  MinScale = 3,

  not_set = 0
}

impl OverloadAdjustCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<OverloadAdjustCase> {
    match v {
      0 => Some(OverloadAdjustCase::not_set),
      2 => Some(OverloadAdjustCase::MinTimeout),
      3 => Some(OverloadAdjustCase::MinScale),
      _ => None
    }
  }
}
}  // pub mod scale_timer

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimerType(i32);

#[allow(non_upper_case_globals)]
impl TimerType {
  pub const Unspecified: TimerType = TimerType(0);
  pub const HttpDownstreamConnectionIdle: TimerType = TimerType(1);
  pub const HttpDownstreamStreamIdle: TimerType = TimerType(2);
  pub const TransportSocketConnect: TimerType = TimerType(3);
  pub const HttpDownstreamConnectionMax: TimerType = TimerType(4);
  pub const HttpDownstreamStreamFlush: TimerType = TimerType(5);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "HttpDownstreamConnectionIdle",
      2 => "HttpDownstreamStreamIdle",
      3 => "TransportSocketConnect",
      4 => "HttpDownstreamConnectionMax",
      5 => "HttpDownstreamStreamFlush",
      _ => return None
    })
  }
}

impl ::std::convert::From<TimerType> for i32 {
  fn from(val: TimerType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for TimerType {
  fn from(val: i32) -> TimerType {
    Self(val)
  }
}

impl ::std::default::Default for TimerType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for TimerType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "TimerType::{}", constant_name)
    } else {
      write!(f, "TimerType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for TimerType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for TimerType {}

impl ::protobuf::Proxied for TimerType {
  type View<'a> = TimerType;
}

impl ::protobuf::AsView for TimerType {
  type Proxied = TimerType;

  fn as_view(&self) -> TimerType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TimerType {
  fn into_view<'shorter>(self) -> TimerType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for TimerType {
  const NAME: &'static str = "TimerType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5)
  }
}

impl ::protobuf::__internal::EntityType for TimerType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod scale_timers_overload_action_config


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__OverloadAction_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OverloadAction {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OverloadAction>
}

impl ::protobuf::Message for OverloadAction {
  type MessageView<'msg> = OverloadActionView<'msg>;
  type MessageMut<'msg> = OverloadActionMut<'msg>;
}

impl ::std::default::Default for OverloadAction {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OverloadAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OverloadAction` is `Sync` because it does not implement interior mutability.
//    Neither does `OverloadActionMut`.
unsafe impl ::std::marker::Sync for OverloadAction {}

// SAFETY:
// - `OverloadAction` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OverloadAction {}

impl ::protobuf::Proxied for OverloadAction {
  type View<'msg> = OverloadActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OverloadAction {}

impl ::protobuf::MutProxied for OverloadAction {
  type Mut<'msg> = OverloadActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OverloadActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OverloadAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OverloadActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OverloadActionView<'msg> {
  type Message = OverloadAction;
}

impl ::std::fmt::Debug for OverloadActionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OverloadActionView<'_> {
  fn default() -> OverloadActionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OverloadAction>> for OverloadActionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OverloadAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OverloadActionView<'msg> {

  pub fn to_owned(&self) -> OverloadAction {
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

  // triggers: repeated message envoy.config.overload.v3.Trigger
  pub fn triggers(self) -> ::protobuf::RepeatedView<'msg, super::Trigger> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Trigger>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
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

}

// SAFETY:
// - `OverloadActionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OverloadActionView<'_> {}

// SAFETY:
// - `OverloadActionView` is `Send` because while its alive a `OverloadActionMut` cannot.
// - `OverloadActionView` does not use thread-local data.
unsafe impl ::std::marker::Send for OverloadActionView<'_> {}

impl<'msg> ::protobuf::AsView for OverloadActionView<'msg> {
  type Proxied = OverloadAction;
  fn as_view(&self) -> ::protobuf::View<'msg, OverloadAction> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OverloadActionView<'msg> {
  fn into_view<'shorter>(self) -> OverloadActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OverloadAction> for OverloadActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OverloadAction {
    let mut dst = OverloadAction::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OverloadAction> for OverloadActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OverloadAction {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OverloadAction {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OverloadActionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OverloadActionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OverloadActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OverloadAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OverloadActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OverloadActionMut<'msg> {
  type Message = OverloadAction;
}

impl ::std::fmt::Debug for OverloadActionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OverloadAction>> for OverloadActionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OverloadAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OverloadActionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OverloadAction> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OverloadAction {
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

  // triggers: repeated message envoy.config.overload.v3.Trigger
  pub fn triggers(&self) -> ::protobuf::RepeatedView<'_, super::Trigger> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Trigger>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn triggers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Trigger> {
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
  pub fn set_triggers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Trigger>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
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

}

// SAFETY:
// - `OverloadActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OverloadActionMut<'_> {}

// SAFETY:
// - `OverloadActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OverloadActionMut<'_> {}

impl<'msg> ::protobuf::AsView for OverloadActionMut<'msg> {
  type Proxied = OverloadAction;
  fn as_view(&self) -> ::protobuf::View<'_, OverloadAction> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OverloadActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OverloadAction>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OverloadActionMut<'msg> {
  type MutProxied = OverloadAction;
  fn as_mut(&mut self) -> OverloadActionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OverloadActionMut<'msg> {
  fn into_mut<'shorter>(self) -> OverloadActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OverloadAction {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OverloadAction> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OverloadActionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OverloadActionMut<'_> {
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

  // triggers: repeated message envoy.config.overload.v3.Trigger
  pub fn triggers(&self) -> ::protobuf::RepeatedView<'_, super::Trigger> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Trigger>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn triggers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Trigger> {
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
  pub fn set_triggers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Trigger>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
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

}  // impl OverloadAction

impl ::std::ops::Drop for OverloadAction {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OverloadAction {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OverloadAction {
  type Proxied = Self;
  fn as_view(&self) -> OverloadActionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OverloadAction {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OverloadActionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OverloadAction {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__overload__v3__OverloadAction_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__overload__v3__OverloadAction_msg_init.0, &[<super::Trigger as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__overload__v3__OverloadAction_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OverloadAction {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OverloadAction {
  type Msg = OverloadAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OverloadAction {
  type Msg = OverloadAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OverloadActionMut<'_> {
  type Msg = OverloadAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OverloadActionMut<'_> {
  type Msg = OverloadAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OverloadActionView<'_> {
  type Msg = OverloadAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadAction> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OverloadActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__LoadShedPoint_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LoadShedPoint {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LoadShedPoint>
}

impl ::protobuf::Message for LoadShedPoint {
  type MessageView<'msg> = LoadShedPointView<'msg>;
  type MessageMut<'msg> = LoadShedPointMut<'msg>;
}

impl ::std::default::Default for LoadShedPoint {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LoadShedPoint {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LoadShedPoint` is `Sync` because it does not implement interior mutability.
//    Neither does `LoadShedPointMut`.
unsafe impl ::std::marker::Sync for LoadShedPoint {}

// SAFETY:
// - `LoadShedPoint` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LoadShedPoint {}

impl ::protobuf::Proxied for LoadShedPoint {
  type View<'msg> = LoadShedPointView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LoadShedPoint {}

impl ::protobuf::MutProxied for LoadShedPoint {
  type Mut<'msg> = LoadShedPointMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LoadShedPointView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LoadShedPoint>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LoadShedPointView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LoadShedPointView<'msg> {
  type Message = LoadShedPoint;
}

impl ::std::fmt::Debug for LoadShedPointView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LoadShedPointView<'_> {
  fn default() -> LoadShedPointView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LoadShedPoint>> for LoadShedPointView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LoadShedPoint>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LoadShedPointView<'msg> {

  pub fn to_owned(&self) -> LoadShedPoint {
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

  // triggers: repeated message envoy.config.overload.v3.Trigger
  pub fn triggers(self) -> ::protobuf::RepeatedView<'msg, super::Trigger> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Trigger>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `LoadShedPointView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LoadShedPointView<'_> {}

// SAFETY:
// - `LoadShedPointView` is `Send` because while its alive a `LoadShedPointMut` cannot.
// - `LoadShedPointView` does not use thread-local data.
unsafe impl ::std::marker::Send for LoadShedPointView<'_> {}

impl<'msg> ::protobuf::AsView for LoadShedPointView<'msg> {
  type Proxied = LoadShedPoint;
  fn as_view(&self) -> ::protobuf::View<'msg, LoadShedPoint> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LoadShedPointView<'msg> {
  fn into_view<'shorter>(self) -> LoadShedPointView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LoadShedPoint> for LoadShedPointView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LoadShedPoint {
    let mut dst = LoadShedPoint::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LoadShedPoint> for LoadShedPointMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LoadShedPoint {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LoadShedPoint {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LoadShedPointView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LoadShedPointMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LoadShedPointMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadShedPoint>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LoadShedPointMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LoadShedPointMut<'msg> {
  type Message = LoadShedPoint;
}

impl ::std::fmt::Debug for LoadShedPointMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LoadShedPoint>> for LoadShedPointMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadShedPoint>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LoadShedPointMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadShedPoint> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LoadShedPoint {
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

  // triggers: repeated message envoy.config.overload.v3.Trigger
  pub fn triggers(&self) -> ::protobuf::RepeatedView<'_, super::Trigger> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Trigger>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn triggers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Trigger> {
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
  pub fn set_triggers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Trigger>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `LoadShedPointMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LoadShedPointMut<'_> {}

// SAFETY:
// - `LoadShedPointMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LoadShedPointMut<'_> {}

impl<'msg> ::protobuf::AsView for LoadShedPointMut<'msg> {
  type Proxied = LoadShedPoint;
  fn as_view(&self) -> ::protobuf::View<'_, LoadShedPoint> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LoadShedPointMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LoadShedPoint>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LoadShedPointMut<'msg> {
  type MutProxied = LoadShedPoint;
  fn as_mut(&mut self) -> LoadShedPointMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LoadShedPointMut<'msg> {
  fn into_mut<'shorter>(self) -> LoadShedPointMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LoadShedPoint {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LoadShedPoint> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LoadShedPointView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LoadShedPointMut<'_> {
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

  // triggers: repeated message envoy.config.overload.v3.Trigger
  pub fn triggers(&self) -> ::protobuf::RepeatedView<'_, super::Trigger> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Trigger>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn triggers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Trigger> {
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
  pub fn set_triggers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Trigger>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl LoadShedPoint

impl ::std::ops::Drop for LoadShedPoint {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LoadShedPoint {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LoadShedPoint {
  type Proxied = Self;
  fn as_view(&self) -> LoadShedPointView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LoadShedPoint {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LoadShedPointMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LoadShedPoint {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__overload__v3__LoadShedPoint_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__overload__v3__LoadShedPoint_msg_init.0, &[<super::Trigger as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__overload__v3__LoadShedPoint_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LoadShedPoint {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LoadShedPoint {
  type Msg = LoadShedPoint;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadShedPoint> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadShedPoint {
  type Msg = LoadShedPoint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadShedPoint> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LoadShedPointMut<'_> {
  type Msg = LoadShedPoint;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadShedPoint> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadShedPointMut<'_> {
  type Msg = LoadShedPoint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadShedPoint> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadShedPointView<'_> {
  type Msg = LoadShedPoint;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadShedPoint> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LoadShedPointMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__BufferFactoryConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BufferFactoryConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BufferFactoryConfig>
}

impl ::protobuf::Message for BufferFactoryConfig {
  type MessageView<'msg> = BufferFactoryConfigView<'msg>;
  type MessageMut<'msg> = BufferFactoryConfigMut<'msg>;
}

impl ::std::default::Default for BufferFactoryConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BufferFactoryConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BufferFactoryConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `BufferFactoryConfigMut`.
unsafe impl ::std::marker::Sync for BufferFactoryConfig {}

// SAFETY:
// - `BufferFactoryConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BufferFactoryConfig {}

impl ::protobuf::Proxied for BufferFactoryConfig {
  type View<'msg> = BufferFactoryConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BufferFactoryConfig {}

impl ::protobuf::MutProxied for BufferFactoryConfig {
  type Mut<'msg> = BufferFactoryConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BufferFactoryConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BufferFactoryConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BufferFactoryConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BufferFactoryConfigView<'msg> {
  type Message = BufferFactoryConfig;
}

impl ::std::fmt::Debug for BufferFactoryConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BufferFactoryConfigView<'_> {
  fn default() -> BufferFactoryConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BufferFactoryConfig>> for BufferFactoryConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BufferFactoryConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BufferFactoryConfigView<'msg> {

  pub fn to_owned(&self) -> BufferFactoryConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // minimum_account_to_track_power_of_two: optional uint32
  pub fn minimum_account_to_track_power_of_two(self) -> u32 {
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
// - `BufferFactoryConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BufferFactoryConfigView<'_> {}

// SAFETY:
// - `BufferFactoryConfigView` is `Send` because while its alive a `BufferFactoryConfigMut` cannot.
// - `BufferFactoryConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for BufferFactoryConfigView<'_> {}

impl<'msg> ::protobuf::AsView for BufferFactoryConfigView<'msg> {
  type Proxied = BufferFactoryConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, BufferFactoryConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BufferFactoryConfigView<'msg> {
  fn into_view<'shorter>(self) -> BufferFactoryConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BufferFactoryConfig> for BufferFactoryConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BufferFactoryConfig {
    let mut dst = BufferFactoryConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BufferFactoryConfig> for BufferFactoryConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BufferFactoryConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BufferFactoryConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BufferFactoryConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BufferFactoryConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BufferFactoryConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BufferFactoryConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BufferFactoryConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BufferFactoryConfigMut<'msg> {
  type Message = BufferFactoryConfig;
}

impl ::std::fmt::Debug for BufferFactoryConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BufferFactoryConfig>> for BufferFactoryConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BufferFactoryConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BufferFactoryConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BufferFactoryConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BufferFactoryConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // minimum_account_to_track_power_of_two: optional uint32
  pub fn minimum_account_to_track_power_of_two(&self) -> u32 {
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
  pub fn set_minimum_account_to_track_power_of_two(&mut self, val: u32) {
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
// - `BufferFactoryConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BufferFactoryConfigMut<'_> {}

// SAFETY:
// - `BufferFactoryConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BufferFactoryConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for BufferFactoryConfigMut<'msg> {
  type Proxied = BufferFactoryConfig;
  fn as_view(&self) -> ::protobuf::View<'_, BufferFactoryConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BufferFactoryConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BufferFactoryConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BufferFactoryConfigMut<'msg> {
  type MutProxied = BufferFactoryConfig;
  fn as_mut(&mut self) -> BufferFactoryConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BufferFactoryConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> BufferFactoryConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BufferFactoryConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BufferFactoryConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BufferFactoryConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BufferFactoryConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // minimum_account_to_track_power_of_two: optional uint32
  pub fn minimum_account_to_track_power_of_two(&self) -> u32 {
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
  pub fn set_minimum_account_to_track_power_of_two(&mut self, val: u32) {
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

}  // impl BufferFactoryConfig

impl ::std::ops::Drop for BufferFactoryConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BufferFactoryConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BufferFactoryConfig {
  type Proxied = Self;
  fn as_view(&self) -> BufferFactoryConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BufferFactoryConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BufferFactoryConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BufferFactoryConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__overload__v3__BufferFactoryConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__overload__v3__BufferFactoryConfig_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__overload__v3__BufferFactoryConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BufferFactoryConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BufferFactoryConfig {
  type Msg = BufferFactoryConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferFactoryConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BufferFactoryConfig {
  type Msg = BufferFactoryConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferFactoryConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BufferFactoryConfigMut<'_> {
  type Msg = BufferFactoryConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferFactoryConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BufferFactoryConfigMut<'_> {
  type Msg = BufferFactoryConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferFactoryConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BufferFactoryConfigView<'_> {
  type Msg = BufferFactoryConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BufferFactoryConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BufferFactoryConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__overload__v3__OverloadManager_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OverloadManager {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OverloadManager>
}

impl ::protobuf::Message for OverloadManager {
  type MessageView<'msg> = OverloadManagerView<'msg>;
  type MessageMut<'msg> = OverloadManagerMut<'msg>;
}

impl ::std::default::Default for OverloadManager {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OverloadManager {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OverloadManager` is `Sync` because it does not implement interior mutability.
//    Neither does `OverloadManagerMut`.
unsafe impl ::std::marker::Sync for OverloadManager {}

// SAFETY:
// - `OverloadManager` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OverloadManager {}

impl ::protobuf::Proxied for OverloadManager {
  type View<'msg> = OverloadManagerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OverloadManager {}

impl ::protobuf::MutProxied for OverloadManager {
  type Mut<'msg> = OverloadManagerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OverloadManagerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OverloadManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OverloadManagerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OverloadManagerView<'msg> {
  type Message = OverloadManager;
}

impl ::std::fmt::Debug for OverloadManagerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OverloadManagerView<'_> {
  fn default() -> OverloadManagerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OverloadManager>> for OverloadManagerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OverloadManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OverloadManagerView<'msg> {

  pub fn to_owned(&self) -> OverloadManager {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // refresh_interval: optional message google.protobuf.Duration
  pub fn has_refresh_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn refresh_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_refresh_interval().then(|| self.refresh_interval())
  }
  pub fn refresh_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // resource_monitors: repeated message envoy.config.overload.v3.ResourceMonitor
  pub fn resource_monitors(self) -> ::protobuf::RepeatedView<'msg, super::ResourceMonitor> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceMonitor>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // actions: repeated message envoy.config.overload.v3.OverloadAction
  pub fn actions(self) -> ::protobuf::RepeatedView<'msg, super::OverloadAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::OverloadAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // loadshed_points: repeated message envoy.config.overload.v3.LoadShedPoint
  pub fn loadshed_points(self) -> ::protobuf::RepeatedView<'msg, super::LoadShedPoint> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::LoadShedPoint>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // buffer_factory_config: optional message envoy.config.overload.v3.BufferFactoryConfig
  pub fn has_buffer_factory_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn buffer_factory_config_opt(self) -> ::std::option::Option<super::BufferFactoryConfigView<'msg>> {
    self.has_buffer_factory_config().then(|| self.buffer_factory_config())
  }
  pub fn buffer_factory_config(self) -> super::BufferFactoryConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BufferFactoryConfigView::default())
  }

}

// SAFETY:
// - `OverloadManagerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OverloadManagerView<'_> {}

// SAFETY:
// - `OverloadManagerView` is `Send` because while its alive a `OverloadManagerMut` cannot.
// - `OverloadManagerView` does not use thread-local data.
unsafe impl ::std::marker::Send for OverloadManagerView<'_> {}

impl<'msg> ::protobuf::AsView for OverloadManagerView<'msg> {
  type Proxied = OverloadManager;
  fn as_view(&self) -> ::protobuf::View<'msg, OverloadManager> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OverloadManagerView<'msg> {
  fn into_view<'shorter>(self) -> OverloadManagerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OverloadManager> for OverloadManagerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OverloadManager {
    let mut dst = OverloadManager::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OverloadManager> for OverloadManagerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OverloadManager {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OverloadManager {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OverloadManagerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OverloadManagerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OverloadManagerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OverloadManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OverloadManagerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OverloadManagerMut<'msg> {
  type Message = OverloadManager;
}

impl ::std::fmt::Debug for OverloadManagerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OverloadManager>> for OverloadManagerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OverloadManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OverloadManagerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OverloadManager> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OverloadManager {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // refresh_interval: optional message google.protobuf.Duration
  pub fn has_refresh_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_refresh_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn refresh_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_refresh_interval().then(|| self.refresh_interval())
  }
  pub fn refresh_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn refresh_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_refresh_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // resource_monitors: repeated message envoy.config.overload.v3.ResourceMonitor
  pub fn resource_monitors(&self) -> ::protobuf::RepeatedView<'_, super::ResourceMonitor> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceMonitor>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_monitors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceMonitor> {
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
  pub fn set_resource_monitors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceMonitor>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // actions: repeated message envoy.config.overload.v3.OverloadAction
  pub fn actions(&self) -> ::protobuf::RepeatedView<'_, super::OverloadAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::OverloadAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn actions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::OverloadAction> {
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
  pub fn set_actions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::OverloadAction>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // loadshed_points: repeated message envoy.config.overload.v3.LoadShedPoint
  pub fn loadshed_points(&self) -> ::protobuf::RepeatedView<'_, super::LoadShedPoint> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::LoadShedPoint>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn loadshed_points_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::LoadShedPoint> {
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
  pub fn set_loadshed_points(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::LoadShedPoint>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // buffer_factory_config: optional message envoy.config.overload.v3.BufferFactoryConfig
  pub fn has_buffer_factory_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_buffer_factory_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn buffer_factory_config_opt(&self) -> ::std::option::Option<super::BufferFactoryConfigView<'_>> {
    self.has_buffer_factory_config().then(|| self.buffer_factory_config())
  }
  pub fn buffer_factory_config(&self) -> super::BufferFactoryConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BufferFactoryConfigView::default())
  }
  pub fn buffer_factory_config_mut(&mut self) -> super::BufferFactoryConfigMut<'_> {
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
  pub fn set_buffer_factory_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::BufferFactoryConfig>) {

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
// - `OverloadManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OverloadManagerMut<'_> {}

// SAFETY:
// - `OverloadManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OverloadManagerMut<'_> {}

impl<'msg> ::protobuf::AsView for OverloadManagerMut<'msg> {
  type Proxied = OverloadManager;
  fn as_view(&self) -> ::protobuf::View<'_, OverloadManager> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OverloadManagerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OverloadManager>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OverloadManagerMut<'msg> {
  type MutProxied = OverloadManager;
  fn as_mut(&mut self) -> OverloadManagerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OverloadManagerMut<'msg> {
  fn into_mut<'shorter>(self) -> OverloadManagerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OverloadManager {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OverloadManager> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OverloadManagerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OverloadManagerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // refresh_interval: optional message google.protobuf.Duration
  pub fn has_refresh_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_refresh_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn refresh_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_refresh_interval().then(|| self.refresh_interval())
  }
  pub fn refresh_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn refresh_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_refresh_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // resource_monitors: repeated message envoy.config.overload.v3.ResourceMonitor
  pub fn resource_monitors(&self) -> ::protobuf::RepeatedView<'_, super::ResourceMonitor> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ResourceMonitor>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resource_monitors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ResourceMonitor> {
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
  pub fn set_resource_monitors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ResourceMonitor>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // actions: repeated message envoy.config.overload.v3.OverloadAction
  pub fn actions(&self) -> ::protobuf::RepeatedView<'_, super::OverloadAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::OverloadAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn actions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::OverloadAction> {
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
  pub fn set_actions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::OverloadAction>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // loadshed_points: repeated message envoy.config.overload.v3.LoadShedPoint
  pub fn loadshed_points(&self) -> ::protobuf::RepeatedView<'_, super::LoadShedPoint> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::LoadShedPoint>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn loadshed_points_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::LoadShedPoint> {
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
  pub fn set_loadshed_points(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::LoadShedPoint>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // buffer_factory_config: optional message envoy.config.overload.v3.BufferFactoryConfig
  pub fn has_buffer_factory_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_buffer_factory_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn buffer_factory_config_opt(&self) -> ::std::option::Option<super::BufferFactoryConfigView<'_>> {
    self.has_buffer_factory_config().then(|| self.buffer_factory_config())
  }
  pub fn buffer_factory_config(&self) -> super::BufferFactoryConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BufferFactoryConfigView::default())
  }
  pub fn buffer_factory_config_mut(&mut self) -> super::BufferFactoryConfigMut<'_> {
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
  pub fn set_buffer_factory_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::BufferFactoryConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl OverloadManager

impl ::std::ops::Drop for OverloadManager {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OverloadManager {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OverloadManager {
  type Proxied = Self;
  fn as_view(&self) -> OverloadManagerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OverloadManager {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OverloadManagerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OverloadManager {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__overload__v3__OverloadManager_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3GG3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__overload__v3__OverloadManager_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ResourceMonitor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::OverloadAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BufferFactoryConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::LoadShedPoint as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__overload__v3__OverloadManager_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OverloadManager {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OverloadManager {
  type Msg = OverloadManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OverloadManager {
  type Msg = OverloadManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OverloadManagerMut<'_> {
  type Msg = OverloadManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OverloadManagerMut<'_> {
  type Msg = OverloadManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OverloadManagerView<'_> {
  type Msg = OverloadManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OverloadManager> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OverloadManagerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



