const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__clusters__aggregate__v3__ClusterConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClusterConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClusterConfig>
}

impl ::protobuf::Message for ClusterConfig {
  type MessageView<'msg> = ClusterConfigView<'msg>;
  type MessageMut<'msg> = ClusterConfigMut<'msg>;
}

impl ::std::default::Default for ClusterConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClusterConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClusterConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ClusterConfigMut`.
unsafe impl ::std::marker::Sync for ClusterConfig {}

// SAFETY:
// - `ClusterConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClusterConfig {}

impl ::protobuf::Proxied for ClusterConfig {
  type View<'msg> = ClusterConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClusterConfig {}

impl ::protobuf::MutProxied for ClusterConfig {
  type Mut<'msg> = ClusterConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClusterConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClusterConfigView<'msg> {
  type Message = ClusterConfig;
}

impl ::std::fmt::Debug for ClusterConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClusterConfigView<'_> {
  fn default() -> ClusterConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterConfig>> for ClusterConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterConfigView<'msg> {

  pub fn to_owned(&self) -> ClusterConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // clusters: repeated string
  pub fn clusters(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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
// - `ClusterConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClusterConfigView<'_> {}

// SAFETY:
// - `ClusterConfigView` is `Send` because while its alive a `ClusterConfigMut` cannot.
// - `ClusterConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClusterConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ClusterConfigView<'msg> {
  type Proxied = ClusterConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ClusterConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterConfigView<'msg> {
  fn into_view<'shorter>(self) -> ClusterConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterConfig> for ClusterConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterConfig {
    let mut dst = ClusterConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterConfig> for ClusterConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClusterConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClusterConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClusterConfigMut<'msg> {
  type Message = ClusterConfig;
}

impl ::std::fmt::Debug for ClusterConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterConfig>> for ClusterConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClusterConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // clusters: repeated string
  pub fn clusters(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ClusterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClusterConfigMut<'_> {}

// SAFETY:
// - `ClusterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClusterConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ClusterConfigMut<'msg> {
  type Proxied = ClusterConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ClusterConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClusterConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClusterConfigMut<'msg> {
  type MutProxied = ClusterConfig;
  fn as_mut(&mut self) -> ClusterConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClusterConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ClusterConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClusterConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClusterConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClusterConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClusterConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // clusters: repeated string
  pub fn clusters(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ClusterConfig

impl ::std::ops::Drop for ClusterConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClusterConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClusterConfig {
  type Proxied = Self;
  fn as_view(&self) -> ClusterConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClusterConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClusterConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClusterConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__clusters__aggregate__v3__ClusterConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ME");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__clusters__aggregate__v3__ClusterConfig_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__clusters__aggregate__v3__ClusterConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterConfig {
  type Msg = ClusterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterConfig {
  type Msg = ClusterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterConfigMut<'_> {
  type Msg = ClusterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterConfigMut<'_> {
  type Msg = ClusterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterConfigView<'_> {
  type Msg = ClusterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__clusters__aggregate__v3__AggregateClusterResource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AggregateClusterResource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AggregateClusterResource>
}

impl ::protobuf::Message for AggregateClusterResource {
  type MessageView<'msg> = AggregateClusterResourceView<'msg>;
  type MessageMut<'msg> = AggregateClusterResourceMut<'msg>;
}

impl ::std::default::Default for AggregateClusterResource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AggregateClusterResource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AggregateClusterResource` is `Sync` because it does not implement interior mutability.
//    Neither does `AggregateClusterResourceMut`.
unsafe impl ::std::marker::Sync for AggregateClusterResource {}

// SAFETY:
// - `AggregateClusterResource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AggregateClusterResource {}

impl ::protobuf::Proxied for AggregateClusterResource {
  type View<'msg> = AggregateClusterResourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AggregateClusterResource {}

impl ::protobuf::MutProxied for AggregateClusterResource {
  type Mut<'msg> = AggregateClusterResourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AggregateClusterResourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AggregateClusterResource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AggregateClusterResourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AggregateClusterResourceView<'msg> {
  type Message = AggregateClusterResource;
}

impl ::std::fmt::Debug for AggregateClusterResourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AggregateClusterResourceView<'_> {
  fn default() -> AggregateClusterResourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AggregateClusterResource>> for AggregateClusterResourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AggregateClusterResource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AggregateClusterResourceView<'msg> {

  pub fn to_owned(&self) -> AggregateClusterResource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn config_source_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // resource_name: optional string
  pub fn resource_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `AggregateClusterResourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AggregateClusterResourceView<'_> {}

// SAFETY:
// - `AggregateClusterResourceView` is `Send` because while its alive a `AggregateClusterResourceMut` cannot.
// - `AggregateClusterResourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for AggregateClusterResourceView<'_> {}

impl<'msg> ::protobuf::AsView for AggregateClusterResourceView<'msg> {
  type Proxied = AggregateClusterResource;
  fn as_view(&self) -> ::protobuf::View<'msg, AggregateClusterResource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AggregateClusterResourceView<'msg> {
  fn into_view<'shorter>(self) -> AggregateClusterResourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AggregateClusterResource> for AggregateClusterResourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AggregateClusterResource {
    let mut dst = AggregateClusterResource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AggregateClusterResource> for AggregateClusterResourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AggregateClusterResource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AggregateClusterResource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AggregateClusterResourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AggregateClusterResourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AggregateClusterResourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AggregateClusterResource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AggregateClusterResourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AggregateClusterResourceMut<'msg> {
  type Message = AggregateClusterResource;
}

impl ::std::fmt::Debug for AggregateClusterResourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AggregateClusterResource>> for AggregateClusterResourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AggregateClusterResource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AggregateClusterResourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AggregateClusterResource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AggregateClusterResource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // resource_name: optional string
  pub fn resource_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resource_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `AggregateClusterResourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AggregateClusterResourceMut<'_> {}

// SAFETY:
// - `AggregateClusterResourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AggregateClusterResourceMut<'_> {}

impl<'msg> ::protobuf::AsView for AggregateClusterResourceMut<'msg> {
  type Proxied = AggregateClusterResource;
  fn as_view(&self) -> ::protobuf::View<'_, AggregateClusterResource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AggregateClusterResourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AggregateClusterResource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AggregateClusterResourceMut<'msg> {
  type MutProxied = AggregateClusterResource;
  fn as_mut(&mut self) -> AggregateClusterResourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AggregateClusterResourceMut<'msg> {
  fn into_mut<'shorter>(self) -> AggregateClusterResourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AggregateClusterResource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AggregateClusterResource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AggregateClusterResourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AggregateClusterResourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_config_source().then(|| self.config_source())
  }
  pub fn config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // resource_name: optional string
  pub fn resource_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resource_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl AggregateClusterResource

impl ::std::ops::Drop for AggregateClusterResource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AggregateClusterResource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AggregateClusterResource {
  type Proxied = Self;
  fn as_view(&self) -> AggregateClusterResourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AggregateClusterResource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AggregateClusterResourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AggregateClusterResource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__clusters__aggregate__v3__AggregateClusterResource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__clusters__aggregate__v3__AggregateClusterResource_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__clusters__aggregate__v3__AggregateClusterResource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AggregateClusterResource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AggregateClusterResource {
  type Msg = AggregateClusterResource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregateClusterResource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AggregateClusterResource {
  type Msg = AggregateClusterResource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregateClusterResource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AggregateClusterResourceMut<'_> {
  type Msg = AggregateClusterResource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregateClusterResource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AggregateClusterResourceMut<'_> {
  type Msg = AggregateClusterResource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregateClusterResource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AggregateClusterResourceView<'_> {
  type Msg = AggregateClusterResource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AggregateClusterResource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AggregateClusterResourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



