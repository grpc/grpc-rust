const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__trace__v3__DatadogRemoteConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DatadogRemoteConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DatadogRemoteConfig>
}

impl ::protobuf::Message for DatadogRemoteConfig {
  type MessageView<'msg> = DatadogRemoteConfigView<'msg>;
  type MessageMut<'msg> = DatadogRemoteConfigMut<'msg>;
}

impl ::std::default::Default for DatadogRemoteConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DatadogRemoteConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DatadogRemoteConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `DatadogRemoteConfigMut`.
unsafe impl ::std::marker::Sync for DatadogRemoteConfig {}

// SAFETY:
// - `DatadogRemoteConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DatadogRemoteConfig {}

impl ::protobuf::Proxied for DatadogRemoteConfig {
  type View<'msg> = DatadogRemoteConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DatadogRemoteConfig {}

impl ::protobuf::MutProxied for DatadogRemoteConfig {
  type Mut<'msg> = DatadogRemoteConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DatadogRemoteConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DatadogRemoteConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DatadogRemoteConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DatadogRemoteConfigView<'msg> {
  type Message = DatadogRemoteConfig;
}

impl ::std::fmt::Debug for DatadogRemoteConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DatadogRemoteConfigView<'_> {
  fn default() -> DatadogRemoteConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DatadogRemoteConfig>> for DatadogRemoteConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DatadogRemoteConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DatadogRemoteConfigView<'msg> {

  pub fn to_owned(&self) -> DatadogRemoteConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // polling_interval: optional message google.protobuf.Duration
  pub fn has_polling_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn polling_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_polling_interval().then(|| self.polling_interval())
  }
  pub fn polling_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `DatadogRemoteConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DatadogRemoteConfigView<'_> {}

// SAFETY:
// - `DatadogRemoteConfigView` is `Send` because while its alive a `DatadogRemoteConfigMut` cannot.
// - `DatadogRemoteConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for DatadogRemoteConfigView<'_> {}

impl<'msg> ::protobuf::AsView for DatadogRemoteConfigView<'msg> {
  type Proxied = DatadogRemoteConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, DatadogRemoteConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DatadogRemoteConfigView<'msg> {
  fn into_view<'shorter>(self) -> DatadogRemoteConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DatadogRemoteConfig> for DatadogRemoteConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DatadogRemoteConfig {
    let mut dst = DatadogRemoteConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DatadogRemoteConfig> for DatadogRemoteConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DatadogRemoteConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DatadogRemoteConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DatadogRemoteConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DatadogRemoteConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DatadogRemoteConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DatadogRemoteConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DatadogRemoteConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DatadogRemoteConfigMut<'msg> {
  type Message = DatadogRemoteConfig;
}

impl ::std::fmt::Debug for DatadogRemoteConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DatadogRemoteConfig>> for DatadogRemoteConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DatadogRemoteConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DatadogRemoteConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DatadogRemoteConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DatadogRemoteConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // polling_interval: optional message google.protobuf.Duration
  pub fn has_polling_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_polling_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn polling_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_polling_interval().then(|| self.polling_interval())
  }
  pub fn polling_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn polling_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_polling_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

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
// - `DatadogRemoteConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DatadogRemoteConfigMut<'_> {}

// SAFETY:
// - `DatadogRemoteConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DatadogRemoteConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for DatadogRemoteConfigMut<'msg> {
  type Proxied = DatadogRemoteConfig;
  fn as_view(&self) -> ::protobuf::View<'_, DatadogRemoteConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DatadogRemoteConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DatadogRemoteConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DatadogRemoteConfigMut<'msg> {
  type MutProxied = DatadogRemoteConfig;
  fn as_mut(&mut self) -> DatadogRemoteConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DatadogRemoteConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> DatadogRemoteConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DatadogRemoteConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DatadogRemoteConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DatadogRemoteConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DatadogRemoteConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // polling_interval: optional message google.protobuf.Duration
  pub fn has_polling_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_polling_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn polling_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_polling_interval().then(|| self.polling_interval())
  }
  pub fn polling_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn polling_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_polling_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl DatadogRemoteConfig

impl ::std::ops::Drop for DatadogRemoteConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DatadogRemoteConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DatadogRemoteConfig {
  type Proxied = Self;
  fn as_view(&self) -> DatadogRemoteConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DatadogRemoteConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DatadogRemoteConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DatadogRemoteConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__trace__v3__DatadogRemoteConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__trace__v3__DatadogRemoteConfig_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__trace__v3__DatadogRemoteConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DatadogRemoteConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DatadogRemoteConfig {
  type Msg = DatadogRemoteConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogRemoteConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DatadogRemoteConfig {
  type Msg = DatadogRemoteConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogRemoteConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DatadogRemoteConfigMut<'_> {
  type Msg = DatadogRemoteConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogRemoteConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DatadogRemoteConfigMut<'_> {
  type Msg = DatadogRemoteConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogRemoteConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DatadogRemoteConfigView<'_> {
  type Msg = DatadogRemoteConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogRemoteConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DatadogRemoteConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__trace__v3__DatadogConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DatadogConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DatadogConfig>
}

impl ::protobuf::Message for DatadogConfig {
  type MessageView<'msg> = DatadogConfigView<'msg>;
  type MessageMut<'msg> = DatadogConfigMut<'msg>;
}

impl ::std::default::Default for DatadogConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DatadogConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DatadogConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `DatadogConfigMut`.
unsafe impl ::std::marker::Sync for DatadogConfig {}

// SAFETY:
// - `DatadogConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DatadogConfig {}

impl ::protobuf::Proxied for DatadogConfig {
  type View<'msg> = DatadogConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DatadogConfig {}

impl ::protobuf::MutProxied for DatadogConfig {
  type Mut<'msg> = DatadogConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DatadogConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DatadogConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DatadogConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DatadogConfigView<'msg> {
  type Message = DatadogConfig;
}

impl ::std::fmt::Debug for DatadogConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DatadogConfigView<'_> {
  fn default() -> DatadogConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DatadogConfig>> for DatadogConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DatadogConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DatadogConfigView<'msg> {

  pub fn to_owned(&self) -> DatadogConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // collector_cluster: optional string
  pub fn collector_cluster(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // service_name: optional string
  pub fn service_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // collector_hostname: optional string
  pub fn collector_hostname(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // remote_config: optional message envoy.config.trace.v3.DatadogRemoteConfig
  pub fn has_remote_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn remote_config_opt(self) -> ::std::option::Option<super::DatadogRemoteConfigView<'msg>> {
    self.has_remote_config().then(|| self.remote_config())
  }
  pub fn remote_config(self) -> super::DatadogRemoteConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DatadogRemoteConfigView::default())
  }

}

// SAFETY:
// - `DatadogConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DatadogConfigView<'_> {}

// SAFETY:
// - `DatadogConfigView` is `Send` because while its alive a `DatadogConfigMut` cannot.
// - `DatadogConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for DatadogConfigView<'_> {}

impl<'msg> ::protobuf::AsView for DatadogConfigView<'msg> {
  type Proxied = DatadogConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, DatadogConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DatadogConfigView<'msg> {
  fn into_view<'shorter>(self) -> DatadogConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DatadogConfig> for DatadogConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DatadogConfig {
    let mut dst = DatadogConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DatadogConfig> for DatadogConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DatadogConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DatadogConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DatadogConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DatadogConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DatadogConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DatadogConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DatadogConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DatadogConfigMut<'msg> {
  type Message = DatadogConfig;
}

impl ::std::fmt::Debug for DatadogConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DatadogConfig>> for DatadogConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DatadogConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DatadogConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DatadogConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DatadogConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // collector_cluster: optional string
  pub fn collector_cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // service_name: optional string
  pub fn service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // collector_hostname: optional string
  pub fn collector_hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // remote_config: optional message envoy.config.trace.v3.DatadogRemoteConfig
  pub fn has_remote_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_remote_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn remote_config_opt(&self) -> ::std::option::Option<super::DatadogRemoteConfigView<'_>> {
    self.has_remote_config().then(|| self.remote_config())
  }
  pub fn remote_config(&self) -> super::DatadogRemoteConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DatadogRemoteConfigView::default())
  }
  pub fn remote_config_mut(&mut self) -> super::DatadogRemoteConfigMut<'_> {
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
  pub fn set_remote_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::DatadogRemoteConfig>) {

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
// - `DatadogConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DatadogConfigMut<'_> {}

// SAFETY:
// - `DatadogConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DatadogConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for DatadogConfigMut<'msg> {
  type Proxied = DatadogConfig;
  fn as_view(&self) -> ::protobuf::View<'_, DatadogConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DatadogConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DatadogConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DatadogConfigMut<'msg> {
  type MutProxied = DatadogConfig;
  fn as_mut(&mut self) -> DatadogConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DatadogConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> DatadogConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DatadogConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DatadogConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DatadogConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DatadogConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // collector_cluster: optional string
  pub fn collector_cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // service_name: optional string
  pub fn service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // collector_hostname: optional string
  pub fn collector_hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // remote_config: optional message envoy.config.trace.v3.DatadogRemoteConfig
  pub fn has_remote_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_remote_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn remote_config_opt(&self) -> ::std::option::Option<super::DatadogRemoteConfigView<'_>> {
    self.has_remote_config().then(|| self.remote_config())
  }
  pub fn remote_config(&self) -> super::DatadogRemoteConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DatadogRemoteConfigView::default())
  }
  pub fn remote_config_mut(&mut self) -> super::DatadogRemoteConfigMut<'_> {
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
  pub fn set_remote_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::DatadogRemoteConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl DatadogConfig

impl ::std::ops::Drop for DatadogConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DatadogConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DatadogConfig {
  type Proxied = Self;
  fn as_view(&self) -> DatadogConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DatadogConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DatadogConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DatadogConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__trace__v3__DatadogConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__trace__v3__DatadogConfig_msg_init.0, &[<super::DatadogRemoteConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__trace__v3__DatadogConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DatadogConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DatadogConfig {
  type Msg = DatadogConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DatadogConfig {
  type Msg = DatadogConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DatadogConfigMut<'_> {
  type Msg = DatadogConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DatadogConfigMut<'_> {
  type Msg = DatadogConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DatadogConfigView<'_> {
  type Msg = DatadogConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DatadogConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DatadogConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



