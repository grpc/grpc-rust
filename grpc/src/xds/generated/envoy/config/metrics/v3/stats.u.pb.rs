const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__metrics__v3__StatsSink_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StatsSink {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StatsSink>
}

impl ::protobuf::Message for StatsSink {
  type MessageView<'msg> = StatsSinkView<'msg>;
  type MessageMut<'msg> = StatsSinkMut<'msg>;
}

impl ::std::default::Default for StatsSink {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StatsSink {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StatsSink` is `Sync` because it does not implement interior mutability.
//    Neither does `StatsSinkMut`.
unsafe impl ::std::marker::Sync for StatsSink {}

// SAFETY:
// - `StatsSink` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StatsSink {}

impl ::protobuf::Proxied for StatsSink {
  type View<'msg> = StatsSinkView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StatsSink {}

impl ::protobuf::MutProxied for StatsSink {
  type Mut<'msg> = StatsSinkMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StatsSinkView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatsSink>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatsSinkView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StatsSinkView<'msg> {
  type Message = StatsSink;
}

impl ::std::fmt::Debug for StatsSinkView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StatsSinkView<'_> {
  fn default() -> StatsSinkView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StatsSink>> for StatsSinkView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatsSink>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatsSinkView<'msg> {

  pub fn to_owned(&self) -> StatsSink {
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

  pub fn config_type(self) -> super::stats_sink::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::stats_sink::ConfigTypeCase::TypedConfig =>
          super::stats_sink::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::stats_sink::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::stats_sink::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::stats_sink::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StatsSinkView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StatsSinkView<'_> {}

// SAFETY:
// - `StatsSinkView` is `Send` because while its alive a `StatsSinkMut` cannot.
// - `StatsSinkView` does not use thread-local data.
unsafe impl ::std::marker::Send for StatsSinkView<'_> {}

impl<'msg> ::protobuf::AsView for StatsSinkView<'msg> {
  type Proxied = StatsSink;
  fn as_view(&self) -> ::protobuf::View<'msg, StatsSink> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatsSinkView<'msg> {
  fn into_view<'shorter>(self) -> StatsSinkView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StatsSink> for StatsSinkView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatsSink {
    let mut dst = StatsSink::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StatsSink> for StatsSinkMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatsSink {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StatsSink {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatsSinkView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatsSinkMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StatsSinkMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsSink>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatsSinkMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StatsSinkMut<'msg> {
  type Message = StatsSink;
}

impl ::std::fmt::Debug for StatsSinkMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StatsSink>> for StatsSinkMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsSink>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatsSinkMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsSink> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StatsSink {
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

  pub fn config_type(&self) -> super::stats_sink::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::stats_sink::ConfigTypeCase::TypedConfig =>
          super::stats_sink::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::stats_sink::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::stats_sink::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::stats_sink::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StatsSinkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StatsSinkMut<'_> {}

// SAFETY:
// - `StatsSinkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StatsSinkMut<'_> {}

impl<'msg> ::protobuf::AsView for StatsSinkMut<'msg> {
  type Proxied = StatsSink;
  fn as_view(&self) -> ::protobuf::View<'_, StatsSink> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatsSinkMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StatsSink>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StatsSinkMut<'msg> {
  type MutProxied = StatsSink;
  fn as_mut(&mut self) -> StatsSinkMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StatsSinkMut<'msg> {
  fn into_mut<'shorter>(self) -> StatsSinkMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StatsSink {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StatsSink> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StatsSinkView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StatsSinkMut<'_> {
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

  pub fn config_type(&self) -> super::stats_sink::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::stats_sink::ConfigTypeCase::TypedConfig =>
          super::stats_sink::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::stats_sink::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::stats_sink::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::stats_sink::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl StatsSink

impl ::std::ops::Drop for StatsSink {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StatsSink {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StatsSink {
  type Proxied = Self;
  fn as_view(&self) -> StatsSinkView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StatsSink {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StatsSinkMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StatsSink {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__metrics__v3__StatsSink_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa3^$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__metrics__v3__StatsSink_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__metrics__v3__StatsSink_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatsSink {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatsSink {
  type Msg = StatsSink;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsSink> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsSink {
  type Msg = StatsSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsSink> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatsSinkMut<'_> {
  type Msg = StatsSink;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsSink> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsSinkMut<'_> {
  type Msg = StatsSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsSink> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsSinkView<'_> {
  type Msg = StatsSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsSink> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatsSinkMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod stats_sink {

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
}  // pub mod stats_sink


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__metrics__v3__StatsConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StatsConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StatsConfig>
}

impl ::protobuf::Message for StatsConfig {
  type MessageView<'msg> = StatsConfigView<'msg>;
  type MessageMut<'msg> = StatsConfigMut<'msg>;
}

impl ::std::default::Default for StatsConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StatsConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StatsConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `StatsConfigMut`.
unsafe impl ::std::marker::Sync for StatsConfig {}

// SAFETY:
// - `StatsConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StatsConfig {}

impl ::protobuf::Proxied for StatsConfig {
  type View<'msg> = StatsConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StatsConfig {}

impl ::protobuf::MutProxied for StatsConfig {
  type Mut<'msg> = StatsConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StatsConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatsConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StatsConfigView<'msg> {
  type Message = StatsConfig;
}

impl ::std::fmt::Debug for StatsConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StatsConfigView<'_> {
  fn default() -> StatsConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StatsConfig>> for StatsConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatsConfigView<'msg> {

  pub fn to_owned(&self) -> StatsConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // stats_tags: repeated message envoy.config.metrics.v3.TagSpecifier
  pub fn stats_tags(self) -> ::protobuf::RepeatedView<'msg, super::TagSpecifier> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TagSpecifier>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // use_all_default_tags: optional message google.protobuf.BoolValue
  pub fn has_use_all_default_tags(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn use_all_default_tags_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_use_all_default_tags().then(|| self.use_all_default_tags())
  }
  pub fn use_all_default_tags(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // stats_matcher: optional message envoy.config.metrics.v3.StatsMatcher
  pub fn has_stats_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn stats_matcher_opt(self) -> ::std::option::Option<super::StatsMatcherView<'msg>> {
    self.has_stats_matcher().then(|| self.stats_matcher())
  }
  pub fn stats_matcher(self) -> super::StatsMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StatsMatcherView::default())
  }

  // histogram_bucket_settings: repeated message envoy.config.metrics.v3.HistogramBucketSettings
  pub fn histogram_bucket_settings(self) -> ::protobuf::RepeatedView<'msg, super::HistogramBucketSettings> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HistogramBucketSettings>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `StatsConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StatsConfigView<'_> {}

// SAFETY:
// - `StatsConfigView` is `Send` because while its alive a `StatsConfigMut` cannot.
// - `StatsConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for StatsConfigView<'_> {}

impl<'msg> ::protobuf::AsView for StatsConfigView<'msg> {
  type Proxied = StatsConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, StatsConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatsConfigView<'msg> {
  fn into_view<'shorter>(self) -> StatsConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StatsConfig> for StatsConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatsConfig {
    let mut dst = StatsConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StatsConfig> for StatsConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatsConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StatsConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatsConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatsConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StatsConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatsConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StatsConfigMut<'msg> {
  type Message = StatsConfig;
}

impl ::std::fmt::Debug for StatsConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StatsConfig>> for StatsConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatsConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StatsConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // stats_tags: repeated message envoy.config.metrics.v3.TagSpecifier
  pub fn stats_tags(&self) -> ::protobuf::RepeatedView<'_, super::TagSpecifier> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TagSpecifier>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn stats_tags_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TagSpecifier> {
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
  pub fn set_stats_tags(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TagSpecifier>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // use_all_default_tags: optional message google.protobuf.BoolValue
  pub fn has_use_all_default_tags(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_use_all_default_tags(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn use_all_default_tags_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_all_default_tags().then(|| self.use_all_default_tags())
  }
  pub fn use_all_default_tags(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_all_default_tags_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_all_default_tags(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // stats_matcher: optional message envoy.config.metrics.v3.StatsMatcher
  pub fn has_stats_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_stats_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn stats_matcher_opt(&self) -> ::std::option::Option<super::StatsMatcherView<'_>> {
    self.has_stats_matcher().then(|| self.stats_matcher())
  }
  pub fn stats_matcher(&self) -> super::StatsMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StatsMatcherView::default())
  }
  pub fn stats_matcher_mut(&mut self) -> super::StatsMatcherMut<'_> {
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
  pub fn set_stats_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<super::StatsMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // histogram_bucket_settings: repeated message envoy.config.metrics.v3.HistogramBucketSettings
  pub fn histogram_bucket_settings(&self) -> ::protobuf::RepeatedView<'_, super::HistogramBucketSettings> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HistogramBucketSettings>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn histogram_bucket_settings_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HistogramBucketSettings> {
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
  pub fn set_histogram_bucket_settings(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HistogramBucketSettings>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `StatsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StatsConfigMut<'_> {}

// SAFETY:
// - `StatsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StatsConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for StatsConfigMut<'msg> {
  type Proxied = StatsConfig;
  fn as_view(&self) -> ::protobuf::View<'_, StatsConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatsConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StatsConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StatsConfigMut<'msg> {
  type MutProxied = StatsConfig;
  fn as_mut(&mut self) -> StatsConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StatsConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> StatsConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StatsConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StatsConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StatsConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StatsConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // stats_tags: repeated message envoy.config.metrics.v3.TagSpecifier
  pub fn stats_tags(&self) -> ::protobuf::RepeatedView<'_, super::TagSpecifier> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TagSpecifier>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn stats_tags_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TagSpecifier> {
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
  pub fn set_stats_tags(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TagSpecifier>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // use_all_default_tags: optional message google.protobuf.BoolValue
  pub fn has_use_all_default_tags(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_use_all_default_tags(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn use_all_default_tags_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_all_default_tags().then(|| self.use_all_default_tags())
  }
  pub fn use_all_default_tags(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_all_default_tags_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_all_default_tags(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // stats_matcher: optional message envoy.config.metrics.v3.StatsMatcher
  pub fn has_stats_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_stats_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn stats_matcher_opt(&self) -> ::std::option::Option<super::StatsMatcherView<'_>> {
    self.has_stats_matcher().then(|| self.stats_matcher())
  }
  pub fn stats_matcher(&self) -> super::StatsMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StatsMatcherView::default())
  }
  pub fn stats_matcher_mut(&mut self) -> super::StatsMatcherMut<'_> {
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
  pub fn set_stats_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<super::StatsMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // histogram_bucket_settings: repeated message envoy.config.metrics.v3.HistogramBucketSettings
  pub fn histogram_bucket_settings(&self) -> ::protobuf::RepeatedView<'_, super::HistogramBucketSettings> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HistogramBucketSettings>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn histogram_bucket_settings_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HistogramBucketSettings> {
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
  pub fn set_histogram_bucket_settings(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HistogramBucketSettings>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl StatsConfig

impl ::std::ops::Drop for StatsConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StatsConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StatsConfig {
  type Proxied = Self;
  fn as_view(&self) -> StatsConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StatsConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StatsConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StatsConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__metrics__v3__StatsConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G33G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__metrics__v3__StatsConfig_msg_init.0, &[<super::TagSpecifier as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::StatsMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HistogramBucketSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__metrics__v3__StatsConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatsConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatsConfig {
  type Msg = StatsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsConfig {
  type Msg = StatsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatsConfigMut<'_> {
  type Msg = StatsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsConfigMut<'_> {
  type Msg = StatsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsConfigView<'_> {
  type Msg = StatsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatsConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__metrics__v3__StatsMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StatsMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StatsMatcher>
}

impl ::protobuf::Message for StatsMatcher {
  type MessageView<'msg> = StatsMatcherView<'msg>;
  type MessageMut<'msg> = StatsMatcherMut<'msg>;
}

impl ::std::default::Default for StatsMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StatsMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StatsMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `StatsMatcherMut`.
unsafe impl ::std::marker::Sync for StatsMatcher {}

// SAFETY:
// - `StatsMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StatsMatcher {}

impl ::protobuf::Proxied for StatsMatcher {
  type View<'msg> = StatsMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StatsMatcher {}

impl ::protobuf::MutProxied for StatsMatcher {
  type Mut<'msg> = StatsMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StatsMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatsMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatsMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StatsMatcherView<'msg> {
  type Message = StatsMatcher;
}

impl ::std::fmt::Debug for StatsMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StatsMatcherView<'_> {
  fn default() -> StatsMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StatsMatcher>> for StatsMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatsMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatsMatcherView<'msg> {

  pub fn to_owned(&self) -> StatsMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // reject_all: optional bool
  pub fn has_reject_all(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn reject_all_opt(self) -> ::std::option::Option<bool> {
    self.has_reject_all().then(|| self.reject_all())
  }
  pub fn reject_all(self) -> bool {
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

  // exclusion_list: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_exclusion_list(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn exclusion_list_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_exclusion_list().then(|| self.exclusion_list())
  }
  pub fn exclusion_list(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

  // inclusion_list: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_inclusion_list(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn inclusion_list_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg>> {
    self.has_inclusion_list().then(|| self.inclusion_list())
  }
  pub fn inclusion_list(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }

  pub fn stats_matcher(self) -> super::stats_matcher::StatsMatcherOneof<'msg> {
    match self.stats_matcher_case() {
      super::stats_matcher::StatsMatcherCase::RejectAll =>
          super::stats_matcher::StatsMatcherOneof::RejectAll(self.reject_all()),
      super::stats_matcher::StatsMatcherCase::ExclusionList =>
          super::stats_matcher::StatsMatcherOneof::ExclusionList(self.exclusion_list()),
      super::stats_matcher::StatsMatcherCase::InclusionList =>
          super::stats_matcher::StatsMatcherOneof::InclusionList(self.inclusion_list()),
      _ => super::stats_matcher::StatsMatcherOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn stats_matcher_case(self) -> super::stats_matcher::StatsMatcherCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::stats_matcher::StatsMatcherCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StatsMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StatsMatcherView<'_> {}

// SAFETY:
// - `StatsMatcherView` is `Send` because while its alive a `StatsMatcherMut` cannot.
// - `StatsMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for StatsMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for StatsMatcherView<'msg> {
  type Proxied = StatsMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, StatsMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatsMatcherView<'msg> {
  fn into_view<'shorter>(self) -> StatsMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StatsMatcher> for StatsMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatsMatcher {
    let mut dst = StatsMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StatsMatcher> for StatsMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatsMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StatsMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatsMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatsMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StatsMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatsMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StatsMatcherMut<'msg> {
  type Message = StatsMatcher;
}

impl ::std::fmt::Debug for StatsMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StatsMatcher>> for StatsMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatsMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StatsMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // reject_all: optional bool
  pub fn has_reject_all(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_reject_all(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn reject_all_opt(&self) -> ::std::option::Option<bool> {
    self.has_reject_all().then(|| self.reject_all())
  }
  pub fn reject_all(&self) -> bool {
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
  pub fn set_reject_all(&mut self, val: bool) {
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

  // exclusion_list: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_exclusion_list(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_exclusion_list(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn exclusion_list_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_exclusion_list().then(|| self.exclusion_list())
  }
  pub fn exclusion_list(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn exclusion_list_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_exclusion_list(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // inclusion_list: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_inclusion_list(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_inclusion_list(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn inclusion_list_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_inclusion_list().then(|| self.inclusion_list())
  }
  pub fn inclusion_list(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn inclusion_list_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_inclusion_list(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn stats_matcher(&self) -> super::stats_matcher::StatsMatcherOneof<'_> {
    match &self.stats_matcher_case() {
      super::stats_matcher::StatsMatcherCase::RejectAll =>
          super::stats_matcher::StatsMatcherOneof::RejectAll(self.reject_all()),
      super::stats_matcher::StatsMatcherCase::ExclusionList =>
          super::stats_matcher::StatsMatcherOneof::ExclusionList(self.exclusion_list()),
      super::stats_matcher::StatsMatcherCase::InclusionList =>
          super::stats_matcher::StatsMatcherOneof::InclusionList(self.inclusion_list()),
      _ => super::stats_matcher::StatsMatcherOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn stats_matcher_case(&self) -> super::stats_matcher::StatsMatcherCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::stats_matcher::StatsMatcherCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StatsMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StatsMatcherMut<'_> {}

// SAFETY:
// - `StatsMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StatsMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for StatsMatcherMut<'msg> {
  type Proxied = StatsMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, StatsMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatsMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StatsMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StatsMatcherMut<'msg> {
  type MutProxied = StatsMatcher;
  fn as_mut(&mut self) -> StatsMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StatsMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> StatsMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StatsMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StatsMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StatsMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StatsMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // reject_all: optional bool
  pub fn has_reject_all(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_reject_all(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn reject_all_opt(&self) -> ::std::option::Option<bool> {
    self.has_reject_all().then(|| self.reject_all())
  }
  pub fn reject_all(&self) -> bool {
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
  pub fn set_reject_all(&mut self, val: bool) {
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

  // exclusion_list: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_exclusion_list(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_exclusion_list(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn exclusion_list_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_exclusion_list().then(|| self.exclusion_list())
  }
  pub fn exclusion_list(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn exclusion_list_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_exclusion_list(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // inclusion_list: optional message envoy.type.matcher.v3.ListStringMatcher
  pub fn has_inclusion_list(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_inclusion_list(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn inclusion_list_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_>> {
    self.has_inclusion_list().then(|| self.inclusion_list())
  }
  pub fn inclusion_list(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherView::default())
  }
  pub fn inclusion_list_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcherMut<'_> {
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
  pub fn set_inclusion_list(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn stats_matcher(&self) -> super::stats_matcher::StatsMatcherOneof<'_> {
    match &self.stats_matcher_case() {
      super::stats_matcher::StatsMatcherCase::RejectAll =>
          super::stats_matcher::StatsMatcherOneof::RejectAll(self.reject_all()),
      super::stats_matcher::StatsMatcherCase::ExclusionList =>
          super::stats_matcher::StatsMatcherOneof::ExclusionList(self.exclusion_list()),
      super::stats_matcher::StatsMatcherCase::InclusionList =>
          super::stats_matcher::StatsMatcherOneof::InclusionList(self.inclusion_list()),
      _ => super::stats_matcher::StatsMatcherOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn stats_matcher_case(&self) -> super::stats_matcher::StatsMatcherCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::stats_matcher::StatsMatcherCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl StatsMatcher

impl ::std::ops::Drop for StatsMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StatsMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StatsMatcher {
  type Proxied = Self;
  fn as_view(&self) -> StatsMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StatsMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StatsMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StatsMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__metrics__v3__StatsMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/33^!|#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__metrics__v3__StatsMatcher_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__metrics__v3__StatsMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatsMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatsMatcher {
  type Msg = StatsMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsMatcher {
  type Msg = StatsMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatsMatcherMut<'_> {
  type Msg = StatsMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsMatcherMut<'_> {
  type Msg = StatsMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsMatcherView<'_> {
  type Msg = StatsMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatsMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod stats_matcher {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum StatsMatcherOneof<'msg> {
  RejectAll(bool) = 1,
  ExclusionList(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) = 2,
  InclusionList(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::ListStringMatcher>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum StatsMatcherCase {
  RejectAll = 1,
  ExclusionList = 2,
  InclusionList = 3,

  not_set = 0
}

impl StatsMatcherCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<StatsMatcherCase> {
    match v {
      0 => Some(StatsMatcherCase::not_set),
      1 => Some(StatsMatcherCase::RejectAll),
      2 => Some(StatsMatcherCase::ExclusionList),
      3 => Some(StatsMatcherCase::InclusionList),
      _ => None
    }
  }
}
}  // pub mod stats_matcher


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__metrics__v3__TagSpecifier_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TagSpecifier {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TagSpecifier>
}

impl ::protobuf::Message for TagSpecifier {
  type MessageView<'msg> = TagSpecifierView<'msg>;
  type MessageMut<'msg> = TagSpecifierMut<'msg>;
}

impl ::std::default::Default for TagSpecifier {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TagSpecifier {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TagSpecifier` is `Sync` because it does not implement interior mutability.
//    Neither does `TagSpecifierMut`.
unsafe impl ::std::marker::Sync for TagSpecifier {}

// SAFETY:
// - `TagSpecifier` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TagSpecifier {}

impl ::protobuf::Proxied for TagSpecifier {
  type View<'msg> = TagSpecifierView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TagSpecifier {}

impl ::protobuf::MutProxied for TagSpecifier {
  type Mut<'msg> = TagSpecifierMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TagSpecifierView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TagSpecifier>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TagSpecifierView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TagSpecifierView<'msg> {
  type Message = TagSpecifier;
}

impl ::std::fmt::Debug for TagSpecifierView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TagSpecifierView<'_> {
  fn default() -> TagSpecifierView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TagSpecifier>> for TagSpecifierView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TagSpecifier>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TagSpecifierView<'msg> {

  pub fn to_owned(&self) -> TagSpecifier {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // tag_name: optional string
  pub fn tag_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // regex: optional string
  pub fn has_regex(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn regex_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_regex().then(|| self.regex())
  }
  pub fn regex(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // fixed_value: optional string
  pub fn has_fixed_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn fixed_value_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_fixed_value().then(|| self.fixed_value())
  }
  pub fn fixed_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn tag_value(self) -> super::tag_specifier::TagValueOneof<'msg> {
    match self.tag_value_case() {
      super::tag_specifier::TagValueCase::Regex =>
          super::tag_specifier::TagValueOneof::Regex(self.regex()),
      super::tag_specifier::TagValueCase::FixedValue =>
          super::tag_specifier::TagValueOneof::FixedValue(self.fixed_value()),
      _ => super::tag_specifier::TagValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn tag_value_case(self) -> super::tag_specifier::TagValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::tag_specifier::TagValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `TagSpecifierView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TagSpecifierView<'_> {}

// SAFETY:
// - `TagSpecifierView` is `Send` because while its alive a `TagSpecifierMut` cannot.
// - `TagSpecifierView` does not use thread-local data.
unsafe impl ::std::marker::Send for TagSpecifierView<'_> {}

impl<'msg> ::protobuf::AsView for TagSpecifierView<'msg> {
  type Proxied = TagSpecifier;
  fn as_view(&self) -> ::protobuf::View<'msg, TagSpecifier> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TagSpecifierView<'msg> {
  fn into_view<'shorter>(self) -> TagSpecifierView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TagSpecifier> for TagSpecifierView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TagSpecifier {
    let mut dst = TagSpecifier::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TagSpecifier> for TagSpecifierMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TagSpecifier {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TagSpecifier {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TagSpecifierView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TagSpecifierMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TagSpecifierMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TagSpecifier>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TagSpecifierMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TagSpecifierMut<'msg> {
  type Message = TagSpecifier;
}

impl ::std::fmt::Debug for TagSpecifierMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TagSpecifier>> for TagSpecifierMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TagSpecifier>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TagSpecifierMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TagSpecifier> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TagSpecifier {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // tag_name: optional string
  pub fn tag_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tag_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // regex: optional string
  pub fn has_regex(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_regex(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn regex_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_regex().then(|| self.regex())
  }
  pub fn regex(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_regex(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // fixed_value: optional string
  pub fn has_fixed_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_fixed_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn fixed_value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_fixed_value().then(|| self.fixed_value())
  }
  pub fn fixed_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_fixed_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  pub fn tag_value(&self) -> super::tag_specifier::TagValueOneof<'_> {
    match &self.tag_value_case() {
      super::tag_specifier::TagValueCase::Regex =>
          super::tag_specifier::TagValueOneof::Regex(self.regex()),
      super::tag_specifier::TagValueCase::FixedValue =>
          super::tag_specifier::TagValueOneof::FixedValue(self.fixed_value()),
      _ => super::tag_specifier::TagValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn tag_value_case(&self) -> super::tag_specifier::TagValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::tag_specifier::TagValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `TagSpecifierMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TagSpecifierMut<'_> {}

// SAFETY:
// - `TagSpecifierMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TagSpecifierMut<'_> {}

impl<'msg> ::protobuf::AsView for TagSpecifierMut<'msg> {
  type Proxied = TagSpecifier;
  fn as_view(&self) -> ::protobuf::View<'_, TagSpecifier> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TagSpecifierMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TagSpecifier>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TagSpecifierMut<'msg> {
  type MutProxied = TagSpecifier;
  fn as_mut(&mut self) -> TagSpecifierMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TagSpecifierMut<'msg> {
  fn into_mut<'shorter>(self) -> TagSpecifierMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TagSpecifier {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TagSpecifier> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TagSpecifierView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TagSpecifierMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // tag_name: optional string
  pub fn tag_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tag_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // regex: optional string
  pub fn has_regex(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_regex(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn regex_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_regex().then(|| self.regex())
  }
  pub fn regex(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_regex(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // fixed_value: optional string
  pub fn has_fixed_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_fixed_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn fixed_value_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_fixed_value().then(|| self.fixed_value())
  }
  pub fn fixed_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_fixed_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  pub fn tag_value(&self) -> super::tag_specifier::TagValueOneof<'_> {
    match &self.tag_value_case() {
      super::tag_specifier::TagValueCase::Regex =>
          super::tag_specifier::TagValueOneof::Regex(self.regex()),
      super::tag_specifier::TagValueCase::FixedValue =>
          super::tag_specifier::TagValueOneof::FixedValue(self.fixed_value()),
      _ => super::tag_specifier::TagValueOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn tag_value_case(&self) -> super::tag_specifier::TagValueCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::tag_specifier::TagValueCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl TagSpecifier

impl ::std::ops::Drop for TagSpecifier {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TagSpecifier {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TagSpecifier {
  type Proxied = Self;
  fn as_view(&self) -> TagSpecifierView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TagSpecifier {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TagSpecifierMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TagSpecifier {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__metrics__v3__TagSpecifier_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P11^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__metrics__v3__TagSpecifier_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__metrics__v3__TagSpecifier_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TagSpecifier {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TagSpecifier {
  type Msg = TagSpecifier;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TagSpecifier> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TagSpecifier {
  type Msg = TagSpecifier;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TagSpecifier> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TagSpecifierMut<'_> {
  type Msg = TagSpecifier;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TagSpecifier> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TagSpecifierMut<'_> {
  type Msg = TagSpecifier;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TagSpecifier> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TagSpecifierView<'_> {
  type Msg = TagSpecifier;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TagSpecifier> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TagSpecifierMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod tag_specifier {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TagValueOneof<'msg> {
  Regex(&'msg ::protobuf::ProtoStr) = 2,
  FixedValue(&'msg ::protobuf::ProtoStr) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TagValueCase {
  Regex = 2,
  FixedValue = 3,

  not_set = 0
}

impl TagValueCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TagValueCase> {
    match v {
      0 => Some(TagValueCase::not_set),
      2 => Some(TagValueCase::Regex),
      3 => Some(TagValueCase::FixedValue),
      _ => None
    }
  }
}
}  // pub mod tag_specifier


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__metrics__v3__HistogramBucketSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HistogramBucketSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HistogramBucketSettings>
}

impl ::protobuf::Message for HistogramBucketSettings {
  type MessageView<'msg> = HistogramBucketSettingsView<'msg>;
  type MessageMut<'msg> = HistogramBucketSettingsMut<'msg>;
}

impl ::std::default::Default for HistogramBucketSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HistogramBucketSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HistogramBucketSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `HistogramBucketSettingsMut`.
unsafe impl ::std::marker::Sync for HistogramBucketSettings {}

// SAFETY:
// - `HistogramBucketSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HistogramBucketSettings {}

impl ::protobuf::Proxied for HistogramBucketSettings {
  type View<'msg> = HistogramBucketSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HistogramBucketSettings {}

impl ::protobuf::MutProxied for HistogramBucketSettings {
  type Mut<'msg> = HistogramBucketSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HistogramBucketSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HistogramBucketSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HistogramBucketSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HistogramBucketSettingsView<'msg> {
  type Message = HistogramBucketSettings;
}

impl ::std::fmt::Debug for HistogramBucketSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HistogramBucketSettingsView<'_> {
  fn default() -> HistogramBucketSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HistogramBucketSettings>> for HistogramBucketSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HistogramBucketSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HistogramBucketSettingsView<'msg> {

  pub fn to_owned(&self) -> HistogramBucketSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn match_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_match().then(|| self.r#match())
  }
  pub fn r#match(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

  // buckets: repeated double
  pub fn buckets(self) -> ::protobuf::RepeatedView<'msg, f64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // bins: optional message google.protobuf.UInt32Value
  pub fn has_bins(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn bins_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_bins().then(|| self.bins())
  }
  pub fn bins(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `HistogramBucketSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HistogramBucketSettingsView<'_> {}

// SAFETY:
// - `HistogramBucketSettingsView` is `Send` because while its alive a `HistogramBucketSettingsMut` cannot.
// - `HistogramBucketSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for HistogramBucketSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for HistogramBucketSettingsView<'msg> {
  type Proxied = HistogramBucketSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, HistogramBucketSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HistogramBucketSettingsView<'msg> {
  fn into_view<'shorter>(self) -> HistogramBucketSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HistogramBucketSettings> for HistogramBucketSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HistogramBucketSettings {
    let mut dst = HistogramBucketSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HistogramBucketSettings> for HistogramBucketSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HistogramBucketSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HistogramBucketSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HistogramBucketSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HistogramBucketSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HistogramBucketSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HistogramBucketSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HistogramBucketSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HistogramBucketSettingsMut<'msg> {
  type Message = HistogramBucketSettings;
}

impl ::std::fmt::Debug for HistogramBucketSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HistogramBucketSettings>> for HistogramBucketSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HistogramBucketSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HistogramBucketSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HistogramBucketSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HistogramBucketSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_match().then(|| self.r#match())
  }
  pub fn r#match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // buckets: repeated double
  pub fn buckets(&self) -> ::protobuf::RepeatedView<'_, f64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn buckets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f64> {
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
  pub fn set_buckets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // bins: optional message google.protobuf.UInt32Value
  pub fn has_bins(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_bins(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn bins_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_bins().then(|| self.bins())
  }
  pub fn bins(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn bins_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_bins(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

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
// - `HistogramBucketSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HistogramBucketSettingsMut<'_> {}

// SAFETY:
// - `HistogramBucketSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HistogramBucketSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for HistogramBucketSettingsMut<'msg> {
  type Proxied = HistogramBucketSettings;
  fn as_view(&self) -> ::protobuf::View<'_, HistogramBucketSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HistogramBucketSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HistogramBucketSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HistogramBucketSettingsMut<'msg> {
  type MutProxied = HistogramBucketSettings;
  fn as_mut(&mut self) -> HistogramBucketSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HistogramBucketSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> HistogramBucketSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HistogramBucketSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HistogramBucketSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HistogramBucketSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HistogramBucketSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_match().then(|| self.r#match())
  }
  pub fn r#match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // buckets: repeated double
  pub fn buckets(&self) -> ::protobuf::RepeatedView<'_, f64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<f64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn buckets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, f64> {
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
  pub fn set_buckets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<f64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // bins: optional message google.protobuf.UInt32Value
  pub fn has_bins(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_bins(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn bins_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_bins().then(|| self.bins())
  }
  pub fn bins(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn bins_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_bins(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl HistogramBucketSettings

impl ::std::ops::Drop for HistogramBucketSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HistogramBucketSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HistogramBucketSettings {
  type Proxied = Self;
  fn as_view(&self) -> HistogramBucketSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HistogramBucketSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HistogramBucketSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HistogramBucketSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__metrics__v3__HistogramBucketSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N363");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__metrics__v3__HistogramBucketSettings_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__metrics__v3__HistogramBucketSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HistogramBucketSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HistogramBucketSettings {
  type Msg = HistogramBucketSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramBucketSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HistogramBucketSettings {
  type Msg = HistogramBucketSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramBucketSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HistogramBucketSettingsMut<'_> {
  type Msg = HistogramBucketSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramBucketSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HistogramBucketSettingsMut<'_> {
  type Msg = HistogramBucketSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramBucketSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HistogramBucketSettingsView<'_> {
  type Msg = HistogramBucketSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HistogramBucketSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HistogramBucketSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__metrics__v3__StatsdSink_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StatsdSink {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StatsdSink>
}

impl ::protobuf::Message for StatsdSink {
  type MessageView<'msg> = StatsdSinkView<'msg>;
  type MessageMut<'msg> = StatsdSinkMut<'msg>;
}

impl ::std::default::Default for StatsdSink {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StatsdSink {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StatsdSink` is `Sync` because it does not implement interior mutability.
//    Neither does `StatsdSinkMut`.
unsafe impl ::std::marker::Sync for StatsdSink {}

// SAFETY:
// - `StatsdSink` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StatsdSink {}

impl ::protobuf::Proxied for StatsdSink {
  type View<'msg> = StatsdSinkView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StatsdSink {}

impl ::protobuf::MutProxied for StatsdSink {
  type Mut<'msg> = StatsdSinkMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StatsdSinkView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatsdSink>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatsdSinkView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StatsdSinkView<'msg> {
  type Message = StatsdSink;
}

impl ::std::fmt::Debug for StatsdSinkView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StatsdSinkView<'_> {
  fn default() -> StatsdSinkView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StatsdSink>> for StatsdSinkView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StatsdSink>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatsdSinkView<'msg> {

  pub fn to_owned(&self) -> StatsdSink {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // tcp_cluster_name: optional string
  pub fn has_tcp_cluster_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn tcp_cluster_name_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_tcp_cluster_name().then(|| self.tcp_cluster_name())
  }
  pub fn tcp_cluster_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // prefix: optional string
  pub fn prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn statsd_specifier(self) -> super::statsd_sink::StatsdSpecifierOneof<'msg> {
    match self.statsd_specifier_case() {
      super::statsd_sink::StatsdSpecifierCase::Address =>
          super::statsd_sink::StatsdSpecifierOneof::Address(self.address()),
      super::statsd_sink::StatsdSpecifierCase::TcpClusterName =>
          super::statsd_sink::StatsdSpecifierOneof::TcpClusterName(self.tcp_cluster_name()),
      _ => super::statsd_sink::StatsdSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn statsd_specifier_case(self) -> super::statsd_sink::StatsdSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::statsd_sink::StatsdSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StatsdSinkView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StatsdSinkView<'_> {}

// SAFETY:
// - `StatsdSinkView` is `Send` because while its alive a `StatsdSinkMut` cannot.
// - `StatsdSinkView` does not use thread-local data.
unsafe impl ::std::marker::Send for StatsdSinkView<'_> {}

impl<'msg> ::protobuf::AsView for StatsdSinkView<'msg> {
  type Proxied = StatsdSink;
  fn as_view(&self) -> ::protobuf::View<'msg, StatsdSink> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatsdSinkView<'msg> {
  fn into_view<'shorter>(self) -> StatsdSinkView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StatsdSink> for StatsdSinkView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatsdSink {
    let mut dst = StatsdSink::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StatsdSink> for StatsdSinkMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StatsdSink {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StatsdSink {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatsdSinkView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StatsdSinkMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StatsdSinkMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsdSink>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StatsdSinkMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StatsdSinkMut<'msg> {
  type Message = StatsdSink;
}

impl ::std::fmt::Debug for StatsdSinkMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StatsdSink>> for StatsdSinkMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsdSink>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StatsdSinkMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StatsdSink> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StatsdSink {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // tcp_cluster_name: optional string
  pub fn has_tcp_cluster_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_tcp_cluster_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn tcp_cluster_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_tcp_cluster_name().then(|| self.tcp_cluster_name())
  }
  pub fn tcp_cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tcp_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // prefix: optional string
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  pub fn statsd_specifier(&self) -> super::statsd_sink::StatsdSpecifierOneof<'_> {
    match &self.statsd_specifier_case() {
      super::statsd_sink::StatsdSpecifierCase::Address =>
          super::statsd_sink::StatsdSpecifierOneof::Address(self.address()),
      super::statsd_sink::StatsdSpecifierCase::TcpClusterName =>
          super::statsd_sink::StatsdSpecifierOneof::TcpClusterName(self.tcp_cluster_name()),
      _ => super::statsd_sink::StatsdSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn statsd_specifier_case(&self) -> super::statsd_sink::StatsdSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::statsd_sink::StatsdSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `StatsdSinkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StatsdSinkMut<'_> {}

// SAFETY:
// - `StatsdSinkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StatsdSinkMut<'_> {}

impl<'msg> ::protobuf::AsView for StatsdSinkMut<'msg> {
  type Proxied = StatsdSink;
  fn as_view(&self) -> ::protobuf::View<'_, StatsdSink> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StatsdSinkMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StatsdSink>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StatsdSinkMut<'msg> {
  type MutProxied = StatsdSink;
  fn as_mut(&mut self) -> StatsdSinkMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StatsdSinkMut<'msg> {
  fn into_mut<'shorter>(self) -> StatsdSinkMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StatsdSink {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StatsdSink> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StatsdSinkView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StatsdSinkMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // tcp_cluster_name: optional string
  pub fn has_tcp_cluster_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_tcp_cluster_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn tcp_cluster_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_tcp_cluster_name().then(|| self.tcp_cluster_name())
  }
  pub fn tcp_cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_tcp_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // prefix: optional string
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  pub fn statsd_specifier(&self) -> super::statsd_sink::StatsdSpecifierOneof<'_> {
    match &self.statsd_specifier_case() {
      super::statsd_sink::StatsdSpecifierCase::Address =>
          super::statsd_sink::StatsdSpecifierOneof::Address(self.address()),
      super::statsd_sink::StatsdSpecifierCase::TcpClusterName =>
          super::statsd_sink::StatsdSpecifierOneof::TcpClusterName(self.tcp_cluster_name()),
      _ => super::statsd_sink::StatsdSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn statsd_specifier_case(&self) -> super::statsd_sink::StatsdSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::statsd_sink::StatsdSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl StatsdSink

impl ::std::ops::Drop for StatsdSink {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StatsdSink {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StatsdSink {
  type Proxied = Self;
  fn as_view(&self) -> StatsdSinkView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StatsdSink {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StatsdSinkMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StatsdSink {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__metrics__v3__StatsdSink_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31T1X^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__metrics__v3__StatsdSink_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__metrics__v3__StatsdSink_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatsdSink {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatsdSink {
  type Msg = StatsdSink;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsdSink> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsdSink {
  type Msg = StatsdSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsdSink> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StatsdSinkMut<'_> {
  type Msg = StatsdSink;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsdSink> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsdSinkMut<'_> {
  type Msg = StatsdSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsdSink> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StatsdSinkView<'_> {
  type Msg = StatsdSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StatsdSink> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StatsdSinkMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod statsd_sink {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum StatsdSpecifierOneof<'msg> {
  Address(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::address::Address>) = 1,
  TcpClusterName(&'msg ::protobuf::ProtoStr) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum StatsdSpecifierCase {
  Address = 1,
  TcpClusterName = 2,

  not_set = 0
}

impl StatsdSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<StatsdSpecifierCase> {
    match v {
      0 => Some(StatsdSpecifierCase::not_set),
      1 => Some(StatsdSpecifierCase::Address),
      2 => Some(StatsdSpecifierCase::TcpClusterName),
      _ => None
    }
  }
}
}  // pub mod statsd_sink


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__metrics__v3__DogStatsdSink_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DogStatsdSink {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DogStatsdSink>
}

impl ::protobuf::Message for DogStatsdSink {
  type MessageView<'msg> = DogStatsdSinkView<'msg>;
  type MessageMut<'msg> = DogStatsdSinkMut<'msg>;
}

impl ::std::default::Default for DogStatsdSink {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DogStatsdSink {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DogStatsdSink` is `Sync` because it does not implement interior mutability.
//    Neither does `DogStatsdSinkMut`.
unsafe impl ::std::marker::Sync for DogStatsdSink {}

// SAFETY:
// - `DogStatsdSink` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DogStatsdSink {}

impl ::protobuf::Proxied for DogStatsdSink {
  type View<'msg> = DogStatsdSinkView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DogStatsdSink {}

impl ::protobuf::MutProxied for DogStatsdSink {
  type Mut<'msg> = DogStatsdSinkMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DogStatsdSinkView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DogStatsdSink>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DogStatsdSinkView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DogStatsdSinkView<'msg> {
  type Message = DogStatsdSink;
}

impl ::std::fmt::Debug for DogStatsdSinkView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DogStatsdSinkView<'_> {
  fn default() -> DogStatsdSinkView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DogStatsdSink>> for DogStatsdSinkView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DogStatsdSink>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DogStatsdSinkView<'msg> {

  pub fn to_owned(&self) -> DogStatsdSink {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // prefix: optional string
  pub fn prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // max_bytes_per_datagram: optional message google.protobuf.UInt64Value
  pub fn has_max_bytes_per_datagram(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn max_bytes_per_datagram_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_max_bytes_per_datagram().then(|| self.max_bytes_per_datagram())
  }
  pub fn max_bytes_per_datagram(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  pub fn dog_statsd_specifier(self) -> super::dog_statsd_sink::DogStatsdSpecifierOneof<'msg> {
    match self.dog_statsd_specifier_case() {
      super::dog_statsd_sink::DogStatsdSpecifierCase::Address =>
          super::dog_statsd_sink::DogStatsdSpecifierOneof::Address(self.address()),
      _ => super::dog_statsd_sink::DogStatsdSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn dog_statsd_specifier_case(self) -> super::dog_statsd_sink::DogStatsdSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::dog_statsd_sink::DogStatsdSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DogStatsdSinkView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DogStatsdSinkView<'_> {}

// SAFETY:
// - `DogStatsdSinkView` is `Send` because while its alive a `DogStatsdSinkMut` cannot.
// - `DogStatsdSinkView` does not use thread-local data.
unsafe impl ::std::marker::Send for DogStatsdSinkView<'_> {}

impl<'msg> ::protobuf::AsView for DogStatsdSinkView<'msg> {
  type Proxied = DogStatsdSink;
  fn as_view(&self) -> ::protobuf::View<'msg, DogStatsdSink> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DogStatsdSinkView<'msg> {
  fn into_view<'shorter>(self) -> DogStatsdSinkView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DogStatsdSink> for DogStatsdSinkView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DogStatsdSink {
    let mut dst = DogStatsdSink::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DogStatsdSink> for DogStatsdSinkMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DogStatsdSink {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DogStatsdSink {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DogStatsdSinkView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DogStatsdSinkMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DogStatsdSinkMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DogStatsdSink>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DogStatsdSinkMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DogStatsdSinkMut<'msg> {
  type Message = DogStatsdSink;
}

impl ::std::fmt::Debug for DogStatsdSinkMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DogStatsdSink>> for DogStatsdSinkMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DogStatsdSink>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DogStatsdSinkMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DogStatsdSink> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DogStatsdSink {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // prefix: optional string
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // max_bytes_per_datagram: optional message google.protobuf.UInt64Value
  pub fn has_max_bytes_per_datagram(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_bytes_per_datagram(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_bytes_per_datagram_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_max_bytes_per_datagram().then(|| self.max_bytes_per_datagram())
  }
  pub fn max_bytes_per_datagram(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn max_bytes_per_datagram_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_max_bytes_per_datagram(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn dog_statsd_specifier(&self) -> super::dog_statsd_sink::DogStatsdSpecifierOneof<'_> {
    match &self.dog_statsd_specifier_case() {
      super::dog_statsd_sink::DogStatsdSpecifierCase::Address =>
          super::dog_statsd_sink::DogStatsdSpecifierOneof::Address(self.address()),
      _ => super::dog_statsd_sink::DogStatsdSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn dog_statsd_specifier_case(&self) -> super::dog_statsd_sink::DogStatsdSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::dog_statsd_sink::DogStatsdSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DogStatsdSinkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DogStatsdSinkMut<'_> {}

// SAFETY:
// - `DogStatsdSinkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DogStatsdSinkMut<'_> {}

impl<'msg> ::protobuf::AsView for DogStatsdSinkMut<'msg> {
  type Proxied = DogStatsdSink;
  fn as_view(&self) -> ::protobuf::View<'_, DogStatsdSink> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DogStatsdSinkMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DogStatsdSink>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DogStatsdSinkMut<'msg> {
  type MutProxied = DogStatsdSink;
  fn as_mut(&mut self) -> DogStatsdSinkMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DogStatsdSinkMut<'msg> {
  fn into_mut<'shorter>(self) -> DogStatsdSinkMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DogStatsdSink {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DogStatsdSink> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DogStatsdSinkView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DogStatsdSinkMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
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
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // prefix: optional string
  pub fn prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // max_bytes_per_datagram: optional message google.protobuf.UInt64Value
  pub fn has_max_bytes_per_datagram(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_bytes_per_datagram(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_bytes_per_datagram_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_max_bytes_per_datagram().then(|| self.max_bytes_per_datagram())
  }
  pub fn max_bytes_per_datagram(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn max_bytes_per_datagram_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_max_bytes_per_datagram(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn dog_statsd_specifier(&self) -> super::dog_statsd_sink::DogStatsdSpecifierOneof<'_> {
    match &self.dog_statsd_specifier_case() {
      super::dog_statsd_sink::DogStatsdSpecifierCase::Address =>
          super::dog_statsd_sink::DogStatsdSpecifierOneof::Address(self.address()),
      _ => super::dog_statsd_sink::DogStatsdSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn dog_statsd_specifier_case(&self) -> super::dog_statsd_sink::DogStatsdSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::dog_statsd_sink::DogStatsdSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl DogStatsdSink

impl ::std::ops::Drop for DogStatsdSink {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DogStatsdSink {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DogStatsdSink {
  type Proxied = Self;
  fn as_view(&self) -> DogStatsdSinkView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DogStatsdSink {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DogStatsdSinkMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DogStatsdSink {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__metrics__v3__DogStatsdSink_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3a1X3^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__metrics__v3__DogStatsdSink_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__metrics__v3__DogStatsdSink_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DogStatsdSink {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DogStatsdSink {
  type Msg = DogStatsdSink;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DogStatsdSink> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DogStatsdSink {
  type Msg = DogStatsdSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DogStatsdSink> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DogStatsdSinkMut<'_> {
  type Msg = DogStatsdSink;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DogStatsdSink> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DogStatsdSinkMut<'_> {
  type Msg = DogStatsdSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DogStatsdSink> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DogStatsdSinkView<'_> {
  type Msg = DogStatsdSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DogStatsdSink> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DogStatsdSinkMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod dog_statsd_sink {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum DogStatsdSpecifierOneof<'msg> {
  Address(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::address::Address>) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum DogStatsdSpecifierCase {
  Address = 1,

  not_set = 0
}

impl DogStatsdSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<DogStatsdSpecifierCase> {
    match v {
      0 => Some(DogStatsdSpecifierCase::not_set),
      1 => Some(DogStatsdSpecifierCase::Address),
      _ => None
    }
  }
}
}  // pub mod dog_statsd_sink


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__metrics__v3__HystrixSink_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HystrixSink {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HystrixSink>
}

impl ::protobuf::Message for HystrixSink {
  type MessageView<'msg> = HystrixSinkView<'msg>;
  type MessageMut<'msg> = HystrixSinkMut<'msg>;
}

impl ::std::default::Default for HystrixSink {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HystrixSink {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HystrixSink` is `Sync` because it does not implement interior mutability.
//    Neither does `HystrixSinkMut`.
unsafe impl ::std::marker::Sync for HystrixSink {}

// SAFETY:
// - `HystrixSink` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HystrixSink {}

impl ::protobuf::Proxied for HystrixSink {
  type View<'msg> = HystrixSinkView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HystrixSink {}

impl ::protobuf::MutProxied for HystrixSink {
  type Mut<'msg> = HystrixSinkMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HystrixSinkView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HystrixSink>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HystrixSinkView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HystrixSinkView<'msg> {
  type Message = HystrixSink;
}

impl ::std::fmt::Debug for HystrixSinkView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HystrixSinkView<'_> {
  fn default() -> HystrixSinkView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HystrixSink>> for HystrixSinkView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HystrixSink>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HystrixSinkView<'msg> {

  pub fn to_owned(&self) -> HystrixSink {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // num_buckets: optional int64
  pub fn num_buckets(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `HystrixSinkView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HystrixSinkView<'_> {}

// SAFETY:
// - `HystrixSinkView` is `Send` because while its alive a `HystrixSinkMut` cannot.
// - `HystrixSinkView` does not use thread-local data.
unsafe impl ::std::marker::Send for HystrixSinkView<'_> {}

impl<'msg> ::protobuf::AsView for HystrixSinkView<'msg> {
  type Proxied = HystrixSink;
  fn as_view(&self) -> ::protobuf::View<'msg, HystrixSink> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HystrixSinkView<'msg> {
  fn into_view<'shorter>(self) -> HystrixSinkView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HystrixSink> for HystrixSinkView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HystrixSink {
    let mut dst = HystrixSink::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HystrixSink> for HystrixSinkMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HystrixSink {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HystrixSink {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HystrixSinkView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HystrixSinkMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HystrixSinkMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HystrixSink>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HystrixSinkMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HystrixSinkMut<'msg> {
  type Message = HystrixSink;
}

impl ::std::fmt::Debug for HystrixSinkMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HystrixSink>> for HystrixSinkMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HystrixSink>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HystrixSinkMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HystrixSink> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HystrixSink {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // num_buckets: optional int64
  pub fn num_buckets(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_num_buckets(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `HystrixSinkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HystrixSinkMut<'_> {}

// SAFETY:
// - `HystrixSinkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HystrixSinkMut<'_> {}

impl<'msg> ::protobuf::AsView for HystrixSinkMut<'msg> {
  type Proxied = HystrixSink;
  fn as_view(&self) -> ::protobuf::View<'_, HystrixSink> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HystrixSinkMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HystrixSink>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HystrixSinkMut<'msg> {
  type MutProxied = HystrixSink;
  fn as_mut(&mut self) -> HystrixSinkMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HystrixSinkMut<'msg> {
  fn into_mut<'shorter>(self) -> HystrixSinkMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HystrixSink {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HystrixSink> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HystrixSinkView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HystrixSinkMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // num_buckets: optional int64
  pub fn num_buckets(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        0, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_num_buckets(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        0, val.into()
      )
    }
  }

}  // impl HystrixSink

impl ::std::ops::Drop for HystrixSink {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HystrixSink {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HystrixSink {
  type Proxied = Self;
  fn as_view(&self) -> HystrixSinkView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HystrixSink {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HystrixSinkMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HystrixSink {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__metrics__v3__HystrixSink_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$+P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__metrics__v3__HystrixSink_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__metrics__v3__HystrixSink_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HystrixSink {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HystrixSink {
  type Msg = HystrixSink;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HystrixSink> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HystrixSink {
  type Msg = HystrixSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HystrixSink> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HystrixSinkMut<'_> {
  type Msg = HystrixSink;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HystrixSink> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HystrixSinkMut<'_> {
  type Msg = HystrixSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HystrixSink> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HystrixSinkView<'_> {
  type Msg = HystrixSink;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HystrixSink> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HystrixSinkMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



