const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__trace__v3__LightstepConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LightstepConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LightstepConfig>
}

impl ::protobuf::Message for LightstepConfig {
  type MessageView<'msg> = LightstepConfigView<'msg>;
  type MessageMut<'msg> = LightstepConfigMut<'msg>;
}

impl ::std::default::Default for LightstepConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LightstepConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LightstepConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `LightstepConfigMut`.
unsafe impl ::std::marker::Sync for LightstepConfig {}

// SAFETY:
// - `LightstepConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LightstepConfig {}

impl ::protobuf::Proxied for LightstepConfig {
  type View<'msg> = LightstepConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LightstepConfig {}

impl ::protobuf::MutProxied for LightstepConfig {
  type Mut<'msg> = LightstepConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LightstepConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LightstepConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LightstepConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LightstepConfigView<'msg> {
  type Message = LightstepConfig;
}

impl ::std::fmt::Debug for LightstepConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LightstepConfigView<'_> {
  fn default() -> LightstepConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LightstepConfig>> for LightstepConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LightstepConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LightstepConfigView<'msg> {

  pub fn to_owned(&self) -> LightstepConfig {
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

  // access_token_file: optional string
  pub fn access_token_file(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // access_token: optional message envoy.config.core.v3.DataSource
  pub fn has_access_token(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn access_token_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_access_token().then(|| self.access_token())
  }
  pub fn access_token(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // propagation_modes: repeated enum envoy.config.trace.v3.LightstepConfig.PropagationMode
  pub fn propagation_modes(self) -> ::protobuf::RepeatedView<'msg, super::lightstep_config::PropagationMode> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::lightstep_config::PropagationMode>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `LightstepConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LightstepConfigView<'_> {}

// SAFETY:
// - `LightstepConfigView` is `Send` because while its alive a `LightstepConfigMut` cannot.
// - `LightstepConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for LightstepConfigView<'_> {}

impl<'msg> ::protobuf::AsView for LightstepConfigView<'msg> {
  type Proxied = LightstepConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, LightstepConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LightstepConfigView<'msg> {
  fn into_view<'shorter>(self) -> LightstepConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LightstepConfig> for LightstepConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LightstepConfig {
    let mut dst = LightstepConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LightstepConfig> for LightstepConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LightstepConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LightstepConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LightstepConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LightstepConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LightstepConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LightstepConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LightstepConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LightstepConfigMut<'msg> {
  type Message = LightstepConfig;
}

impl ::std::fmt::Debug for LightstepConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LightstepConfig>> for LightstepConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LightstepConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LightstepConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LightstepConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LightstepConfig {
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

  // access_token_file: optional string
  pub fn access_token_file(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_access_token_file(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // access_token: optional message envoy.config.core.v3.DataSource
  pub fn has_access_token(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_access_token(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn access_token_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_access_token().then(|| self.access_token())
  }
  pub fn access_token(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn access_token_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_access_token(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // propagation_modes: repeated enum envoy.config.trace.v3.LightstepConfig.PropagationMode
  pub fn propagation_modes(&self) -> ::protobuf::RepeatedView<'_, super::lightstep_config::PropagationMode> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::lightstep_config::PropagationMode>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn propagation_modes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::lightstep_config::PropagationMode> {
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
  pub fn set_propagation_modes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::lightstep_config::PropagationMode>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `LightstepConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LightstepConfigMut<'_> {}

// SAFETY:
// - `LightstepConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LightstepConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for LightstepConfigMut<'msg> {
  type Proxied = LightstepConfig;
  fn as_view(&self) -> ::protobuf::View<'_, LightstepConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LightstepConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LightstepConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LightstepConfigMut<'msg> {
  type MutProxied = LightstepConfig;
  fn as_mut(&mut self) -> LightstepConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LightstepConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> LightstepConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LightstepConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LightstepConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LightstepConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LightstepConfigMut<'_> {
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

  // access_token_file: optional string
  pub fn access_token_file(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_access_token_file(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // access_token: optional message envoy.config.core.v3.DataSource
  pub fn has_access_token(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_access_token(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn access_token_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_access_token().then(|| self.access_token())
  }
  pub fn access_token(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn access_token_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_access_token(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // propagation_modes: repeated enum envoy.config.trace.v3.LightstepConfig.PropagationMode
  pub fn propagation_modes(&self) -> ::protobuf::RepeatedView<'_, super::lightstep_config::PropagationMode> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::lightstep_config::PropagationMode>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn propagation_modes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::lightstep_config::PropagationMode> {
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
  pub fn set_propagation_modes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::lightstep_config::PropagationMode>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl LightstepConfig

impl ::std::ops::Drop for LightstepConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LightstepConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LightstepConfig {
  type Proxied = Self;
  fn as_view(&self) -> LightstepConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LightstepConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LightstepConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LightstepConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__trace__v3__LightstepConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N1X1XB3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__trace__v3__LightstepConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__trace__v3__LightstepConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LightstepConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LightstepConfig {
  type Msg = LightstepConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LightstepConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LightstepConfig {
  type Msg = LightstepConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LightstepConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LightstepConfigMut<'_> {
  type Msg = LightstepConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LightstepConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LightstepConfigMut<'_> {
  type Msg = LightstepConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LightstepConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LightstepConfigView<'_> {
  type Msg = LightstepConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LightstepConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LightstepConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod lightstep_config {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PropagationMode(i32);

#[allow(non_upper_case_globals)]
impl PropagationMode {
  pub const Envoy: PropagationMode = PropagationMode(0);
  pub const Lightstep: PropagationMode = PropagationMode(1);
  pub const B3: PropagationMode = PropagationMode(2);
  pub const TraceContext: PropagationMode = PropagationMode(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Envoy",
      1 => "Lightstep",
      2 => "B3",
      3 => "TraceContext",
      _ => return None
    })
  }
}

impl ::std::convert::From<PropagationMode> for i32 {
  fn from(val: PropagationMode) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for PropagationMode {
  fn from(val: i32) -> PropagationMode {
    Self(val)
  }
}

impl ::std::default::Default for PropagationMode {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for PropagationMode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "PropagationMode::{}", constant_name)
    } else {
      write!(f, "PropagationMode::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for PropagationMode {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for PropagationMode {}

impl ::protobuf::Proxied for PropagationMode {
  type View<'a> = PropagationMode;
}

impl ::protobuf::AsView for PropagationMode {
  type Proxied = PropagationMode;

  fn as_view(&self) -> PropagationMode {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PropagationMode {
  fn into_view<'shorter>(self) -> PropagationMode where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for PropagationMode {
  const NAME: &'static str = "PropagationMode";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for PropagationMode {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod lightstep_config


