const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__ConfigDump_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConfigDump {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConfigDump>
}

impl ::protobuf::Message for ConfigDump {
  type MessageView<'msg> = ConfigDumpView<'msg>;
  type MessageMut<'msg> = ConfigDumpMut<'msg>;
}

impl ::std::default::Default for ConfigDump {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConfigDump {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConfigDump` is `Sync` because it does not implement interior mutability.
//    Neither does `ConfigDumpMut`.
unsafe impl ::std::marker::Sync for ConfigDump {}

// SAFETY:
// - `ConfigDump` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ConfigDump {}

impl ::protobuf::Proxied for ConfigDump {
  type View<'msg> = ConfigDumpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConfigDump {}

impl ::protobuf::MutProxied for ConfigDump {
  type Mut<'msg> = ConfigDumpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConfigDumpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConfigDumpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConfigDumpView<'msg> {
  type Message = ConfigDump;
}

impl ::std::fmt::Debug for ConfigDumpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConfigDumpView<'_> {
  fn default() -> ConfigDumpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConfigDump>> for ConfigDumpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConfigDumpView<'msg> {

  pub fn to_owned(&self) -> ConfigDump {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // configs: repeated message google.protobuf.Any
  pub fn configs(self) -> ::protobuf::RepeatedView<'msg, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ConfigDumpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ConfigDumpView<'_> {}

// SAFETY:
// - `ConfigDumpView` is `Send` because while its alive a `ConfigDumpMut` cannot.
// - `ConfigDumpView` does not use thread-local data.
unsafe impl ::std::marker::Send for ConfigDumpView<'_> {}

impl<'msg> ::protobuf::AsView for ConfigDumpView<'msg> {
  type Proxied = ConfigDump;
  fn as_view(&self) -> ::protobuf::View<'msg, ConfigDump> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConfigDumpView<'msg> {
  fn into_view<'shorter>(self) -> ConfigDumpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConfigDump> for ConfigDumpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConfigDump {
    let mut dst = ConfigDump::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConfigDump> for ConfigDumpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConfigDump {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ConfigDump {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConfigDumpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConfigDumpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConfigDumpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConfigDumpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConfigDumpMut<'msg> {
  type Message = ConfigDump;
}

impl ::std::fmt::Debug for ConfigDumpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConfigDump>> for ConfigDumpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConfigDumpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConfigDump> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ConfigDump {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // configs: repeated message google.protobuf.Any
  pub fn configs(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ConfigDumpMut<'_> {}

// SAFETY:
// - `ConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ConfigDumpMut<'_> {}

impl<'msg> ::protobuf::AsView for ConfigDumpMut<'msg> {
  type Proxied = ConfigDump;
  fn as_view(&self) -> ::protobuf::View<'_, ConfigDump> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConfigDumpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConfigDump>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ConfigDumpMut<'msg> {
  type MutProxied = ConfigDump;
  fn as_mut(&mut self) -> ConfigDumpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConfigDumpMut<'msg> {
  fn into_mut<'shorter>(self) -> ConfigDumpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConfigDump {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConfigDump> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConfigDumpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConfigDumpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // configs: repeated message google.protobuf.Any
  pub fn configs(&self) -> ::protobuf::RepeatedView<'_, ::protobuf_well_known_types::Any> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf_well_known_types::Any>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf_well_known_types::Any> {
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
  pub fn set_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ConfigDump

impl ::std::ops::Drop for ConfigDump {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConfigDump {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConfigDump {
  type Proxied = Self;
  fn as_view(&self) -> ConfigDumpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConfigDump {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConfigDumpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConfigDump {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__ConfigDump_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__ConfigDump_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__ConfigDump_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConfigDump {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConfigDump {
  type Msg = ConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConfigDump {
  type Msg = ConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConfigDumpMut<'_> {
  type Msg = ConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConfigDumpMut<'_> {
  type Msg = ConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConfigDumpView<'_> {
  type Msg = ConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConfigDump> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConfigDumpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__BootstrapConfigDump_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BootstrapConfigDump {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BootstrapConfigDump>
}

impl ::protobuf::Message for BootstrapConfigDump {
  type MessageView<'msg> = BootstrapConfigDumpView<'msg>;
  type MessageMut<'msg> = BootstrapConfigDumpMut<'msg>;
}

impl ::std::default::Default for BootstrapConfigDump {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BootstrapConfigDump {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BootstrapConfigDump` is `Sync` because it does not implement interior mutability.
//    Neither does `BootstrapConfigDumpMut`.
unsafe impl ::std::marker::Sync for BootstrapConfigDump {}

// SAFETY:
// - `BootstrapConfigDump` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BootstrapConfigDump {}

impl ::protobuf::Proxied for BootstrapConfigDump {
  type View<'msg> = BootstrapConfigDumpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BootstrapConfigDump {}

impl ::protobuf::MutProxied for BootstrapConfigDump {
  type Mut<'msg> = BootstrapConfigDumpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BootstrapConfigDumpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BootstrapConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BootstrapConfigDumpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BootstrapConfigDumpView<'msg> {
  type Message = BootstrapConfigDump;
}

impl ::std::fmt::Debug for BootstrapConfigDumpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BootstrapConfigDumpView<'_> {
  fn default() -> BootstrapConfigDumpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BootstrapConfigDump>> for BootstrapConfigDumpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BootstrapConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BootstrapConfigDumpView<'msg> {

  pub fn to_owned(&self) -> BootstrapConfigDump {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bootstrap: optional message envoy.config.bootstrap.v3.Bootstrap
  pub fn has_bootstrap(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn bootstrap_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapView<'msg>> {
    self.has_bootstrap().then(|| self.bootstrap())
  }
  pub fn bootstrap(self) -> crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `BootstrapConfigDumpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BootstrapConfigDumpView<'_> {}

// SAFETY:
// - `BootstrapConfigDumpView` is `Send` because while its alive a `BootstrapConfigDumpMut` cannot.
// - `BootstrapConfigDumpView` does not use thread-local data.
unsafe impl ::std::marker::Send for BootstrapConfigDumpView<'_> {}

impl<'msg> ::protobuf::AsView for BootstrapConfigDumpView<'msg> {
  type Proxied = BootstrapConfigDump;
  fn as_view(&self) -> ::protobuf::View<'msg, BootstrapConfigDump> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BootstrapConfigDumpView<'msg> {
  fn into_view<'shorter>(self) -> BootstrapConfigDumpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BootstrapConfigDump> for BootstrapConfigDumpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BootstrapConfigDump {
    let mut dst = BootstrapConfigDump::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BootstrapConfigDump> for BootstrapConfigDumpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BootstrapConfigDump {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BootstrapConfigDump {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BootstrapConfigDumpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BootstrapConfigDumpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BootstrapConfigDumpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BootstrapConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BootstrapConfigDumpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BootstrapConfigDumpMut<'msg> {
  type Message = BootstrapConfigDump;
}

impl ::std::fmt::Debug for BootstrapConfigDumpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BootstrapConfigDump>> for BootstrapConfigDumpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BootstrapConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BootstrapConfigDumpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BootstrapConfigDump> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BootstrapConfigDump {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bootstrap: optional message envoy.config.bootstrap.v3.Bootstrap
  pub fn has_bootstrap(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bootstrap(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bootstrap_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapView<'_>> {
    self.has_bootstrap().then(|| self.bootstrap())
  }
  pub fn bootstrap(&self) -> crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapView::default())
  }
  pub fn bootstrap_mut(&mut self) -> crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapMut<'_> {
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
  pub fn set_bootstrap(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::Bootstrap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

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
// - `BootstrapConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BootstrapConfigDumpMut<'_> {}

// SAFETY:
// - `BootstrapConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BootstrapConfigDumpMut<'_> {}

impl<'msg> ::protobuf::AsView for BootstrapConfigDumpMut<'msg> {
  type Proxied = BootstrapConfigDump;
  fn as_view(&self) -> ::protobuf::View<'_, BootstrapConfigDump> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BootstrapConfigDumpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BootstrapConfigDump>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BootstrapConfigDumpMut<'msg> {
  type MutProxied = BootstrapConfigDump;
  fn as_mut(&mut self) -> BootstrapConfigDumpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BootstrapConfigDumpMut<'msg> {
  fn into_mut<'shorter>(self) -> BootstrapConfigDumpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BootstrapConfigDump {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BootstrapConfigDump> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BootstrapConfigDumpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BootstrapConfigDumpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bootstrap: optional message envoy.config.bootstrap.v3.Bootstrap
  pub fn has_bootstrap(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_bootstrap(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn bootstrap_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapView<'_>> {
    self.has_bootstrap().then(|| self.bootstrap())
  }
  pub fn bootstrap(&self) -> crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapView::default())
  }
  pub fn bootstrap_mut(&mut self) -> crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::BootstrapMut<'_> {
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
  pub fn set_bootstrap(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::Bootstrap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl BootstrapConfigDump

impl ::std::ops::Drop for BootstrapConfigDump {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BootstrapConfigDump {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BootstrapConfigDump {
  type Proxied = Self;
  fn as_view(&self) -> BootstrapConfigDumpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BootstrapConfigDump {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BootstrapConfigDumpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BootstrapConfigDump {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__BootstrapConfigDump_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__BootstrapConfigDump_msg_init.0, &[<crate::xds::generated::envoy::config::bootstrap::v3::bootstrap::Bootstrap as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__BootstrapConfigDump_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BootstrapConfigDump {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BootstrapConfigDump {
  type Msg = BootstrapConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BootstrapConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BootstrapConfigDump {
  type Msg = BootstrapConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BootstrapConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BootstrapConfigDumpMut<'_> {
  type Msg = BootstrapConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BootstrapConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BootstrapConfigDumpMut<'_> {
  type Msg = BootstrapConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BootstrapConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BootstrapConfigDumpView<'_> {
  type Msg = BootstrapConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BootstrapConfigDump> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BootstrapConfigDumpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__SecretsConfigDump_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SecretsConfigDump {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SecretsConfigDump>
}

impl ::protobuf::Message for SecretsConfigDump {
  type MessageView<'msg> = SecretsConfigDumpView<'msg>;
  type MessageMut<'msg> = SecretsConfigDumpMut<'msg>;
}

impl ::std::default::Default for SecretsConfigDump {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SecretsConfigDump {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SecretsConfigDump` is `Sync` because it does not implement interior mutability.
//    Neither does `SecretsConfigDumpMut`.
unsafe impl ::std::marker::Sync for SecretsConfigDump {}

// SAFETY:
// - `SecretsConfigDump` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SecretsConfigDump {}

impl ::protobuf::Proxied for SecretsConfigDump {
  type View<'msg> = SecretsConfigDumpView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SecretsConfigDump {}

impl ::protobuf::MutProxied for SecretsConfigDump {
  type Mut<'msg> = SecretsConfigDumpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SecretsConfigDumpView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SecretsConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SecretsConfigDumpView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SecretsConfigDumpView<'msg> {
  type Message = SecretsConfigDump;
}

impl ::std::fmt::Debug for SecretsConfigDumpView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SecretsConfigDumpView<'_> {
  fn default() -> SecretsConfigDumpView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SecretsConfigDump>> for SecretsConfigDumpView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SecretsConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SecretsConfigDumpView<'msg> {

  pub fn to_owned(&self) -> SecretsConfigDump {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // static_secrets: repeated message envoy.admin.v3.SecretsConfigDump.StaticSecret
  pub fn static_secrets(self) -> ::protobuf::RepeatedView<'msg, super::secrets_config_dump::StaticSecret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::secrets_config_dump::StaticSecret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dynamic_active_secrets: repeated message envoy.admin.v3.SecretsConfigDump.DynamicSecret
  pub fn dynamic_active_secrets(self) -> ::protobuf::RepeatedView<'msg, super::secrets_config_dump::DynamicSecret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::secrets_config_dump::DynamicSecret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dynamic_warming_secrets: repeated message envoy.admin.v3.SecretsConfigDump.DynamicSecret
  pub fn dynamic_warming_secrets(self) -> ::protobuf::RepeatedView<'msg, super::secrets_config_dump::DynamicSecret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::secrets_config_dump::DynamicSecret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `SecretsConfigDumpView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SecretsConfigDumpView<'_> {}

// SAFETY:
// - `SecretsConfigDumpView` is `Send` because while its alive a `SecretsConfigDumpMut` cannot.
// - `SecretsConfigDumpView` does not use thread-local data.
unsafe impl ::std::marker::Send for SecretsConfigDumpView<'_> {}

impl<'msg> ::protobuf::AsView for SecretsConfigDumpView<'msg> {
  type Proxied = SecretsConfigDump;
  fn as_view(&self) -> ::protobuf::View<'msg, SecretsConfigDump> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SecretsConfigDumpView<'msg> {
  fn into_view<'shorter>(self) -> SecretsConfigDumpView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SecretsConfigDump> for SecretsConfigDumpView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SecretsConfigDump {
    let mut dst = SecretsConfigDump::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SecretsConfigDump> for SecretsConfigDumpMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SecretsConfigDump {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SecretsConfigDump {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SecretsConfigDumpView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SecretsConfigDumpMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SecretsConfigDumpMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SecretsConfigDump>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SecretsConfigDumpMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SecretsConfigDumpMut<'msg> {
  type Message = SecretsConfigDump;
}

impl ::std::fmt::Debug for SecretsConfigDumpMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SecretsConfigDump>> for SecretsConfigDumpMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SecretsConfigDump>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SecretsConfigDumpMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SecretsConfigDump> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SecretsConfigDump {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // static_secrets: repeated message envoy.admin.v3.SecretsConfigDump.StaticSecret
  pub fn static_secrets(&self) -> ::protobuf::RepeatedView<'_, super::secrets_config_dump::StaticSecret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::secrets_config_dump::StaticSecret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_secrets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::secrets_config_dump::StaticSecret> {
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
  pub fn set_static_secrets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::secrets_config_dump::StaticSecret>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dynamic_active_secrets: repeated message envoy.admin.v3.SecretsConfigDump.DynamicSecret
  pub fn dynamic_active_secrets(&self) -> ::protobuf::RepeatedView<'_, super::secrets_config_dump::DynamicSecret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::secrets_config_dump::DynamicSecret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_active_secrets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::secrets_config_dump::DynamicSecret> {
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
  pub fn set_dynamic_active_secrets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::secrets_config_dump::DynamicSecret>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // dynamic_warming_secrets: repeated message envoy.admin.v3.SecretsConfigDump.DynamicSecret
  pub fn dynamic_warming_secrets(&self) -> ::protobuf::RepeatedView<'_, super::secrets_config_dump::DynamicSecret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::secrets_config_dump::DynamicSecret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_warming_secrets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::secrets_config_dump::DynamicSecret> {
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
  pub fn set_dynamic_warming_secrets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::secrets_config_dump::DynamicSecret>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `SecretsConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SecretsConfigDumpMut<'_> {}

// SAFETY:
// - `SecretsConfigDumpMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SecretsConfigDumpMut<'_> {}

impl<'msg> ::protobuf::AsView for SecretsConfigDumpMut<'msg> {
  type Proxied = SecretsConfigDump;
  fn as_view(&self) -> ::protobuf::View<'_, SecretsConfigDump> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SecretsConfigDumpMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SecretsConfigDump>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SecretsConfigDumpMut<'msg> {
  type MutProxied = SecretsConfigDump;
  fn as_mut(&mut self) -> SecretsConfigDumpMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SecretsConfigDumpMut<'msg> {
  fn into_mut<'shorter>(self) -> SecretsConfigDumpMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SecretsConfigDump {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SecretsConfigDump> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SecretsConfigDumpView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SecretsConfigDumpMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // static_secrets: repeated message envoy.admin.v3.SecretsConfigDump.StaticSecret
  pub fn static_secrets(&self) -> ::protobuf::RepeatedView<'_, super::secrets_config_dump::StaticSecret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::secrets_config_dump::StaticSecret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn static_secrets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::secrets_config_dump::StaticSecret> {
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
  pub fn set_static_secrets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::secrets_config_dump::StaticSecret>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dynamic_active_secrets: repeated message envoy.admin.v3.SecretsConfigDump.DynamicSecret
  pub fn dynamic_active_secrets(&self) -> ::protobuf::RepeatedView<'_, super::secrets_config_dump::DynamicSecret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::secrets_config_dump::DynamicSecret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_active_secrets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::secrets_config_dump::DynamicSecret> {
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
  pub fn set_dynamic_active_secrets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::secrets_config_dump::DynamicSecret>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // dynamic_warming_secrets: repeated message envoy.admin.v3.SecretsConfigDump.DynamicSecret
  pub fn dynamic_warming_secrets(&self) -> ::protobuf::RepeatedView<'_, super::secrets_config_dump::DynamicSecret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::secrets_config_dump::DynamicSecret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dynamic_warming_secrets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::secrets_config_dump::DynamicSecret> {
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
  pub fn set_dynamic_warming_secrets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::secrets_config_dump::DynamicSecret>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl SecretsConfigDump

impl ::std::ops::Drop for SecretsConfigDump {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SecretsConfigDump {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SecretsConfigDump {
  type Proxied = Self;
  fn as_view(&self) -> SecretsConfigDumpView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SecretsConfigDump {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SecretsConfigDumpMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SecretsConfigDump {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__admin__v3__SecretsConfigDump_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GGG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__admin__v3__SecretsConfigDump_msg_init.0, &[<super::secrets_config_dump::StaticSecret as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::secrets_config_dump::DynamicSecret as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::secrets_config_dump::DynamicSecret as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__admin__v3__SecretsConfigDump_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SecretsConfigDump {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SecretsConfigDump {
  type Msg = SecretsConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SecretsConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SecretsConfigDump {
  type Msg = SecretsConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SecretsConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SecretsConfigDumpMut<'_> {
  type Msg = SecretsConfigDump;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SecretsConfigDump> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SecretsConfigDumpMut<'_> {
  type Msg = SecretsConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SecretsConfigDump> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SecretsConfigDumpView<'_> {
  type Msg = SecretsConfigDump;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SecretsConfigDump> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SecretsConfigDumpMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod secrets_config_dump {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__SecretsConfigDump__DynamicSecret_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicSecret {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicSecret>
}

impl ::protobuf::Message for DynamicSecret {
  type MessageView<'msg> = DynamicSecretView<'msg>;
  type MessageMut<'msg> = DynamicSecretMut<'msg>;
}

impl ::std::default::Default for DynamicSecret {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicSecret {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicSecret` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicSecretMut`.
unsafe impl ::std::marker::Sync for DynamicSecret {}

// SAFETY:
// - `DynamicSecret` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicSecret {}

impl ::protobuf::Proxied for DynamicSecret {
  type View<'msg> = DynamicSecretView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicSecret {}

impl ::protobuf::MutProxied for DynamicSecret {
  type Mut<'msg> = DynamicSecretMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicSecretView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicSecret>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicSecretView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicSecretView<'msg> {
  type Message = DynamicSecret;
}

impl ::std::fmt::Debug for DynamicSecretView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicSecretView<'_> {
  fn default() -> DynamicSecretView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicSecret>> for DynamicSecretView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicSecret>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicSecretView<'msg> {

  pub fn to_owned(&self) -> DynamicSecret {
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

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // secret: optional message google.protobuf.Any
  pub fn has_secret(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn secret_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_secret().then(|| self.secret())
  }
  pub fn secret(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn error_state_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'msg>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView::default())
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DynamicSecretView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicSecretView<'_> {}

// SAFETY:
// - `DynamicSecretView` is `Send` because while its alive a `DynamicSecretMut` cannot.
// - `DynamicSecretView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicSecretView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicSecretView<'msg> {
  type Proxied = DynamicSecret;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicSecret> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicSecretView<'msg> {
  fn into_view<'shorter>(self) -> DynamicSecretView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicSecret> for DynamicSecretView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicSecret {
    let mut dst = DynamicSecret::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicSecret> for DynamicSecretMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicSecret {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicSecret {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicSecretView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicSecretMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicSecretMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicSecret>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicSecretMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicSecretMut<'msg> {
  type Message = DynamicSecret;
}

impl ::std::fmt::Debug for DynamicSecretMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicSecret>> for DynamicSecretMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicSecret>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicSecretMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicSecret> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicSecret {
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

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // secret: optional message google.protobuf.Any
  pub fn has_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn secret_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_secret().then(|| self.secret())
  }
  pub fn secret(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn secret_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_secret(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `DynamicSecretMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicSecretMut<'_> {}

// SAFETY:
// - `DynamicSecretMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicSecretMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicSecretMut<'msg> {
  type Proxied = DynamicSecret;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicSecret> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicSecretMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicSecret>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicSecretMut<'msg> {
  type MutProxied = DynamicSecret;
  fn as_mut(&mut self) -> DynamicSecretMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicSecretMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicSecretMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicSecret {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicSecret> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicSecretView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicSecretMut<'_> {
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

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // secret: optional message google.protobuf.Any
  pub fn has_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn secret_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_secret().then(|| self.secret())
  }
  pub fn secret(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn secret_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_secret(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

}  // impl DynamicSecret

impl ::std::ops::Drop for DynamicSecret {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicSecret {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicSecret {
  type Proxied = Self;
  fn as_view(&self) -> DynamicSecretView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicSecret {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicSecretMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicSecret {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::secrets_config_dump::envoy__admin__v3__SecretsConfigDump__DynamicSecret_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X333.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::secrets_config_dump::envoy__admin__v3__SecretsConfigDump__DynamicSecret_msg_init.0, &[<::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::secrets_config_dump::envoy__admin__v3__SecretsConfigDump__DynamicSecret_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicSecret {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicSecret {
  type Msg = DynamicSecret;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicSecret> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicSecret {
  type Msg = DynamicSecret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicSecret> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicSecretMut<'_> {
  type Msg = DynamicSecret;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicSecret> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicSecretMut<'_> {
  type Msg = DynamicSecret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicSecret> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicSecretView<'_> {
  type Msg = DynamicSecret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicSecret> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicSecretMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__admin__v3__SecretsConfigDump__StaticSecret_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StaticSecret {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StaticSecret>
}

impl ::protobuf::Message for StaticSecret {
  type MessageView<'msg> = StaticSecretView<'msg>;
  type MessageMut<'msg> = StaticSecretMut<'msg>;
}

impl ::std::default::Default for StaticSecret {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StaticSecret {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StaticSecret` is `Sync` because it does not implement interior mutability.
//    Neither does `StaticSecretMut`.
unsafe impl ::std::marker::Sync for StaticSecret {}

// SAFETY:
// - `StaticSecret` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StaticSecret {}

impl ::protobuf::Proxied for StaticSecret {
  type View<'msg> = StaticSecretView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StaticSecret {}

impl ::protobuf::MutProxied for StaticSecret {
  type Mut<'msg> = StaticSecretMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StaticSecretView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticSecret>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticSecretView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StaticSecretView<'msg> {
  type Message = StaticSecret;
}

impl ::std::fmt::Debug for StaticSecretView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StaticSecretView<'_> {
  fn default() -> StaticSecretView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StaticSecret>> for StaticSecretView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticSecret>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticSecretView<'msg> {

  pub fn to_owned(&self) -> StaticSecret {
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

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // secret: optional message google.protobuf.Any
  pub fn has_secret(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn secret_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_secret().then(|| self.secret())
  }
  pub fn secret(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

}

// SAFETY:
// - `StaticSecretView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StaticSecretView<'_> {}

// SAFETY:
// - `StaticSecretView` is `Send` because while its alive a `StaticSecretMut` cannot.
// - `StaticSecretView` does not use thread-local data.
unsafe impl ::std::marker::Send for StaticSecretView<'_> {}

impl<'msg> ::protobuf::AsView for StaticSecretView<'msg> {
  type Proxied = StaticSecret;
  fn as_view(&self) -> ::protobuf::View<'msg, StaticSecret> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticSecretView<'msg> {
  fn into_view<'shorter>(self) -> StaticSecretView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticSecret> for StaticSecretView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticSecret {
    let mut dst = StaticSecret::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticSecret> for StaticSecretMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticSecret {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StaticSecret {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticSecretView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticSecretMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StaticSecretMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticSecret>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticSecretMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StaticSecretMut<'msg> {
  type Message = StaticSecret;
}

impl ::std::fmt::Debug for StaticSecretMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StaticSecret>> for StaticSecretMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticSecret>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticSecretMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticSecret> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StaticSecret {
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

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // secret: optional message google.protobuf.Any
  pub fn has_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn secret_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_secret().then(|| self.secret())
  }
  pub fn secret(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn secret_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_secret(&mut self,
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
// - `StaticSecretMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StaticSecretMut<'_> {}

// SAFETY:
// - `StaticSecretMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StaticSecretMut<'_> {}

impl<'msg> ::protobuf::AsView for StaticSecretMut<'msg> {
  type Proxied = StaticSecret;
  fn as_view(&self) -> ::protobuf::View<'_, StaticSecret> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticSecretMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StaticSecret>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StaticSecretMut<'msg> {
  type MutProxied = StaticSecret;
  fn as_mut(&mut self) -> StaticSecretMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StaticSecretMut<'msg> {
  fn into_mut<'shorter>(self) -> StaticSecretMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StaticSecret {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StaticSecret> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StaticSecretView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StaticSecretMut<'_> {
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

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // secret: optional message google.protobuf.Any
  pub fn has_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn secret_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_secret().then(|| self.secret())
  }
  pub fn secret(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn secret_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_secret(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl StaticSecret

impl ::std::ops::Drop for StaticSecret {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StaticSecret {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StaticSecret {
  type Proxied = Self;
  fn as_view(&self) -> StaticSecretView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StaticSecret {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StaticSecretMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StaticSecret {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::secrets_config_dump::envoy__admin__v3__SecretsConfigDump__StaticSecret_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::secrets_config_dump::envoy__admin__v3__SecretsConfigDump__StaticSecret_msg_init.0, &[<::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::secrets_config_dump::envoy__admin__v3__SecretsConfigDump__StaticSecret_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticSecret {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticSecret {
  type Msg = StaticSecret;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticSecret> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticSecret {
  type Msg = StaticSecret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticSecret> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticSecretMut<'_> {
  type Msg = StaticSecret;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticSecret> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticSecretMut<'_> {
  type Msg = StaticSecret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticSecret> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticSecretView<'_> {
  type Msg = StaticSecret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticSecret> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticSecretMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod secrets_config_dump


