const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__TcpProtocolOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TcpProtocolOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TcpProtocolOptions>
}

impl ::protobuf::Message for TcpProtocolOptions {
  type MessageView<'msg> = TcpProtocolOptionsView<'msg>;
  type MessageMut<'msg> = TcpProtocolOptionsMut<'msg>;
}

impl ::std::default::Default for TcpProtocolOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TcpProtocolOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TcpProtocolOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `TcpProtocolOptionsMut`.
unsafe impl ::std::marker::Sync for TcpProtocolOptions {}

// SAFETY:
// - `TcpProtocolOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TcpProtocolOptions {}

impl ::protobuf::Proxied for TcpProtocolOptions {
  type View<'msg> = TcpProtocolOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TcpProtocolOptions {}

impl ::protobuf::MutProxied for TcpProtocolOptions {
  type Mut<'msg> = TcpProtocolOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TcpProtocolOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TcpProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TcpProtocolOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TcpProtocolOptionsView<'msg> {
  type Message = TcpProtocolOptions;
}

impl ::std::fmt::Debug for TcpProtocolOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TcpProtocolOptionsView<'_> {
  fn default() -> TcpProtocolOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TcpProtocolOptions>> for TcpProtocolOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TcpProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TcpProtocolOptionsView<'msg> {

  pub fn to_owned(&self) -> TcpProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `TcpProtocolOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TcpProtocolOptionsView<'_> {}

// SAFETY:
// - `TcpProtocolOptionsView` is `Send` because while its alive a `TcpProtocolOptionsMut` cannot.
// - `TcpProtocolOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for TcpProtocolOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for TcpProtocolOptionsView<'msg> {
  type Proxied = TcpProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, TcpProtocolOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TcpProtocolOptionsView<'msg> {
  fn into_view<'shorter>(self) -> TcpProtocolOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TcpProtocolOptions> for TcpProtocolOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TcpProtocolOptions {
    let mut dst = TcpProtocolOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TcpProtocolOptions> for TcpProtocolOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TcpProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TcpProtocolOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TcpProtocolOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TcpProtocolOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TcpProtocolOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TcpProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TcpProtocolOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TcpProtocolOptionsMut<'msg> {
  type Message = TcpProtocolOptions;
}

impl ::std::fmt::Debug for TcpProtocolOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TcpProtocolOptions>> for TcpProtocolOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TcpProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TcpProtocolOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TcpProtocolOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TcpProtocolOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `TcpProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TcpProtocolOptionsMut<'_> {}

// SAFETY:
// - `TcpProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TcpProtocolOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for TcpProtocolOptionsMut<'msg> {
  type Proxied = TcpProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'_, TcpProtocolOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TcpProtocolOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TcpProtocolOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TcpProtocolOptionsMut<'msg> {
  type MutProxied = TcpProtocolOptions;
  fn as_mut(&mut self) -> TcpProtocolOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TcpProtocolOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> TcpProtocolOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TcpProtocolOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TcpProtocolOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TcpProtocolOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TcpProtocolOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl TcpProtocolOptions

impl ::std::ops::Drop for TcpProtocolOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TcpProtocolOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TcpProtocolOptions {
  type Proxied = Self;
  fn as_view(&self) -> TcpProtocolOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TcpProtocolOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TcpProtocolOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TcpProtocolOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__TcpProtocolOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__TcpProtocolOptions_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__TcpProtocolOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TcpProtocolOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TcpProtocolOptions {
  type Msg = TcpProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TcpProtocolOptions {
  type Msg = TcpProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TcpProtocolOptionsMut<'_> {
  type Msg = TcpProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TcpProtocolOptionsMut<'_> {
  type Msg = TcpProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TcpProtocolOptionsView<'_> {
  type Msg = TcpProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpProtocolOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TcpProtocolOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__QuicKeepAliveSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct QuicKeepAliveSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<QuicKeepAliveSettings>
}

impl ::protobuf::Message for QuicKeepAliveSettings {
  type MessageView<'msg> = QuicKeepAliveSettingsView<'msg>;
  type MessageMut<'msg> = QuicKeepAliveSettingsMut<'msg>;
}

impl ::std::default::Default for QuicKeepAliveSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for QuicKeepAliveSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `QuicKeepAliveSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `QuicKeepAliveSettingsMut`.
unsafe impl ::std::marker::Sync for QuicKeepAliveSettings {}

// SAFETY:
// - `QuicKeepAliveSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for QuicKeepAliveSettings {}

impl ::protobuf::Proxied for QuicKeepAliveSettings {
  type View<'msg> = QuicKeepAliveSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for QuicKeepAliveSettings {}

impl ::protobuf::MutProxied for QuicKeepAliveSettings {
  type Mut<'msg> = QuicKeepAliveSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct QuicKeepAliveSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QuicKeepAliveSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QuicKeepAliveSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for QuicKeepAliveSettingsView<'msg> {
  type Message = QuicKeepAliveSettings;
}

impl ::std::fmt::Debug for QuicKeepAliveSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for QuicKeepAliveSettingsView<'_> {
  fn default() -> QuicKeepAliveSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, QuicKeepAliveSettings>> for QuicKeepAliveSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QuicKeepAliveSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QuicKeepAliveSettingsView<'msg> {

  pub fn to_owned(&self) -> QuicKeepAliveSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // max_interval: optional message google.protobuf.Duration
  pub fn has_max_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn max_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_interval().then(|| self.max_interval())
  }
  pub fn max_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // initial_interval: optional message google.protobuf.Duration
  pub fn has_initial_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn initial_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_initial_interval().then(|| self.initial_interval())
  }
  pub fn initial_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `QuicKeepAliveSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for QuicKeepAliveSettingsView<'_> {}

// SAFETY:
// - `QuicKeepAliveSettingsView` is `Send` because while its alive a `QuicKeepAliveSettingsMut` cannot.
// - `QuicKeepAliveSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for QuicKeepAliveSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for QuicKeepAliveSettingsView<'msg> {
  type Proxied = QuicKeepAliveSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, QuicKeepAliveSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QuicKeepAliveSettingsView<'msg> {
  fn into_view<'shorter>(self) -> QuicKeepAliveSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<QuicKeepAliveSettings> for QuicKeepAliveSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QuicKeepAliveSettings {
    let mut dst = QuicKeepAliveSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<QuicKeepAliveSettings> for QuicKeepAliveSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QuicKeepAliveSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for QuicKeepAliveSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QuicKeepAliveSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QuicKeepAliveSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct QuicKeepAliveSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QuicKeepAliveSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QuicKeepAliveSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for QuicKeepAliveSettingsMut<'msg> {
  type Message = QuicKeepAliveSettings;
}

impl ::std::fmt::Debug for QuicKeepAliveSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, QuicKeepAliveSettings>> for QuicKeepAliveSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QuicKeepAliveSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QuicKeepAliveSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, QuicKeepAliveSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> QuicKeepAliveSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // max_interval: optional message google.protobuf.Duration
  pub fn has_max_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_interval().then(|| self.max_interval())
  }
  pub fn max_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // initial_interval: optional message google.protobuf.Duration
  pub fn has_initial_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_initial_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn initial_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_initial_interval().then(|| self.initial_interval())
  }
  pub fn initial_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn initial_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_initial_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

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
// - `QuicKeepAliveSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for QuicKeepAliveSettingsMut<'_> {}

// SAFETY:
// - `QuicKeepAliveSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for QuicKeepAliveSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for QuicKeepAliveSettingsMut<'msg> {
  type Proxied = QuicKeepAliveSettings;
  fn as_view(&self) -> ::protobuf::View<'_, QuicKeepAliveSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QuicKeepAliveSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, QuicKeepAliveSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for QuicKeepAliveSettingsMut<'msg> {
  type MutProxied = QuicKeepAliveSettings;
  fn as_mut(&mut self) -> QuicKeepAliveSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for QuicKeepAliveSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> QuicKeepAliveSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl QuicKeepAliveSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, QuicKeepAliveSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> QuicKeepAliveSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> QuicKeepAliveSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // max_interval: optional message google.protobuf.Duration
  pub fn has_max_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_interval().then(|| self.max_interval())
  }
  pub fn max_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // initial_interval: optional message google.protobuf.Duration
  pub fn has_initial_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_initial_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn initial_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_initial_interval().then(|| self.initial_interval())
  }
  pub fn initial_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn initial_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_initial_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl QuicKeepAliveSettings

impl ::std::ops::Drop for QuicKeepAliveSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for QuicKeepAliveSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for QuicKeepAliveSettings {
  type Proxied = Self;
  fn as_view(&self) -> QuicKeepAliveSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for QuicKeepAliveSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> QuicKeepAliveSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for QuicKeepAliveSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__QuicKeepAliveSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__QuicKeepAliveSettings_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__QuicKeepAliveSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QuicKeepAliveSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QuicKeepAliveSettings {
  type Msg = QuicKeepAliveSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicKeepAliveSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QuicKeepAliveSettings {
  type Msg = QuicKeepAliveSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicKeepAliveSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QuicKeepAliveSettingsMut<'_> {
  type Msg = QuicKeepAliveSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicKeepAliveSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QuicKeepAliveSettingsMut<'_> {
  type Msg = QuicKeepAliveSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicKeepAliveSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QuicKeepAliveSettingsView<'_> {
  type Msg = QuicKeepAliveSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicKeepAliveSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QuicKeepAliveSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__QuicProtocolOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct QuicProtocolOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<QuicProtocolOptions>
}

impl ::protobuf::Message for QuicProtocolOptions {
  type MessageView<'msg> = QuicProtocolOptionsView<'msg>;
  type MessageMut<'msg> = QuicProtocolOptionsMut<'msg>;
}

impl ::std::default::Default for QuicProtocolOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for QuicProtocolOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `QuicProtocolOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `QuicProtocolOptionsMut`.
unsafe impl ::std::marker::Sync for QuicProtocolOptions {}

// SAFETY:
// - `QuicProtocolOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for QuicProtocolOptions {}

impl ::protobuf::Proxied for QuicProtocolOptions {
  type View<'msg> = QuicProtocolOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for QuicProtocolOptions {}

impl ::protobuf::MutProxied for QuicProtocolOptions {
  type Mut<'msg> = QuicProtocolOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct QuicProtocolOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QuicProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QuicProtocolOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for QuicProtocolOptionsView<'msg> {
  type Message = QuicProtocolOptions;
}

impl ::std::fmt::Debug for QuicProtocolOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for QuicProtocolOptionsView<'_> {
  fn default() -> QuicProtocolOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, QuicProtocolOptions>> for QuicProtocolOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QuicProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QuicProtocolOptionsView<'msg> {

  pub fn to_owned(&self) -> QuicProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // max_concurrent_streams: optional message google.protobuf.UInt32Value
  pub fn has_max_concurrent_streams(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn max_concurrent_streams_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_concurrent_streams().then(|| self.max_concurrent_streams())
  }
  pub fn max_concurrent_streams(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // initial_stream_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_stream_window_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn initial_stream_window_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_initial_stream_window_size().then(|| self.initial_stream_window_size())
  }
  pub fn initial_stream_window_size(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // initial_connection_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_connection_window_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn initial_connection_window_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_initial_connection_window_size().then(|| self.initial_connection_window_size())
  }
  pub fn initial_connection_window_size(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // num_timeouts_to_trigger_port_migration: optional message google.protobuf.UInt32Value
  pub fn has_num_timeouts_to_trigger_port_migration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn num_timeouts_to_trigger_port_migration_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_num_timeouts_to_trigger_port_migration().then(|| self.num_timeouts_to_trigger_port_migration())
  }
  pub fn num_timeouts_to_trigger_port_migration(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // connection_keepalive: optional message envoy.config.core.v3.QuicKeepAliveSettings
  pub fn has_connection_keepalive(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn connection_keepalive_opt(self) -> ::std::option::Option<super::QuicKeepAliveSettingsView<'msg>> {
    self.has_connection_keepalive().then(|| self.connection_keepalive())
  }
  pub fn connection_keepalive(self) -> super::QuicKeepAliveSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::QuicKeepAliveSettingsView::default())
  }

  // connection_options: optional string
  pub fn connection_options(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // client_connection_options: optional string
  pub fn client_connection_options(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // idle_network_timeout: optional message google.protobuf.Duration
  pub fn has_idle_network_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn idle_network_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_idle_network_timeout().then(|| self.idle_network_timeout())
  }
  pub fn idle_network_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // max_packet_length: optional message google.protobuf.UInt64Value
  pub fn has_max_packet_length(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn max_packet_length_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_max_packet_length().then(|| self.max_packet_length())
  }
  pub fn max_packet_length(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // client_packet_writer: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_client_packet_writer(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn client_packet_writer_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_client_packet_writer().then(|| self.client_packet_writer())
  }
  pub fn client_packet_writer(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // connection_migration: optional message envoy.config.core.v3.QuicProtocolOptions.ConnectionMigrationSettings
  pub fn has_connection_migration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn connection_migration_opt(self) -> ::std::option::Option<super::quic_protocol_options::ConnectionMigrationSettingsView<'msg>> {
    self.has_connection_migration().then(|| self.connection_migration())
  }
  pub fn connection_migration(self) -> super::quic_protocol_options::ConnectionMigrationSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::quic_protocol_options::ConnectionMigrationSettingsView::default())
  }

}

// SAFETY:
// - `QuicProtocolOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for QuicProtocolOptionsView<'_> {}

// SAFETY:
// - `QuicProtocolOptionsView` is `Send` because while its alive a `QuicProtocolOptionsMut` cannot.
// - `QuicProtocolOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for QuicProtocolOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for QuicProtocolOptionsView<'msg> {
  type Proxied = QuicProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, QuicProtocolOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QuicProtocolOptionsView<'msg> {
  fn into_view<'shorter>(self) -> QuicProtocolOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<QuicProtocolOptions> for QuicProtocolOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QuicProtocolOptions {
    let mut dst = QuicProtocolOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<QuicProtocolOptions> for QuicProtocolOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QuicProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for QuicProtocolOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QuicProtocolOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QuicProtocolOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct QuicProtocolOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QuicProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QuicProtocolOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for QuicProtocolOptionsMut<'msg> {
  type Message = QuicProtocolOptions;
}

impl ::std::fmt::Debug for QuicProtocolOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, QuicProtocolOptions>> for QuicProtocolOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QuicProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QuicProtocolOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, QuicProtocolOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> QuicProtocolOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // max_concurrent_streams: optional message google.protobuf.UInt32Value
  pub fn has_max_concurrent_streams(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_concurrent_streams(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_concurrent_streams_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_concurrent_streams().then(|| self.max_concurrent_streams())
  }
  pub fn max_concurrent_streams(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_concurrent_streams_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_concurrent_streams(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // initial_stream_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_stream_window_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_initial_stream_window_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn initial_stream_window_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_initial_stream_window_size().then(|| self.initial_stream_window_size())
  }
  pub fn initial_stream_window_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn initial_stream_window_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_initial_stream_window_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // initial_connection_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_connection_window_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_initial_connection_window_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn initial_connection_window_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_initial_connection_window_size().then(|| self.initial_connection_window_size())
  }
  pub fn initial_connection_window_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn initial_connection_window_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_initial_connection_window_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // num_timeouts_to_trigger_port_migration: optional message google.protobuf.UInt32Value
  pub fn has_num_timeouts_to_trigger_port_migration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_num_timeouts_to_trigger_port_migration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn num_timeouts_to_trigger_port_migration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_num_timeouts_to_trigger_port_migration().then(|| self.num_timeouts_to_trigger_port_migration())
  }
  pub fn num_timeouts_to_trigger_port_migration(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn num_timeouts_to_trigger_port_migration_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_num_timeouts_to_trigger_port_migration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // connection_keepalive: optional message envoy.config.core.v3.QuicKeepAliveSettings
  pub fn has_connection_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_connection_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn connection_keepalive_opt(&self) -> ::std::option::Option<super::QuicKeepAliveSettingsView<'_>> {
    self.has_connection_keepalive().then(|| self.connection_keepalive())
  }
  pub fn connection_keepalive(&self) -> super::QuicKeepAliveSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::QuicKeepAliveSettingsView::default())
  }
  pub fn connection_keepalive_mut(&mut self) -> super::QuicKeepAliveSettingsMut<'_> {
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
  pub fn set_connection_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<super::QuicKeepAliveSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // connection_options: optional string
  pub fn connection_options(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_connection_options(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // client_connection_options: optional string
  pub fn client_connection_options(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_client_connection_options(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // idle_network_timeout: optional message google.protobuf.Duration
  pub fn has_idle_network_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_idle_network_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn idle_network_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_idle_network_timeout().then(|| self.idle_network_timeout())
  }
  pub fn idle_network_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn idle_network_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_idle_network_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // max_packet_length: optional message google.protobuf.UInt64Value
  pub fn has_max_packet_length(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_max_packet_length(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn max_packet_length_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_max_packet_length().then(|| self.max_packet_length())
  }
  pub fn max_packet_length(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn max_packet_length_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_max_packet_length(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // client_packet_writer: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_client_packet_writer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_client_packet_writer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn client_packet_writer_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_client_packet_writer().then(|| self.client_packet_writer())
  }
  pub fn client_packet_writer(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn client_packet_writer_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_client_packet_writer(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // connection_migration: optional message envoy.config.core.v3.QuicProtocolOptions.ConnectionMigrationSettings
  pub fn has_connection_migration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_connection_migration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn connection_migration_opt(&self) -> ::std::option::Option<super::quic_protocol_options::ConnectionMigrationSettingsView<'_>> {
    self.has_connection_migration().then(|| self.connection_migration())
  }
  pub fn connection_migration(&self) -> super::quic_protocol_options::ConnectionMigrationSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::quic_protocol_options::ConnectionMigrationSettingsView::default())
  }
  pub fn connection_migration_mut(&mut self) -> super::quic_protocol_options::ConnectionMigrationSettingsMut<'_> {
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
  pub fn set_connection_migration(&mut self,
    val: impl ::protobuf::IntoProxied<super::quic_protocol_options::ConnectionMigrationSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

}

// SAFETY:
// - `QuicProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for QuicProtocolOptionsMut<'_> {}

// SAFETY:
// - `QuicProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for QuicProtocolOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for QuicProtocolOptionsMut<'msg> {
  type Proxied = QuicProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'_, QuicProtocolOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QuicProtocolOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, QuicProtocolOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for QuicProtocolOptionsMut<'msg> {
  type MutProxied = QuicProtocolOptions;
  fn as_mut(&mut self) -> QuicProtocolOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for QuicProtocolOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> QuicProtocolOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl QuicProtocolOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, QuicProtocolOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> QuicProtocolOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> QuicProtocolOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // max_concurrent_streams: optional message google.protobuf.UInt32Value
  pub fn has_max_concurrent_streams(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_concurrent_streams(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_concurrent_streams_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_concurrent_streams().then(|| self.max_concurrent_streams())
  }
  pub fn max_concurrent_streams(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_concurrent_streams_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_concurrent_streams(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // initial_stream_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_stream_window_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_initial_stream_window_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn initial_stream_window_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_initial_stream_window_size().then(|| self.initial_stream_window_size())
  }
  pub fn initial_stream_window_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn initial_stream_window_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_initial_stream_window_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // initial_connection_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_connection_window_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_initial_connection_window_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn initial_connection_window_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_initial_connection_window_size().then(|| self.initial_connection_window_size())
  }
  pub fn initial_connection_window_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn initial_connection_window_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_initial_connection_window_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // num_timeouts_to_trigger_port_migration: optional message google.protobuf.UInt32Value
  pub fn has_num_timeouts_to_trigger_port_migration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_num_timeouts_to_trigger_port_migration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn num_timeouts_to_trigger_port_migration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_num_timeouts_to_trigger_port_migration().then(|| self.num_timeouts_to_trigger_port_migration())
  }
  pub fn num_timeouts_to_trigger_port_migration(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn num_timeouts_to_trigger_port_migration_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_num_timeouts_to_trigger_port_migration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // connection_keepalive: optional message envoy.config.core.v3.QuicKeepAliveSettings
  pub fn has_connection_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_connection_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn connection_keepalive_opt(&self) -> ::std::option::Option<super::QuicKeepAliveSettingsView<'_>> {
    self.has_connection_keepalive().then(|| self.connection_keepalive())
  }
  pub fn connection_keepalive(&self) -> super::QuicKeepAliveSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::QuicKeepAliveSettingsView::default())
  }
  pub fn connection_keepalive_mut(&mut self) -> super::QuicKeepAliveSettingsMut<'_> {
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
  pub fn set_connection_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<super::QuicKeepAliveSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // connection_options: optional string
  pub fn connection_options(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_connection_options(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // client_connection_options: optional string
  pub fn client_connection_options(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_client_connection_options(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // idle_network_timeout: optional message google.protobuf.Duration
  pub fn has_idle_network_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_idle_network_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn idle_network_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_idle_network_timeout().then(|| self.idle_network_timeout())
  }
  pub fn idle_network_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn idle_network_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_idle_network_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // max_packet_length: optional message google.protobuf.UInt64Value
  pub fn has_max_packet_length(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_max_packet_length(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn max_packet_length_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_max_packet_length().then(|| self.max_packet_length())
  }
  pub fn max_packet_length(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn max_packet_length_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_max_packet_length(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // client_packet_writer: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_client_packet_writer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_client_packet_writer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn client_packet_writer_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_client_packet_writer().then(|| self.client_packet_writer())
  }
  pub fn client_packet_writer(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn client_packet_writer_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_client_packet_writer(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // connection_migration: optional message envoy.config.core.v3.QuicProtocolOptions.ConnectionMigrationSettings
  pub fn has_connection_migration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_connection_migration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn connection_migration_opt(&self) -> ::std::option::Option<super::quic_protocol_options::ConnectionMigrationSettingsView<'_>> {
    self.has_connection_migration().then(|| self.connection_migration())
  }
  pub fn connection_migration(&self) -> super::quic_protocol_options::ConnectionMigrationSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::quic_protocol_options::ConnectionMigrationSettingsView::default())
  }
  pub fn connection_migration_mut(&mut self) -> super::quic_protocol_options::ConnectionMigrationSettingsMut<'_> {
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
  pub fn set_connection_migration(&mut self,
    val: impl ::protobuf::IntoProxied<super::quic_protocol_options::ConnectionMigrationSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

}  // impl QuicProtocolOptions

impl ::std::ops::Drop for QuicProtocolOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for QuicProtocolOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for QuicProtocolOptions {
  type Proxied = Self;
  fn as_view(&self) -> QuicProtocolOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for QuicProtocolOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> QuicProtocolOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for QuicProtocolOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__QuicProtocolOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333331X1X3333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__QuicProtocolOptions_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::QuicKeepAliveSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::quic_protocol_options::ConnectionMigrationSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__QuicProtocolOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QuicProtocolOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QuicProtocolOptions {
  type Msg = QuicProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QuicProtocolOptions {
  type Msg = QuicProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QuicProtocolOptionsMut<'_> {
  type Msg = QuicProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QuicProtocolOptionsMut<'_> {
  type Msg = QuicProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QuicProtocolOptionsView<'_> {
  type Msg = QuicProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QuicProtocolOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QuicProtocolOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod quic_protocol_options {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__QuicProtocolOptions__ConnectionMigrationSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConnectionMigrationSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConnectionMigrationSettings>
}

impl ::protobuf::Message for ConnectionMigrationSettings {
  type MessageView<'msg> = ConnectionMigrationSettingsView<'msg>;
  type MessageMut<'msg> = ConnectionMigrationSettingsMut<'msg>;
}

impl ::std::default::Default for ConnectionMigrationSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConnectionMigrationSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConnectionMigrationSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `ConnectionMigrationSettingsMut`.
unsafe impl ::std::marker::Sync for ConnectionMigrationSettings {}

// SAFETY:
// - `ConnectionMigrationSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ConnectionMigrationSettings {}

impl ::protobuf::Proxied for ConnectionMigrationSettings {
  type View<'msg> = ConnectionMigrationSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConnectionMigrationSettings {}

impl ::protobuf::MutProxied for ConnectionMigrationSettings {
  type Mut<'msg> = ConnectionMigrationSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConnectionMigrationSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionMigrationSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionMigrationSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConnectionMigrationSettingsView<'msg> {
  type Message = ConnectionMigrationSettings;
}

impl ::std::fmt::Debug for ConnectionMigrationSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConnectionMigrationSettingsView<'_> {
  fn default() -> ConnectionMigrationSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionMigrationSettings>> for ConnectionMigrationSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConnectionMigrationSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionMigrationSettingsView<'msg> {

  pub fn to_owned(&self) -> ConnectionMigrationSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // migrate_idle_connections: optional message envoy.config.core.v3.QuicProtocolOptions.ConnectionMigrationSettings.MigrateIdleConnectionSettings
  pub fn has_migrate_idle_connections(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn migrate_idle_connections_opt(self) -> ::std::option::Option<super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsView<'msg>> {
    self.has_migrate_idle_connections().then(|| self.migrate_idle_connections())
  }
  pub fn migrate_idle_connections(self) -> super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsView::default())
  }

  // max_time_on_non_default_network: optional message google.protobuf.Duration
  pub fn has_max_time_on_non_default_network(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn max_time_on_non_default_network_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_time_on_non_default_network().then(|| self.max_time_on_non_default_network())
  }
  pub fn max_time_on_non_default_network(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `ConnectionMigrationSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ConnectionMigrationSettingsView<'_> {}

// SAFETY:
// - `ConnectionMigrationSettingsView` is `Send` because while its alive a `ConnectionMigrationSettingsMut` cannot.
// - `ConnectionMigrationSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ConnectionMigrationSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for ConnectionMigrationSettingsView<'msg> {
  type Proxied = ConnectionMigrationSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, ConnectionMigrationSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionMigrationSettingsView<'msg> {
  fn into_view<'shorter>(self) -> ConnectionMigrationSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionMigrationSettings> for ConnectionMigrationSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionMigrationSettings {
    let mut dst = ConnectionMigrationSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConnectionMigrationSettings> for ConnectionMigrationSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConnectionMigrationSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ConnectionMigrationSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConnectionMigrationSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConnectionMigrationSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConnectionMigrationSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionMigrationSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConnectionMigrationSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConnectionMigrationSettingsMut<'msg> {
  type Message = ConnectionMigrationSettings;
}

impl ::std::fmt::Debug for ConnectionMigrationSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionMigrationSettings>> for ConnectionMigrationSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionMigrationSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConnectionMigrationSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConnectionMigrationSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ConnectionMigrationSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // migrate_idle_connections: optional message envoy.config.core.v3.QuicProtocolOptions.ConnectionMigrationSettings.MigrateIdleConnectionSettings
  pub fn has_migrate_idle_connections(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_migrate_idle_connections(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn migrate_idle_connections_opt(&self) -> ::std::option::Option<super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsView<'_>> {
    self.has_migrate_idle_connections().then(|| self.migrate_idle_connections())
  }
  pub fn migrate_idle_connections(&self) -> super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsView::default())
  }
  pub fn migrate_idle_connections_mut(&mut self) -> super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsMut<'_> {
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
  pub fn set_migrate_idle_connections(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_time_on_non_default_network: optional message google.protobuf.Duration
  pub fn has_max_time_on_non_default_network(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_time_on_non_default_network(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_time_on_non_default_network_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_time_on_non_default_network().then(|| self.max_time_on_non_default_network())
  }
  pub fn max_time_on_non_default_network(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_time_on_non_default_network_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_time_on_non_default_network(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

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
// - `ConnectionMigrationSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ConnectionMigrationSettingsMut<'_> {}

// SAFETY:
// - `ConnectionMigrationSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ConnectionMigrationSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for ConnectionMigrationSettingsMut<'msg> {
  type Proxied = ConnectionMigrationSettings;
  fn as_view(&self) -> ::protobuf::View<'_, ConnectionMigrationSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConnectionMigrationSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConnectionMigrationSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ConnectionMigrationSettingsMut<'msg> {
  type MutProxied = ConnectionMigrationSettings;
  fn as_mut(&mut self) -> ConnectionMigrationSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConnectionMigrationSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> ConnectionMigrationSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConnectionMigrationSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConnectionMigrationSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConnectionMigrationSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConnectionMigrationSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // migrate_idle_connections: optional message envoy.config.core.v3.QuicProtocolOptions.ConnectionMigrationSettings.MigrateIdleConnectionSettings
  pub fn has_migrate_idle_connections(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_migrate_idle_connections(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn migrate_idle_connections_opt(&self) -> ::std::option::Option<super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsView<'_>> {
    self.has_migrate_idle_connections().then(|| self.migrate_idle_connections())
  }
  pub fn migrate_idle_connections(&self) -> super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsView::default())
  }
  pub fn migrate_idle_connections_mut(&mut self) -> super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettingsMut<'_> {
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
  pub fn set_migrate_idle_connections(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_time_on_non_default_network: optional message google.protobuf.Duration
  pub fn has_max_time_on_non_default_network(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_time_on_non_default_network(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_time_on_non_default_network_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_time_on_non_default_network().then(|| self.max_time_on_non_default_network())
  }
  pub fn max_time_on_non_default_network(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_time_on_non_default_network_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_time_on_non_default_network(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ConnectionMigrationSettings

impl ::std::ops::Drop for ConnectionMigrationSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConnectionMigrationSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConnectionMigrationSettings {
  type Proxied = Self;
  fn as_view(&self) -> ConnectionMigrationSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConnectionMigrationSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConnectionMigrationSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConnectionMigrationSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::quic_protocol_options::envoy__config__core__v3__QuicProtocolOptions__ConnectionMigrationSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::quic_protocol_options::envoy__config__core__v3__QuicProtocolOptions__ConnectionMigrationSettings_msg_init.0, &[<super::super::quic_protocol_options::connection_migration_settings::MigrateIdleConnectionSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::quic_protocol_options::envoy__config__core__v3__QuicProtocolOptions__ConnectionMigrationSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionMigrationSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionMigrationSettings {
  type Msg = ConnectionMigrationSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionMigrationSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionMigrationSettings {
  type Msg = ConnectionMigrationSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionMigrationSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConnectionMigrationSettingsMut<'_> {
  type Msg = ConnectionMigrationSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionMigrationSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionMigrationSettingsMut<'_> {
  type Msg = ConnectionMigrationSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionMigrationSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConnectionMigrationSettingsView<'_> {
  type Msg = ConnectionMigrationSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConnectionMigrationSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConnectionMigrationSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod connection_migration_settings {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__QuicProtocolOptions__ConnectionMigrationSettings__MigrateIdleConnectionSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MigrateIdleConnectionSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MigrateIdleConnectionSettings>
}

impl ::protobuf::Message for MigrateIdleConnectionSettings {
  type MessageView<'msg> = MigrateIdleConnectionSettingsView<'msg>;
  type MessageMut<'msg> = MigrateIdleConnectionSettingsMut<'msg>;
}

impl ::std::default::Default for MigrateIdleConnectionSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MigrateIdleConnectionSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MigrateIdleConnectionSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `MigrateIdleConnectionSettingsMut`.
unsafe impl ::std::marker::Sync for MigrateIdleConnectionSettings {}

// SAFETY:
// - `MigrateIdleConnectionSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MigrateIdleConnectionSettings {}

impl ::protobuf::Proxied for MigrateIdleConnectionSettings {
  type View<'msg> = MigrateIdleConnectionSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MigrateIdleConnectionSettings {}

impl ::protobuf::MutProxied for MigrateIdleConnectionSettings {
  type Mut<'msg> = MigrateIdleConnectionSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MigrateIdleConnectionSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MigrateIdleConnectionSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MigrateIdleConnectionSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MigrateIdleConnectionSettingsView<'msg> {
  type Message = MigrateIdleConnectionSettings;
}

impl ::std::fmt::Debug for MigrateIdleConnectionSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MigrateIdleConnectionSettingsView<'_> {
  fn default() -> MigrateIdleConnectionSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MigrateIdleConnectionSettings>> for MigrateIdleConnectionSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MigrateIdleConnectionSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MigrateIdleConnectionSettingsView<'msg> {

  pub fn to_owned(&self) -> MigrateIdleConnectionSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // max_idle_time_before_migration: optional message google.protobuf.Duration
  pub fn has_max_idle_time_before_migration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn max_idle_time_before_migration_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_idle_time_before_migration().then(|| self.max_idle_time_before_migration())
  }
  pub fn max_idle_time_before_migration(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `MigrateIdleConnectionSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MigrateIdleConnectionSettingsView<'_> {}

// SAFETY:
// - `MigrateIdleConnectionSettingsView` is `Send` because while its alive a `MigrateIdleConnectionSettingsMut` cannot.
// - `MigrateIdleConnectionSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for MigrateIdleConnectionSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for MigrateIdleConnectionSettingsView<'msg> {
  type Proxied = MigrateIdleConnectionSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, MigrateIdleConnectionSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MigrateIdleConnectionSettingsView<'msg> {
  fn into_view<'shorter>(self) -> MigrateIdleConnectionSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MigrateIdleConnectionSettings> for MigrateIdleConnectionSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MigrateIdleConnectionSettings {
    let mut dst = MigrateIdleConnectionSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MigrateIdleConnectionSettings> for MigrateIdleConnectionSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MigrateIdleConnectionSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MigrateIdleConnectionSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MigrateIdleConnectionSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MigrateIdleConnectionSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MigrateIdleConnectionSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MigrateIdleConnectionSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MigrateIdleConnectionSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MigrateIdleConnectionSettingsMut<'msg> {
  type Message = MigrateIdleConnectionSettings;
}

impl ::std::fmt::Debug for MigrateIdleConnectionSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MigrateIdleConnectionSettings>> for MigrateIdleConnectionSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MigrateIdleConnectionSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MigrateIdleConnectionSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MigrateIdleConnectionSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MigrateIdleConnectionSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // max_idle_time_before_migration: optional message google.protobuf.Duration
  pub fn has_max_idle_time_before_migration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_idle_time_before_migration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_idle_time_before_migration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_idle_time_before_migration().then(|| self.max_idle_time_before_migration())
  }
  pub fn max_idle_time_before_migration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_idle_time_before_migration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_idle_time_before_migration(&mut self,
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
// - `MigrateIdleConnectionSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MigrateIdleConnectionSettingsMut<'_> {}

// SAFETY:
// - `MigrateIdleConnectionSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MigrateIdleConnectionSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for MigrateIdleConnectionSettingsMut<'msg> {
  type Proxied = MigrateIdleConnectionSettings;
  fn as_view(&self) -> ::protobuf::View<'_, MigrateIdleConnectionSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MigrateIdleConnectionSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MigrateIdleConnectionSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MigrateIdleConnectionSettingsMut<'msg> {
  type MutProxied = MigrateIdleConnectionSettings;
  fn as_mut(&mut self) -> MigrateIdleConnectionSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MigrateIdleConnectionSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> MigrateIdleConnectionSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MigrateIdleConnectionSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MigrateIdleConnectionSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MigrateIdleConnectionSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MigrateIdleConnectionSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // max_idle_time_before_migration: optional message google.protobuf.Duration
  pub fn has_max_idle_time_before_migration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_idle_time_before_migration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_idle_time_before_migration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_idle_time_before_migration().then(|| self.max_idle_time_before_migration())
  }
  pub fn max_idle_time_before_migration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_idle_time_before_migration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_idle_time_before_migration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl MigrateIdleConnectionSettings

impl ::std::ops::Drop for MigrateIdleConnectionSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MigrateIdleConnectionSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MigrateIdleConnectionSettings {
  type Proxied = Self;
  fn as_view(&self) -> MigrateIdleConnectionSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MigrateIdleConnectionSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MigrateIdleConnectionSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MigrateIdleConnectionSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::quic_protocol_options::connection_migration_settings::envoy__config__core__v3__QuicProtocolOptions__ConnectionMigrationSettings__MigrateIdleConnectionSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::quic_protocol_options::connection_migration_settings::envoy__config__core__v3__QuicProtocolOptions__ConnectionMigrationSettings__MigrateIdleConnectionSettings_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::quic_protocol_options::connection_migration_settings::envoy__config__core__v3__QuicProtocolOptions__ConnectionMigrationSettings__MigrateIdleConnectionSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MigrateIdleConnectionSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MigrateIdleConnectionSettings {
  type Msg = MigrateIdleConnectionSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateIdleConnectionSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MigrateIdleConnectionSettings {
  type Msg = MigrateIdleConnectionSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateIdleConnectionSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MigrateIdleConnectionSettingsMut<'_> {
  type Msg = MigrateIdleConnectionSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateIdleConnectionSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MigrateIdleConnectionSettingsMut<'_> {
  type Msg = MigrateIdleConnectionSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateIdleConnectionSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MigrateIdleConnectionSettingsView<'_> {
  type Msg = MigrateIdleConnectionSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MigrateIdleConnectionSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MigrateIdleConnectionSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod connection_migration_settings


}  // pub mod quic_protocol_options


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__UpstreamHttpProtocolOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpstreamHttpProtocolOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpstreamHttpProtocolOptions>
}

impl ::protobuf::Message for UpstreamHttpProtocolOptions {
  type MessageView<'msg> = UpstreamHttpProtocolOptionsView<'msg>;
  type MessageMut<'msg> = UpstreamHttpProtocolOptionsMut<'msg>;
}

impl ::std::default::Default for UpstreamHttpProtocolOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpstreamHttpProtocolOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpstreamHttpProtocolOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `UpstreamHttpProtocolOptionsMut`.
unsafe impl ::std::marker::Sync for UpstreamHttpProtocolOptions {}

// SAFETY:
// - `UpstreamHttpProtocolOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamHttpProtocolOptions {}

impl ::protobuf::Proxied for UpstreamHttpProtocolOptions {
  type View<'msg> = UpstreamHttpProtocolOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpstreamHttpProtocolOptions {}

impl ::protobuf::MutProxied for UpstreamHttpProtocolOptions {
  type Mut<'msg> = UpstreamHttpProtocolOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpstreamHttpProtocolOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamHttpProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamHttpProtocolOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpstreamHttpProtocolOptionsView<'msg> {
  type Message = UpstreamHttpProtocolOptions;
}

impl ::std::fmt::Debug for UpstreamHttpProtocolOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpstreamHttpProtocolOptionsView<'_> {
  fn default() -> UpstreamHttpProtocolOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamHttpProtocolOptions>> for UpstreamHttpProtocolOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamHttpProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamHttpProtocolOptionsView<'msg> {

  pub fn to_owned(&self) -> UpstreamHttpProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // auto_sni: optional bool
  pub fn auto_sni(self) -> bool {
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

  // auto_san_validation: optional bool
  pub fn auto_san_validation(self) -> bool {
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

  // override_auto_sni_header: optional string
  pub fn override_auto_sni_header(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `UpstreamHttpProtocolOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpstreamHttpProtocolOptionsView<'_> {}

// SAFETY:
// - `UpstreamHttpProtocolOptionsView` is `Send` because while its alive a `UpstreamHttpProtocolOptionsMut` cannot.
// - `UpstreamHttpProtocolOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamHttpProtocolOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamHttpProtocolOptionsView<'msg> {
  type Proxied = UpstreamHttpProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, UpstreamHttpProtocolOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamHttpProtocolOptionsView<'msg> {
  fn into_view<'shorter>(self) -> UpstreamHttpProtocolOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamHttpProtocolOptions> for UpstreamHttpProtocolOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamHttpProtocolOptions {
    let mut dst = UpstreamHttpProtocolOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamHttpProtocolOptions> for UpstreamHttpProtocolOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamHttpProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UpstreamHttpProtocolOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamHttpProtocolOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamHttpProtocolOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpstreamHttpProtocolOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamHttpProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamHttpProtocolOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpstreamHttpProtocolOptionsMut<'msg> {
  type Message = UpstreamHttpProtocolOptions;
}

impl ::std::fmt::Debug for UpstreamHttpProtocolOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamHttpProtocolOptions>> for UpstreamHttpProtocolOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamHttpProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamHttpProtocolOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamHttpProtocolOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UpstreamHttpProtocolOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // auto_sni: optional bool
  pub fn auto_sni(&self) -> bool {
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
  pub fn set_auto_sni(&mut self, val: bool) {
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

  // auto_san_validation: optional bool
  pub fn auto_san_validation(&self) -> bool {
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
  pub fn set_auto_san_validation(&mut self, val: bool) {
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

  // override_auto_sni_header: optional string
  pub fn override_auto_sni_header(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_override_auto_sni_header(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `UpstreamHttpProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpstreamHttpProtocolOptionsMut<'_> {}

// SAFETY:
// - `UpstreamHttpProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpstreamHttpProtocolOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamHttpProtocolOptionsMut<'msg> {
  type Proxied = UpstreamHttpProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'_, UpstreamHttpProtocolOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamHttpProtocolOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpstreamHttpProtocolOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpstreamHttpProtocolOptionsMut<'msg> {
  type MutProxied = UpstreamHttpProtocolOptions;
  fn as_mut(&mut self) -> UpstreamHttpProtocolOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpstreamHttpProtocolOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> UpstreamHttpProtocolOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpstreamHttpProtocolOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpstreamHttpProtocolOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpstreamHttpProtocolOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpstreamHttpProtocolOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // auto_sni: optional bool
  pub fn auto_sni(&self) -> bool {
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
  pub fn set_auto_sni(&mut self, val: bool) {
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

  // auto_san_validation: optional bool
  pub fn auto_san_validation(&self) -> bool {
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
  pub fn set_auto_san_validation(&mut self, val: bool) {
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

  // override_auto_sni_header: optional string
  pub fn override_auto_sni_header(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_override_auto_sni_header(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl UpstreamHttpProtocolOptions

impl ::std::ops::Drop for UpstreamHttpProtocolOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpstreamHttpProtocolOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpstreamHttpProtocolOptions {
  type Proxied = Self;
  fn as_view(&self) -> UpstreamHttpProtocolOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpstreamHttpProtocolOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpstreamHttpProtocolOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpstreamHttpProtocolOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__UpstreamHttpProtocolOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P/P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__UpstreamHttpProtocolOptions_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__UpstreamHttpProtocolOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamHttpProtocolOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamHttpProtocolOptions {
  type Msg = UpstreamHttpProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamHttpProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamHttpProtocolOptions {
  type Msg = UpstreamHttpProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamHttpProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamHttpProtocolOptionsMut<'_> {
  type Msg = UpstreamHttpProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamHttpProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamHttpProtocolOptionsMut<'_> {
  type Msg = UpstreamHttpProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamHttpProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamHttpProtocolOptionsView<'_> {
  type Msg = UpstreamHttpProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamHttpProtocolOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamHttpProtocolOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__AlternateProtocolsCacheOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AlternateProtocolsCacheOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AlternateProtocolsCacheOptions>
}

impl ::protobuf::Message for AlternateProtocolsCacheOptions {
  type MessageView<'msg> = AlternateProtocolsCacheOptionsView<'msg>;
  type MessageMut<'msg> = AlternateProtocolsCacheOptionsMut<'msg>;
}

impl ::std::default::Default for AlternateProtocolsCacheOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AlternateProtocolsCacheOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AlternateProtocolsCacheOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `AlternateProtocolsCacheOptionsMut`.
unsafe impl ::std::marker::Sync for AlternateProtocolsCacheOptions {}

// SAFETY:
// - `AlternateProtocolsCacheOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AlternateProtocolsCacheOptions {}

impl ::protobuf::Proxied for AlternateProtocolsCacheOptions {
  type View<'msg> = AlternateProtocolsCacheOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AlternateProtocolsCacheOptions {}

impl ::protobuf::MutProxied for AlternateProtocolsCacheOptions {
  type Mut<'msg> = AlternateProtocolsCacheOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AlternateProtocolsCacheOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AlternateProtocolsCacheOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AlternateProtocolsCacheOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AlternateProtocolsCacheOptionsView<'msg> {
  type Message = AlternateProtocolsCacheOptions;
}

impl ::std::fmt::Debug for AlternateProtocolsCacheOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AlternateProtocolsCacheOptionsView<'_> {
  fn default() -> AlternateProtocolsCacheOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AlternateProtocolsCacheOptions>> for AlternateProtocolsCacheOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AlternateProtocolsCacheOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AlternateProtocolsCacheOptionsView<'msg> {

  pub fn to_owned(&self) -> AlternateProtocolsCacheOptions {
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

  // max_entries: optional message google.protobuf.UInt32Value
  pub fn has_max_entries(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn max_entries_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_entries().then(|| self.max_entries())
  }
  pub fn max_entries(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // key_value_store_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_key_value_store_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn key_value_store_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_key_value_store_config().then(|| self.key_value_store_config())
  }
  pub fn key_value_store_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // prepopulated_entries: repeated message envoy.config.core.v3.AlternateProtocolsCacheOptions.AlternateProtocolsCacheEntry
  pub fn prepopulated_entries(self) -> ::protobuf::RepeatedView<'msg, super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // canonical_suffixes: repeated string
  pub fn canonical_suffixes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

}

// SAFETY:
// - `AlternateProtocolsCacheOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AlternateProtocolsCacheOptionsView<'_> {}

// SAFETY:
// - `AlternateProtocolsCacheOptionsView` is `Send` because while its alive a `AlternateProtocolsCacheOptionsMut` cannot.
// - `AlternateProtocolsCacheOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for AlternateProtocolsCacheOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for AlternateProtocolsCacheOptionsView<'msg> {
  type Proxied = AlternateProtocolsCacheOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, AlternateProtocolsCacheOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AlternateProtocolsCacheOptionsView<'msg> {
  fn into_view<'shorter>(self) -> AlternateProtocolsCacheOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AlternateProtocolsCacheOptions> for AlternateProtocolsCacheOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AlternateProtocolsCacheOptions {
    let mut dst = AlternateProtocolsCacheOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AlternateProtocolsCacheOptions> for AlternateProtocolsCacheOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AlternateProtocolsCacheOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AlternateProtocolsCacheOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AlternateProtocolsCacheOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AlternateProtocolsCacheOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AlternateProtocolsCacheOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AlternateProtocolsCacheOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AlternateProtocolsCacheOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AlternateProtocolsCacheOptionsMut<'msg> {
  type Message = AlternateProtocolsCacheOptions;
}

impl ::std::fmt::Debug for AlternateProtocolsCacheOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AlternateProtocolsCacheOptions>> for AlternateProtocolsCacheOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AlternateProtocolsCacheOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AlternateProtocolsCacheOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AlternateProtocolsCacheOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AlternateProtocolsCacheOptions {
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

  // max_entries: optional message google.protobuf.UInt32Value
  pub fn has_max_entries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_entries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_entries_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_entries().then(|| self.max_entries())
  }
  pub fn max_entries(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_entries_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_entries(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // key_value_store_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_key_value_store_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_key_value_store_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn key_value_store_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_key_value_store_config().then(|| self.key_value_store_config())
  }
  pub fn key_value_store_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn key_value_store_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_key_value_store_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // prepopulated_entries: repeated message envoy.config.core.v3.AlternateProtocolsCacheOptions.AlternateProtocolsCacheEntry
  pub fn prepopulated_entries(&self) -> ::protobuf::RepeatedView<'_, super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn prepopulated_entries_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry> {
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
  pub fn set_prepopulated_entries(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // canonical_suffixes: repeated string
  pub fn canonical_suffixes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn canonical_suffixes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_canonical_suffixes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}

// SAFETY:
// - `AlternateProtocolsCacheOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AlternateProtocolsCacheOptionsMut<'_> {}

// SAFETY:
// - `AlternateProtocolsCacheOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AlternateProtocolsCacheOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for AlternateProtocolsCacheOptionsMut<'msg> {
  type Proxied = AlternateProtocolsCacheOptions;
  fn as_view(&self) -> ::protobuf::View<'_, AlternateProtocolsCacheOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AlternateProtocolsCacheOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AlternateProtocolsCacheOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AlternateProtocolsCacheOptionsMut<'msg> {
  type MutProxied = AlternateProtocolsCacheOptions;
  fn as_mut(&mut self) -> AlternateProtocolsCacheOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AlternateProtocolsCacheOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> AlternateProtocolsCacheOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AlternateProtocolsCacheOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AlternateProtocolsCacheOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AlternateProtocolsCacheOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AlternateProtocolsCacheOptionsMut<'_> {
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

  // max_entries: optional message google.protobuf.UInt32Value
  pub fn has_max_entries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_entries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_entries_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_entries().then(|| self.max_entries())
  }
  pub fn max_entries(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_entries_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_entries(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // key_value_store_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_key_value_store_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_key_value_store_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn key_value_store_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_key_value_store_config().then(|| self.key_value_store_config())
  }
  pub fn key_value_store_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn key_value_store_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_key_value_store_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // prepopulated_entries: repeated message envoy.config.core.v3.AlternateProtocolsCacheOptions.AlternateProtocolsCacheEntry
  pub fn prepopulated_entries(&self) -> ::protobuf::RepeatedView<'_, super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn prepopulated_entries_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry> {
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
  pub fn set_prepopulated_entries(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // canonical_suffixes: repeated string
  pub fn canonical_suffixes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn canonical_suffixes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_canonical_suffixes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}  // impl AlternateProtocolsCacheOptions

impl ::std::ops::Drop for AlternateProtocolsCacheOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AlternateProtocolsCacheOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AlternateProtocolsCacheOptions {
  type Proxied = Self;
  fn as_view(&self) -> AlternateProtocolsCacheOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AlternateProtocolsCacheOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AlternateProtocolsCacheOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AlternateProtocolsCacheOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__AlternateProtocolsCacheOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X33GET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__AlternateProtocolsCacheOptions_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::alternate_protocols_cache_options::AlternateProtocolsCacheEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__AlternateProtocolsCacheOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AlternateProtocolsCacheOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AlternateProtocolsCacheOptions {
  type Msg = AlternateProtocolsCacheOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AlternateProtocolsCacheOptions {
  type Msg = AlternateProtocolsCacheOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AlternateProtocolsCacheOptionsMut<'_> {
  type Msg = AlternateProtocolsCacheOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AlternateProtocolsCacheOptionsMut<'_> {
  type Msg = AlternateProtocolsCacheOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AlternateProtocolsCacheOptionsView<'_> {
  type Msg = AlternateProtocolsCacheOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AlternateProtocolsCacheOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod alternate_protocols_cache_options {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__AlternateProtocolsCacheOptions__AlternateProtocolsCacheEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AlternateProtocolsCacheEntry {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AlternateProtocolsCacheEntry>
}

impl ::protobuf::Message for AlternateProtocolsCacheEntry {
  type MessageView<'msg> = AlternateProtocolsCacheEntryView<'msg>;
  type MessageMut<'msg> = AlternateProtocolsCacheEntryMut<'msg>;
}

impl ::std::default::Default for AlternateProtocolsCacheEntry {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AlternateProtocolsCacheEntry {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AlternateProtocolsCacheEntry` is `Sync` because it does not implement interior mutability.
//    Neither does `AlternateProtocolsCacheEntryMut`.
unsafe impl ::std::marker::Sync for AlternateProtocolsCacheEntry {}

// SAFETY:
// - `AlternateProtocolsCacheEntry` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AlternateProtocolsCacheEntry {}

impl ::protobuf::Proxied for AlternateProtocolsCacheEntry {
  type View<'msg> = AlternateProtocolsCacheEntryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AlternateProtocolsCacheEntry {}

impl ::protobuf::MutProxied for AlternateProtocolsCacheEntry {
  type Mut<'msg> = AlternateProtocolsCacheEntryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AlternateProtocolsCacheEntryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AlternateProtocolsCacheEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AlternateProtocolsCacheEntryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AlternateProtocolsCacheEntryView<'msg> {
  type Message = AlternateProtocolsCacheEntry;
}

impl ::std::fmt::Debug for AlternateProtocolsCacheEntryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AlternateProtocolsCacheEntryView<'_> {
  fn default() -> AlternateProtocolsCacheEntryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AlternateProtocolsCacheEntry>> for AlternateProtocolsCacheEntryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AlternateProtocolsCacheEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AlternateProtocolsCacheEntryView<'msg> {

  pub fn to_owned(&self) -> AlternateProtocolsCacheEntry {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // hostname: optional string
  pub fn hostname(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // port: optional uint32
  pub fn port(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `AlternateProtocolsCacheEntryView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AlternateProtocolsCacheEntryView<'_> {}

// SAFETY:
// - `AlternateProtocolsCacheEntryView` is `Send` because while its alive a `AlternateProtocolsCacheEntryMut` cannot.
// - `AlternateProtocolsCacheEntryView` does not use thread-local data.
unsafe impl ::std::marker::Send for AlternateProtocolsCacheEntryView<'_> {}

impl<'msg> ::protobuf::AsView for AlternateProtocolsCacheEntryView<'msg> {
  type Proxied = AlternateProtocolsCacheEntry;
  fn as_view(&self) -> ::protobuf::View<'msg, AlternateProtocolsCacheEntry> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AlternateProtocolsCacheEntryView<'msg> {
  fn into_view<'shorter>(self) -> AlternateProtocolsCacheEntryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AlternateProtocolsCacheEntry> for AlternateProtocolsCacheEntryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AlternateProtocolsCacheEntry {
    let mut dst = AlternateProtocolsCacheEntry::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AlternateProtocolsCacheEntry> for AlternateProtocolsCacheEntryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AlternateProtocolsCacheEntry {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AlternateProtocolsCacheEntry {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AlternateProtocolsCacheEntryView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AlternateProtocolsCacheEntryMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AlternateProtocolsCacheEntryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AlternateProtocolsCacheEntry>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AlternateProtocolsCacheEntryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AlternateProtocolsCacheEntryMut<'msg> {
  type Message = AlternateProtocolsCacheEntry;
}

impl ::std::fmt::Debug for AlternateProtocolsCacheEntryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AlternateProtocolsCacheEntry>> for AlternateProtocolsCacheEntryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AlternateProtocolsCacheEntry>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AlternateProtocolsCacheEntryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AlternateProtocolsCacheEntry> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AlternateProtocolsCacheEntry {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // hostname: optional string
  pub fn hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // port: optional uint32
  pub fn port(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_port(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `AlternateProtocolsCacheEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AlternateProtocolsCacheEntryMut<'_> {}

// SAFETY:
// - `AlternateProtocolsCacheEntryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AlternateProtocolsCacheEntryMut<'_> {}

impl<'msg> ::protobuf::AsView for AlternateProtocolsCacheEntryMut<'msg> {
  type Proxied = AlternateProtocolsCacheEntry;
  fn as_view(&self) -> ::protobuf::View<'_, AlternateProtocolsCacheEntry> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AlternateProtocolsCacheEntryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AlternateProtocolsCacheEntry>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AlternateProtocolsCacheEntryMut<'msg> {
  type MutProxied = AlternateProtocolsCacheEntry;
  fn as_mut(&mut self) -> AlternateProtocolsCacheEntryMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AlternateProtocolsCacheEntryMut<'msg> {
  fn into_mut<'shorter>(self) -> AlternateProtocolsCacheEntryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AlternateProtocolsCacheEntry {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AlternateProtocolsCacheEntry> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AlternateProtocolsCacheEntryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AlternateProtocolsCacheEntryMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // hostname: optional string
  pub fn hostname(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_hostname(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // port: optional uint32
  pub fn port(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_port(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

}  // impl AlternateProtocolsCacheEntry

impl ::std::ops::Drop for AlternateProtocolsCacheEntry {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AlternateProtocolsCacheEntry {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AlternateProtocolsCacheEntry {
  type Proxied = Self;
  fn as_view(&self) -> AlternateProtocolsCacheEntryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AlternateProtocolsCacheEntry {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AlternateProtocolsCacheEntryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AlternateProtocolsCacheEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::alternate_protocols_cache_options::envoy__config__core__v3__AlternateProtocolsCacheOptions__AlternateProtocolsCacheEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::alternate_protocols_cache_options::envoy__config__core__v3__AlternateProtocolsCacheOptions__AlternateProtocolsCacheEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::alternate_protocols_cache_options::envoy__config__core__v3__AlternateProtocolsCacheOptions__AlternateProtocolsCacheEntry_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AlternateProtocolsCacheEntry {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AlternateProtocolsCacheEntry {
  type Msg = AlternateProtocolsCacheEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AlternateProtocolsCacheEntry {
  type Msg = AlternateProtocolsCacheEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AlternateProtocolsCacheEntryMut<'_> {
  type Msg = AlternateProtocolsCacheEntry;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheEntry> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AlternateProtocolsCacheEntryMut<'_> {
  type Msg = AlternateProtocolsCacheEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheEntry> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AlternateProtocolsCacheEntryView<'_> {
  type Msg = AlternateProtocolsCacheEntry;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AlternateProtocolsCacheEntry> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AlternateProtocolsCacheEntryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod alternate_protocols_cache_options


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HttpProtocolOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpProtocolOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpProtocolOptions>
}

impl ::protobuf::Message for HttpProtocolOptions {
  type MessageView<'msg> = HttpProtocolOptionsView<'msg>;
  type MessageMut<'msg> = HttpProtocolOptionsMut<'msg>;
}

impl ::std::default::Default for HttpProtocolOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpProtocolOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpProtocolOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpProtocolOptionsMut`.
unsafe impl ::std::marker::Sync for HttpProtocolOptions {}

// SAFETY:
// - `HttpProtocolOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpProtocolOptions {}

impl ::protobuf::Proxied for HttpProtocolOptions {
  type View<'msg> = HttpProtocolOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpProtocolOptions {}

impl ::protobuf::MutProxied for HttpProtocolOptions {
  type Mut<'msg> = HttpProtocolOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpProtocolOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpProtocolOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpProtocolOptionsView<'msg> {
  type Message = HttpProtocolOptions;
}

impl ::std::fmt::Debug for HttpProtocolOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpProtocolOptionsView<'_> {
  fn default() -> HttpProtocolOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpProtocolOptions>> for HttpProtocolOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpProtocolOptionsView<'msg> {

  pub fn to_owned(&self) -> HttpProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // idle_timeout: optional message google.protobuf.Duration
  pub fn has_idle_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn idle_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_idle_timeout().then(|| self.idle_timeout())
  }
  pub fn idle_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // max_connection_duration: optional message google.protobuf.Duration
  pub fn has_max_connection_duration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn max_connection_duration_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_connection_duration().then(|| self.max_connection_duration())
  }
  pub fn max_connection_duration(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // max_headers_count: optional message google.protobuf.UInt32Value
  pub fn has_max_headers_count(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn max_headers_count_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_headers_count().then(|| self.max_headers_count())
  }
  pub fn max_headers_count(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_response_headers_kb: optional message google.protobuf.UInt32Value
  pub fn has_max_response_headers_kb(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn max_response_headers_kb_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_response_headers_kb().then(|| self.max_response_headers_kb())
  }
  pub fn max_response_headers_kb(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_stream_duration: optional message google.protobuf.Duration
  pub fn has_max_stream_duration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn max_stream_duration_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_stream_duration().then(|| self.max_stream_duration())
  }
  pub fn max_stream_duration(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // headers_with_underscores_action: optional enum envoy.config.core.v3.HttpProtocolOptions.HeadersWithUnderscoresAction
  pub fn headers_with_underscores_action(self) -> super::http_protocol_options::HeadersWithUnderscoresAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::http_protocol_options::HeadersWithUnderscoresAction::Allow).into()
      ).try_into().unwrap()
    }
  }

  // max_requests_per_connection: optional message google.protobuf.UInt32Value
  pub fn has_max_requests_per_connection(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn max_requests_per_connection_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_requests_per_connection().then(|| self.max_requests_per_connection())
  }
  pub fn max_requests_per_connection(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `HttpProtocolOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpProtocolOptionsView<'_> {}

// SAFETY:
// - `HttpProtocolOptionsView` is `Send` because while its alive a `HttpProtocolOptionsMut` cannot.
// - `HttpProtocolOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpProtocolOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for HttpProtocolOptionsView<'msg> {
  type Proxied = HttpProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpProtocolOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpProtocolOptionsView<'msg> {
  fn into_view<'shorter>(self) -> HttpProtocolOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpProtocolOptions> for HttpProtocolOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpProtocolOptions {
    let mut dst = HttpProtocolOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpProtocolOptions> for HttpProtocolOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpProtocolOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpProtocolOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpProtocolOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpProtocolOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpProtocolOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpProtocolOptionsMut<'msg> {
  type Message = HttpProtocolOptions;
}

impl ::std::fmt::Debug for HttpProtocolOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpProtocolOptions>> for HttpProtocolOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpProtocolOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpProtocolOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpProtocolOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // idle_timeout: optional message google.protobuf.Duration
  pub fn has_idle_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_idle_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn idle_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_idle_timeout().then(|| self.idle_timeout())
  }
  pub fn idle_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn idle_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_idle_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_connection_duration: optional message google.protobuf.Duration
  pub fn has_max_connection_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_connection_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_connection_duration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_connection_duration().then(|| self.max_connection_duration())
  }
  pub fn max_connection_duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_connection_duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_connection_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // max_headers_count: optional message google.protobuf.UInt32Value
  pub fn has_max_headers_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_headers_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_headers_count_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_headers_count().then(|| self.max_headers_count())
  }
  pub fn max_headers_count(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_headers_count_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_headers_count(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // max_response_headers_kb: optional message google.protobuf.UInt32Value
  pub fn has_max_response_headers_kb(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_max_response_headers_kb(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn max_response_headers_kb_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_response_headers_kb().then(|| self.max_response_headers_kb())
  }
  pub fn max_response_headers_kb(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_response_headers_kb_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_response_headers_kb(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // max_stream_duration: optional message google.protobuf.Duration
  pub fn has_max_stream_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_max_stream_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn max_stream_duration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_stream_duration().then(|| self.max_stream_duration())
  }
  pub fn max_stream_duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_stream_duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_stream_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // headers_with_underscores_action: optional enum envoy.config.core.v3.HttpProtocolOptions.HeadersWithUnderscoresAction
  pub fn headers_with_underscores_action(&self) -> super::http_protocol_options::HeadersWithUnderscoresAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::http_protocol_options::HeadersWithUnderscoresAction::Allow).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_headers_with_underscores_action(&mut self, val: super::http_protocol_options::HeadersWithUnderscoresAction) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // max_requests_per_connection: optional message google.protobuf.UInt32Value
  pub fn has_max_requests_per_connection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_max_requests_per_connection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn max_requests_per_connection_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_requests_per_connection().then(|| self.max_requests_per_connection())
  }
  pub fn max_requests_per_connection(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_requests_per_connection_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_requests_per_connection(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}

// SAFETY:
// - `HttpProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpProtocolOptionsMut<'_> {}

// SAFETY:
// - `HttpProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpProtocolOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpProtocolOptionsMut<'msg> {
  type Proxied = HttpProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'_, HttpProtocolOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpProtocolOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpProtocolOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpProtocolOptionsMut<'msg> {
  type MutProxied = HttpProtocolOptions;
  fn as_mut(&mut self) -> HttpProtocolOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpProtocolOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpProtocolOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpProtocolOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpProtocolOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpProtocolOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpProtocolOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // idle_timeout: optional message google.protobuf.Duration
  pub fn has_idle_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_idle_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn idle_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_idle_timeout().then(|| self.idle_timeout())
  }
  pub fn idle_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn idle_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_idle_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_connection_duration: optional message google.protobuf.Duration
  pub fn has_max_connection_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_max_connection_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn max_connection_duration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_connection_duration().then(|| self.max_connection_duration())
  }
  pub fn max_connection_duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_connection_duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_connection_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // max_headers_count: optional message google.protobuf.UInt32Value
  pub fn has_max_headers_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_headers_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_headers_count_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_headers_count().then(|| self.max_headers_count())
  }
  pub fn max_headers_count(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_headers_count_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_headers_count(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // max_response_headers_kb: optional message google.protobuf.UInt32Value
  pub fn has_max_response_headers_kb(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_max_response_headers_kb(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn max_response_headers_kb_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_response_headers_kb().then(|| self.max_response_headers_kb())
  }
  pub fn max_response_headers_kb(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_response_headers_kb_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_response_headers_kb(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // max_stream_duration: optional message google.protobuf.Duration
  pub fn has_max_stream_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_max_stream_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn max_stream_duration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_stream_duration().then(|| self.max_stream_duration())
  }
  pub fn max_stream_duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_stream_duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_stream_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // headers_with_underscores_action: optional enum envoy.config.core.v3.HttpProtocolOptions.HeadersWithUnderscoresAction
  pub fn headers_with_underscores_action(&self) -> super::http_protocol_options::HeadersWithUnderscoresAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::http_protocol_options::HeadersWithUnderscoresAction::Allow).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_headers_with_underscores_action(&mut self, val: super::http_protocol_options::HeadersWithUnderscoresAction) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // max_requests_per_connection: optional message google.protobuf.UInt32Value
  pub fn has_max_requests_per_connection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_max_requests_per_connection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn max_requests_per_connection_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_requests_per_connection().then(|| self.max_requests_per_connection())
  }
  pub fn max_requests_per_connection(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_requests_per_connection_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_requests_per_connection(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}  // impl HttpProtocolOptions

impl ::std::ops::Drop for HttpProtocolOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpProtocolOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpProtocolOptions {
  type Proxied = Self;
  fn as_view(&self) -> HttpProtocolOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpProtocolOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpProtocolOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpProtocolOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__HttpProtocolOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333.P33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__HttpProtocolOptions_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__HttpProtocolOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpProtocolOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpProtocolOptions {
  type Msg = HttpProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpProtocolOptions {
  type Msg = HttpProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpProtocolOptionsMut<'_> {
  type Msg = HttpProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpProtocolOptionsMut<'_> {
  type Msg = HttpProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpProtocolOptionsView<'_> {
  type Msg = HttpProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpProtocolOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpProtocolOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http_protocol_options {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HeadersWithUnderscoresAction(i32);

#[allow(non_upper_case_globals)]
impl HeadersWithUnderscoresAction {
  pub const Allow: HeadersWithUnderscoresAction = HeadersWithUnderscoresAction(0);
  pub const RejectRequest: HeadersWithUnderscoresAction = HeadersWithUnderscoresAction(1);
  pub const DropHeader: HeadersWithUnderscoresAction = HeadersWithUnderscoresAction(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Allow",
      1 => "RejectRequest",
      2 => "DropHeader",
      _ => return None
    })
  }
}

impl ::std::convert::From<HeadersWithUnderscoresAction> for i32 {
  fn from(val: HeadersWithUnderscoresAction) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for HeadersWithUnderscoresAction {
  fn from(val: i32) -> HeadersWithUnderscoresAction {
    Self(val)
  }
}

impl ::std::default::Default for HeadersWithUnderscoresAction {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for HeadersWithUnderscoresAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "HeadersWithUnderscoresAction::{}", constant_name)
    } else {
      write!(f, "HeadersWithUnderscoresAction::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for HeadersWithUnderscoresAction {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for HeadersWithUnderscoresAction {}

impl ::protobuf::Proxied for HeadersWithUnderscoresAction {
  type View<'a> = HeadersWithUnderscoresAction;
}

impl ::protobuf::AsView for HeadersWithUnderscoresAction {
  type Proxied = HeadersWithUnderscoresAction;

  fn as_view(&self) -> HeadersWithUnderscoresAction {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeadersWithUnderscoresAction {
  fn into_view<'shorter>(self) -> HeadersWithUnderscoresAction where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for HeadersWithUnderscoresAction {
  const NAME: &'static str = "HeadersWithUnderscoresAction";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for HeadersWithUnderscoresAction {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod http_protocol_options


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Http1ProtocolOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Http1ProtocolOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Http1ProtocolOptions>
}

impl ::protobuf::Message for Http1ProtocolOptions {
  type MessageView<'msg> = Http1ProtocolOptionsView<'msg>;
  type MessageMut<'msg> = Http1ProtocolOptionsMut<'msg>;
}

impl ::std::default::Default for Http1ProtocolOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Http1ProtocolOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Http1ProtocolOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `Http1ProtocolOptionsMut`.
unsafe impl ::std::marker::Sync for Http1ProtocolOptions {}

// SAFETY:
// - `Http1ProtocolOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Http1ProtocolOptions {}

impl ::protobuf::Proxied for Http1ProtocolOptions {
  type View<'msg> = Http1ProtocolOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Http1ProtocolOptions {}

impl ::protobuf::MutProxied for Http1ProtocolOptions {
  type Mut<'msg> = Http1ProtocolOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Http1ProtocolOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http1ProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Http1ProtocolOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Http1ProtocolOptionsView<'msg> {
  type Message = Http1ProtocolOptions;
}

impl ::std::fmt::Debug for Http1ProtocolOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Http1ProtocolOptionsView<'_> {
  fn default() -> Http1ProtocolOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Http1ProtocolOptions>> for Http1ProtocolOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http1ProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Http1ProtocolOptionsView<'msg> {

  pub fn to_owned(&self) -> Http1ProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // allow_absolute_url: optional message google.protobuf.BoolValue
  pub fn has_allow_absolute_url(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn allow_absolute_url_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_allow_absolute_url().then(|| self.allow_absolute_url())
  }
  pub fn allow_absolute_url(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // accept_http_10: optional bool
  pub fn accept_http_10(self) -> bool {
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

  // default_host_for_http_10: optional string
  pub fn default_host_for_http_10(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // header_key_format: optional message envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat
  pub fn has_header_key_format(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn header_key_format_opt(self) -> ::std::option::Option<super::http1_protocol_options::HeaderKeyFormatView<'msg>> {
    self.has_header_key_format().then(|| self.header_key_format())
  }
  pub fn header_key_format(self) -> super::http1_protocol_options::HeaderKeyFormatView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http1_protocol_options::HeaderKeyFormatView::default())
  }

  // enable_trailers: optional bool
  pub fn enable_trailers(self) -> bool {
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

  // allow_chunked_length: optional bool
  pub fn allow_chunked_length(self) -> bool {
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

  // override_stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_override_stream_error_on_invalid_http_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn override_stream_error_on_invalid_http_message_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_override_stream_error_on_invalid_http_message().then(|| self.override_stream_error_on_invalid_http_message())
  }
  pub fn override_stream_error_on_invalid_http_message(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // send_fully_qualified_url: optional bool
  pub fn send_fully_qualified_url(self) -> bool {
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

  // use_balsa_parser: optional message google.protobuf.BoolValue
  pub fn has_use_balsa_parser(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn use_balsa_parser_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_use_balsa_parser().then(|| self.use_balsa_parser())
  }
  pub fn use_balsa_parser(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // allow_custom_methods: optional bool
  pub fn allow_custom_methods(self) -> bool {
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

  // ignore_http_11_upgrade: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn ignore_http_11_upgrade(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `Http1ProtocolOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Http1ProtocolOptionsView<'_> {}

// SAFETY:
// - `Http1ProtocolOptionsView` is `Send` because while its alive a `Http1ProtocolOptionsMut` cannot.
// - `Http1ProtocolOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for Http1ProtocolOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for Http1ProtocolOptionsView<'msg> {
  type Proxied = Http1ProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, Http1ProtocolOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Http1ProtocolOptionsView<'msg> {
  fn into_view<'shorter>(self) -> Http1ProtocolOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Http1ProtocolOptions> for Http1ProtocolOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http1ProtocolOptions {
    let mut dst = Http1ProtocolOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Http1ProtocolOptions> for Http1ProtocolOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http1ProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Http1ProtocolOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Http1ProtocolOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Http1ProtocolOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Http1ProtocolOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http1ProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Http1ProtocolOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Http1ProtocolOptionsMut<'msg> {
  type Message = Http1ProtocolOptions;
}

impl ::std::fmt::Debug for Http1ProtocolOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Http1ProtocolOptions>> for Http1ProtocolOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http1ProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Http1ProtocolOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Http1ProtocolOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Http1ProtocolOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // allow_absolute_url: optional message google.protobuf.BoolValue
  pub fn has_allow_absolute_url(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_allow_absolute_url(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn allow_absolute_url_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_allow_absolute_url().then(|| self.allow_absolute_url())
  }
  pub fn allow_absolute_url(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn allow_absolute_url_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_allow_absolute_url(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // accept_http_10: optional bool
  pub fn accept_http_10(&self) -> bool {
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
  pub fn set_accept_http_10(&mut self, val: bool) {
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

  // default_host_for_http_10: optional string
  pub fn default_host_for_http_10(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_host_for_http_10(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // header_key_format: optional message envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat
  pub fn has_header_key_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_header_key_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn header_key_format_opt(&self) -> ::std::option::Option<super::http1_protocol_options::HeaderKeyFormatView<'_>> {
    self.has_header_key_format().then(|| self.header_key_format())
  }
  pub fn header_key_format(&self) -> super::http1_protocol_options::HeaderKeyFormatView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http1_protocol_options::HeaderKeyFormatView::default())
  }
  pub fn header_key_format_mut(&mut self) -> super::http1_protocol_options::HeaderKeyFormatMut<'_> {
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
  pub fn set_header_key_format(&mut self,
    val: impl ::protobuf::IntoProxied<super::http1_protocol_options::HeaderKeyFormat>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // enable_trailers: optional bool
  pub fn enable_trailers(&self) -> bool {
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
  pub fn set_enable_trailers(&mut self, val: bool) {
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

  // allow_chunked_length: optional bool
  pub fn allow_chunked_length(&self) -> bool {
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
  pub fn set_allow_chunked_length(&mut self, val: bool) {
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

  // override_stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_override_stream_error_on_invalid_http_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_override_stream_error_on_invalid_http_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn override_stream_error_on_invalid_http_message_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_override_stream_error_on_invalid_http_message().then(|| self.override_stream_error_on_invalid_http_message())
  }
  pub fn override_stream_error_on_invalid_http_message(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn override_stream_error_on_invalid_http_message_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_override_stream_error_on_invalid_http_message(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // send_fully_qualified_url: optional bool
  pub fn send_fully_qualified_url(&self) -> bool {
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
  pub fn set_send_fully_qualified_url(&mut self, val: bool) {
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

  // use_balsa_parser: optional message google.protobuf.BoolValue
  pub fn has_use_balsa_parser(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_use_balsa_parser(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn use_balsa_parser_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_balsa_parser().then(|| self.use_balsa_parser())
  }
  pub fn use_balsa_parser(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_balsa_parser_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_balsa_parser(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // allow_custom_methods: optional bool
  pub fn allow_custom_methods(&self) -> bool {
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
  pub fn set_allow_custom_methods(&mut self, val: bool) {
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

  // ignore_http_11_upgrade: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn ignore_http_11_upgrade(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ignore_http_11_upgrade_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        10,
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
  pub fn set_ignore_http_11_upgrade(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

}

// SAFETY:
// - `Http1ProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Http1ProtocolOptionsMut<'_> {}

// SAFETY:
// - `Http1ProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Http1ProtocolOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for Http1ProtocolOptionsMut<'msg> {
  type Proxied = Http1ProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'_, Http1ProtocolOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Http1ProtocolOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Http1ProtocolOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Http1ProtocolOptionsMut<'msg> {
  type MutProxied = Http1ProtocolOptions;
  fn as_mut(&mut self) -> Http1ProtocolOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Http1ProtocolOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> Http1ProtocolOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Http1ProtocolOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Http1ProtocolOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Http1ProtocolOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Http1ProtocolOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // allow_absolute_url: optional message google.protobuf.BoolValue
  pub fn has_allow_absolute_url(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_allow_absolute_url(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn allow_absolute_url_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_allow_absolute_url().then(|| self.allow_absolute_url())
  }
  pub fn allow_absolute_url(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn allow_absolute_url_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_allow_absolute_url(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // accept_http_10: optional bool
  pub fn accept_http_10(&self) -> bool {
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
  pub fn set_accept_http_10(&mut self, val: bool) {
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

  // default_host_for_http_10: optional string
  pub fn default_host_for_http_10(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_host_for_http_10(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // header_key_format: optional message envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat
  pub fn has_header_key_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_header_key_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn header_key_format_opt(&self) -> ::std::option::Option<super::http1_protocol_options::HeaderKeyFormatView<'_>> {
    self.has_header_key_format().then(|| self.header_key_format())
  }
  pub fn header_key_format(&self) -> super::http1_protocol_options::HeaderKeyFormatView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::http1_protocol_options::HeaderKeyFormatView::default())
  }
  pub fn header_key_format_mut(&mut self) -> super::http1_protocol_options::HeaderKeyFormatMut<'_> {
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
  pub fn set_header_key_format(&mut self,
    val: impl ::protobuf::IntoProxied<super::http1_protocol_options::HeaderKeyFormat>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // enable_trailers: optional bool
  pub fn enable_trailers(&self) -> bool {
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
  pub fn set_enable_trailers(&mut self, val: bool) {
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

  // allow_chunked_length: optional bool
  pub fn allow_chunked_length(&self) -> bool {
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
  pub fn set_allow_chunked_length(&mut self, val: bool) {
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

  // override_stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_override_stream_error_on_invalid_http_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_override_stream_error_on_invalid_http_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn override_stream_error_on_invalid_http_message_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_override_stream_error_on_invalid_http_message().then(|| self.override_stream_error_on_invalid_http_message())
  }
  pub fn override_stream_error_on_invalid_http_message(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn override_stream_error_on_invalid_http_message_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_override_stream_error_on_invalid_http_message(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // send_fully_qualified_url: optional bool
  pub fn send_fully_qualified_url(&self) -> bool {
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
  pub fn set_send_fully_qualified_url(&mut self, val: bool) {
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

  // use_balsa_parser: optional message google.protobuf.BoolValue
  pub fn has_use_balsa_parser(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_use_balsa_parser(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn use_balsa_parser_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_balsa_parser().then(|| self.use_balsa_parser())
  }
  pub fn use_balsa_parser(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_balsa_parser_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_balsa_parser(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // allow_custom_methods: optional bool
  pub fn allow_custom_methods(&self) -> bool {
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
  pub fn set_allow_custom_methods(&mut self, val: bool) {
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

  // ignore_http_11_upgrade: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn ignore_http_11_upgrade(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ignore_http_11_upgrade_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        10,
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
  pub fn set_ignore_http_11_upgrade(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

}  // impl Http1ProtocolOptions

impl ::std::ops::Drop for Http1ProtocolOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Http1ProtocolOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Http1ProtocolOptions {
  type Proxied = Self;
  fn as_view(&self) -> Http1ProtocolOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Http1ProtocolOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Http1ProtocolOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Http1ProtocolOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__Http1ProtocolOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3/P1X3/P/P3/P3/PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__Http1ProtocolOptions_msg_init.0, &[<::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http1_protocol_options::HeaderKeyFormat as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__Http1ProtocolOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http1ProtocolOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http1ProtocolOptions {
  type Msg = Http1ProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http1ProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http1ProtocolOptions {
  type Msg = Http1ProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http1ProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http1ProtocolOptionsMut<'_> {
  type Msg = Http1ProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http1ProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http1ProtocolOptionsMut<'_> {
  type Msg = Http1ProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http1ProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http1ProtocolOptionsView<'_> {
  type Msg = Http1ProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http1ProtocolOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http1ProtocolOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http1_protocol_options {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Http1ProtocolOptions__HeaderKeyFormat_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderKeyFormat {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderKeyFormat>
}

impl ::protobuf::Message for HeaderKeyFormat {
  type MessageView<'msg> = HeaderKeyFormatView<'msg>;
  type MessageMut<'msg> = HeaderKeyFormatMut<'msg>;
}

impl ::std::default::Default for HeaderKeyFormat {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderKeyFormat {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderKeyFormat` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderKeyFormatMut`.
unsafe impl ::std::marker::Sync for HeaderKeyFormat {}

// SAFETY:
// - `HeaderKeyFormat` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderKeyFormat {}

impl ::protobuf::Proxied for HeaderKeyFormat {
  type View<'msg> = HeaderKeyFormatView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderKeyFormat {}

impl ::protobuf::MutProxied for HeaderKeyFormat {
  type Mut<'msg> = HeaderKeyFormatMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderKeyFormatView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderKeyFormat>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderKeyFormatView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderKeyFormatView<'msg> {
  type Message = HeaderKeyFormat;
}

impl ::std::fmt::Debug for HeaderKeyFormatView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderKeyFormatView<'_> {
  fn default() -> HeaderKeyFormatView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderKeyFormat>> for HeaderKeyFormatView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderKeyFormat>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderKeyFormatView<'msg> {

  pub fn to_owned(&self) -> HeaderKeyFormat {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // proper_case_words: optional message envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat.ProperCaseWords
  pub fn has_proper_case_words(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn proper_case_words_opt(self) -> ::std::option::Option<super::super::http1_protocol_options::header_key_format::ProperCaseWordsView<'msg>> {
    self.has_proper_case_words().then(|| self.proper_case_words())
  }
  pub fn proper_case_words(self) -> super::super::http1_protocol_options::header_key_format::ProperCaseWordsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::http1_protocol_options::header_key_format::ProperCaseWordsView::default())
  }

  // stateful_formatter: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_stateful_formatter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn stateful_formatter_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_stateful_formatter().then(|| self.stateful_formatter())
  }
  pub fn stateful_formatter(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  pub fn header_format(self) -> super::super::http1_protocol_options::header_key_format::HeaderFormatOneof<'msg> {
    match self.header_format_case() {
      super::super::http1_protocol_options::header_key_format::HeaderFormatCase::ProperCaseWords =>
          super::super::http1_protocol_options::header_key_format::HeaderFormatOneof::ProperCaseWords(self.proper_case_words()),
      super::super::http1_protocol_options::header_key_format::HeaderFormatCase::StatefulFormatter =>
          super::super::http1_protocol_options::header_key_format::HeaderFormatOneof::StatefulFormatter(self.stateful_formatter()),
      _ => super::super::http1_protocol_options::header_key_format::HeaderFormatOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn header_format_case(self) -> super::super::http1_protocol_options::header_key_format::HeaderFormatCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::http1_protocol_options::header_key_format::HeaderFormatCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HeaderKeyFormatView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderKeyFormatView<'_> {}

// SAFETY:
// - `HeaderKeyFormatView` is `Send` because while its alive a `HeaderKeyFormatMut` cannot.
// - `HeaderKeyFormatView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderKeyFormatView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderKeyFormatView<'msg> {
  type Proxied = HeaderKeyFormat;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderKeyFormat> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderKeyFormatView<'msg> {
  fn into_view<'shorter>(self) -> HeaderKeyFormatView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderKeyFormat> for HeaderKeyFormatView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderKeyFormat {
    let mut dst = HeaderKeyFormat::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderKeyFormat> for HeaderKeyFormatMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderKeyFormat {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderKeyFormat {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderKeyFormatView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderKeyFormatMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderKeyFormatMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderKeyFormat>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderKeyFormatMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderKeyFormatMut<'msg> {
  type Message = HeaderKeyFormat;
}

impl ::std::fmt::Debug for HeaderKeyFormatMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderKeyFormat>> for HeaderKeyFormatMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderKeyFormat>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderKeyFormatMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderKeyFormat> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderKeyFormat {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // proper_case_words: optional message envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat.ProperCaseWords
  pub fn has_proper_case_words(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_proper_case_words(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn proper_case_words_opt(&self) -> ::std::option::Option<super::super::http1_protocol_options::header_key_format::ProperCaseWordsView<'_>> {
    self.has_proper_case_words().then(|| self.proper_case_words())
  }
  pub fn proper_case_words(&self) -> super::super::http1_protocol_options::header_key_format::ProperCaseWordsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::http1_protocol_options::header_key_format::ProperCaseWordsView::default())
  }
  pub fn proper_case_words_mut(&mut self) -> super::super::http1_protocol_options::header_key_format::ProperCaseWordsMut<'_> {
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
  pub fn set_proper_case_words(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::http1_protocol_options::header_key_format::ProperCaseWords>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // stateful_formatter: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_stateful_formatter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stateful_formatter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stateful_formatter_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_stateful_formatter().then(|| self.stateful_formatter())
  }
  pub fn stateful_formatter(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn stateful_formatter_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_stateful_formatter(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn header_format(&self) -> super::super::http1_protocol_options::header_key_format::HeaderFormatOneof<'_> {
    match &self.header_format_case() {
      super::super::http1_protocol_options::header_key_format::HeaderFormatCase::ProperCaseWords =>
          super::super::http1_protocol_options::header_key_format::HeaderFormatOneof::ProperCaseWords(self.proper_case_words()),
      super::super::http1_protocol_options::header_key_format::HeaderFormatCase::StatefulFormatter =>
          super::super::http1_protocol_options::header_key_format::HeaderFormatOneof::StatefulFormatter(self.stateful_formatter()),
      _ => super::super::http1_protocol_options::header_key_format::HeaderFormatOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn header_format_case(&self) -> super::super::http1_protocol_options::header_key_format::HeaderFormatCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::http1_protocol_options::header_key_format::HeaderFormatCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HeaderKeyFormatMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderKeyFormatMut<'_> {}

// SAFETY:
// - `HeaderKeyFormatMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderKeyFormatMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderKeyFormatMut<'msg> {
  type Proxied = HeaderKeyFormat;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderKeyFormat> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderKeyFormatMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderKeyFormat>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderKeyFormatMut<'msg> {
  type MutProxied = HeaderKeyFormat;
  fn as_mut(&mut self) -> HeaderKeyFormatMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderKeyFormatMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderKeyFormatMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderKeyFormat {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderKeyFormat> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderKeyFormatView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderKeyFormatMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // proper_case_words: optional message envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat.ProperCaseWords
  pub fn has_proper_case_words(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_proper_case_words(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn proper_case_words_opt(&self) -> ::std::option::Option<super::super::http1_protocol_options::header_key_format::ProperCaseWordsView<'_>> {
    self.has_proper_case_words().then(|| self.proper_case_words())
  }
  pub fn proper_case_words(&self) -> super::super::http1_protocol_options::header_key_format::ProperCaseWordsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::http1_protocol_options::header_key_format::ProperCaseWordsView::default())
  }
  pub fn proper_case_words_mut(&mut self) -> super::super::http1_protocol_options::header_key_format::ProperCaseWordsMut<'_> {
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
  pub fn set_proper_case_words(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::http1_protocol_options::header_key_format::ProperCaseWords>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // stateful_formatter: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_stateful_formatter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stateful_formatter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stateful_formatter_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_stateful_formatter().then(|| self.stateful_formatter())
  }
  pub fn stateful_formatter(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn stateful_formatter_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_stateful_formatter(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn header_format(&self) -> super::super::http1_protocol_options::header_key_format::HeaderFormatOneof<'_> {
    match &self.header_format_case() {
      super::super::http1_protocol_options::header_key_format::HeaderFormatCase::ProperCaseWords =>
          super::super::http1_protocol_options::header_key_format::HeaderFormatOneof::ProperCaseWords(self.proper_case_words()),
      super::super::http1_protocol_options::header_key_format::HeaderFormatCase::StatefulFormatter =>
          super::super::http1_protocol_options::header_key_format::HeaderFormatOneof::StatefulFormatter(self.stateful_formatter()),
      _ => super::super::http1_protocol_options::header_key_format::HeaderFormatOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn header_format_case(&self) -> super::super::http1_protocol_options::header_key_format::HeaderFormatCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::http1_protocol_options::header_key_format::HeaderFormatCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl HeaderKeyFormat

impl ::std::ops::Drop for HeaderKeyFormat {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderKeyFormat {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderKeyFormat {
  type Proxied = Self;
  fn as_view(&self) -> HeaderKeyFormatView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderKeyFormat {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderKeyFormatMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderKeyFormat {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http1_protocol_options::envoy__config__core__v3__Http1ProtocolOptions__HeaderKeyFormat_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3f3^!|*");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http1_protocol_options::envoy__config__core__v3__Http1ProtocolOptions__HeaderKeyFormat_msg_init.0, &[<super::super::http1_protocol_options::header_key_format::ProperCaseWords as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http1_protocol_options::envoy__config__core__v3__Http1ProtocolOptions__HeaderKeyFormat_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderKeyFormat {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderKeyFormat {
  type Msg = HeaderKeyFormat;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderKeyFormat> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderKeyFormat {
  type Msg = HeaderKeyFormat;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderKeyFormat> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderKeyFormatMut<'_> {
  type Msg = HeaderKeyFormat;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderKeyFormat> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderKeyFormatMut<'_> {
  type Msg = HeaderKeyFormat;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderKeyFormat> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderKeyFormatView<'_> {
  type Msg = HeaderKeyFormat;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderKeyFormat> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderKeyFormatMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod header_key_format {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Http1ProtocolOptions__HeaderKeyFormat__ProperCaseWords_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ProperCaseWords {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ProperCaseWords>
}

impl ::protobuf::Message for ProperCaseWords {
  type MessageView<'msg> = ProperCaseWordsView<'msg>;
  type MessageMut<'msg> = ProperCaseWordsMut<'msg>;
}

impl ::std::default::Default for ProperCaseWords {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ProperCaseWords {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ProperCaseWords` is `Sync` because it does not implement interior mutability.
//    Neither does `ProperCaseWordsMut`.
unsafe impl ::std::marker::Sync for ProperCaseWords {}

// SAFETY:
// - `ProperCaseWords` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ProperCaseWords {}

impl ::protobuf::Proxied for ProperCaseWords {
  type View<'msg> = ProperCaseWordsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ProperCaseWords {}

impl ::protobuf::MutProxied for ProperCaseWords {
  type Mut<'msg> = ProperCaseWordsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProperCaseWordsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProperCaseWords>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProperCaseWordsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ProperCaseWordsView<'msg> {
  type Message = ProperCaseWords;
}

impl ::std::fmt::Debug for ProperCaseWordsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ProperCaseWordsView<'_> {
  fn default() -> ProperCaseWordsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ProperCaseWords>> for ProperCaseWordsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ProperCaseWords>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProperCaseWordsView<'msg> {

  pub fn to_owned(&self) -> ProperCaseWords {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `ProperCaseWordsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ProperCaseWordsView<'_> {}

// SAFETY:
// - `ProperCaseWordsView` is `Send` because while its alive a `ProperCaseWordsMut` cannot.
// - `ProperCaseWordsView` does not use thread-local data.
unsafe impl ::std::marker::Send for ProperCaseWordsView<'_> {}

impl<'msg> ::protobuf::AsView for ProperCaseWordsView<'msg> {
  type Proxied = ProperCaseWords;
  fn as_view(&self) -> ::protobuf::View<'msg, ProperCaseWords> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProperCaseWordsView<'msg> {
  fn into_view<'shorter>(self) -> ProperCaseWordsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ProperCaseWords> for ProperCaseWordsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProperCaseWords {
    let mut dst = ProperCaseWords::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ProperCaseWords> for ProperCaseWordsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ProperCaseWords {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ProperCaseWords {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProperCaseWordsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ProperCaseWordsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProperCaseWordsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProperCaseWords>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ProperCaseWordsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ProperCaseWordsMut<'msg> {
  type Message = ProperCaseWords;
}

impl ::std::fmt::Debug for ProperCaseWordsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ProperCaseWords>> for ProperCaseWordsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ProperCaseWords>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ProperCaseWordsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ProperCaseWords> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ProperCaseWords {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `ProperCaseWordsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ProperCaseWordsMut<'_> {}

// SAFETY:
// - `ProperCaseWordsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ProperCaseWordsMut<'_> {}

impl<'msg> ::protobuf::AsView for ProperCaseWordsMut<'msg> {
  type Proxied = ProperCaseWords;
  fn as_view(&self) -> ::protobuf::View<'_, ProperCaseWords> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ProperCaseWordsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ProperCaseWords>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ProperCaseWordsMut<'msg> {
  type MutProxied = ProperCaseWords;
  fn as_mut(&mut self) -> ProperCaseWordsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ProperCaseWordsMut<'msg> {
  fn into_mut<'shorter>(self) -> ProperCaseWordsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ProperCaseWords {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ProperCaseWords> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ProperCaseWordsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ProperCaseWordsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl ProperCaseWords

impl ::std::ops::Drop for ProperCaseWords {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ProperCaseWords {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ProperCaseWords {
  type Proxied = Self;
  fn as_view(&self) -> ProperCaseWordsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ProperCaseWords {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ProperCaseWordsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ProperCaseWords {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::http1_protocol_options::header_key_format::envoy__config__core__v3__Http1ProtocolOptions__HeaderKeyFormat__ProperCaseWords_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::http1_protocol_options::header_key_format::envoy__config__core__v3__Http1ProtocolOptions__HeaderKeyFormat__ProperCaseWords_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::http1_protocol_options::header_key_format::envoy__config__core__v3__Http1ProtocolOptions__HeaderKeyFormat__ProperCaseWords_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProperCaseWords {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProperCaseWords {
  type Msg = ProperCaseWords;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProperCaseWords> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProperCaseWords {
  type Msg = ProperCaseWords;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProperCaseWords> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ProperCaseWordsMut<'_> {
  type Msg = ProperCaseWords;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProperCaseWords> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProperCaseWordsMut<'_> {
  type Msg = ProperCaseWords;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProperCaseWords> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ProperCaseWordsView<'_> {
  type Msg = ProperCaseWords;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ProperCaseWords> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ProperCaseWordsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum HeaderFormatOneof<'msg> {
  ProperCaseWords(::protobuf::View<'msg, super::super::super::http1_protocol_options::header_key_format::ProperCaseWords>) = 1,
  StatefulFormatter(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 8,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum HeaderFormatCase {
  ProperCaseWords = 1,
  StatefulFormatter = 8,

  not_set = 0
}

impl HeaderFormatCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<HeaderFormatCase> {
    match v {
      0 => Some(HeaderFormatCase::not_set),
      1 => Some(HeaderFormatCase::ProperCaseWords),
      8 => Some(HeaderFormatCase::StatefulFormatter),
      _ => None
    }
  }
}
}  // pub mod header_key_format


}  // pub mod http1_protocol_options


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__KeepaliveSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct KeepaliveSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<KeepaliveSettings>
}

impl ::protobuf::Message for KeepaliveSettings {
  type MessageView<'msg> = KeepaliveSettingsView<'msg>;
  type MessageMut<'msg> = KeepaliveSettingsMut<'msg>;
}

impl ::std::default::Default for KeepaliveSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for KeepaliveSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `KeepaliveSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `KeepaliveSettingsMut`.
unsafe impl ::std::marker::Sync for KeepaliveSettings {}

// SAFETY:
// - `KeepaliveSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for KeepaliveSettings {}

impl ::protobuf::Proxied for KeepaliveSettings {
  type View<'msg> = KeepaliveSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for KeepaliveSettings {}

impl ::protobuf::MutProxied for KeepaliveSettings {
  type Mut<'msg> = KeepaliveSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct KeepaliveSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeepaliveSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeepaliveSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for KeepaliveSettingsView<'msg> {
  type Message = KeepaliveSettings;
}

impl ::std::fmt::Debug for KeepaliveSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for KeepaliveSettingsView<'_> {
  fn default() -> KeepaliveSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, KeepaliveSettings>> for KeepaliveSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeepaliveSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeepaliveSettingsView<'msg> {

  pub fn to_owned(&self) -> KeepaliveSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // interval: optional message google.protobuf.Duration
  pub fn has_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_interval().then(|| self.interval())
  }
  pub fn interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // interval_jitter: optional message envoy.type.v3.Percent
  pub fn has_interval_jitter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn interval_jitter_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_interval_jitter().then(|| self.interval_jitter())
  }
  pub fn interval_jitter(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  // connection_idle_interval: optional message google.protobuf.Duration
  pub fn has_connection_idle_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn connection_idle_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_connection_idle_interval().then(|| self.connection_idle_interval())
  }
  pub fn connection_idle_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `KeepaliveSettingsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for KeepaliveSettingsView<'_> {}

// SAFETY:
// - `KeepaliveSettingsView` is `Send` because while its alive a `KeepaliveSettingsMut` cannot.
// - `KeepaliveSettingsView` does not use thread-local data.
unsafe impl ::std::marker::Send for KeepaliveSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for KeepaliveSettingsView<'msg> {
  type Proxied = KeepaliveSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, KeepaliveSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeepaliveSettingsView<'msg> {
  fn into_view<'shorter>(self) -> KeepaliveSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<KeepaliveSettings> for KeepaliveSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeepaliveSettings {
    let mut dst = KeepaliveSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<KeepaliveSettings> for KeepaliveSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeepaliveSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for KeepaliveSettings {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeepaliveSettingsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeepaliveSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct KeepaliveSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeepaliveSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeepaliveSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for KeepaliveSettingsMut<'msg> {
  type Message = KeepaliveSettings;
}

impl ::std::fmt::Debug for KeepaliveSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, KeepaliveSettings>> for KeepaliveSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeepaliveSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeepaliveSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, KeepaliveSettings> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> KeepaliveSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // interval: optional message google.protobuf.Duration
  pub fn has_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_interval().then(|| self.interval())
  }
  pub fn interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // interval_jitter: optional message envoy.type.v3.Percent
  pub fn has_interval_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_interval_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn interval_jitter_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_interval_jitter().then(|| self.interval_jitter())
  }
  pub fn interval_jitter(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn interval_jitter_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_interval_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // connection_idle_interval: optional message google.protobuf.Duration
  pub fn has_connection_idle_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_connection_idle_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn connection_idle_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_connection_idle_interval().then(|| self.connection_idle_interval())
  }
  pub fn connection_idle_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn connection_idle_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_connection_idle_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

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
// - `KeepaliveSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for KeepaliveSettingsMut<'_> {}

// SAFETY:
// - `KeepaliveSettingsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for KeepaliveSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for KeepaliveSettingsMut<'msg> {
  type Proxied = KeepaliveSettings;
  fn as_view(&self) -> ::protobuf::View<'_, KeepaliveSettings> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeepaliveSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, KeepaliveSettings>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for KeepaliveSettingsMut<'msg> {
  type MutProxied = KeepaliveSettings;
  fn as_mut(&mut self) -> KeepaliveSettingsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for KeepaliveSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> KeepaliveSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl KeepaliveSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, KeepaliveSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> KeepaliveSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> KeepaliveSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // interval: optional message google.protobuf.Duration
  pub fn has_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_interval().then(|| self.interval())
  }
  pub fn interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // timeout: optional message google.protobuf.Duration
  pub fn has_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_timeout().then(|| self.timeout())
  }
  pub fn timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // interval_jitter: optional message envoy.type.v3.Percent
  pub fn has_interval_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_interval_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn interval_jitter_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_interval_jitter().then(|| self.interval_jitter())
  }
  pub fn interval_jitter(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn interval_jitter_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_interval_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // connection_idle_interval: optional message google.protobuf.Duration
  pub fn has_connection_idle_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_connection_idle_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn connection_idle_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_connection_idle_interval().then(|| self.connection_idle_interval())
  }
  pub fn connection_idle_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn connection_idle_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_connection_idle_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl KeepaliveSettings

impl ::std::ops::Drop for KeepaliveSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for KeepaliveSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for KeepaliveSettings {
  type Proxied = Self;
  fn as_view(&self) -> KeepaliveSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for KeepaliveSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> KeepaliveSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for KeepaliveSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__KeepaliveSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__KeepaliveSettings_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__KeepaliveSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeepaliveSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeepaliveSettings {
  type Msg = KeepaliveSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeepaliveSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeepaliveSettings {
  type Msg = KeepaliveSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeepaliveSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeepaliveSettingsMut<'_> {
  type Msg = KeepaliveSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeepaliveSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeepaliveSettingsMut<'_> {
  type Msg = KeepaliveSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeepaliveSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeepaliveSettingsView<'_> {
  type Msg = KeepaliveSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeepaliveSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeepaliveSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Http2ProtocolOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Http2ProtocolOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Http2ProtocolOptions>
}

impl ::protobuf::Message for Http2ProtocolOptions {
  type MessageView<'msg> = Http2ProtocolOptionsView<'msg>;
  type MessageMut<'msg> = Http2ProtocolOptionsMut<'msg>;
}

impl ::std::default::Default for Http2ProtocolOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Http2ProtocolOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Http2ProtocolOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `Http2ProtocolOptionsMut`.
unsafe impl ::std::marker::Sync for Http2ProtocolOptions {}

// SAFETY:
// - `Http2ProtocolOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Http2ProtocolOptions {}

impl ::protobuf::Proxied for Http2ProtocolOptions {
  type View<'msg> = Http2ProtocolOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Http2ProtocolOptions {}

impl ::protobuf::MutProxied for Http2ProtocolOptions {
  type Mut<'msg> = Http2ProtocolOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Http2ProtocolOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http2ProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Http2ProtocolOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Http2ProtocolOptionsView<'msg> {
  type Message = Http2ProtocolOptions;
}

impl ::std::fmt::Debug for Http2ProtocolOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Http2ProtocolOptionsView<'_> {
  fn default() -> Http2ProtocolOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Http2ProtocolOptions>> for Http2ProtocolOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http2ProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Http2ProtocolOptionsView<'msg> {

  pub fn to_owned(&self) -> Http2ProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // hpack_table_size: optional message google.protobuf.UInt32Value
  pub fn has_hpack_table_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn hpack_table_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_hpack_table_size().then(|| self.hpack_table_size())
  }
  pub fn hpack_table_size(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_concurrent_streams: optional message google.protobuf.UInt32Value
  pub fn has_max_concurrent_streams(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn max_concurrent_streams_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_concurrent_streams().then(|| self.max_concurrent_streams())
  }
  pub fn max_concurrent_streams(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // initial_stream_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_stream_window_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn initial_stream_window_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_initial_stream_window_size().then(|| self.initial_stream_window_size())
  }
  pub fn initial_stream_window_size(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // initial_connection_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_connection_window_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn initial_connection_window_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_initial_connection_window_size().then(|| self.initial_connection_window_size())
  }
  pub fn initial_connection_window_size(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // allow_connect: optional bool
  pub fn allow_connect(self) -> bool {
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

  // allow_metadata: optional bool
  pub fn allow_metadata(self) -> bool {
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

  // max_outbound_frames: optional message google.protobuf.UInt32Value
  pub fn has_max_outbound_frames(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn max_outbound_frames_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_outbound_frames().then(|| self.max_outbound_frames())
  }
  pub fn max_outbound_frames(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_outbound_control_frames: optional message google.protobuf.UInt32Value
  pub fn has_max_outbound_control_frames(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn max_outbound_control_frames_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_outbound_control_frames().then(|| self.max_outbound_control_frames())
  }
  pub fn max_outbound_control_frames(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_consecutive_inbound_frames_with_empty_payload: optional message google.protobuf.UInt32Value
  pub fn has_max_consecutive_inbound_frames_with_empty_payload(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn max_consecutive_inbound_frames_with_empty_payload_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_consecutive_inbound_frames_with_empty_payload().then(|| self.max_consecutive_inbound_frames_with_empty_payload())
  }
  pub fn max_consecutive_inbound_frames_with_empty_payload(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_inbound_priority_frames_per_stream: optional message google.protobuf.UInt32Value
  pub fn has_max_inbound_priority_frames_per_stream(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn max_inbound_priority_frames_per_stream_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_inbound_priority_frames_per_stream().then(|| self.max_inbound_priority_frames_per_stream())
  }
  pub fn max_inbound_priority_frames_per_stream(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // max_inbound_window_update_frames_per_data_frame_sent: optional message google.protobuf.UInt32Value
  pub fn has_max_inbound_window_update_frames_per_data_frame_sent(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn max_inbound_window_update_frames_per_data_frame_sent_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_inbound_window_update_frames_per_data_frame_sent().then(|| self.max_inbound_window_update_frames_per_data_frame_sent())
  }
  pub fn max_inbound_window_update_frames_per_data_frame_sent(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // stream_error_on_invalid_http_messaging: optional bool
  pub fn stream_error_on_invalid_http_messaging(self) -> bool {
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

  // override_stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_override_stream_error_on_invalid_http_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn override_stream_error_on_invalid_http_message_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_override_stream_error_on_invalid_http_message().then(|| self.override_stream_error_on_invalid_http_message())
  }
  pub fn override_stream_error_on_invalid_http_message(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // custom_settings_parameters: repeated message envoy.config.core.v3.Http2ProtocolOptions.SettingsParameter
  pub fn custom_settings_parameters(self) -> ::protobuf::RepeatedView<'msg, super::http2_protocol_options::SettingsParameter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http2_protocol_options::SettingsParameter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // connection_keepalive: optional message envoy.config.core.v3.KeepaliveSettings
  pub fn has_connection_keepalive(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn connection_keepalive_opt(self) -> ::std::option::Option<super::KeepaliveSettingsView<'msg>> {
    self.has_connection_keepalive().then(|| self.connection_keepalive())
  }
  pub fn connection_keepalive(self) -> super::KeepaliveSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeepaliveSettingsView::default())
  }

  // use_oghttp2_codec: optional message google.protobuf.BoolValue
  pub fn has_use_oghttp2_codec(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn use_oghttp2_codec_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_use_oghttp2_codec().then(|| self.use_oghttp2_codec())
  }
  pub fn use_oghttp2_codec(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // max_metadata_size: optional message google.protobuf.UInt64Value
  pub fn has_max_metadata_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn max_metadata_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_max_metadata_size().then(|| self.max_metadata_size())
  }
  pub fn max_metadata_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // enable_huffman_encoding: optional message google.protobuf.BoolValue
  pub fn has_enable_huffman_encoding(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn enable_huffman_encoding_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_enable_huffman_encoding().then(|| self.enable_huffman_encoding())
  }
  pub fn enable_huffman_encoding(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

}

// SAFETY:
// - `Http2ProtocolOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Http2ProtocolOptionsView<'_> {}

// SAFETY:
// - `Http2ProtocolOptionsView` is `Send` because while its alive a `Http2ProtocolOptionsMut` cannot.
// - `Http2ProtocolOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for Http2ProtocolOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for Http2ProtocolOptionsView<'msg> {
  type Proxied = Http2ProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, Http2ProtocolOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Http2ProtocolOptionsView<'msg> {
  fn into_view<'shorter>(self) -> Http2ProtocolOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Http2ProtocolOptions> for Http2ProtocolOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http2ProtocolOptions {
    let mut dst = Http2ProtocolOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Http2ProtocolOptions> for Http2ProtocolOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http2ProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Http2ProtocolOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Http2ProtocolOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Http2ProtocolOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Http2ProtocolOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http2ProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Http2ProtocolOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Http2ProtocolOptionsMut<'msg> {
  type Message = Http2ProtocolOptions;
}

impl ::std::fmt::Debug for Http2ProtocolOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Http2ProtocolOptions>> for Http2ProtocolOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http2ProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Http2ProtocolOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Http2ProtocolOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Http2ProtocolOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // hpack_table_size: optional message google.protobuf.UInt32Value
  pub fn has_hpack_table_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_hpack_table_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn hpack_table_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_hpack_table_size().then(|| self.hpack_table_size())
  }
  pub fn hpack_table_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn hpack_table_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_hpack_table_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_concurrent_streams: optional message google.protobuf.UInt32Value
  pub fn has_max_concurrent_streams(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_concurrent_streams(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_concurrent_streams_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_concurrent_streams().then(|| self.max_concurrent_streams())
  }
  pub fn max_concurrent_streams(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_concurrent_streams_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_concurrent_streams(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // initial_stream_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_stream_window_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_initial_stream_window_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn initial_stream_window_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_initial_stream_window_size().then(|| self.initial_stream_window_size())
  }
  pub fn initial_stream_window_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn initial_stream_window_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_initial_stream_window_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // initial_connection_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_connection_window_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_initial_connection_window_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn initial_connection_window_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_initial_connection_window_size().then(|| self.initial_connection_window_size())
  }
  pub fn initial_connection_window_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn initial_connection_window_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_initial_connection_window_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // allow_connect: optional bool
  pub fn allow_connect(&self) -> bool {
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
  pub fn set_allow_connect(&mut self, val: bool) {
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

  // allow_metadata: optional bool
  pub fn allow_metadata(&self) -> bool {
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
  pub fn set_allow_metadata(&mut self, val: bool) {
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

  // max_outbound_frames: optional message google.protobuf.UInt32Value
  pub fn has_max_outbound_frames(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_max_outbound_frames(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn max_outbound_frames_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_outbound_frames().then(|| self.max_outbound_frames())
  }
  pub fn max_outbound_frames(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_outbound_frames_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_outbound_frames(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // max_outbound_control_frames: optional message google.protobuf.UInt32Value
  pub fn has_max_outbound_control_frames(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_max_outbound_control_frames(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn max_outbound_control_frames_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_outbound_control_frames().then(|| self.max_outbound_control_frames())
  }
  pub fn max_outbound_control_frames(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_outbound_control_frames_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_outbound_control_frames(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // max_consecutive_inbound_frames_with_empty_payload: optional message google.protobuf.UInt32Value
  pub fn has_max_consecutive_inbound_frames_with_empty_payload(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_max_consecutive_inbound_frames_with_empty_payload(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn max_consecutive_inbound_frames_with_empty_payload_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_consecutive_inbound_frames_with_empty_payload().then(|| self.max_consecutive_inbound_frames_with_empty_payload())
  }
  pub fn max_consecutive_inbound_frames_with_empty_payload(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_consecutive_inbound_frames_with_empty_payload_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_consecutive_inbound_frames_with_empty_payload(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // max_inbound_priority_frames_per_stream: optional message google.protobuf.UInt32Value
  pub fn has_max_inbound_priority_frames_per_stream(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_max_inbound_priority_frames_per_stream(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn max_inbound_priority_frames_per_stream_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_inbound_priority_frames_per_stream().then(|| self.max_inbound_priority_frames_per_stream())
  }
  pub fn max_inbound_priority_frames_per_stream(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_inbound_priority_frames_per_stream_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_inbound_priority_frames_per_stream(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // max_inbound_window_update_frames_per_data_frame_sent: optional message google.protobuf.UInt32Value
  pub fn has_max_inbound_window_update_frames_per_data_frame_sent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_max_inbound_window_update_frames_per_data_frame_sent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn max_inbound_window_update_frames_per_data_frame_sent_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_inbound_window_update_frames_per_data_frame_sent().then(|| self.max_inbound_window_update_frames_per_data_frame_sent())
  }
  pub fn max_inbound_window_update_frames_per_data_frame_sent(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_inbound_window_update_frames_per_data_frame_sent_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_inbound_window_update_frames_per_data_frame_sent(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // stream_error_on_invalid_http_messaging: optional bool
  pub fn stream_error_on_invalid_http_messaging(&self) -> bool {
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
  pub fn set_stream_error_on_invalid_http_messaging(&mut self, val: bool) {
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

  // override_stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_override_stream_error_on_invalid_http_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_override_stream_error_on_invalid_http_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn override_stream_error_on_invalid_http_message_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_override_stream_error_on_invalid_http_message().then(|| self.override_stream_error_on_invalid_http_message())
  }
  pub fn override_stream_error_on_invalid_http_message(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn override_stream_error_on_invalid_http_message_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_override_stream_error_on_invalid_http_message(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // custom_settings_parameters: repeated message envoy.config.core.v3.Http2ProtocolOptions.SettingsParameter
  pub fn custom_settings_parameters(&self) -> ::protobuf::RepeatedView<'_, super::http2_protocol_options::SettingsParameter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http2_protocol_options::SettingsParameter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn custom_settings_parameters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::http2_protocol_options::SettingsParameter> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        12,
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
  pub fn set_custom_settings_parameters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::http2_protocol_options::SettingsParameter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // connection_keepalive: optional message envoy.config.core.v3.KeepaliveSettings
  pub fn has_connection_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_connection_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn connection_keepalive_opt(&self) -> ::std::option::Option<super::KeepaliveSettingsView<'_>> {
    self.has_connection_keepalive().then(|| self.connection_keepalive())
  }
  pub fn connection_keepalive(&self) -> super::KeepaliveSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeepaliveSettingsView::default())
  }
  pub fn connection_keepalive_mut(&mut self) -> super::KeepaliveSettingsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_connection_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<super::KeepaliveSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // use_oghttp2_codec: optional message google.protobuf.BoolValue
  pub fn has_use_oghttp2_codec(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_use_oghttp2_codec(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn use_oghttp2_codec_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_oghttp2_codec().then(|| self.use_oghttp2_codec())
  }
  pub fn use_oghttp2_codec(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_oghttp2_codec_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_oghttp2_codec(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // max_metadata_size: optional message google.protobuf.UInt64Value
  pub fn has_max_metadata_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_max_metadata_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn max_metadata_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_max_metadata_size().then(|| self.max_metadata_size())
  }
  pub fn max_metadata_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn max_metadata_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_max_metadata_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // enable_huffman_encoding: optional message google.protobuf.BoolValue
  pub fn has_enable_huffman_encoding(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_enable_huffman_encoding(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn enable_huffman_encoding_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_huffman_encoding().then(|| self.enable_huffman_encoding())
  }
  pub fn enable_huffman_encoding(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_huffman_encoding_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_enable_huffman_encoding(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

}

// SAFETY:
// - `Http2ProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Http2ProtocolOptionsMut<'_> {}

// SAFETY:
// - `Http2ProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Http2ProtocolOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for Http2ProtocolOptionsMut<'msg> {
  type Proxied = Http2ProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'_, Http2ProtocolOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Http2ProtocolOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Http2ProtocolOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Http2ProtocolOptionsMut<'msg> {
  type MutProxied = Http2ProtocolOptions;
  fn as_mut(&mut self) -> Http2ProtocolOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Http2ProtocolOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> Http2ProtocolOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Http2ProtocolOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Http2ProtocolOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Http2ProtocolOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Http2ProtocolOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // hpack_table_size: optional message google.protobuf.UInt32Value
  pub fn has_hpack_table_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_hpack_table_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn hpack_table_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_hpack_table_size().then(|| self.hpack_table_size())
  }
  pub fn hpack_table_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn hpack_table_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_hpack_table_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_concurrent_streams: optional message google.protobuf.UInt32Value
  pub fn has_max_concurrent_streams(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_concurrent_streams(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_concurrent_streams_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_concurrent_streams().then(|| self.max_concurrent_streams())
  }
  pub fn max_concurrent_streams(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_concurrent_streams_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_concurrent_streams(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // initial_stream_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_stream_window_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_initial_stream_window_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn initial_stream_window_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_initial_stream_window_size().then(|| self.initial_stream_window_size())
  }
  pub fn initial_stream_window_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn initial_stream_window_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_initial_stream_window_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // initial_connection_window_size: optional message google.protobuf.UInt32Value
  pub fn has_initial_connection_window_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_initial_connection_window_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn initial_connection_window_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_initial_connection_window_size().then(|| self.initial_connection_window_size())
  }
  pub fn initial_connection_window_size(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn initial_connection_window_size_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_initial_connection_window_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // allow_connect: optional bool
  pub fn allow_connect(&self) -> bool {
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
  pub fn set_allow_connect(&mut self, val: bool) {
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

  // allow_metadata: optional bool
  pub fn allow_metadata(&self) -> bool {
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
  pub fn set_allow_metadata(&mut self, val: bool) {
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

  // max_outbound_frames: optional message google.protobuf.UInt32Value
  pub fn has_max_outbound_frames(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_max_outbound_frames(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn max_outbound_frames_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_outbound_frames().then(|| self.max_outbound_frames())
  }
  pub fn max_outbound_frames(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_outbound_frames_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_outbound_frames(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // max_outbound_control_frames: optional message google.protobuf.UInt32Value
  pub fn has_max_outbound_control_frames(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_max_outbound_control_frames(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn max_outbound_control_frames_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_outbound_control_frames().then(|| self.max_outbound_control_frames())
  }
  pub fn max_outbound_control_frames(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_outbound_control_frames_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_outbound_control_frames(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // max_consecutive_inbound_frames_with_empty_payload: optional message google.protobuf.UInt32Value
  pub fn has_max_consecutive_inbound_frames_with_empty_payload(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_max_consecutive_inbound_frames_with_empty_payload(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn max_consecutive_inbound_frames_with_empty_payload_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_consecutive_inbound_frames_with_empty_payload().then(|| self.max_consecutive_inbound_frames_with_empty_payload())
  }
  pub fn max_consecutive_inbound_frames_with_empty_payload(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_consecutive_inbound_frames_with_empty_payload_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_consecutive_inbound_frames_with_empty_payload(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // max_inbound_priority_frames_per_stream: optional message google.protobuf.UInt32Value
  pub fn has_max_inbound_priority_frames_per_stream(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_max_inbound_priority_frames_per_stream(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn max_inbound_priority_frames_per_stream_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_inbound_priority_frames_per_stream().then(|| self.max_inbound_priority_frames_per_stream())
  }
  pub fn max_inbound_priority_frames_per_stream(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_inbound_priority_frames_per_stream_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_inbound_priority_frames_per_stream(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // max_inbound_window_update_frames_per_data_frame_sent: optional message google.protobuf.UInt32Value
  pub fn has_max_inbound_window_update_frames_per_data_frame_sent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_max_inbound_window_update_frames_per_data_frame_sent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn max_inbound_window_update_frames_per_data_frame_sent_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_inbound_window_update_frames_per_data_frame_sent().then(|| self.max_inbound_window_update_frames_per_data_frame_sent())
  }
  pub fn max_inbound_window_update_frames_per_data_frame_sent(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_inbound_window_update_frames_per_data_frame_sent_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_inbound_window_update_frames_per_data_frame_sent(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // stream_error_on_invalid_http_messaging: optional bool
  pub fn stream_error_on_invalid_http_messaging(&self) -> bool {
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
  pub fn set_stream_error_on_invalid_http_messaging(&mut self, val: bool) {
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

  // override_stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_override_stream_error_on_invalid_http_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_override_stream_error_on_invalid_http_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn override_stream_error_on_invalid_http_message_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_override_stream_error_on_invalid_http_message().then(|| self.override_stream_error_on_invalid_http_message())
  }
  pub fn override_stream_error_on_invalid_http_message(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn override_stream_error_on_invalid_http_message_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_override_stream_error_on_invalid_http_message(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  // custom_settings_parameters: repeated message envoy.config.core.v3.Http2ProtocolOptions.SettingsParameter
  pub fn custom_settings_parameters(&self) -> ::protobuf::RepeatedView<'_, super::http2_protocol_options::SettingsParameter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http2_protocol_options::SettingsParameter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn custom_settings_parameters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::http2_protocol_options::SettingsParameter> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        12,
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
  pub fn set_custom_settings_parameters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::http2_protocol_options::SettingsParameter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // connection_keepalive: optional message envoy.config.core.v3.KeepaliveSettings
  pub fn has_connection_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_connection_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn connection_keepalive_opt(&self) -> ::std::option::Option<super::KeepaliveSettingsView<'_>> {
    self.has_connection_keepalive().then(|| self.connection_keepalive())
  }
  pub fn connection_keepalive(&self) -> super::KeepaliveSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeepaliveSettingsView::default())
  }
  pub fn connection_keepalive_mut(&mut self) -> super::KeepaliveSettingsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_connection_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<super::KeepaliveSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // use_oghttp2_codec: optional message google.protobuf.BoolValue
  pub fn has_use_oghttp2_codec(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_use_oghttp2_codec(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn use_oghttp2_codec_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_use_oghttp2_codec().then(|| self.use_oghttp2_codec())
  }
  pub fn use_oghttp2_codec(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn use_oghttp2_codec_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_use_oghttp2_codec(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // max_metadata_size: optional message google.protobuf.UInt64Value
  pub fn has_max_metadata_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_max_metadata_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn max_metadata_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_max_metadata_size().then(|| self.max_metadata_size())
  }
  pub fn max_metadata_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn max_metadata_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_max_metadata_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // enable_huffman_encoding: optional message google.protobuf.BoolValue
  pub fn has_enable_huffman_encoding(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_enable_huffman_encoding(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn enable_huffman_encoding_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enable_huffman_encoding().then(|| self.enable_huffman_encoding())
  }
  pub fn enable_huffman_encoding(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enable_huffman_encoding_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_enable_huffman_encoding(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

}  // impl Http2ProtocolOptions

impl ::std::ops::Drop for Http2ProtocolOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Http2ProtocolOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Http2ProtocolOptions {
  type Proxied = Self;
  fn as_view(&self) -> Http2ProtocolOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Http2ProtocolOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Http2ProtocolOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Http2ProtocolOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__Http2ProtocolOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333/P/P33333/PG33333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__Http2ProtocolOptions_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::http2_protocol_options::SettingsParameter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::KeepaliveSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__Http2ProtocolOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http2ProtocolOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http2ProtocolOptions {
  type Msg = Http2ProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http2ProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http2ProtocolOptions {
  type Msg = Http2ProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http2ProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http2ProtocolOptionsMut<'_> {
  type Msg = Http2ProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http2ProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http2ProtocolOptionsMut<'_> {
  type Msg = Http2ProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http2ProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http2ProtocolOptionsView<'_> {
  type Msg = Http2ProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http2ProtocolOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http2ProtocolOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http2_protocol_options {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Http2ProtocolOptions__SettingsParameter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SettingsParameter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SettingsParameter>
}

impl ::protobuf::Message for SettingsParameter {
  type MessageView<'msg> = SettingsParameterView<'msg>;
  type MessageMut<'msg> = SettingsParameterMut<'msg>;
}

impl ::std::default::Default for SettingsParameter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SettingsParameter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SettingsParameter` is `Sync` because it does not implement interior mutability.
//    Neither does `SettingsParameterMut`.
unsafe impl ::std::marker::Sync for SettingsParameter {}

// SAFETY:
// - `SettingsParameter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SettingsParameter {}

impl ::protobuf::Proxied for SettingsParameter {
  type View<'msg> = SettingsParameterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SettingsParameter {}

impl ::protobuf::MutProxied for SettingsParameter {
  type Mut<'msg> = SettingsParameterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SettingsParameterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SettingsParameter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SettingsParameterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SettingsParameterView<'msg> {
  type Message = SettingsParameter;
}

impl ::std::fmt::Debug for SettingsParameterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SettingsParameterView<'_> {
  fn default() -> SettingsParameterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SettingsParameter>> for SettingsParameterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SettingsParameter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SettingsParameterView<'msg> {

  pub fn to_owned(&self) -> SettingsParameter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // identifier: optional message google.protobuf.UInt32Value
  pub fn has_identifier(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn identifier_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_identifier().then(|| self.identifier())
  }
  pub fn identifier(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // value: optional message google.protobuf.UInt32Value
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `SettingsParameterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SettingsParameterView<'_> {}

// SAFETY:
// - `SettingsParameterView` is `Send` because while its alive a `SettingsParameterMut` cannot.
// - `SettingsParameterView` does not use thread-local data.
unsafe impl ::std::marker::Send for SettingsParameterView<'_> {}

impl<'msg> ::protobuf::AsView for SettingsParameterView<'msg> {
  type Proxied = SettingsParameter;
  fn as_view(&self) -> ::protobuf::View<'msg, SettingsParameter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SettingsParameterView<'msg> {
  fn into_view<'shorter>(self) -> SettingsParameterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SettingsParameter> for SettingsParameterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SettingsParameter {
    let mut dst = SettingsParameter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SettingsParameter> for SettingsParameterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SettingsParameter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SettingsParameter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SettingsParameterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SettingsParameterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SettingsParameterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SettingsParameter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SettingsParameterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SettingsParameterMut<'msg> {
  type Message = SettingsParameter;
}

impl ::std::fmt::Debug for SettingsParameterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SettingsParameter>> for SettingsParameterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SettingsParameter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SettingsParameterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SettingsParameter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SettingsParameter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // identifier: optional message google.protobuf.UInt32Value
  pub fn has_identifier(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_identifier(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn identifier_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_identifier().then(|| self.identifier())
  }
  pub fn identifier(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn identifier_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_identifier(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value: optional message google.protobuf.UInt32Value
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
  pub fn value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn value_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

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
// - `SettingsParameterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SettingsParameterMut<'_> {}

// SAFETY:
// - `SettingsParameterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SettingsParameterMut<'_> {}

impl<'msg> ::protobuf::AsView for SettingsParameterMut<'msg> {
  type Proxied = SettingsParameter;
  fn as_view(&self) -> ::protobuf::View<'_, SettingsParameter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SettingsParameterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SettingsParameter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SettingsParameterMut<'msg> {
  type MutProxied = SettingsParameter;
  fn as_mut(&mut self) -> SettingsParameterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SettingsParameterMut<'msg> {
  fn into_mut<'shorter>(self) -> SettingsParameterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SettingsParameter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SettingsParameter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SettingsParameterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SettingsParameterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // identifier: optional message google.protobuf.UInt32Value
  pub fn has_identifier(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_identifier(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn identifier_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_identifier().then(|| self.identifier())
  }
  pub fn identifier(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn identifier_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_identifier(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value: optional message google.protobuf.UInt32Value
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
  pub fn value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn value_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl SettingsParameter

impl ::std::ops::Drop for SettingsParameter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SettingsParameter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SettingsParameter {
  type Proxied = Self;
  fn as_view(&self) -> SettingsParameterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SettingsParameter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SettingsParameterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SettingsParameter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http2_protocol_options::envoy__config__core__v3__Http2ProtocolOptions__SettingsParameter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http2_protocol_options::envoy__config__core__v3__Http2ProtocolOptions__SettingsParameter_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http2_protocol_options::envoy__config__core__v3__Http2ProtocolOptions__SettingsParameter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SettingsParameter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SettingsParameter {
  type Msg = SettingsParameter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SettingsParameter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SettingsParameter {
  type Msg = SettingsParameter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SettingsParameter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SettingsParameterMut<'_> {
  type Msg = SettingsParameter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SettingsParameter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SettingsParameterMut<'_> {
  type Msg = SettingsParameter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SettingsParameter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SettingsParameterView<'_> {
  type Msg = SettingsParameter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SettingsParameter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SettingsParameterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod http2_protocol_options


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__GrpcProtocolOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GrpcProtocolOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GrpcProtocolOptions>
}

impl ::protobuf::Message for GrpcProtocolOptions {
  type MessageView<'msg> = GrpcProtocolOptionsView<'msg>;
  type MessageMut<'msg> = GrpcProtocolOptionsMut<'msg>;
}

impl ::std::default::Default for GrpcProtocolOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GrpcProtocolOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GrpcProtocolOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `GrpcProtocolOptionsMut`.
unsafe impl ::std::marker::Sync for GrpcProtocolOptions {}

// SAFETY:
// - `GrpcProtocolOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GrpcProtocolOptions {}

impl ::protobuf::Proxied for GrpcProtocolOptions {
  type View<'msg> = GrpcProtocolOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GrpcProtocolOptions {}

impl ::protobuf::MutProxied for GrpcProtocolOptions {
  type Mut<'msg> = GrpcProtocolOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GrpcProtocolOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcProtocolOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GrpcProtocolOptionsView<'msg> {
  type Message = GrpcProtocolOptions;
}

impl ::std::fmt::Debug for GrpcProtocolOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GrpcProtocolOptionsView<'_> {
  fn default() -> GrpcProtocolOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcProtocolOptions>> for GrpcProtocolOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcProtocolOptionsView<'msg> {

  pub fn to_owned(&self) -> GrpcProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // http2_protocol_options: optional message envoy.config.core.v3.Http2ProtocolOptions
  pub fn has_http2_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn http2_protocol_options_opt(self) -> ::std::option::Option<super::Http2ProtocolOptionsView<'msg>> {
    self.has_http2_protocol_options().then(|| self.http2_protocol_options())
  }
  pub fn http2_protocol_options(self) -> super::Http2ProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Http2ProtocolOptionsView::default())
  }

}

// SAFETY:
// - `GrpcProtocolOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GrpcProtocolOptionsView<'_> {}

// SAFETY:
// - `GrpcProtocolOptionsView` is `Send` because while its alive a `GrpcProtocolOptionsMut` cannot.
// - `GrpcProtocolOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for GrpcProtocolOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for GrpcProtocolOptionsView<'msg> {
  type Proxied = GrpcProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, GrpcProtocolOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcProtocolOptionsView<'msg> {
  fn into_view<'shorter>(self) -> GrpcProtocolOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcProtocolOptions> for GrpcProtocolOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcProtocolOptions {
    let mut dst = GrpcProtocolOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcProtocolOptions> for GrpcProtocolOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GrpcProtocolOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcProtocolOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcProtocolOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GrpcProtocolOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcProtocolOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GrpcProtocolOptionsMut<'msg> {
  type Message = GrpcProtocolOptions;
}

impl ::std::fmt::Debug for GrpcProtocolOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcProtocolOptions>> for GrpcProtocolOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcProtocolOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcProtocolOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GrpcProtocolOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // http2_protocol_options: optional message envoy.config.core.v3.Http2ProtocolOptions
  pub fn has_http2_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http2_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http2_protocol_options_opt(&self) -> ::std::option::Option<super::Http2ProtocolOptionsView<'_>> {
    self.has_http2_protocol_options().then(|| self.http2_protocol_options())
  }
  pub fn http2_protocol_options(&self) -> super::Http2ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Http2ProtocolOptionsView::default())
  }
  pub fn http2_protocol_options_mut(&mut self) -> super::Http2ProtocolOptionsMut<'_> {
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
  pub fn set_http2_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::Http2ProtocolOptions>) {

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
// - `GrpcProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GrpcProtocolOptionsMut<'_> {}

// SAFETY:
// - `GrpcProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GrpcProtocolOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for GrpcProtocolOptionsMut<'msg> {
  type Proxied = GrpcProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'_, GrpcProtocolOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcProtocolOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GrpcProtocolOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GrpcProtocolOptionsMut<'msg> {
  type MutProxied = GrpcProtocolOptions;
  fn as_mut(&mut self) -> GrpcProtocolOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GrpcProtocolOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> GrpcProtocolOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GrpcProtocolOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GrpcProtocolOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GrpcProtocolOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GrpcProtocolOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // http2_protocol_options: optional message envoy.config.core.v3.Http2ProtocolOptions
  pub fn has_http2_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http2_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http2_protocol_options_opt(&self) -> ::std::option::Option<super::Http2ProtocolOptionsView<'_>> {
    self.has_http2_protocol_options().then(|| self.http2_protocol_options())
  }
  pub fn http2_protocol_options(&self) -> super::Http2ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Http2ProtocolOptionsView::default())
  }
  pub fn http2_protocol_options_mut(&mut self) -> super::Http2ProtocolOptionsMut<'_> {
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
  pub fn set_http2_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::Http2ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl GrpcProtocolOptions

impl ::std::ops::Drop for GrpcProtocolOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GrpcProtocolOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GrpcProtocolOptions {
  type Proxied = Self;
  fn as_view(&self) -> GrpcProtocolOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GrpcProtocolOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GrpcProtocolOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GrpcProtocolOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__GrpcProtocolOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__GrpcProtocolOptions_msg_init.0, &[<super::Http2ProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__GrpcProtocolOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcProtocolOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcProtocolOptions {
  type Msg = GrpcProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcProtocolOptions {
  type Msg = GrpcProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcProtocolOptionsMut<'_> {
  type Msg = GrpcProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcProtocolOptionsMut<'_> {
  type Msg = GrpcProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcProtocolOptionsView<'_> {
  type Msg = GrpcProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcProtocolOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcProtocolOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Http3ProtocolOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Http3ProtocolOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Http3ProtocolOptions>
}

impl ::protobuf::Message for Http3ProtocolOptions {
  type MessageView<'msg> = Http3ProtocolOptionsView<'msg>;
  type MessageMut<'msg> = Http3ProtocolOptionsMut<'msg>;
}

impl ::std::default::Default for Http3ProtocolOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Http3ProtocolOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Http3ProtocolOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `Http3ProtocolOptionsMut`.
unsafe impl ::std::marker::Sync for Http3ProtocolOptions {}

// SAFETY:
// - `Http3ProtocolOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Http3ProtocolOptions {}

impl ::protobuf::Proxied for Http3ProtocolOptions {
  type View<'msg> = Http3ProtocolOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Http3ProtocolOptions {}

impl ::protobuf::MutProxied for Http3ProtocolOptions {
  type Mut<'msg> = Http3ProtocolOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Http3ProtocolOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http3ProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Http3ProtocolOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Http3ProtocolOptionsView<'msg> {
  type Message = Http3ProtocolOptions;
}

impl ::std::fmt::Debug for Http3ProtocolOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Http3ProtocolOptionsView<'_> {
  fn default() -> Http3ProtocolOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Http3ProtocolOptions>> for Http3ProtocolOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Http3ProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Http3ProtocolOptionsView<'msg> {

  pub fn to_owned(&self) -> Http3ProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // quic_protocol_options: optional message envoy.config.core.v3.QuicProtocolOptions
  pub fn has_quic_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn quic_protocol_options_opt(self) -> ::std::option::Option<super::QuicProtocolOptionsView<'msg>> {
    self.has_quic_protocol_options().then(|| self.quic_protocol_options())
  }
  pub fn quic_protocol_options(self) -> super::QuicProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::QuicProtocolOptionsView::default())
  }

  // override_stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_override_stream_error_on_invalid_http_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn override_stream_error_on_invalid_http_message_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_override_stream_error_on_invalid_http_message().then(|| self.override_stream_error_on_invalid_http_message())
  }
  pub fn override_stream_error_on_invalid_http_message(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // allow_extended_connect: optional bool
  pub fn allow_extended_connect(self) -> bool {
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

  // allow_metadata: optional bool
  pub fn allow_metadata(self) -> bool {
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

  // disable_qpack: optional bool
  pub fn disable_qpack(self) -> bool {
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

  // disable_connection_flow_control_for_streams: optional bool
  pub fn disable_connection_flow_control_for_streams(self) -> bool {
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

}

// SAFETY:
// - `Http3ProtocolOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Http3ProtocolOptionsView<'_> {}

// SAFETY:
// - `Http3ProtocolOptionsView` is `Send` because while its alive a `Http3ProtocolOptionsMut` cannot.
// - `Http3ProtocolOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for Http3ProtocolOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for Http3ProtocolOptionsView<'msg> {
  type Proxied = Http3ProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, Http3ProtocolOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Http3ProtocolOptionsView<'msg> {
  fn into_view<'shorter>(self) -> Http3ProtocolOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Http3ProtocolOptions> for Http3ProtocolOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http3ProtocolOptions {
    let mut dst = Http3ProtocolOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Http3ProtocolOptions> for Http3ProtocolOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Http3ProtocolOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Http3ProtocolOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Http3ProtocolOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Http3ProtocolOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Http3ProtocolOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http3ProtocolOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Http3ProtocolOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Http3ProtocolOptionsMut<'msg> {
  type Message = Http3ProtocolOptions;
}

impl ::std::fmt::Debug for Http3ProtocolOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Http3ProtocolOptions>> for Http3ProtocolOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Http3ProtocolOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Http3ProtocolOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Http3ProtocolOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Http3ProtocolOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // quic_protocol_options: optional message envoy.config.core.v3.QuicProtocolOptions
  pub fn has_quic_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_quic_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn quic_protocol_options_opt(&self) -> ::std::option::Option<super::QuicProtocolOptionsView<'_>> {
    self.has_quic_protocol_options().then(|| self.quic_protocol_options())
  }
  pub fn quic_protocol_options(&self) -> super::QuicProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::QuicProtocolOptionsView::default())
  }
  pub fn quic_protocol_options_mut(&mut self) -> super::QuicProtocolOptionsMut<'_> {
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
  pub fn set_quic_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::QuicProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // override_stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_override_stream_error_on_invalid_http_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_override_stream_error_on_invalid_http_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn override_stream_error_on_invalid_http_message_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_override_stream_error_on_invalid_http_message().then(|| self.override_stream_error_on_invalid_http_message())
  }
  pub fn override_stream_error_on_invalid_http_message(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn override_stream_error_on_invalid_http_message_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_override_stream_error_on_invalid_http_message(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // allow_extended_connect: optional bool
  pub fn allow_extended_connect(&self) -> bool {
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
  pub fn set_allow_extended_connect(&mut self, val: bool) {
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

  // allow_metadata: optional bool
  pub fn allow_metadata(&self) -> bool {
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
  pub fn set_allow_metadata(&mut self, val: bool) {
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

  // disable_qpack: optional bool
  pub fn disable_qpack(&self) -> bool {
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
  pub fn set_disable_qpack(&mut self, val: bool) {
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

  // disable_connection_flow_control_for_streams: optional bool
  pub fn disable_connection_flow_control_for_streams(&self) -> bool {
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
  pub fn set_disable_connection_flow_control_for_streams(&mut self, val: bool) {
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

}

// SAFETY:
// - `Http3ProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Http3ProtocolOptionsMut<'_> {}

// SAFETY:
// - `Http3ProtocolOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Http3ProtocolOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for Http3ProtocolOptionsMut<'msg> {
  type Proxied = Http3ProtocolOptions;
  fn as_view(&self) -> ::protobuf::View<'_, Http3ProtocolOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Http3ProtocolOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Http3ProtocolOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Http3ProtocolOptionsMut<'msg> {
  type MutProxied = Http3ProtocolOptions;
  fn as_mut(&mut self) -> Http3ProtocolOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Http3ProtocolOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> Http3ProtocolOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Http3ProtocolOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Http3ProtocolOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Http3ProtocolOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Http3ProtocolOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // quic_protocol_options: optional message envoy.config.core.v3.QuicProtocolOptions
  pub fn has_quic_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_quic_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn quic_protocol_options_opt(&self) -> ::std::option::Option<super::QuicProtocolOptionsView<'_>> {
    self.has_quic_protocol_options().then(|| self.quic_protocol_options())
  }
  pub fn quic_protocol_options(&self) -> super::QuicProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::QuicProtocolOptionsView::default())
  }
  pub fn quic_protocol_options_mut(&mut self) -> super::QuicProtocolOptionsMut<'_> {
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
  pub fn set_quic_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::QuicProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // override_stream_error_on_invalid_http_message: optional message google.protobuf.BoolValue
  pub fn has_override_stream_error_on_invalid_http_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_override_stream_error_on_invalid_http_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn override_stream_error_on_invalid_http_message_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_override_stream_error_on_invalid_http_message().then(|| self.override_stream_error_on_invalid_http_message())
  }
  pub fn override_stream_error_on_invalid_http_message(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn override_stream_error_on_invalid_http_message_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_override_stream_error_on_invalid_http_message(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // allow_extended_connect: optional bool
  pub fn allow_extended_connect(&self) -> bool {
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
  pub fn set_allow_extended_connect(&mut self, val: bool) {
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

  // allow_metadata: optional bool
  pub fn allow_metadata(&self) -> bool {
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
  pub fn set_allow_metadata(&mut self, val: bool) {
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

  // disable_qpack: optional bool
  pub fn disable_qpack(&self) -> bool {
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
  pub fn set_disable_qpack(&mut self, val: bool) {
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

  // disable_connection_flow_control_for_streams: optional bool
  pub fn disable_connection_flow_control_for_streams(&self) -> bool {
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
  pub fn set_disable_connection_flow_control_for_streams(&mut self, val: bool) {
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

}  // impl Http3ProtocolOptions

impl ::std::ops::Drop for Http3ProtocolOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Http3ProtocolOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Http3ProtocolOptions {
  type Proxied = Self;
  fn as_view(&self) -> Http3ProtocolOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Http3ProtocolOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Http3ProtocolOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Http3ProtocolOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__Http3ProtocolOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33b/P/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__Http3ProtocolOptions_msg_init.0, &[<super::QuicProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__Http3ProtocolOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http3ProtocolOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http3ProtocolOptions {
  type Msg = Http3ProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http3ProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http3ProtocolOptions {
  type Msg = Http3ProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http3ProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Http3ProtocolOptionsMut<'_> {
  type Msg = Http3ProtocolOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http3ProtocolOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http3ProtocolOptionsMut<'_> {
  type Msg = Http3ProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http3ProtocolOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Http3ProtocolOptionsView<'_> {
  type Msg = Http3ProtocolOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Http3ProtocolOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Http3ProtocolOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SchemeHeaderTransformation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SchemeHeaderTransformation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SchemeHeaderTransformation>
}

impl ::protobuf::Message for SchemeHeaderTransformation {
  type MessageView<'msg> = SchemeHeaderTransformationView<'msg>;
  type MessageMut<'msg> = SchemeHeaderTransformationMut<'msg>;
}

impl ::std::default::Default for SchemeHeaderTransformation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SchemeHeaderTransformation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SchemeHeaderTransformation` is `Sync` because it does not implement interior mutability.
//    Neither does `SchemeHeaderTransformationMut`.
unsafe impl ::std::marker::Sync for SchemeHeaderTransformation {}

// SAFETY:
// - `SchemeHeaderTransformation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SchemeHeaderTransformation {}

impl ::protobuf::Proxied for SchemeHeaderTransformation {
  type View<'msg> = SchemeHeaderTransformationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SchemeHeaderTransformation {}

impl ::protobuf::MutProxied for SchemeHeaderTransformation {
  type Mut<'msg> = SchemeHeaderTransformationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SchemeHeaderTransformationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemeHeaderTransformation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemeHeaderTransformationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SchemeHeaderTransformationView<'msg> {
  type Message = SchemeHeaderTransformation;
}

impl ::std::fmt::Debug for SchemeHeaderTransformationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SchemeHeaderTransformationView<'_> {
  fn default() -> SchemeHeaderTransformationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SchemeHeaderTransformation>> for SchemeHeaderTransformationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemeHeaderTransformation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemeHeaderTransformationView<'msg> {

  pub fn to_owned(&self) -> SchemeHeaderTransformation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // scheme_to_overwrite: optional string
  pub fn has_scheme_to_overwrite(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn scheme_to_overwrite_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_scheme_to_overwrite().then(|| self.scheme_to_overwrite())
  }
  pub fn scheme_to_overwrite(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // match_upstream: optional bool
  pub fn match_upstream(self) -> bool {
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

  pub fn transformation(self) -> super::scheme_header_transformation::TransformationOneof<'msg> {
    match self.transformation_case() {
      super::scheme_header_transformation::TransformationCase::SchemeToOverwrite =>
          super::scheme_header_transformation::TransformationOneof::SchemeToOverwrite(self.scheme_to_overwrite()),
      _ => super::scheme_header_transformation::TransformationOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn transformation_case(self) -> super::scheme_header_transformation::TransformationCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::scheme_header_transformation::TransformationCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SchemeHeaderTransformationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SchemeHeaderTransformationView<'_> {}

// SAFETY:
// - `SchemeHeaderTransformationView` is `Send` because while its alive a `SchemeHeaderTransformationMut` cannot.
// - `SchemeHeaderTransformationView` does not use thread-local data.
unsafe impl ::std::marker::Send for SchemeHeaderTransformationView<'_> {}

impl<'msg> ::protobuf::AsView for SchemeHeaderTransformationView<'msg> {
  type Proxied = SchemeHeaderTransformation;
  fn as_view(&self) -> ::protobuf::View<'msg, SchemeHeaderTransformation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemeHeaderTransformationView<'msg> {
  fn into_view<'shorter>(self) -> SchemeHeaderTransformationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemeHeaderTransformation> for SchemeHeaderTransformationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemeHeaderTransformation {
    let mut dst = SchemeHeaderTransformation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemeHeaderTransformation> for SchemeHeaderTransformationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemeHeaderTransformation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SchemeHeaderTransformation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SchemeHeaderTransformationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SchemeHeaderTransformationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SchemeHeaderTransformationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemeHeaderTransformation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemeHeaderTransformationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SchemeHeaderTransformationMut<'msg> {
  type Message = SchemeHeaderTransformation;
}

impl ::std::fmt::Debug for SchemeHeaderTransformationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SchemeHeaderTransformation>> for SchemeHeaderTransformationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemeHeaderTransformation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemeHeaderTransformationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemeHeaderTransformation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SchemeHeaderTransformation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // scheme_to_overwrite: optional string
  pub fn has_scheme_to_overwrite(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_scheme_to_overwrite(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn scheme_to_overwrite_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_scheme_to_overwrite().then(|| self.scheme_to_overwrite())
  }
  pub fn scheme_to_overwrite(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scheme_to_overwrite(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // match_upstream: optional bool
  pub fn match_upstream(&self) -> bool {
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
  pub fn set_match_upstream(&mut self, val: bool) {
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

  pub fn transformation(&self) -> super::scheme_header_transformation::TransformationOneof<'_> {
    match &self.transformation_case() {
      super::scheme_header_transformation::TransformationCase::SchemeToOverwrite =>
          super::scheme_header_transformation::TransformationOneof::SchemeToOverwrite(self.scheme_to_overwrite()),
      _ => super::scheme_header_transformation::TransformationOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn transformation_case(&self) -> super::scheme_header_transformation::TransformationCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::scheme_header_transformation::TransformationCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SchemeHeaderTransformationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SchemeHeaderTransformationMut<'_> {}

// SAFETY:
// - `SchemeHeaderTransformationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SchemeHeaderTransformationMut<'_> {}

impl<'msg> ::protobuf::AsView for SchemeHeaderTransformationMut<'msg> {
  type Proxied = SchemeHeaderTransformation;
  fn as_view(&self) -> ::protobuf::View<'_, SchemeHeaderTransformation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemeHeaderTransformationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SchemeHeaderTransformation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SchemeHeaderTransformationMut<'msg> {
  type MutProxied = SchemeHeaderTransformation;
  fn as_mut(&mut self) -> SchemeHeaderTransformationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SchemeHeaderTransformationMut<'msg> {
  fn into_mut<'shorter>(self) -> SchemeHeaderTransformationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SchemeHeaderTransformation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SchemeHeaderTransformation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SchemeHeaderTransformationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SchemeHeaderTransformationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // scheme_to_overwrite: optional string
  pub fn has_scheme_to_overwrite(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_scheme_to_overwrite(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn scheme_to_overwrite_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_scheme_to_overwrite().then(|| self.scheme_to_overwrite())
  }
  pub fn scheme_to_overwrite(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scheme_to_overwrite(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // match_upstream: optional bool
  pub fn match_upstream(&self) -> bool {
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
  pub fn set_match_upstream(&mut self, val: bool) {
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

  pub fn transformation(&self) -> super::scheme_header_transformation::TransformationOneof<'_> {
    match &self.transformation_case() {
      super::scheme_header_transformation::TransformationCase::SchemeToOverwrite =>
          super::scheme_header_transformation::TransformationOneof::SchemeToOverwrite(self.scheme_to_overwrite()),
      _ => super::scheme_header_transformation::TransformationOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn transformation_case(&self) -> super::scheme_header_transformation::TransformationCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::scheme_header_transformation::TransformationCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl SchemeHeaderTransformation

impl ::std::ops::Drop for SchemeHeaderTransformation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SchemeHeaderTransformation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SchemeHeaderTransformation {
  type Proxied = Self;
  fn as_view(&self) -> SchemeHeaderTransformationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SchemeHeaderTransformation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SchemeHeaderTransformationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SchemeHeaderTransformation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__SchemeHeaderTransformation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T/P^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__SchemeHeaderTransformation_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__SchemeHeaderTransformation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemeHeaderTransformation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemeHeaderTransformation {
  type Msg = SchemeHeaderTransformation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemeHeaderTransformation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemeHeaderTransformation {
  type Msg = SchemeHeaderTransformation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemeHeaderTransformation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemeHeaderTransformationMut<'_> {
  type Msg = SchemeHeaderTransformation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemeHeaderTransformation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemeHeaderTransformationMut<'_> {
  type Msg = SchemeHeaderTransformation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemeHeaderTransformation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemeHeaderTransformationView<'_> {
  type Msg = SchemeHeaderTransformation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemeHeaderTransformation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemeHeaderTransformationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod scheme_header_transformation {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TransformationOneof<'msg> {
  SchemeToOverwrite(&'msg ::protobuf::ProtoStr) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TransformationCase {
  SchemeToOverwrite = 1,

  not_set = 0
}

impl TransformationCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TransformationCase> {
    match v {
      0 => Some(TransformationCase::not_set),
      1 => Some(TransformationCase::SchemeToOverwrite),
      _ => None
    }
  }
}
}  // pub mod scheme_header_transformation


