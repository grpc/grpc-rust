const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.34.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__MessageStoragePolicy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MessageStoragePolicy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MessageStoragePolicy>
}

impl ::protobuf::Message for MessageStoragePolicy {}

impl ::std::default::Default for MessageStoragePolicy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MessageStoragePolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MessageStoragePolicy` is `Sync` because it does not implement interior mutability.
//    Neither does `MessageStoragePolicyMut`.
unsafe impl Sync for MessageStoragePolicy {}

// SAFETY:
// - `MessageStoragePolicy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MessageStoragePolicy {}

impl ::protobuf::Proxied for MessageStoragePolicy {
  type View<'msg> = MessageStoragePolicyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MessageStoragePolicy {}

impl ::protobuf::MutProxied for MessageStoragePolicy {
  type Mut<'msg> = MessageStoragePolicyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MessageStoragePolicyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MessageStoragePolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MessageStoragePolicyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MessageStoragePolicyView<'msg> {
  type Message = MessageStoragePolicy;
}

impl ::std::fmt::Debug for MessageStoragePolicyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MessageStoragePolicyView<'_> {
  fn default() -> MessageStoragePolicyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MessageStoragePolicy>> for MessageStoragePolicyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MessageStoragePolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MessageStoragePolicyView<'msg> {

  pub fn to_owned(&self) -> MessageStoragePolicy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // allowed_persistence_regions: repeated string
  pub fn allowed_persistence_regions(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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
// - `MessageStoragePolicyView` is `Sync` because it does not support mutation.
unsafe impl Sync for MessageStoragePolicyView<'_> {}

// SAFETY:
// - `MessageStoragePolicyView` is `Send` because while its alive a `MessageStoragePolicyMut` cannot.
// - `MessageStoragePolicyView` does not use thread-local data.
unsafe impl Send for MessageStoragePolicyView<'_> {}

impl<'msg> ::protobuf::AsView for MessageStoragePolicyView<'msg> {
  type Proxied = MessageStoragePolicy;
  fn as_view(&self) -> ::protobuf::View<'msg, MessageStoragePolicy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MessageStoragePolicyView<'msg> {
  fn into_view<'shorter>(self) -> MessageStoragePolicyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MessageStoragePolicy> for MessageStoragePolicyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MessageStoragePolicy {
    let mut dst = MessageStoragePolicy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MessageStoragePolicy> for MessageStoragePolicyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MessageStoragePolicy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for MessageStoragePolicy {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MessageStoragePolicyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for MessageStoragePolicyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MessageStoragePolicyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MessageStoragePolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MessageStoragePolicyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MessageStoragePolicyMut<'msg> {
  type Message = MessageStoragePolicy;
}

impl ::std::fmt::Debug for MessageStoragePolicyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MessageStoragePolicy>> for MessageStoragePolicyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MessageStoragePolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MessageStoragePolicyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MessageStoragePolicy> {
    self.inner
  }

  pub fn to_owned(&self) -> MessageStoragePolicy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // allowed_persistence_regions: repeated string
  pub fn allowed_persistence_regions(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn allowed_persistence_regions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_allowed_persistence_regions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `MessageStoragePolicyMut` does not perform any shared mutation.
unsafe impl Send for MessageStoragePolicyMut<'_> {}

// SAFETY:
// - `MessageStoragePolicyMut` does not perform any shared mutation.
unsafe impl Sync for MessageStoragePolicyMut<'_> {}

impl<'msg> ::protobuf::AsView for MessageStoragePolicyMut<'msg> {
  type Proxied = MessageStoragePolicy;
  fn as_view(&self) -> ::protobuf::View<'_, MessageStoragePolicy> {
    MessageStoragePolicyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MessageStoragePolicyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MessageStoragePolicy>
  where
      'msg: 'shorter {
    MessageStoragePolicyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for MessageStoragePolicyMut<'msg> {
  type MutProxied = MessageStoragePolicy;
  fn as_mut(&mut self) -> MessageStoragePolicyMut<'msg> {
    MessageStoragePolicyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MessageStoragePolicyMut<'msg> {
  fn into_mut<'shorter>(self) -> MessageStoragePolicyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MessageStoragePolicy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MessageStoragePolicy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MessageStoragePolicyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MessageStoragePolicyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // allowed_persistence_regions: repeated string
  pub fn allowed_persistence_regions(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn allowed_persistence_regions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_allowed_persistence_regions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl MessageStoragePolicy

impl ::std::ops::Drop for MessageStoragePolicy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MessageStoragePolicy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MessageStoragePolicy {
  type Proxied = Self;
  fn as_view(&self) -> MessageStoragePolicyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MessageStoragePolicy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MessageStoragePolicyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MessageStoragePolicy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__MessageStoragePolicy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ME");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__MessageStoragePolicy_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__MessageStoragePolicy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MessageStoragePolicy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MessageStoragePolicy {
  type Msg = MessageStoragePolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageStoragePolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MessageStoragePolicy {
  type Msg = MessageStoragePolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageStoragePolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MessageStoragePolicyMut<'_> {
  type Msg = MessageStoragePolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageStoragePolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MessageStoragePolicyMut<'_> {
  type Msg = MessageStoragePolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageStoragePolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MessageStoragePolicyView<'_> {
  type Msg = MessageStoragePolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MessageStoragePolicy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MessageStoragePolicyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__SchemaSettings_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SchemaSettings {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SchemaSettings>
}

impl ::protobuf::Message for SchemaSettings {}

impl ::std::default::Default for SchemaSettings {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SchemaSettings {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SchemaSettings` is `Sync` because it does not implement interior mutability.
//    Neither does `SchemaSettingsMut`.
unsafe impl Sync for SchemaSettings {}

// SAFETY:
// - `SchemaSettings` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SchemaSettings {}

impl ::protobuf::Proxied for SchemaSettings {
  type View<'msg> = SchemaSettingsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SchemaSettings {}

impl ::protobuf::MutProxied for SchemaSettings {
  type Mut<'msg> = SchemaSettingsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SchemaSettingsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaSettingsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SchemaSettingsView<'msg> {
  type Message = SchemaSettings;
}

impl ::std::fmt::Debug for SchemaSettingsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SchemaSettingsView<'_> {
  fn default() -> SchemaSettingsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaSettings>> for SchemaSettingsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SchemaSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaSettingsView<'msg> {

  pub fn to_owned(&self) -> SchemaSettings {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // schema: optional string
  pub fn schema(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // encoding: optional enum google.pubsub.v1.Encoding
  pub fn encoding(self) -> super::Encoding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::Encoding::Unspecified).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SchemaSettingsView` is `Sync` because it does not support mutation.
unsafe impl Sync for SchemaSettingsView<'_> {}

// SAFETY:
// - `SchemaSettingsView` is `Send` because while its alive a `SchemaSettingsMut` cannot.
// - `SchemaSettingsView` does not use thread-local data.
unsafe impl Send for SchemaSettingsView<'_> {}

impl<'msg> ::protobuf::AsView for SchemaSettingsView<'msg> {
  type Proxied = SchemaSettings;
  fn as_view(&self) -> ::protobuf::View<'msg, SchemaSettings> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaSettingsView<'msg> {
  fn into_view<'shorter>(self) -> SchemaSettingsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemaSettings> for SchemaSettingsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemaSettings {
    let mut dst = SchemaSettings::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SchemaSettings> for SchemaSettingsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SchemaSettings {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SchemaSettings {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaSettingsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SchemaSettingsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SchemaSettingsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaSettings>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SchemaSettingsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SchemaSettingsMut<'msg> {
  type Message = SchemaSettings;
}

impl ::std::fmt::Debug for SchemaSettingsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaSettings>> for SchemaSettingsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaSettings>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SchemaSettingsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SchemaSettings> {
    self.inner
  }

  pub fn to_owned(&self) -> SchemaSettings {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // schema: optional string
  pub fn schema(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_schema(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // encoding: optional enum google.pubsub.v1.Encoding
  pub fn encoding(&self) -> super::Encoding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::Encoding::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encoding(&mut self, val: super::Encoding) {
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

}

// SAFETY:
// - `SchemaSettingsMut` does not perform any shared mutation.
unsafe impl Send for SchemaSettingsMut<'_> {}

// SAFETY:
// - `SchemaSettingsMut` does not perform any shared mutation.
unsafe impl Sync for SchemaSettingsMut<'_> {}

impl<'msg> ::protobuf::AsView for SchemaSettingsMut<'msg> {
  type Proxied = SchemaSettings;
  fn as_view(&self) -> ::protobuf::View<'_, SchemaSettings> {
    SchemaSettingsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SchemaSettingsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SchemaSettings>
  where
      'msg: 'shorter {
    SchemaSettingsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SchemaSettingsMut<'msg> {
  type MutProxied = SchemaSettings;
  fn as_mut(&mut self) -> SchemaSettingsMut<'msg> {
    SchemaSettingsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SchemaSettingsMut<'msg> {
  fn into_mut<'shorter>(self) -> SchemaSettingsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SchemaSettings {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SchemaSettings> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SchemaSettingsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SchemaSettingsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // schema: optional string
  pub fn schema(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_schema(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // encoding: optional enum google.pubsub.v1.Encoding
  pub fn encoding(&self) -> super::Encoding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::Encoding::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_encoding(&mut self, val: super::Encoding) {
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

}  // impl SchemaSettings

impl ::std::ops::Drop for SchemaSettings {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SchemaSettings {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SchemaSettings {
  type Proxied = Self;
  fn as_view(&self) -> SchemaSettingsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SchemaSettings {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SchemaSettingsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SchemaSettings {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__SchemaSettings_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__SchemaSettings_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__SchemaSettings_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaSettings {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaSettings {
  type Msg = SchemaSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaSettings {
  type Msg = SchemaSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SchemaSettingsMut<'_> {
  type Msg = SchemaSettings;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaSettings> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaSettingsMut<'_> {
  type Msg = SchemaSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaSettings> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SchemaSettingsView<'_> {
  type Msg = SchemaSettings;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SchemaSettings> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SchemaSettingsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__Topic_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Topic {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Topic>
}

impl ::protobuf::Message for Topic {}

impl ::std::default::Default for Topic {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Topic {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Topic` is `Sync` because it does not implement interior mutability.
//    Neither does `TopicMut`.
unsafe impl Sync for Topic {}

// SAFETY:
// - `Topic` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Topic {}

impl ::protobuf::Proxied for Topic {
  type View<'msg> = TopicView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Topic {}

impl ::protobuf::MutProxied for Topic {
  type Mut<'msg> = TopicMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TopicView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Topic>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TopicView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TopicView<'msg> {
  type Message = Topic;
}

impl ::std::fmt::Debug for TopicView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TopicView<'_> {
  fn default() -> TopicView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Topic>> for TopicView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Topic>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TopicView<'msg> {

  pub fn to_owned(&self) -> Topic {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // labels: repeated message google.pubsub.v1.Topic.LabelsEntry
  pub fn labels(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // message_storage_policy: optional message google.pubsub.v1.MessageStoragePolicy
  pub fn has_message_storage_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn message_storage_policy_opt(self) -> ::protobuf::Optional<super::MessageStoragePolicyView<'msg>> {
        ::protobuf::Optional::new(self.message_storage_policy(), self.has_message_storage_policy())
  }
  pub fn message_storage_policy(self) -> super::MessageStoragePolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MessageStoragePolicyView::default())
  }

  // kms_key_name: optional string
  pub fn kms_key_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // schema_settings: optional message google.pubsub.v1.SchemaSettings
  pub fn has_schema_settings(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn schema_settings_opt(self) -> ::protobuf::Optional<super::SchemaSettingsView<'msg>> {
        ::protobuf::Optional::new(self.schema_settings(), self.has_schema_settings())
  }
  pub fn schema_settings(self) -> super::SchemaSettingsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaSettingsView::default())
  }

  // satisfies_pzs: optional bool
  pub fn satisfies_pzs(self) -> bool {
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
// - `TopicView` is `Sync` because it does not support mutation.
unsafe impl Sync for TopicView<'_> {}

// SAFETY:
// - `TopicView` is `Send` because while its alive a `TopicMut` cannot.
// - `TopicView` does not use thread-local data.
unsafe impl Send for TopicView<'_> {}

impl<'msg> ::protobuf::AsView for TopicView<'msg> {
  type Proxied = Topic;
  fn as_view(&self) -> ::protobuf::View<'msg, Topic> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TopicView<'msg> {
  fn into_view<'shorter>(self) -> TopicView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Topic> for TopicView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Topic {
    let mut dst = Topic::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Topic> for TopicMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Topic {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Topic {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for TopicView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for TopicMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TopicMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Topic>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TopicMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TopicMut<'msg> {
  type Message = Topic;
}

impl ::std::fmt::Debug for TopicMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Topic>> for TopicMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Topic>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TopicMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Topic> {
    self.inner
  }

  pub fn to_owned(&self) -> Topic {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // labels: repeated message google.pubsub.v1.Topic.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // message_storage_policy: optional message google.pubsub.v1.MessageStoragePolicy
  pub fn has_message_storage_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_message_storage_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn message_storage_policy_opt(&self) -> ::protobuf::Optional<super::MessageStoragePolicyView<'_>> {
        ::protobuf::Optional::new(self.message_storage_policy(), self.has_message_storage_policy())
  }
  pub fn message_storage_policy(&self) -> super::MessageStoragePolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MessageStoragePolicyView::default())
  }
  pub fn message_storage_policy_mut(&mut self) -> super::MessageStoragePolicyMut<'_> {
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
  pub fn set_message_storage_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::MessageStoragePolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // kms_key_name: optional string
  pub fn kms_key_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_kms_key_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // schema_settings: optional message google.pubsub.v1.SchemaSettings
  pub fn has_schema_settings(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_schema_settings(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn schema_settings_opt(&self) -> ::protobuf::Optional<super::SchemaSettingsView<'_>> {
        ::protobuf::Optional::new(self.schema_settings(), self.has_schema_settings())
  }
  pub fn schema_settings(&self) -> super::SchemaSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaSettingsView::default())
  }
  pub fn schema_settings_mut(&mut self) -> super::SchemaSettingsMut<'_> {
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
  pub fn set_schema_settings(&mut self,
    val: impl ::protobuf::IntoProxied<super::SchemaSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // satisfies_pzs: optional bool
  pub fn satisfies_pzs(&self) -> bool {
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
  pub fn set_satisfies_pzs(&mut self, val: bool) {
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
// - `TopicMut` does not perform any shared mutation.
unsafe impl Send for TopicMut<'_> {}

// SAFETY:
// - `TopicMut` does not perform any shared mutation.
unsafe impl Sync for TopicMut<'_> {}

impl<'msg> ::protobuf::AsView for TopicMut<'msg> {
  type Proxied = Topic;
  fn as_view(&self) -> ::protobuf::View<'_, Topic> {
    TopicView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TopicMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Topic>
  where
      'msg: 'shorter {
    TopicView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for TopicMut<'msg> {
  type MutProxied = Topic;
  fn as_mut(&mut self) -> TopicMut<'msg> {
    TopicMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TopicMut<'msg> {
  fn into_mut<'shorter>(self) -> TopicMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Topic {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Topic> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TopicView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TopicMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // labels: repeated message google.pubsub.v1.Topic.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // message_storage_policy: optional message google.pubsub.v1.MessageStoragePolicy
  pub fn has_message_storage_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_message_storage_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn message_storage_policy_opt(&self) -> ::protobuf::Optional<super::MessageStoragePolicyView<'_>> {
        ::protobuf::Optional::new(self.message_storage_policy(), self.has_message_storage_policy())
  }
  pub fn message_storage_policy(&self) -> super::MessageStoragePolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MessageStoragePolicyView::default())
  }
  pub fn message_storage_policy_mut(&mut self) -> super::MessageStoragePolicyMut<'_> {
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
  pub fn set_message_storage_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::MessageStoragePolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // kms_key_name: optional string
  pub fn kms_key_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_kms_key_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // schema_settings: optional message google.pubsub.v1.SchemaSettings
  pub fn has_schema_settings(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_schema_settings(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn schema_settings_opt(&self) -> ::protobuf::Optional<super::SchemaSettingsView<'_>> {
        ::protobuf::Optional::new(self.schema_settings(), self.has_schema_settings())
  }
  pub fn schema_settings(&self) -> super::SchemaSettingsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SchemaSettingsView::default())
  }
  pub fn schema_settings_mut(&mut self) -> super::SchemaSettingsMut<'_> {
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
  pub fn set_schema_settings(&mut self,
    val: impl ::protobuf::IntoProxied<super::SchemaSettings>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // satisfies_pzs: optional bool
  pub fn satisfies_pzs(&self) -> bool {
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
  pub fn set_satisfies_pzs(&mut self, val: bool) {
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

}  // impl Topic

impl ::std::ops::Drop for Topic {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Topic {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Topic {
  type Proxied = Self;
  fn as_view(&self) -> TopicView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Topic {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TopicMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Topic {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__Topic_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG3a1X3/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__Topic_msg_init.0, &[<super::topic::LabelsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::MessageStoragePolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SchemaSettings as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__Topic_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Topic {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Topic {
  type Msg = Topic;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Topic> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Topic {
  type Msg = Topic;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Topic> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TopicMut<'_> {
  type Msg = Topic;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Topic> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TopicMut<'_> {
  type Msg = Topic;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Topic> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TopicView<'_> {
  type Msg = Topic;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Topic> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TopicMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod topic {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__Topic__LabelsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct LabelsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LabelsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::topic::google__pubsub__v1__Topic__LabelsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::topic::google__pubsub__v1__Topic__LabelsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::topic::google__pubsub__v1__Topic__LabelsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod topic


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__PubsubMessage_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PubsubMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PubsubMessage>
}

impl ::protobuf::Message for PubsubMessage {}

impl ::std::default::Default for PubsubMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PubsubMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PubsubMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `PubsubMessageMut`.
unsafe impl Sync for PubsubMessage {}

// SAFETY:
// - `PubsubMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PubsubMessage {}

impl ::protobuf::Proxied for PubsubMessage {
  type View<'msg> = PubsubMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PubsubMessage {}

impl ::protobuf::MutProxied for PubsubMessage {
  type Mut<'msg> = PubsubMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PubsubMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PubsubMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PubsubMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PubsubMessageView<'msg> {
  type Message = PubsubMessage;
}

impl ::std::fmt::Debug for PubsubMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PubsubMessageView<'_> {
  fn default() -> PubsubMessageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PubsubMessage>> for PubsubMessageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PubsubMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PubsubMessageView<'msg> {

  pub fn to_owned(&self) -> PubsubMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // data: optional bytes
  pub fn data(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // attributes: repeated message google.pubsub.v1.PubsubMessage.AttributesEntry
  pub fn attributes(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // message_id: optional string
  pub fn message_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // publish_time: optional message google.protobuf.Timestamp
  pub fn has_publish_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn publish_time_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.publish_time(), self.has_publish_time())
  }
  pub fn publish_time(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // ordering_key: optional string
  pub fn ordering_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `PubsubMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for PubsubMessageView<'_> {}

// SAFETY:
// - `PubsubMessageView` is `Send` because while its alive a `PubsubMessageMut` cannot.
// - `PubsubMessageView` does not use thread-local data.
unsafe impl Send for PubsubMessageView<'_> {}

impl<'msg> ::protobuf::AsView for PubsubMessageView<'msg> {
  type Proxied = PubsubMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, PubsubMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PubsubMessageView<'msg> {
  fn into_view<'shorter>(self) -> PubsubMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PubsubMessage> for PubsubMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PubsubMessage {
    let mut dst = PubsubMessage::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PubsubMessage> for PubsubMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PubsubMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PubsubMessage {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PubsubMessageView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PubsubMessageMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PubsubMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PubsubMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PubsubMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PubsubMessageMut<'msg> {
  type Message = PubsubMessage;
}

impl ::std::fmt::Debug for PubsubMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PubsubMessage>> for PubsubMessageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PubsubMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PubsubMessageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PubsubMessage> {
    self.inner
  }

  pub fn to_owned(&self) -> PubsubMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // data: optional bytes
  pub fn data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // attributes: repeated message google.pubsub.v1.PubsubMessage.AttributesEntry
  pub fn attributes(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn attributes_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_attributes(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // message_id: optional string
  pub fn message_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // publish_time: optional message google.protobuf.Timestamp
  pub fn has_publish_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_publish_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn publish_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.publish_time(), self.has_publish_time())
  }
  pub fn publish_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn publish_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_publish_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // ordering_key: optional string
  pub fn ordering_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_ordering_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

}

// SAFETY:
// - `PubsubMessageMut` does not perform any shared mutation.
unsafe impl Send for PubsubMessageMut<'_> {}

// SAFETY:
// - `PubsubMessageMut` does not perform any shared mutation.
unsafe impl Sync for PubsubMessageMut<'_> {}

impl<'msg> ::protobuf::AsView for PubsubMessageMut<'msg> {
  type Proxied = PubsubMessage;
  fn as_view(&self) -> ::protobuf::View<'_, PubsubMessage> {
    PubsubMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PubsubMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PubsubMessage>
  where
      'msg: 'shorter {
    PubsubMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PubsubMessageMut<'msg> {
  type MutProxied = PubsubMessage;
  fn as_mut(&mut self) -> PubsubMessageMut<'msg> {
    PubsubMessageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PubsubMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> PubsubMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PubsubMessage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PubsubMessage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PubsubMessageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PubsubMessageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // data: optional bytes
  pub fn data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // attributes: repeated message google.pubsub.v1.PubsubMessage.AttributesEntry
  pub fn attributes(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn attributes_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_attributes(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // message_id: optional string
  pub fn message_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_message_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // publish_time: optional message google.protobuf.Timestamp
  pub fn has_publish_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_publish_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn publish_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.publish_time(), self.has_publish_time())
  }
  pub fn publish_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn publish_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_publish_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // ordering_key: optional string
  pub fn ordering_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_ordering_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

}  // impl PubsubMessage

impl ::std::ops::Drop for PubsubMessage {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PubsubMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PubsubMessage {
  type Proxied = Self;
  fn as_view(&self) -> PubsubMessageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PubsubMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PubsubMessageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PubsubMessage {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__PubsubMessage_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$0PG1X31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__PubsubMessage_msg_init.0, &[<super::pubsub_message::AttributesEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__PubsubMessage_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PubsubMessage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PubsubMessage {
  type Msg = PubsubMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PubsubMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PubsubMessage {
  type Msg = PubsubMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PubsubMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PubsubMessageMut<'_> {
  type Msg = PubsubMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PubsubMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PubsubMessageMut<'_> {
  type Msg = PubsubMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PubsubMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PubsubMessageView<'_> {
  type Msg = PubsubMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PubsubMessage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PubsubMessageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod pubsub_message {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__PubsubMessage__AttributesEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct AttributesEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AttributesEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::pubsub_message::google__pubsub__v1__PubsubMessage__AttributesEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::pubsub_message::google__pubsub__v1__PubsubMessage__AttributesEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::pubsub_message::google__pubsub__v1__PubsubMessage__AttributesEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod pubsub_message


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__GetTopicRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetTopicRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetTopicRequest>
}

impl ::protobuf::Message for GetTopicRequest {}

impl ::std::default::Default for GetTopicRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetTopicRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetTopicRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetTopicRequestMut`.
unsafe impl Sync for GetTopicRequest {}

// SAFETY:
// - `GetTopicRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetTopicRequest {}

impl ::protobuf::Proxied for GetTopicRequest {
  type View<'msg> = GetTopicRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetTopicRequest {}

impl ::protobuf::MutProxied for GetTopicRequest {
  type Mut<'msg> = GetTopicRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetTopicRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetTopicRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetTopicRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetTopicRequestView<'msg> {
  type Message = GetTopicRequest;
}

impl ::std::fmt::Debug for GetTopicRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetTopicRequestView<'_> {
  fn default() -> GetTopicRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetTopicRequest>> for GetTopicRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetTopicRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetTopicRequestView<'msg> {

  pub fn to_owned(&self) -> GetTopicRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // topic: optional string
  pub fn topic(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetTopicRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetTopicRequestView<'_> {}

// SAFETY:
// - `GetTopicRequestView` is `Send` because while its alive a `GetTopicRequestMut` cannot.
// - `GetTopicRequestView` does not use thread-local data.
unsafe impl Send for GetTopicRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetTopicRequestView<'msg> {
  type Proxied = GetTopicRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetTopicRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetTopicRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetTopicRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetTopicRequest> for GetTopicRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetTopicRequest {
    let mut dst = GetTopicRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetTopicRequest> for GetTopicRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetTopicRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetTopicRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetTopicRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetTopicRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetTopicRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetTopicRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetTopicRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetTopicRequestMut<'msg> {
  type Message = GetTopicRequest;
}

impl ::std::fmt::Debug for GetTopicRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetTopicRequest>> for GetTopicRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetTopicRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetTopicRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetTopicRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetTopicRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `GetTopicRequestMut` does not perform any shared mutation.
unsafe impl Send for GetTopicRequestMut<'_> {}

// SAFETY:
// - `GetTopicRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetTopicRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetTopicRequestMut<'msg> {
  type Proxied = GetTopicRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetTopicRequest> {
    GetTopicRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetTopicRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetTopicRequest>
  where
      'msg: 'shorter {
    GetTopicRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetTopicRequestMut<'msg> {
  type MutProxied = GetTopicRequest;
  fn as_mut(&mut self) -> GetTopicRequestMut<'msg> {
    GetTopicRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetTopicRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetTopicRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetTopicRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetTopicRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetTopicRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetTopicRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl GetTopicRequest

impl ::std::ops::Drop for GetTopicRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetTopicRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetTopicRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetTopicRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetTopicRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetTopicRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetTopicRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__GetTopicRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__GetTopicRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__GetTopicRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetTopicRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetTopicRequest {
  type Msg = GetTopicRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetTopicRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetTopicRequest {
  type Msg = GetTopicRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetTopicRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetTopicRequestMut<'_> {
  type Msg = GetTopicRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetTopicRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetTopicRequestMut<'_> {
  type Msg = GetTopicRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetTopicRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetTopicRequestView<'_> {
  type Msg = GetTopicRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetTopicRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetTopicRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__UpdateTopicRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpdateTopicRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpdateTopicRequest>
}

impl ::protobuf::Message for UpdateTopicRequest {}

impl ::std::default::Default for UpdateTopicRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpdateTopicRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpdateTopicRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `UpdateTopicRequestMut`.
unsafe impl Sync for UpdateTopicRequest {}

// SAFETY:
// - `UpdateTopicRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UpdateTopicRequest {}

impl ::protobuf::Proxied for UpdateTopicRequest {
  type View<'msg> = UpdateTopicRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpdateTopicRequest {}

impl ::protobuf::MutProxied for UpdateTopicRequest {
  type Mut<'msg> = UpdateTopicRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpdateTopicRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateTopicRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateTopicRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpdateTopicRequestView<'msg> {
  type Message = UpdateTopicRequest;
}

impl ::std::fmt::Debug for UpdateTopicRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpdateTopicRequestView<'_> {
  fn default() -> UpdateTopicRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateTopicRequest>> for UpdateTopicRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateTopicRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateTopicRequestView<'msg> {

  pub fn to_owned(&self) -> UpdateTopicRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // topic: optional message google.pubsub.v1.Topic
  pub fn has_topic(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn topic_opt(self) -> ::protobuf::Optional<super::TopicView<'msg>> {
        ::protobuf::Optional::new(self.topic(), self.has_topic())
  }
  pub fn topic(self) -> super::TopicView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TopicView::default())
  }

  // update_mask: optional message google.protobuf.FieldMask
  pub fn has_update_mask(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn update_mask_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::FieldMaskView<'msg>> {
        ::protobuf::Optional::new(self.update_mask(), self.has_update_mask())
  }
  pub fn update_mask(self) -> ::protobuf_well_known_types::FieldMaskView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FieldMaskView::default())
  }

}

// SAFETY:
// - `UpdateTopicRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for UpdateTopicRequestView<'_> {}

// SAFETY:
// - `UpdateTopicRequestView` is `Send` because while its alive a `UpdateTopicRequestMut` cannot.
// - `UpdateTopicRequestView` does not use thread-local data.
unsafe impl Send for UpdateTopicRequestView<'_> {}

impl<'msg> ::protobuf::AsView for UpdateTopicRequestView<'msg> {
  type Proxied = UpdateTopicRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, UpdateTopicRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateTopicRequestView<'msg> {
  fn into_view<'shorter>(self) -> UpdateTopicRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpdateTopicRequest> for UpdateTopicRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpdateTopicRequest {
    let mut dst = UpdateTopicRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpdateTopicRequest> for UpdateTopicRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpdateTopicRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for UpdateTopicRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UpdateTopicRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UpdateTopicRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpdateTopicRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateTopicRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateTopicRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpdateTopicRequestMut<'msg> {
  type Message = UpdateTopicRequest;
}

impl ::std::fmt::Debug for UpdateTopicRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateTopicRequest>> for UpdateTopicRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateTopicRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateTopicRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateTopicRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> UpdateTopicRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // topic: optional message google.pubsub.v1.Topic
  pub fn has_topic(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_topic(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn topic_opt(&self) -> ::protobuf::Optional<super::TopicView<'_>> {
        ::protobuf::Optional::new(self.topic(), self.has_topic())
  }
  pub fn topic(&self) -> super::TopicView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TopicView::default())
  }
  pub fn topic_mut(&mut self) -> super::TopicMut<'_> {
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
  pub fn set_topic(&mut self,
    val: impl ::protobuf::IntoProxied<super::Topic>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // update_mask: optional message google.protobuf.FieldMask
  pub fn has_update_mask(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_update_mask(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn update_mask_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::FieldMaskView<'_>> {
        ::protobuf::Optional::new(self.update_mask(), self.has_update_mask())
  }
  pub fn update_mask(&self) -> ::protobuf_well_known_types::FieldMaskView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FieldMaskView::default())
  }
  pub fn update_mask_mut(&mut self) -> ::protobuf_well_known_types::FieldMaskMut<'_> {
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
  pub fn set_update_mask(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::FieldMask>) {

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
// - `UpdateTopicRequestMut` does not perform any shared mutation.
unsafe impl Send for UpdateTopicRequestMut<'_> {}

// SAFETY:
// - `UpdateTopicRequestMut` does not perform any shared mutation.
unsafe impl Sync for UpdateTopicRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for UpdateTopicRequestMut<'msg> {
  type Proxied = UpdateTopicRequest;
  fn as_view(&self) -> ::protobuf::View<'_, UpdateTopicRequest> {
    UpdateTopicRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateTopicRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpdateTopicRequest>
  where
      'msg: 'shorter {
    UpdateTopicRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for UpdateTopicRequestMut<'msg> {
  type MutProxied = UpdateTopicRequest;
  fn as_mut(&mut self) -> UpdateTopicRequestMut<'msg> {
    UpdateTopicRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpdateTopicRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> UpdateTopicRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpdateTopicRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpdateTopicRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpdateTopicRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpdateTopicRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // topic: optional message google.pubsub.v1.Topic
  pub fn has_topic(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_topic(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn topic_opt(&self) -> ::protobuf::Optional<super::TopicView<'_>> {
        ::protobuf::Optional::new(self.topic(), self.has_topic())
  }
  pub fn topic(&self) -> super::TopicView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TopicView::default())
  }
  pub fn topic_mut(&mut self) -> super::TopicMut<'_> {
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
  pub fn set_topic(&mut self,
    val: impl ::protobuf::IntoProxied<super::Topic>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // update_mask: optional message google.protobuf.FieldMask
  pub fn has_update_mask(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_update_mask(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn update_mask_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::FieldMaskView<'_>> {
        ::protobuf::Optional::new(self.update_mask(), self.has_update_mask())
  }
  pub fn update_mask(&self) -> ::protobuf_well_known_types::FieldMaskView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FieldMaskView::default())
  }
  pub fn update_mask_mut(&mut self) -> ::protobuf_well_known_types::FieldMaskMut<'_> {
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
  pub fn set_update_mask(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::FieldMask>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl UpdateTopicRequest

impl ::std::ops::Drop for UpdateTopicRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpdateTopicRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpdateTopicRequest {
  type Proxied = Self;
  fn as_view(&self) -> UpdateTopicRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpdateTopicRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpdateTopicRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpdateTopicRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__UpdateTopicRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__UpdateTopicRequest_msg_init.0, &[<super::Topic as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::FieldMask as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__UpdateTopicRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdateTopicRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdateTopicRequest {
  type Msg = UpdateTopicRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateTopicRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateTopicRequest {
  type Msg = UpdateTopicRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateTopicRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdateTopicRequestMut<'_> {
  type Msg = UpdateTopicRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateTopicRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateTopicRequestMut<'_> {
  type Msg = UpdateTopicRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateTopicRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateTopicRequestView<'_> {
  type Msg = UpdateTopicRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateTopicRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdateTopicRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__PublishRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PublishRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PublishRequest>
}

impl ::protobuf::Message for PublishRequest {}

impl ::std::default::Default for PublishRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PublishRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PublishRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PublishRequestMut`.
unsafe impl Sync for PublishRequest {}

// SAFETY:
// - `PublishRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PublishRequest {}

impl ::protobuf::Proxied for PublishRequest {
  type View<'msg> = PublishRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PublishRequest {}

impl ::protobuf::MutProxied for PublishRequest {
  type Mut<'msg> = PublishRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PublishRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PublishRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PublishRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PublishRequestView<'msg> {
  type Message = PublishRequest;
}

impl ::std::fmt::Debug for PublishRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PublishRequestView<'_> {
  fn default() -> PublishRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PublishRequest>> for PublishRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PublishRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PublishRequestView<'msg> {

  pub fn to_owned(&self) -> PublishRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // topic: optional string
  pub fn topic(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // messages: repeated message google.pubsub.v1.PubsubMessage
  pub fn messages(self) -> ::protobuf::RepeatedView<'msg, super::PubsubMessage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PubsubMessage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PublishRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PublishRequestView<'_> {}

// SAFETY:
// - `PublishRequestView` is `Send` because while its alive a `PublishRequestMut` cannot.
// - `PublishRequestView` does not use thread-local data.
unsafe impl Send for PublishRequestView<'_> {}

impl<'msg> ::protobuf::AsView for PublishRequestView<'msg> {
  type Proxied = PublishRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PublishRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PublishRequestView<'msg> {
  fn into_view<'shorter>(self) -> PublishRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PublishRequest> for PublishRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PublishRequest {
    let mut dst = PublishRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PublishRequest> for PublishRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PublishRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PublishRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PublishRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PublishRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PublishRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PublishRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PublishRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PublishRequestMut<'msg> {
  type Message = PublishRequest;
}

impl ::std::fmt::Debug for PublishRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PublishRequest>> for PublishRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PublishRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PublishRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PublishRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> PublishRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // messages: repeated message google.pubsub.v1.PubsubMessage
  pub fn messages(&self) -> ::protobuf::RepeatedView<'_, super::PubsubMessage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PubsubMessage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn messages_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::PubsubMessage> {
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
  pub fn set_messages(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::PubsubMessage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `PublishRequestMut` does not perform any shared mutation.
unsafe impl Send for PublishRequestMut<'_> {}

// SAFETY:
// - `PublishRequestMut` does not perform any shared mutation.
unsafe impl Sync for PublishRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for PublishRequestMut<'msg> {
  type Proxied = PublishRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PublishRequest> {
    PublishRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PublishRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PublishRequest>
  where
      'msg: 'shorter {
    PublishRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PublishRequestMut<'msg> {
  type MutProxied = PublishRequest;
  fn as_mut(&mut self) -> PublishRequestMut<'msg> {
    PublishRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PublishRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PublishRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PublishRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PublishRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PublishRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PublishRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // messages: repeated message google.pubsub.v1.PubsubMessage
  pub fn messages(&self) -> ::protobuf::RepeatedView<'_, super::PubsubMessage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PubsubMessage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn messages_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::PubsubMessage> {
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
  pub fn set_messages(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::PubsubMessage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl PublishRequest

impl ::std::ops::Drop for PublishRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PublishRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PublishRequest {
  type Proxied = Self;
  fn as_view(&self) -> PublishRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PublishRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PublishRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PublishRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__PublishRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__PublishRequest_msg_init.0, &[<super::PubsubMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__PublishRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PublishRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PublishRequest {
  type Msg = PublishRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PublishRequest {
  type Msg = PublishRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PublishRequestMut<'_> {
  type Msg = PublishRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PublishRequestMut<'_> {
  type Msg = PublishRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PublishRequestView<'_> {
  type Msg = PublishRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PublishRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__PublishResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PublishResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PublishResponse>
}

impl ::protobuf::Message for PublishResponse {}

impl ::std::default::Default for PublishResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PublishResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PublishResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PublishResponseMut`.
unsafe impl Sync for PublishResponse {}

// SAFETY:
// - `PublishResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PublishResponse {}

impl ::protobuf::Proxied for PublishResponse {
  type View<'msg> = PublishResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PublishResponse {}

impl ::protobuf::MutProxied for PublishResponse {
  type Mut<'msg> = PublishResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PublishResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PublishResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PublishResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PublishResponseView<'msg> {
  type Message = PublishResponse;
}

impl ::std::fmt::Debug for PublishResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PublishResponseView<'_> {
  fn default() -> PublishResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PublishResponse>> for PublishResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PublishResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PublishResponseView<'msg> {

  pub fn to_owned(&self) -> PublishResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // message_ids: repeated string
  pub fn message_ids(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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
// - `PublishResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PublishResponseView<'_> {}

// SAFETY:
// - `PublishResponseView` is `Send` because while its alive a `PublishResponseMut` cannot.
// - `PublishResponseView` does not use thread-local data.
unsafe impl Send for PublishResponseView<'_> {}

impl<'msg> ::protobuf::AsView for PublishResponseView<'msg> {
  type Proxied = PublishResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PublishResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PublishResponseView<'msg> {
  fn into_view<'shorter>(self) -> PublishResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PublishResponse> for PublishResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PublishResponse {
    let mut dst = PublishResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PublishResponse> for PublishResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PublishResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PublishResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PublishResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PublishResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PublishResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PublishResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PublishResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PublishResponseMut<'msg> {
  type Message = PublishResponse;
}

impl ::std::fmt::Debug for PublishResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PublishResponse>> for PublishResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PublishResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PublishResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PublishResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> PublishResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // message_ids: repeated string
  pub fn message_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn message_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_message_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `PublishResponseMut` does not perform any shared mutation.
unsafe impl Send for PublishResponseMut<'_> {}

// SAFETY:
// - `PublishResponseMut` does not perform any shared mutation.
unsafe impl Sync for PublishResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for PublishResponseMut<'msg> {
  type Proxied = PublishResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PublishResponse> {
    PublishResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PublishResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PublishResponse>
  where
      'msg: 'shorter {
    PublishResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PublishResponseMut<'msg> {
  type MutProxied = PublishResponse;
  fn as_mut(&mut self) -> PublishResponseMut<'msg> {
    PublishResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PublishResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PublishResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PublishResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PublishResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PublishResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PublishResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // message_ids: repeated string
  pub fn message_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn message_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_message_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl PublishResponse

impl ::std::ops::Drop for PublishResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PublishResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PublishResponse {
  type Proxied = Self;
  fn as_view(&self) -> PublishResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PublishResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PublishResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PublishResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__PublishResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ME");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__PublishResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__PublishResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PublishResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PublishResponse {
  type Msg = PublishResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PublishResponse {
  type Msg = PublishResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PublishResponseMut<'_> {
  type Msg = PublishResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PublishResponseMut<'_> {
  type Msg = PublishResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PublishResponseView<'_> {
  type Msg = PublishResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PublishResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PublishResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListTopicsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListTopicsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListTopicsRequest>
}

impl ::protobuf::Message for ListTopicsRequest {}

impl ::std::default::Default for ListTopicsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListTopicsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListTopicsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ListTopicsRequestMut`.
unsafe impl Sync for ListTopicsRequest {}

// SAFETY:
// - `ListTopicsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListTopicsRequest {}

impl ::protobuf::Proxied for ListTopicsRequest {
  type View<'msg> = ListTopicsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListTopicsRequest {}

impl ::protobuf::MutProxied for ListTopicsRequest {
  type Mut<'msg> = ListTopicsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListTopicsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListTopicsRequestView<'msg> {
  type Message = ListTopicsRequest;
}

impl ::std::fmt::Debug for ListTopicsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListTopicsRequestView<'_> {
  fn default() -> ListTopicsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicsRequest>> for ListTopicsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicsRequestView<'msg> {

  pub fn to_owned(&self) -> ListTopicsRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // project: optional string
  pub fn project(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // page_size: optional int32
  pub fn page_size(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // page_token: optional string
  pub fn page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListTopicsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListTopicsRequestView<'_> {}

// SAFETY:
// - `ListTopicsRequestView` is `Send` because while its alive a `ListTopicsRequestMut` cannot.
// - `ListTopicsRequestView` does not use thread-local data.
unsafe impl Send for ListTopicsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicsRequestView<'msg> {
  type Proxied = ListTopicsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ListTopicsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicsRequestView<'msg> {
  fn into_view<'shorter>(self) -> ListTopicsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicsRequest> for ListTopicsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicsRequest {
    let mut dst = ListTopicsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicsRequest> for ListTopicsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListTopicsRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicsRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListTopicsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListTopicsRequestMut<'msg> {
  type Message = ListTopicsRequest;
}

impl ::std::fmt::Debug for ListTopicsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicsRequest>> for ListTopicsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicsRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ListTopicsRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // project: optional string
  pub fn project(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_project(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `ListTopicsRequestMut` does not perform any shared mutation.
unsafe impl Send for ListTopicsRequestMut<'_> {}

// SAFETY:
// - `ListTopicsRequestMut` does not perform any shared mutation.
unsafe impl Sync for ListTopicsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicsRequestMut<'msg> {
  type Proxied = ListTopicsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ListTopicsRequest> {
    ListTopicsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListTopicsRequest>
  where
      'msg: 'shorter {
    ListTopicsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListTopicsRequestMut<'msg> {
  type MutProxied = ListTopicsRequest;
  fn as_mut(&mut self) -> ListTopicsRequestMut<'msg> {
    ListTopicsRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListTopicsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ListTopicsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListTopicsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListTopicsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListTopicsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListTopicsRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // project: optional string
  pub fn project(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_project(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl ListTopicsRequest

impl ::std::ops::Drop for ListTopicsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListTopicsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListTopicsRequest {
  type Proxied = Self;
  fn as_view(&self) -> ListTopicsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListTopicsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListTopicsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListTopicsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListTopicsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X(P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListTopicsRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListTopicsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicsRequest {
  type Msg = ListTopicsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicsRequest {
  type Msg = ListTopicsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicsRequestMut<'_> {
  type Msg = ListTopicsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicsRequestMut<'_> {
  type Msg = ListTopicsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicsRequestView<'_> {
  type Msg = ListTopicsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListTopicsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListTopicsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListTopicsResponse>
}

impl ::protobuf::Message for ListTopicsResponse {}

impl ::std::default::Default for ListTopicsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListTopicsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListTopicsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ListTopicsResponseMut`.
unsafe impl Sync for ListTopicsResponse {}

// SAFETY:
// - `ListTopicsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListTopicsResponse {}

impl ::protobuf::Proxied for ListTopicsResponse {
  type View<'msg> = ListTopicsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListTopicsResponse {}

impl ::protobuf::MutProxied for ListTopicsResponse {
  type Mut<'msg> = ListTopicsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListTopicsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListTopicsResponseView<'msg> {
  type Message = ListTopicsResponse;
}

impl ::std::fmt::Debug for ListTopicsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListTopicsResponseView<'_> {
  fn default() -> ListTopicsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicsResponse>> for ListTopicsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicsResponseView<'msg> {

  pub fn to_owned(&self) -> ListTopicsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // topics: repeated message google.pubsub.v1.Topic
  pub fn topics(self) -> ::protobuf::RepeatedView<'msg, super::Topic> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Topic>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // next_page_token: optional string
  pub fn next_page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListTopicsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListTopicsResponseView<'_> {}

// SAFETY:
// - `ListTopicsResponseView` is `Send` because while its alive a `ListTopicsResponseMut` cannot.
// - `ListTopicsResponseView` does not use thread-local data.
unsafe impl Send for ListTopicsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicsResponseView<'msg> {
  type Proxied = ListTopicsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ListTopicsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicsResponseView<'msg> {
  fn into_view<'shorter>(self) -> ListTopicsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicsResponse> for ListTopicsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicsResponse {
    let mut dst = ListTopicsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicsResponse> for ListTopicsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListTopicsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListTopicsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListTopicsResponseMut<'msg> {
  type Message = ListTopicsResponse;
}

impl ::std::fmt::Debug for ListTopicsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicsResponse>> for ListTopicsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ListTopicsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // topics: repeated message google.pubsub.v1.Topic
  pub fn topics(&self) -> ::protobuf::RepeatedView<'_, super::Topic> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Topic>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn topics_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Topic> {
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
  pub fn set_topics(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Topic>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ListTopicsResponseMut` does not perform any shared mutation.
unsafe impl Send for ListTopicsResponseMut<'_> {}

// SAFETY:
// - `ListTopicsResponseMut` does not perform any shared mutation.
unsafe impl Sync for ListTopicsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicsResponseMut<'msg> {
  type Proxied = ListTopicsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ListTopicsResponse> {
    ListTopicsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListTopicsResponse>
  where
      'msg: 'shorter {
    ListTopicsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListTopicsResponseMut<'msg> {
  type MutProxied = ListTopicsResponse;
  fn as_mut(&mut self) -> ListTopicsResponseMut<'msg> {
    ListTopicsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListTopicsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ListTopicsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListTopicsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListTopicsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListTopicsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListTopicsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // topics: repeated message google.pubsub.v1.Topic
  pub fn topics(&self) -> ::protobuf::RepeatedView<'_, super::Topic> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Topic>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn topics_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Topic> {
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
  pub fn set_topics(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Topic>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl ListTopicsResponse

impl ::std::ops::Drop for ListTopicsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListTopicsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListTopicsResponse {
  type Proxied = Self;
  fn as_view(&self) -> ListTopicsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListTopicsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListTopicsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListTopicsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListTopicsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListTopicsResponse_msg_init.0, &[<super::Topic as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListTopicsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicsResponse {
  type Msg = ListTopicsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicsResponse {
  type Msg = ListTopicsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicsResponseMut<'_> {
  type Msg = ListTopicsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicsResponseMut<'_> {
  type Msg = ListTopicsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicsResponseView<'_> {
  type Msg = ListTopicsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListTopicSubscriptionsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListTopicSubscriptionsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListTopicSubscriptionsRequest>
}

impl ::protobuf::Message for ListTopicSubscriptionsRequest {}

impl ::std::default::Default for ListTopicSubscriptionsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListTopicSubscriptionsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListTopicSubscriptionsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ListTopicSubscriptionsRequestMut`.
unsafe impl Sync for ListTopicSubscriptionsRequest {}

// SAFETY:
// - `ListTopicSubscriptionsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListTopicSubscriptionsRequest {}

impl ::protobuf::Proxied for ListTopicSubscriptionsRequest {
  type View<'msg> = ListTopicSubscriptionsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListTopicSubscriptionsRequest {}

impl ::protobuf::MutProxied for ListTopicSubscriptionsRequest {
  type Mut<'msg> = ListTopicSubscriptionsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListTopicSubscriptionsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSubscriptionsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicSubscriptionsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListTopicSubscriptionsRequestView<'msg> {
  type Message = ListTopicSubscriptionsRequest;
}

impl ::std::fmt::Debug for ListTopicSubscriptionsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListTopicSubscriptionsRequestView<'_> {
  fn default() -> ListTopicSubscriptionsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSubscriptionsRequest>> for ListTopicSubscriptionsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSubscriptionsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicSubscriptionsRequestView<'msg> {

  pub fn to_owned(&self) -> ListTopicSubscriptionsRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // topic: optional string
  pub fn topic(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // page_size: optional int32
  pub fn page_size(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // page_token: optional string
  pub fn page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListTopicSubscriptionsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListTopicSubscriptionsRequestView<'_> {}

// SAFETY:
// - `ListTopicSubscriptionsRequestView` is `Send` because while its alive a `ListTopicSubscriptionsRequestMut` cannot.
// - `ListTopicSubscriptionsRequestView` does not use thread-local data.
unsafe impl Send for ListTopicSubscriptionsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicSubscriptionsRequestView<'msg> {
  type Proxied = ListTopicSubscriptionsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ListTopicSubscriptionsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicSubscriptionsRequestView<'msg> {
  fn into_view<'shorter>(self) -> ListTopicSubscriptionsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicSubscriptionsRequest> for ListTopicSubscriptionsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicSubscriptionsRequest {
    let mut dst = ListTopicSubscriptionsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicSubscriptionsRequest> for ListTopicSubscriptionsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicSubscriptionsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListTopicSubscriptionsRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicSubscriptionsRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicSubscriptionsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListTopicSubscriptionsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSubscriptionsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicSubscriptionsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListTopicSubscriptionsRequestMut<'msg> {
  type Message = ListTopicSubscriptionsRequest;
}

impl ::std::fmt::Debug for ListTopicSubscriptionsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSubscriptionsRequest>> for ListTopicSubscriptionsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSubscriptionsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicSubscriptionsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSubscriptionsRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ListTopicSubscriptionsRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `ListTopicSubscriptionsRequestMut` does not perform any shared mutation.
unsafe impl Send for ListTopicSubscriptionsRequestMut<'_> {}

// SAFETY:
// - `ListTopicSubscriptionsRequestMut` does not perform any shared mutation.
unsafe impl Sync for ListTopicSubscriptionsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicSubscriptionsRequestMut<'msg> {
  type Proxied = ListTopicSubscriptionsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ListTopicSubscriptionsRequest> {
    ListTopicSubscriptionsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicSubscriptionsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListTopicSubscriptionsRequest>
  where
      'msg: 'shorter {
    ListTopicSubscriptionsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListTopicSubscriptionsRequestMut<'msg> {
  type MutProxied = ListTopicSubscriptionsRequest;
  fn as_mut(&mut self) -> ListTopicSubscriptionsRequestMut<'msg> {
    ListTopicSubscriptionsRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListTopicSubscriptionsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ListTopicSubscriptionsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListTopicSubscriptionsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListTopicSubscriptionsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListTopicSubscriptionsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListTopicSubscriptionsRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl ListTopicSubscriptionsRequest

impl ::std::ops::Drop for ListTopicSubscriptionsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListTopicSubscriptionsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListTopicSubscriptionsRequest {
  type Proxied = Self;
  fn as_view(&self) -> ListTopicSubscriptionsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListTopicSubscriptionsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListTopicSubscriptionsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListTopicSubscriptionsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListTopicSubscriptionsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X(P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListTopicSubscriptionsRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListTopicSubscriptionsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicSubscriptionsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicSubscriptionsRequest {
  type Msg = ListTopicSubscriptionsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSubscriptionsRequest {
  type Msg = ListTopicSubscriptionsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicSubscriptionsRequestMut<'_> {
  type Msg = ListTopicSubscriptionsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSubscriptionsRequestMut<'_> {
  type Msg = ListTopicSubscriptionsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSubscriptionsRequestView<'_> {
  type Msg = ListTopicSubscriptionsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicSubscriptionsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListTopicSubscriptionsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListTopicSubscriptionsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListTopicSubscriptionsResponse>
}

impl ::protobuf::Message for ListTopicSubscriptionsResponse {}

impl ::std::default::Default for ListTopicSubscriptionsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListTopicSubscriptionsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListTopicSubscriptionsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ListTopicSubscriptionsResponseMut`.
unsafe impl Sync for ListTopicSubscriptionsResponse {}

// SAFETY:
// - `ListTopicSubscriptionsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListTopicSubscriptionsResponse {}

impl ::protobuf::Proxied for ListTopicSubscriptionsResponse {
  type View<'msg> = ListTopicSubscriptionsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListTopicSubscriptionsResponse {}

impl ::protobuf::MutProxied for ListTopicSubscriptionsResponse {
  type Mut<'msg> = ListTopicSubscriptionsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListTopicSubscriptionsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSubscriptionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicSubscriptionsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListTopicSubscriptionsResponseView<'msg> {
  type Message = ListTopicSubscriptionsResponse;
}

impl ::std::fmt::Debug for ListTopicSubscriptionsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListTopicSubscriptionsResponseView<'_> {
  fn default() -> ListTopicSubscriptionsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSubscriptionsResponse>> for ListTopicSubscriptionsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSubscriptionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicSubscriptionsResponseView<'msg> {

  pub fn to_owned(&self) -> ListTopicSubscriptionsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscriptions: repeated string
  pub fn subscriptions(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // next_page_token: optional string
  pub fn next_page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListTopicSubscriptionsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListTopicSubscriptionsResponseView<'_> {}

// SAFETY:
// - `ListTopicSubscriptionsResponseView` is `Send` because while its alive a `ListTopicSubscriptionsResponseMut` cannot.
// - `ListTopicSubscriptionsResponseView` does not use thread-local data.
unsafe impl Send for ListTopicSubscriptionsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicSubscriptionsResponseView<'msg> {
  type Proxied = ListTopicSubscriptionsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ListTopicSubscriptionsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicSubscriptionsResponseView<'msg> {
  fn into_view<'shorter>(self) -> ListTopicSubscriptionsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicSubscriptionsResponse> for ListTopicSubscriptionsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicSubscriptionsResponse {
    let mut dst = ListTopicSubscriptionsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicSubscriptionsResponse> for ListTopicSubscriptionsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicSubscriptionsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListTopicSubscriptionsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicSubscriptionsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicSubscriptionsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListTopicSubscriptionsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSubscriptionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicSubscriptionsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListTopicSubscriptionsResponseMut<'msg> {
  type Message = ListTopicSubscriptionsResponse;
}

impl ::std::fmt::Debug for ListTopicSubscriptionsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSubscriptionsResponse>> for ListTopicSubscriptionsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSubscriptionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicSubscriptionsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSubscriptionsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ListTopicSubscriptionsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscriptions: repeated string
  pub fn subscriptions(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn subscriptions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_subscriptions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ListTopicSubscriptionsResponseMut` does not perform any shared mutation.
unsafe impl Send for ListTopicSubscriptionsResponseMut<'_> {}

// SAFETY:
// - `ListTopicSubscriptionsResponseMut` does not perform any shared mutation.
unsafe impl Sync for ListTopicSubscriptionsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicSubscriptionsResponseMut<'msg> {
  type Proxied = ListTopicSubscriptionsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ListTopicSubscriptionsResponse> {
    ListTopicSubscriptionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicSubscriptionsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListTopicSubscriptionsResponse>
  where
      'msg: 'shorter {
    ListTopicSubscriptionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListTopicSubscriptionsResponseMut<'msg> {
  type MutProxied = ListTopicSubscriptionsResponse;
  fn as_mut(&mut self) -> ListTopicSubscriptionsResponseMut<'msg> {
    ListTopicSubscriptionsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListTopicSubscriptionsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ListTopicSubscriptionsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListTopicSubscriptionsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListTopicSubscriptionsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListTopicSubscriptionsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListTopicSubscriptionsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscriptions: repeated string
  pub fn subscriptions(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn subscriptions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_subscriptions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl ListTopicSubscriptionsResponse

impl ::std::ops::Drop for ListTopicSubscriptionsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListTopicSubscriptionsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListTopicSubscriptionsResponse {
  type Proxied = Self;
  fn as_view(&self) -> ListTopicSubscriptionsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListTopicSubscriptionsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListTopicSubscriptionsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListTopicSubscriptionsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListTopicSubscriptionsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ME1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListTopicSubscriptionsResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListTopicSubscriptionsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicSubscriptionsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicSubscriptionsResponse {
  type Msg = ListTopicSubscriptionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSubscriptionsResponse {
  type Msg = ListTopicSubscriptionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicSubscriptionsResponseMut<'_> {
  type Msg = ListTopicSubscriptionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSubscriptionsResponseMut<'_> {
  type Msg = ListTopicSubscriptionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSubscriptionsResponseView<'_> {
  type Msg = ListTopicSubscriptionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSubscriptionsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicSubscriptionsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListTopicSnapshotsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListTopicSnapshotsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListTopicSnapshotsRequest>
}

impl ::protobuf::Message for ListTopicSnapshotsRequest {}

impl ::std::default::Default for ListTopicSnapshotsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListTopicSnapshotsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListTopicSnapshotsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ListTopicSnapshotsRequestMut`.
unsafe impl Sync for ListTopicSnapshotsRequest {}

// SAFETY:
// - `ListTopicSnapshotsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListTopicSnapshotsRequest {}

impl ::protobuf::Proxied for ListTopicSnapshotsRequest {
  type View<'msg> = ListTopicSnapshotsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListTopicSnapshotsRequest {}

impl ::protobuf::MutProxied for ListTopicSnapshotsRequest {
  type Mut<'msg> = ListTopicSnapshotsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListTopicSnapshotsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSnapshotsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicSnapshotsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListTopicSnapshotsRequestView<'msg> {
  type Message = ListTopicSnapshotsRequest;
}

impl ::std::fmt::Debug for ListTopicSnapshotsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListTopicSnapshotsRequestView<'_> {
  fn default() -> ListTopicSnapshotsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSnapshotsRequest>> for ListTopicSnapshotsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSnapshotsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicSnapshotsRequestView<'msg> {

  pub fn to_owned(&self) -> ListTopicSnapshotsRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // topic: optional string
  pub fn topic(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // page_size: optional int32
  pub fn page_size(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // page_token: optional string
  pub fn page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListTopicSnapshotsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListTopicSnapshotsRequestView<'_> {}

// SAFETY:
// - `ListTopicSnapshotsRequestView` is `Send` because while its alive a `ListTopicSnapshotsRequestMut` cannot.
// - `ListTopicSnapshotsRequestView` does not use thread-local data.
unsafe impl Send for ListTopicSnapshotsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicSnapshotsRequestView<'msg> {
  type Proxied = ListTopicSnapshotsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ListTopicSnapshotsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicSnapshotsRequestView<'msg> {
  fn into_view<'shorter>(self) -> ListTopicSnapshotsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicSnapshotsRequest> for ListTopicSnapshotsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicSnapshotsRequest {
    let mut dst = ListTopicSnapshotsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicSnapshotsRequest> for ListTopicSnapshotsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicSnapshotsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListTopicSnapshotsRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicSnapshotsRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicSnapshotsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListTopicSnapshotsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSnapshotsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicSnapshotsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListTopicSnapshotsRequestMut<'msg> {
  type Message = ListTopicSnapshotsRequest;
}

impl ::std::fmt::Debug for ListTopicSnapshotsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSnapshotsRequest>> for ListTopicSnapshotsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSnapshotsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicSnapshotsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSnapshotsRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ListTopicSnapshotsRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `ListTopicSnapshotsRequestMut` does not perform any shared mutation.
unsafe impl Send for ListTopicSnapshotsRequestMut<'_> {}

// SAFETY:
// - `ListTopicSnapshotsRequestMut` does not perform any shared mutation.
unsafe impl Sync for ListTopicSnapshotsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicSnapshotsRequestMut<'msg> {
  type Proxied = ListTopicSnapshotsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ListTopicSnapshotsRequest> {
    ListTopicSnapshotsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicSnapshotsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListTopicSnapshotsRequest>
  where
      'msg: 'shorter {
    ListTopicSnapshotsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListTopicSnapshotsRequestMut<'msg> {
  type MutProxied = ListTopicSnapshotsRequest;
  fn as_mut(&mut self) -> ListTopicSnapshotsRequestMut<'msg> {
    ListTopicSnapshotsRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListTopicSnapshotsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ListTopicSnapshotsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListTopicSnapshotsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListTopicSnapshotsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListTopicSnapshotsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListTopicSnapshotsRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl ListTopicSnapshotsRequest

impl ::std::ops::Drop for ListTopicSnapshotsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListTopicSnapshotsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListTopicSnapshotsRequest {
  type Proxied = Self;
  fn as_view(&self) -> ListTopicSnapshotsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListTopicSnapshotsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListTopicSnapshotsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListTopicSnapshotsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListTopicSnapshotsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X(P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListTopicSnapshotsRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListTopicSnapshotsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicSnapshotsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicSnapshotsRequest {
  type Msg = ListTopicSnapshotsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSnapshotsRequest {
  type Msg = ListTopicSnapshotsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicSnapshotsRequestMut<'_> {
  type Msg = ListTopicSnapshotsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSnapshotsRequestMut<'_> {
  type Msg = ListTopicSnapshotsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSnapshotsRequestView<'_> {
  type Msg = ListTopicSnapshotsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicSnapshotsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListTopicSnapshotsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListTopicSnapshotsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListTopicSnapshotsResponse>
}

impl ::protobuf::Message for ListTopicSnapshotsResponse {}

impl ::std::default::Default for ListTopicSnapshotsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListTopicSnapshotsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListTopicSnapshotsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ListTopicSnapshotsResponseMut`.
unsafe impl Sync for ListTopicSnapshotsResponse {}

// SAFETY:
// - `ListTopicSnapshotsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListTopicSnapshotsResponse {}

impl ::protobuf::Proxied for ListTopicSnapshotsResponse {
  type View<'msg> = ListTopicSnapshotsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListTopicSnapshotsResponse {}

impl ::protobuf::MutProxied for ListTopicSnapshotsResponse {
  type Mut<'msg> = ListTopicSnapshotsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListTopicSnapshotsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSnapshotsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicSnapshotsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListTopicSnapshotsResponseView<'msg> {
  type Message = ListTopicSnapshotsResponse;
}

impl ::std::fmt::Debug for ListTopicSnapshotsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListTopicSnapshotsResponseView<'_> {
  fn default() -> ListTopicSnapshotsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSnapshotsResponse>> for ListTopicSnapshotsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListTopicSnapshotsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicSnapshotsResponseView<'msg> {

  pub fn to_owned(&self) -> ListTopicSnapshotsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // snapshots: repeated string
  pub fn snapshots(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // next_page_token: optional string
  pub fn next_page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListTopicSnapshotsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListTopicSnapshotsResponseView<'_> {}

// SAFETY:
// - `ListTopicSnapshotsResponseView` is `Send` because while its alive a `ListTopicSnapshotsResponseMut` cannot.
// - `ListTopicSnapshotsResponseView` does not use thread-local data.
unsafe impl Send for ListTopicSnapshotsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicSnapshotsResponseView<'msg> {
  type Proxied = ListTopicSnapshotsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ListTopicSnapshotsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicSnapshotsResponseView<'msg> {
  fn into_view<'shorter>(self) -> ListTopicSnapshotsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicSnapshotsResponse> for ListTopicSnapshotsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicSnapshotsResponse {
    let mut dst = ListTopicSnapshotsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListTopicSnapshotsResponse> for ListTopicSnapshotsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListTopicSnapshotsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListTopicSnapshotsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicSnapshotsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListTopicSnapshotsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListTopicSnapshotsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSnapshotsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListTopicSnapshotsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListTopicSnapshotsResponseMut<'msg> {
  type Message = ListTopicSnapshotsResponse;
}

impl ::std::fmt::Debug for ListTopicSnapshotsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSnapshotsResponse>> for ListTopicSnapshotsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSnapshotsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListTopicSnapshotsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListTopicSnapshotsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ListTopicSnapshotsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // snapshots: repeated string
  pub fn snapshots(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn snapshots_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_snapshots(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ListTopicSnapshotsResponseMut` does not perform any shared mutation.
unsafe impl Send for ListTopicSnapshotsResponseMut<'_> {}

// SAFETY:
// - `ListTopicSnapshotsResponseMut` does not perform any shared mutation.
unsafe impl Sync for ListTopicSnapshotsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ListTopicSnapshotsResponseMut<'msg> {
  type Proxied = ListTopicSnapshotsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ListTopicSnapshotsResponse> {
    ListTopicSnapshotsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListTopicSnapshotsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListTopicSnapshotsResponse>
  where
      'msg: 'shorter {
    ListTopicSnapshotsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListTopicSnapshotsResponseMut<'msg> {
  type MutProxied = ListTopicSnapshotsResponse;
  fn as_mut(&mut self) -> ListTopicSnapshotsResponseMut<'msg> {
    ListTopicSnapshotsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListTopicSnapshotsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ListTopicSnapshotsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListTopicSnapshotsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListTopicSnapshotsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListTopicSnapshotsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListTopicSnapshotsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // snapshots: repeated string
  pub fn snapshots(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn snapshots_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_snapshots(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl ListTopicSnapshotsResponse

impl ::std::ops::Drop for ListTopicSnapshotsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListTopicSnapshotsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListTopicSnapshotsResponse {
  type Proxied = Self;
  fn as_view(&self) -> ListTopicSnapshotsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListTopicSnapshotsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListTopicSnapshotsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListTopicSnapshotsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListTopicSnapshotsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ME1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListTopicSnapshotsResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListTopicSnapshotsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicSnapshotsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicSnapshotsResponse {
  type Msg = ListTopicSnapshotsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSnapshotsResponse {
  type Msg = ListTopicSnapshotsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListTopicSnapshotsResponseMut<'_> {
  type Msg = ListTopicSnapshotsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSnapshotsResponseMut<'_> {
  type Msg = ListTopicSnapshotsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListTopicSnapshotsResponseView<'_> {
  type Msg = ListTopicSnapshotsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListTopicSnapshotsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListTopicSnapshotsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__DeleteTopicRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeleteTopicRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeleteTopicRequest>
}

impl ::protobuf::Message for DeleteTopicRequest {}

impl ::std::default::Default for DeleteTopicRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeleteTopicRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeleteTopicRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DeleteTopicRequestMut`.
unsafe impl Sync for DeleteTopicRequest {}

// SAFETY:
// - `DeleteTopicRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DeleteTopicRequest {}

impl ::protobuf::Proxied for DeleteTopicRequest {
  type View<'msg> = DeleteTopicRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeleteTopicRequest {}

impl ::protobuf::MutProxied for DeleteTopicRequest {
  type Mut<'msg> = DeleteTopicRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeleteTopicRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteTopicRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeleteTopicRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeleteTopicRequestView<'msg> {
  type Message = DeleteTopicRequest;
}

impl ::std::fmt::Debug for DeleteTopicRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeleteTopicRequestView<'_> {
  fn default() -> DeleteTopicRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteTopicRequest>> for DeleteTopicRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteTopicRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeleteTopicRequestView<'msg> {

  pub fn to_owned(&self) -> DeleteTopicRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // topic: optional string
  pub fn topic(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `DeleteTopicRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for DeleteTopicRequestView<'_> {}

// SAFETY:
// - `DeleteTopicRequestView` is `Send` because while its alive a `DeleteTopicRequestMut` cannot.
// - `DeleteTopicRequestView` does not use thread-local data.
unsafe impl Send for DeleteTopicRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DeleteTopicRequestView<'msg> {
  type Proxied = DeleteTopicRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DeleteTopicRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeleteTopicRequestView<'msg> {
  fn into_view<'shorter>(self) -> DeleteTopicRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeleteTopicRequest> for DeleteTopicRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeleteTopicRequest {
    let mut dst = DeleteTopicRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeleteTopicRequest> for DeleteTopicRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeleteTopicRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DeleteTopicRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeleteTopicRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeleteTopicRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeleteTopicRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteTopicRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeleteTopicRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeleteTopicRequestMut<'msg> {
  type Message = DeleteTopicRequest;
}

impl ::std::fmt::Debug for DeleteTopicRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteTopicRequest>> for DeleteTopicRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteTopicRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeleteTopicRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteTopicRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> DeleteTopicRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `DeleteTopicRequestMut` does not perform any shared mutation.
unsafe impl Send for DeleteTopicRequestMut<'_> {}

// SAFETY:
// - `DeleteTopicRequestMut` does not perform any shared mutation.
unsafe impl Sync for DeleteTopicRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DeleteTopicRequestMut<'msg> {
  type Proxied = DeleteTopicRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DeleteTopicRequest> {
    DeleteTopicRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeleteTopicRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeleteTopicRequest>
  where
      'msg: 'shorter {
    DeleteTopicRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DeleteTopicRequestMut<'msg> {
  type MutProxied = DeleteTopicRequest;
  fn as_mut(&mut self) -> DeleteTopicRequestMut<'msg> {
    DeleteTopicRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeleteTopicRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DeleteTopicRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeleteTopicRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeleteTopicRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeleteTopicRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeleteTopicRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl DeleteTopicRequest

impl ::std::ops::Drop for DeleteTopicRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeleteTopicRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeleteTopicRequest {
  type Proxied = Self;
  fn as_view(&self) -> DeleteTopicRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeleteTopicRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeleteTopicRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeleteTopicRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__DeleteTopicRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__DeleteTopicRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__DeleteTopicRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeleteTopicRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeleteTopicRequest {
  type Msg = DeleteTopicRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteTopicRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteTopicRequest {
  type Msg = DeleteTopicRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteTopicRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeleteTopicRequestMut<'_> {
  type Msg = DeleteTopicRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteTopicRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteTopicRequestMut<'_> {
  type Msg = DeleteTopicRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteTopicRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteTopicRequestView<'_> {
  type Msg = DeleteTopicRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteTopicRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeleteTopicRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__DetachSubscriptionRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DetachSubscriptionRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DetachSubscriptionRequest>
}

impl ::protobuf::Message for DetachSubscriptionRequest {}

impl ::std::default::Default for DetachSubscriptionRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DetachSubscriptionRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DetachSubscriptionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DetachSubscriptionRequestMut`.
unsafe impl Sync for DetachSubscriptionRequest {}

// SAFETY:
// - `DetachSubscriptionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DetachSubscriptionRequest {}

impl ::protobuf::Proxied for DetachSubscriptionRequest {
  type View<'msg> = DetachSubscriptionRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DetachSubscriptionRequest {}

impl ::protobuf::MutProxied for DetachSubscriptionRequest {
  type Mut<'msg> = DetachSubscriptionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DetachSubscriptionRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DetachSubscriptionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DetachSubscriptionRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DetachSubscriptionRequestView<'msg> {
  type Message = DetachSubscriptionRequest;
}

impl ::std::fmt::Debug for DetachSubscriptionRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DetachSubscriptionRequestView<'_> {
  fn default() -> DetachSubscriptionRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DetachSubscriptionRequest>> for DetachSubscriptionRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DetachSubscriptionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DetachSubscriptionRequestView<'msg> {

  pub fn to_owned(&self) -> DetachSubscriptionRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `DetachSubscriptionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for DetachSubscriptionRequestView<'_> {}

// SAFETY:
// - `DetachSubscriptionRequestView` is `Send` because while its alive a `DetachSubscriptionRequestMut` cannot.
// - `DetachSubscriptionRequestView` does not use thread-local data.
unsafe impl Send for DetachSubscriptionRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DetachSubscriptionRequestView<'msg> {
  type Proxied = DetachSubscriptionRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DetachSubscriptionRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DetachSubscriptionRequestView<'msg> {
  fn into_view<'shorter>(self) -> DetachSubscriptionRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DetachSubscriptionRequest> for DetachSubscriptionRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DetachSubscriptionRequest {
    let mut dst = DetachSubscriptionRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DetachSubscriptionRequest> for DetachSubscriptionRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DetachSubscriptionRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DetachSubscriptionRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DetachSubscriptionRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DetachSubscriptionRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DetachSubscriptionRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DetachSubscriptionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DetachSubscriptionRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DetachSubscriptionRequestMut<'msg> {
  type Message = DetachSubscriptionRequest;
}

impl ::std::fmt::Debug for DetachSubscriptionRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DetachSubscriptionRequest>> for DetachSubscriptionRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DetachSubscriptionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DetachSubscriptionRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DetachSubscriptionRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> DetachSubscriptionRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `DetachSubscriptionRequestMut` does not perform any shared mutation.
unsafe impl Send for DetachSubscriptionRequestMut<'_> {}

// SAFETY:
// - `DetachSubscriptionRequestMut` does not perform any shared mutation.
unsafe impl Sync for DetachSubscriptionRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DetachSubscriptionRequestMut<'msg> {
  type Proxied = DetachSubscriptionRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DetachSubscriptionRequest> {
    DetachSubscriptionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DetachSubscriptionRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DetachSubscriptionRequest>
  where
      'msg: 'shorter {
    DetachSubscriptionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DetachSubscriptionRequestMut<'msg> {
  type MutProxied = DetachSubscriptionRequest;
  fn as_mut(&mut self) -> DetachSubscriptionRequestMut<'msg> {
    DetachSubscriptionRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DetachSubscriptionRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DetachSubscriptionRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DetachSubscriptionRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DetachSubscriptionRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DetachSubscriptionRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DetachSubscriptionRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl DetachSubscriptionRequest

impl ::std::ops::Drop for DetachSubscriptionRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DetachSubscriptionRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DetachSubscriptionRequest {
  type Proxied = Self;
  fn as_view(&self) -> DetachSubscriptionRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DetachSubscriptionRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DetachSubscriptionRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DetachSubscriptionRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__DetachSubscriptionRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__DetachSubscriptionRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__DetachSubscriptionRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DetachSubscriptionRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DetachSubscriptionRequest {
  type Msg = DetachSubscriptionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DetachSubscriptionRequest {
  type Msg = DetachSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DetachSubscriptionRequestMut<'_> {
  type Msg = DetachSubscriptionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DetachSubscriptionRequestMut<'_> {
  type Msg = DetachSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DetachSubscriptionRequestView<'_> {
  type Msg = DetachSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DetachSubscriptionRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__DetachSubscriptionResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DetachSubscriptionResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DetachSubscriptionResponse>
}

impl ::protobuf::Message for DetachSubscriptionResponse {}

impl ::std::default::Default for DetachSubscriptionResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DetachSubscriptionResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DetachSubscriptionResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `DetachSubscriptionResponseMut`.
unsafe impl Sync for DetachSubscriptionResponse {}

// SAFETY:
// - `DetachSubscriptionResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DetachSubscriptionResponse {}

impl ::protobuf::Proxied for DetachSubscriptionResponse {
  type View<'msg> = DetachSubscriptionResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DetachSubscriptionResponse {}

impl ::protobuf::MutProxied for DetachSubscriptionResponse {
  type Mut<'msg> = DetachSubscriptionResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DetachSubscriptionResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DetachSubscriptionResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DetachSubscriptionResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DetachSubscriptionResponseView<'msg> {
  type Message = DetachSubscriptionResponse;
}

impl ::std::fmt::Debug for DetachSubscriptionResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DetachSubscriptionResponseView<'_> {
  fn default() -> DetachSubscriptionResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DetachSubscriptionResponse>> for DetachSubscriptionResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DetachSubscriptionResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DetachSubscriptionResponseView<'msg> {

  pub fn to_owned(&self) -> DetachSubscriptionResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `DetachSubscriptionResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for DetachSubscriptionResponseView<'_> {}

// SAFETY:
// - `DetachSubscriptionResponseView` is `Send` because while its alive a `DetachSubscriptionResponseMut` cannot.
// - `DetachSubscriptionResponseView` does not use thread-local data.
unsafe impl Send for DetachSubscriptionResponseView<'_> {}

impl<'msg> ::protobuf::AsView for DetachSubscriptionResponseView<'msg> {
  type Proxied = DetachSubscriptionResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, DetachSubscriptionResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DetachSubscriptionResponseView<'msg> {
  fn into_view<'shorter>(self) -> DetachSubscriptionResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DetachSubscriptionResponse> for DetachSubscriptionResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DetachSubscriptionResponse {
    let mut dst = DetachSubscriptionResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DetachSubscriptionResponse> for DetachSubscriptionResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DetachSubscriptionResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DetachSubscriptionResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DetachSubscriptionResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DetachSubscriptionResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DetachSubscriptionResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DetachSubscriptionResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DetachSubscriptionResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DetachSubscriptionResponseMut<'msg> {
  type Message = DetachSubscriptionResponse;
}

impl ::std::fmt::Debug for DetachSubscriptionResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DetachSubscriptionResponse>> for DetachSubscriptionResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DetachSubscriptionResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DetachSubscriptionResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DetachSubscriptionResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> DetachSubscriptionResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `DetachSubscriptionResponseMut` does not perform any shared mutation.
unsafe impl Send for DetachSubscriptionResponseMut<'_> {}

// SAFETY:
// - `DetachSubscriptionResponseMut` does not perform any shared mutation.
unsafe impl Sync for DetachSubscriptionResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for DetachSubscriptionResponseMut<'msg> {
  type Proxied = DetachSubscriptionResponse;
  fn as_view(&self) -> ::protobuf::View<'_, DetachSubscriptionResponse> {
    DetachSubscriptionResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DetachSubscriptionResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DetachSubscriptionResponse>
  where
      'msg: 'shorter {
    DetachSubscriptionResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DetachSubscriptionResponseMut<'msg> {
  type MutProxied = DetachSubscriptionResponse;
  fn as_mut(&mut self) -> DetachSubscriptionResponseMut<'msg> {
    DetachSubscriptionResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DetachSubscriptionResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> DetachSubscriptionResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DetachSubscriptionResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DetachSubscriptionResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DetachSubscriptionResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DetachSubscriptionResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl DetachSubscriptionResponse

impl ::std::ops::Drop for DetachSubscriptionResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DetachSubscriptionResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DetachSubscriptionResponse {
  type Proxied = Self;
  fn as_view(&self) -> DetachSubscriptionResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DetachSubscriptionResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DetachSubscriptionResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DetachSubscriptionResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__DetachSubscriptionResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__DetachSubscriptionResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__DetachSubscriptionResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DetachSubscriptionResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DetachSubscriptionResponse {
  type Msg = DetachSubscriptionResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DetachSubscriptionResponse {
  type Msg = DetachSubscriptionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DetachSubscriptionResponseMut<'_> {
  type Msg = DetachSubscriptionResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DetachSubscriptionResponseMut<'_> {
  type Msg = DetachSubscriptionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DetachSubscriptionResponseView<'_> {
  type Msg = DetachSubscriptionResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DetachSubscriptionResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DetachSubscriptionResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__Subscription_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Subscription {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Subscription>
}

impl ::protobuf::Message for Subscription {}

impl ::std::default::Default for Subscription {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Subscription {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Subscription` is `Sync` because it does not implement interior mutability.
//    Neither does `SubscriptionMut`.
unsafe impl Sync for Subscription {}

// SAFETY:
// - `Subscription` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Subscription {}

impl ::protobuf::Proxied for Subscription {
  type View<'msg> = SubscriptionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Subscription {}

impl ::protobuf::MutProxied for Subscription {
  type Mut<'msg> = SubscriptionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SubscriptionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Subscription>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubscriptionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SubscriptionView<'msg> {
  type Message = Subscription;
}

impl ::std::fmt::Debug for SubscriptionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SubscriptionView<'_> {
  fn default() -> SubscriptionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Subscription>> for SubscriptionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Subscription>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubscriptionView<'msg> {

  pub fn to_owned(&self) -> Subscription {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // topic: optional string
  pub fn topic(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // push_config: optional message google.pubsub.v1.PushConfig
  pub fn has_push_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn push_config_opt(self) -> ::protobuf::Optional<super::PushConfigView<'msg>> {
        ::protobuf::Optional::new(self.push_config(), self.has_push_config())
  }
  pub fn push_config(self) -> super::PushConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PushConfigView::default())
  }

  // ack_deadline_seconds: optional int32
  pub fn ack_deadline_seconds(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // retain_acked_messages: optional bool
  pub fn retain_acked_messages(self) -> bool {
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

  // message_retention_duration: optional message google.protobuf.Duration
  pub fn has_message_retention_duration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn message_retention_duration_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'msg>> {
        ::protobuf::Optional::new(self.message_retention_duration(), self.has_message_retention_duration())
  }
  pub fn message_retention_duration(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // labels: repeated message google.pubsub.v1.Subscription.LabelsEntry
  pub fn labels(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(6)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // enable_message_ordering: optional bool
  pub fn enable_message_ordering(self) -> bool {
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

  // expiration_policy: optional message google.pubsub.v1.ExpirationPolicy
  pub fn has_expiration_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn expiration_policy_opt(self) -> ::protobuf::Optional<super::ExpirationPolicyView<'msg>> {
        ::protobuf::Optional::new(self.expiration_policy(), self.has_expiration_policy())
  }
  pub fn expiration_policy(self) -> super::ExpirationPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExpirationPolicyView::default())
  }

  // filter: optional string
  pub fn filter(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // dead_letter_policy: optional message google.pubsub.v1.DeadLetterPolicy
  pub fn has_dead_letter_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn dead_letter_policy_opt(self) -> ::protobuf::Optional<super::DeadLetterPolicyView<'msg>> {
        ::protobuf::Optional::new(self.dead_letter_policy(), self.has_dead_letter_policy())
  }
  pub fn dead_letter_policy(self) -> super::DeadLetterPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DeadLetterPolicyView::default())
  }

  // retry_policy: optional message google.pubsub.v1.RetryPolicy
  pub fn has_retry_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn retry_policy_opt(self) -> ::protobuf::Optional<super::RetryPolicyView<'msg>> {
        ::protobuf::Optional::new(self.retry_policy(), self.has_retry_policy())
  }
  pub fn retry_policy(self) -> super::RetryPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RetryPolicyView::default())
  }

  // detached: optional bool
  pub fn detached(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        12, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SubscriptionView` is `Sync` because it does not support mutation.
unsafe impl Sync for SubscriptionView<'_> {}

// SAFETY:
// - `SubscriptionView` is `Send` because while its alive a `SubscriptionMut` cannot.
// - `SubscriptionView` does not use thread-local data.
unsafe impl Send for SubscriptionView<'_> {}

impl<'msg> ::protobuf::AsView for SubscriptionView<'msg> {
  type Proxied = Subscription;
  fn as_view(&self) -> ::protobuf::View<'msg, Subscription> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubscriptionView<'msg> {
  fn into_view<'shorter>(self) -> SubscriptionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Subscription> for SubscriptionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Subscription {
    let mut dst = Subscription::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Subscription> for SubscriptionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Subscription {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Subscription {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SubscriptionView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SubscriptionMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SubscriptionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Subscription>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubscriptionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SubscriptionMut<'msg> {
  type Message = Subscription;
}

impl ::std::fmt::Debug for SubscriptionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Subscription>> for SubscriptionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Subscription>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubscriptionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Subscription> {
    self.inner
  }

  pub fn to_owned(&self) -> Subscription {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // push_config: optional message google.pubsub.v1.PushConfig
  pub fn has_push_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_push_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn push_config_opt(&self) -> ::protobuf::Optional<super::PushConfigView<'_>> {
        ::protobuf::Optional::new(self.push_config(), self.has_push_config())
  }
  pub fn push_config(&self) -> super::PushConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PushConfigView::default())
  }
  pub fn push_config_mut(&mut self) -> super::PushConfigMut<'_> {
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
  pub fn set_push_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::PushConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // ack_deadline_seconds: optional int32
  pub fn ack_deadline_seconds(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ack_deadline_seconds(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // retain_acked_messages: optional bool
  pub fn retain_acked_messages(&self) -> bool {
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
  pub fn set_retain_acked_messages(&mut self, val: bool) {
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

  // message_retention_duration: optional message google.protobuf.Duration
  pub fn has_message_retention_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_message_retention_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn message_retention_duration_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.message_retention_duration(), self.has_message_retention_duration())
  }
  pub fn message_retention_duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn message_retention_duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_message_retention_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // labels: repeated message google.pubsub.v1.Subscription.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(6)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          6, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // enable_message_ordering: optional bool
  pub fn enable_message_ordering(&self) -> bool {
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
  pub fn set_enable_message_ordering(&mut self, val: bool) {
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

  // expiration_policy: optional message google.pubsub.v1.ExpirationPolicy
  pub fn has_expiration_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_expiration_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn expiration_policy_opt(&self) -> ::protobuf::Optional<super::ExpirationPolicyView<'_>> {
        ::protobuf::Optional::new(self.expiration_policy(), self.has_expiration_policy())
  }
  pub fn expiration_policy(&self) -> super::ExpirationPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExpirationPolicyView::default())
  }
  pub fn expiration_policy_mut(&mut self) -> super::ExpirationPolicyMut<'_> {
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
  pub fn set_expiration_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::ExpirationPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // filter: optional string
  pub fn filter(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_filter(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // dead_letter_policy: optional message google.pubsub.v1.DeadLetterPolicy
  pub fn has_dead_letter_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_dead_letter_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn dead_letter_policy_opt(&self) -> ::protobuf::Optional<super::DeadLetterPolicyView<'_>> {
        ::protobuf::Optional::new(self.dead_letter_policy(), self.has_dead_letter_policy())
  }
  pub fn dead_letter_policy(&self) -> super::DeadLetterPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DeadLetterPolicyView::default())
  }
  pub fn dead_letter_policy_mut(&mut self) -> super::DeadLetterPolicyMut<'_> {
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
  pub fn set_dead_letter_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::DeadLetterPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // retry_policy: optional message google.pubsub.v1.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::protobuf::Optional<super::RetryPolicyView<'_>> {
        ::protobuf::Optional::new(self.retry_policy(), self.has_retry_policy())
  }
  pub fn retry_policy(&self) -> super::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> super::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // detached: optional bool
  pub fn detached(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        12, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_detached(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        12, val.into()
      )
    }
  }

}

// SAFETY:
// - `SubscriptionMut` does not perform any shared mutation.
unsafe impl Send for SubscriptionMut<'_> {}

// SAFETY:
// - `SubscriptionMut` does not perform any shared mutation.
unsafe impl Sync for SubscriptionMut<'_> {}

impl<'msg> ::protobuf::AsView for SubscriptionMut<'msg> {
  type Proxied = Subscription;
  fn as_view(&self) -> ::protobuf::View<'_, Subscription> {
    SubscriptionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubscriptionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Subscription>
  where
      'msg: 'shorter {
    SubscriptionView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SubscriptionMut<'msg> {
  type MutProxied = Subscription;
  fn as_mut(&mut self) -> SubscriptionMut<'msg> {
    SubscriptionMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SubscriptionMut<'msg> {
  fn into_mut<'shorter>(self) -> SubscriptionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Subscription {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Subscription> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SubscriptionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SubscriptionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // push_config: optional message google.pubsub.v1.PushConfig
  pub fn has_push_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_push_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn push_config_opt(&self) -> ::protobuf::Optional<super::PushConfigView<'_>> {
        ::protobuf::Optional::new(self.push_config(), self.has_push_config())
  }
  pub fn push_config(&self) -> super::PushConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PushConfigView::default())
  }
  pub fn push_config_mut(&mut self) -> super::PushConfigMut<'_> {
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
  pub fn set_push_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::PushConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // ack_deadline_seconds: optional int32
  pub fn ack_deadline_seconds(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ack_deadline_seconds(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // retain_acked_messages: optional bool
  pub fn retain_acked_messages(&self) -> bool {
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
  pub fn set_retain_acked_messages(&mut self, val: bool) {
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

  // message_retention_duration: optional message google.protobuf.Duration
  pub fn has_message_retention_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_message_retention_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn message_retention_duration_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.message_retention_duration(), self.has_message_retention_duration())
  }
  pub fn message_retention_duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn message_retention_duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_message_retention_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // labels: repeated message google.pubsub.v1.Subscription.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(6)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          6, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // enable_message_ordering: optional bool
  pub fn enable_message_ordering(&self) -> bool {
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
  pub fn set_enable_message_ordering(&mut self, val: bool) {
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

  // expiration_policy: optional message google.pubsub.v1.ExpirationPolicy
  pub fn has_expiration_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_expiration_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn expiration_policy_opt(&self) -> ::protobuf::Optional<super::ExpirationPolicyView<'_>> {
        ::protobuf::Optional::new(self.expiration_policy(), self.has_expiration_policy())
  }
  pub fn expiration_policy(&self) -> super::ExpirationPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ExpirationPolicyView::default())
  }
  pub fn expiration_policy_mut(&mut self) -> super::ExpirationPolicyMut<'_> {
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
  pub fn set_expiration_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::ExpirationPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // filter: optional string
  pub fn filter(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_filter(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // dead_letter_policy: optional message google.pubsub.v1.DeadLetterPolicy
  pub fn has_dead_letter_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_dead_letter_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn dead_letter_policy_opt(&self) -> ::protobuf::Optional<super::DeadLetterPolicyView<'_>> {
        ::protobuf::Optional::new(self.dead_letter_policy(), self.has_dead_letter_policy())
  }
  pub fn dead_letter_policy(&self) -> super::DeadLetterPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DeadLetterPolicyView::default())
  }
  pub fn dead_letter_policy_mut(&mut self) -> super::DeadLetterPolicyMut<'_> {
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
  pub fn set_dead_letter_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::DeadLetterPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // retry_policy: optional message google.pubsub.v1.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::protobuf::Optional<super::RetryPolicyView<'_>> {
        ::protobuf::Optional::new(self.retry_policy(), self.has_retry_policy())
  }
  pub fn retry_policy(&self) -> super::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> super::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // detached: optional bool
  pub fn detached(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        12, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_detached(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        12, val.into()
      )
    }
  }

}  // impl Subscription

impl ::std::ops::Drop for Subscription {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Subscription {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Subscription {
  type Proxied = Self;
  fn as_view(&self) -> SubscriptionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Subscription {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SubscriptionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Subscription {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__Subscription_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1Xa3(Pa/P3G/P31X33/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__Subscription_msg_init.0, &[<super::PushConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::subscription::LabelsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ExpirationPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DeadLetterPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::RetryPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__Subscription_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Subscription {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Subscription {
  type Msg = Subscription;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subscription> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Subscription {
  type Msg = Subscription;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subscription> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubscriptionMut<'_> {
  type Msg = Subscription;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subscription> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubscriptionMut<'_> {
  type Msg = Subscription;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subscription> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubscriptionView<'_> {
  type Msg = Subscription;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Subscription> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubscriptionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod subscription {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__Subscription__LabelsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct LabelsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LabelsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::subscription::google__pubsub__v1__Subscription__LabelsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::subscription::google__pubsub__v1__Subscription__LabelsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::subscription::google__pubsub__v1__Subscription__LabelsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod subscription


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__RetryPolicy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RetryPolicy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RetryPolicy>
}

impl ::protobuf::Message for RetryPolicy {}

impl ::std::default::Default for RetryPolicy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RetryPolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RetryPolicy` is `Sync` because it does not implement interior mutability.
//    Neither does `RetryPolicyMut`.
unsafe impl Sync for RetryPolicy {}

// SAFETY:
// - `RetryPolicy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RetryPolicy {}

impl ::protobuf::Proxied for RetryPolicy {
  type View<'msg> = RetryPolicyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RetryPolicy {}

impl ::protobuf::MutProxied for RetryPolicy {
  type Mut<'msg> = RetryPolicyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RetryPolicyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RetryPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RetryPolicyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RetryPolicyView<'msg> {
  type Message = RetryPolicy;
}

impl ::std::fmt::Debug for RetryPolicyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RetryPolicyView<'_> {
  fn default() -> RetryPolicyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RetryPolicy>> for RetryPolicyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RetryPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RetryPolicyView<'msg> {

  pub fn to_owned(&self) -> RetryPolicy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // minimum_backoff: optional message google.protobuf.Duration
  pub fn has_minimum_backoff(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn minimum_backoff_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'msg>> {
        ::protobuf::Optional::new(self.minimum_backoff(), self.has_minimum_backoff())
  }
  pub fn minimum_backoff(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // maximum_backoff: optional message google.protobuf.Duration
  pub fn has_maximum_backoff(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn maximum_backoff_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'msg>> {
        ::protobuf::Optional::new(self.maximum_backoff(), self.has_maximum_backoff())
  }
  pub fn maximum_backoff(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `RetryPolicyView` is `Sync` because it does not support mutation.
unsafe impl Sync for RetryPolicyView<'_> {}

// SAFETY:
// - `RetryPolicyView` is `Send` because while its alive a `RetryPolicyMut` cannot.
// - `RetryPolicyView` does not use thread-local data.
unsafe impl Send for RetryPolicyView<'_> {}

impl<'msg> ::protobuf::AsView for RetryPolicyView<'msg> {
  type Proxied = RetryPolicy;
  fn as_view(&self) -> ::protobuf::View<'msg, RetryPolicy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetryPolicyView<'msg> {
  fn into_view<'shorter>(self) -> RetryPolicyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RetryPolicy> for RetryPolicyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RetryPolicy {
    let mut dst = RetryPolicy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RetryPolicy> for RetryPolicyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RetryPolicy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for RetryPolicy {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RetryPolicyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for RetryPolicyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RetryPolicyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RetryPolicyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RetryPolicyMut<'msg> {
  type Message = RetryPolicy;
}

impl ::std::fmt::Debug for RetryPolicyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RetryPolicy>> for RetryPolicyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RetryPolicyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryPolicy> {
    self.inner
  }

  pub fn to_owned(&self) -> RetryPolicy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // minimum_backoff: optional message google.protobuf.Duration
  pub fn has_minimum_backoff(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_minimum_backoff(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn minimum_backoff_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.minimum_backoff(), self.has_minimum_backoff())
  }
  pub fn minimum_backoff(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn minimum_backoff_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_minimum_backoff(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // maximum_backoff: optional message google.protobuf.Duration
  pub fn has_maximum_backoff(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_maximum_backoff(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn maximum_backoff_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.maximum_backoff(), self.has_maximum_backoff())
  }
  pub fn maximum_backoff(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn maximum_backoff_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_maximum_backoff(&mut self,
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
// - `RetryPolicyMut` does not perform any shared mutation.
unsafe impl Send for RetryPolicyMut<'_> {}

// SAFETY:
// - `RetryPolicyMut` does not perform any shared mutation.
unsafe impl Sync for RetryPolicyMut<'_> {}

impl<'msg> ::protobuf::AsView for RetryPolicyMut<'msg> {
  type Proxied = RetryPolicy;
  fn as_view(&self) -> ::protobuf::View<'_, RetryPolicy> {
    RetryPolicyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetryPolicyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RetryPolicy>
  where
      'msg: 'shorter {
    RetryPolicyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for RetryPolicyMut<'msg> {
  type MutProxied = RetryPolicy;
  fn as_mut(&mut self) -> RetryPolicyMut<'msg> {
    RetryPolicyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RetryPolicyMut<'msg> {
  fn into_mut<'shorter>(self) -> RetryPolicyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RetryPolicy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RetryPolicy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RetryPolicyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RetryPolicyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // minimum_backoff: optional message google.protobuf.Duration
  pub fn has_minimum_backoff(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_minimum_backoff(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn minimum_backoff_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.minimum_backoff(), self.has_minimum_backoff())
  }
  pub fn minimum_backoff(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn minimum_backoff_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_minimum_backoff(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // maximum_backoff: optional message google.protobuf.Duration
  pub fn has_maximum_backoff(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_maximum_backoff(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn maximum_backoff_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.maximum_backoff(), self.has_maximum_backoff())
  }
  pub fn maximum_backoff(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn maximum_backoff_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_maximum_backoff(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl RetryPolicy

impl ::std::ops::Drop for RetryPolicy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RetryPolicy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RetryPolicy {
  type Proxied = Self;
  fn as_view(&self) -> RetryPolicyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RetryPolicy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RetryPolicyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RetryPolicy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__RetryPolicy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__RetryPolicy_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__RetryPolicy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RetryPolicy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RetryPolicy {
  type Msg = RetryPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryPolicy {
  type Msg = RetryPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RetryPolicyMut<'_> {
  type Msg = RetryPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryPolicyMut<'_> {
  type Msg = RetryPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryPolicyView<'_> {
  type Msg = RetryPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPolicy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RetryPolicyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__DeadLetterPolicy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeadLetterPolicy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeadLetterPolicy>
}

impl ::protobuf::Message for DeadLetterPolicy {}

impl ::std::default::Default for DeadLetterPolicy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeadLetterPolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeadLetterPolicy` is `Sync` because it does not implement interior mutability.
//    Neither does `DeadLetterPolicyMut`.
unsafe impl Sync for DeadLetterPolicy {}

// SAFETY:
// - `DeadLetterPolicy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DeadLetterPolicy {}

impl ::protobuf::Proxied for DeadLetterPolicy {
  type View<'msg> = DeadLetterPolicyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeadLetterPolicy {}

impl ::protobuf::MutProxied for DeadLetterPolicy {
  type Mut<'msg> = DeadLetterPolicyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeadLetterPolicyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeadLetterPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeadLetterPolicyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeadLetterPolicyView<'msg> {
  type Message = DeadLetterPolicy;
}

impl ::std::fmt::Debug for DeadLetterPolicyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeadLetterPolicyView<'_> {
  fn default() -> DeadLetterPolicyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeadLetterPolicy>> for DeadLetterPolicyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeadLetterPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeadLetterPolicyView<'msg> {

  pub fn to_owned(&self) -> DeadLetterPolicy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // dead_letter_topic: optional string
  pub fn dead_letter_topic(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // max_delivery_attempts: optional int32
  pub fn max_delivery_attempts(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DeadLetterPolicyView` is `Sync` because it does not support mutation.
unsafe impl Sync for DeadLetterPolicyView<'_> {}

// SAFETY:
// - `DeadLetterPolicyView` is `Send` because while its alive a `DeadLetterPolicyMut` cannot.
// - `DeadLetterPolicyView` does not use thread-local data.
unsafe impl Send for DeadLetterPolicyView<'_> {}

impl<'msg> ::protobuf::AsView for DeadLetterPolicyView<'msg> {
  type Proxied = DeadLetterPolicy;
  fn as_view(&self) -> ::protobuf::View<'msg, DeadLetterPolicy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeadLetterPolicyView<'msg> {
  fn into_view<'shorter>(self) -> DeadLetterPolicyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeadLetterPolicy> for DeadLetterPolicyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeadLetterPolicy {
    let mut dst = DeadLetterPolicy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeadLetterPolicy> for DeadLetterPolicyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeadLetterPolicy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DeadLetterPolicy {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeadLetterPolicyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeadLetterPolicyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeadLetterPolicyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeadLetterPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeadLetterPolicyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeadLetterPolicyMut<'msg> {
  type Message = DeadLetterPolicy;
}

impl ::std::fmt::Debug for DeadLetterPolicyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeadLetterPolicy>> for DeadLetterPolicyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeadLetterPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeadLetterPolicyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeadLetterPolicy> {
    self.inner
  }

  pub fn to_owned(&self) -> DeadLetterPolicy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // dead_letter_topic: optional string
  pub fn dead_letter_topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_dead_letter_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // max_delivery_attempts: optional int32
  pub fn max_delivery_attempts(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_delivery_attempts(&mut self, val: i32) {
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

}

// SAFETY:
// - `DeadLetterPolicyMut` does not perform any shared mutation.
unsafe impl Send for DeadLetterPolicyMut<'_> {}

// SAFETY:
// - `DeadLetterPolicyMut` does not perform any shared mutation.
unsafe impl Sync for DeadLetterPolicyMut<'_> {}

impl<'msg> ::protobuf::AsView for DeadLetterPolicyMut<'msg> {
  type Proxied = DeadLetterPolicy;
  fn as_view(&self) -> ::protobuf::View<'_, DeadLetterPolicy> {
    DeadLetterPolicyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeadLetterPolicyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeadLetterPolicy>
  where
      'msg: 'shorter {
    DeadLetterPolicyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DeadLetterPolicyMut<'msg> {
  type MutProxied = DeadLetterPolicy;
  fn as_mut(&mut self) -> DeadLetterPolicyMut<'msg> {
    DeadLetterPolicyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeadLetterPolicyMut<'msg> {
  fn into_mut<'shorter>(self) -> DeadLetterPolicyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeadLetterPolicy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeadLetterPolicy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeadLetterPolicyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeadLetterPolicyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // dead_letter_topic: optional string
  pub fn dead_letter_topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_dead_letter_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // max_delivery_attempts: optional int32
  pub fn max_delivery_attempts(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_delivery_attempts(&mut self, val: i32) {
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

}  // impl DeadLetterPolicy

impl ::std::ops::Drop for DeadLetterPolicy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeadLetterPolicy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeadLetterPolicy {
  type Proxied = Self;
  fn as_view(&self) -> DeadLetterPolicyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeadLetterPolicy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeadLetterPolicyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeadLetterPolicy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__DeadLetterPolicy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__DeadLetterPolicy_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__DeadLetterPolicy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeadLetterPolicy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeadLetterPolicy {
  type Msg = DeadLetterPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeadLetterPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeadLetterPolicy {
  type Msg = DeadLetterPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeadLetterPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeadLetterPolicyMut<'_> {
  type Msg = DeadLetterPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeadLetterPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeadLetterPolicyMut<'_> {
  type Msg = DeadLetterPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeadLetterPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeadLetterPolicyView<'_> {
  type Msg = DeadLetterPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeadLetterPolicy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeadLetterPolicyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ExpirationPolicy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExpirationPolicy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExpirationPolicy>
}

impl ::protobuf::Message for ExpirationPolicy {}

impl ::std::default::Default for ExpirationPolicy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExpirationPolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExpirationPolicy` is `Sync` because it does not implement interior mutability.
//    Neither does `ExpirationPolicyMut`.
unsafe impl Sync for ExpirationPolicy {}

// SAFETY:
// - `ExpirationPolicy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ExpirationPolicy {}

impl ::protobuf::Proxied for ExpirationPolicy {
  type View<'msg> = ExpirationPolicyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExpirationPolicy {}

impl ::protobuf::MutProxied for ExpirationPolicy {
  type Mut<'msg> = ExpirationPolicyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExpirationPolicyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpirationPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpirationPolicyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExpirationPolicyView<'msg> {
  type Message = ExpirationPolicy;
}

impl ::std::fmt::Debug for ExpirationPolicyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExpirationPolicyView<'_> {
  fn default() -> ExpirationPolicyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExpirationPolicy>> for ExpirationPolicyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExpirationPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpirationPolicyView<'msg> {

  pub fn to_owned(&self) -> ExpirationPolicy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // ttl: optional message google.protobuf.Duration
  pub fn has_ttl(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn ttl_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'msg>> {
        ::protobuf::Optional::new(self.ttl(), self.has_ttl())
  }
  pub fn ttl(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `ExpirationPolicyView` is `Sync` because it does not support mutation.
unsafe impl Sync for ExpirationPolicyView<'_> {}

// SAFETY:
// - `ExpirationPolicyView` is `Send` because while its alive a `ExpirationPolicyMut` cannot.
// - `ExpirationPolicyView` does not use thread-local data.
unsafe impl Send for ExpirationPolicyView<'_> {}

impl<'msg> ::protobuf::AsView for ExpirationPolicyView<'msg> {
  type Proxied = ExpirationPolicy;
  fn as_view(&self) -> ::protobuf::View<'msg, ExpirationPolicy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpirationPolicyView<'msg> {
  fn into_view<'shorter>(self) -> ExpirationPolicyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpirationPolicy> for ExpirationPolicyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpirationPolicy {
    let mut dst = ExpirationPolicy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExpirationPolicy> for ExpirationPolicyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExpirationPolicy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ExpirationPolicy {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExpirationPolicyView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ExpirationPolicyMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExpirationPolicyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpirationPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExpirationPolicyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExpirationPolicyMut<'msg> {
  type Message = ExpirationPolicy;
}

impl ::std::fmt::Debug for ExpirationPolicyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExpirationPolicy>> for ExpirationPolicyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpirationPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExpirationPolicyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExpirationPolicy> {
    self.inner
  }

  pub fn to_owned(&self) -> ExpirationPolicy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // ttl: optional message google.protobuf.Duration
  pub fn has_ttl(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_ttl(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn ttl_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.ttl(), self.has_ttl())
  }
  pub fn ttl(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn ttl_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_ttl(&mut self,
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
// - `ExpirationPolicyMut` does not perform any shared mutation.
unsafe impl Send for ExpirationPolicyMut<'_> {}

// SAFETY:
// - `ExpirationPolicyMut` does not perform any shared mutation.
unsafe impl Sync for ExpirationPolicyMut<'_> {}

impl<'msg> ::protobuf::AsView for ExpirationPolicyMut<'msg> {
  type Proxied = ExpirationPolicy;
  fn as_view(&self) -> ::protobuf::View<'_, ExpirationPolicy> {
    ExpirationPolicyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExpirationPolicyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExpirationPolicy>
  where
      'msg: 'shorter {
    ExpirationPolicyView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ExpirationPolicyMut<'msg> {
  type MutProxied = ExpirationPolicy;
  fn as_mut(&mut self) -> ExpirationPolicyMut<'msg> {
    ExpirationPolicyMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExpirationPolicyMut<'msg> {
  fn into_mut<'shorter>(self) -> ExpirationPolicyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExpirationPolicy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExpirationPolicy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExpirationPolicyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExpirationPolicyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // ttl: optional message google.protobuf.Duration
  pub fn has_ttl(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_ttl(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn ttl_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::DurationView<'_>> {
        ::protobuf::Optional::new(self.ttl(), self.has_ttl())
  }
  pub fn ttl(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn ttl_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_ttl(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl ExpirationPolicy

impl ::std::ops::Drop for ExpirationPolicy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExpirationPolicy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExpirationPolicy {
  type Proxied = Self;
  fn as_view(&self) -> ExpirationPolicyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExpirationPolicy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExpirationPolicyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExpirationPolicy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ExpirationPolicy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ExpirationPolicy_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ExpirationPolicy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpirationPolicy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpirationPolicy {
  type Msg = ExpirationPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpirationPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpirationPolicy {
  type Msg = ExpirationPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpirationPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExpirationPolicyMut<'_> {
  type Msg = ExpirationPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpirationPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpirationPolicyMut<'_> {
  type Msg = ExpirationPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpirationPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExpirationPolicyView<'_> {
  type Msg = ExpirationPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExpirationPolicy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExpirationPolicyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__PushConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PushConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PushConfig>
}

impl ::protobuf::Message for PushConfig {}

impl ::std::default::Default for PushConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PushConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PushConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `PushConfigMut`.
unsafe impl Sync for PushConfig {}

// SAFETY:
// - `PushConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PushConfig {}

impl ::protobuf::Proxied for PushConfig {
  type View<'msg> = PushConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PushConfig {}

impl ::protobuf::MutProxied for PushConfig {
  type Mut<'msg> = PushConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PushConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PushConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PushConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PushConfigView<'msg> {
  type Message = PushConfig;
}

impl ::std::fmt::Debug for PushConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PushConfigView<'_> {
  fn default() -> PushConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PushConfig>> for PushConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PushConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PushConfigView<'msg> {

  pub fn to_owned(&self) -> PushConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // push_endpoint: optional string
  pub fn push_endpoint(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // attributes: repeated message google.pubsub.v1.PushConfig.AttributesEntry
  pub fn attributes(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // oidc_token: optional message google.pubsub.v1.PushConfig.OidcToken
  pub fn has_oidc_token(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn oidc_token_opt(self) -> ::protobuf::Optional<super::push_config::OidcTokenView<'msg>> {
        ::protobuf::Optional::new(self.oidc_token(), self.has_oidc_token())
  }
  pub fn oidc_token(self) -> super::push_config::OidcTokenView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::push_config::OidcTokenView::default())
  }

  pub fn authentication_method(self) -> super::push_config::AuthenticationMethodOneof<'msg> {
    match self.authentication_method_case() {
      super::push_config::AuthenticationMethodCase::OidcToken =>
          super::push_config::AuthenticationMethodOneof::OidcToken(self.oidc_token()),
      _ => super::push_config::AuthenticationMethodOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn authentication_method_case(self) -> super::push_config::AuthenticationMethodCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::push_config::AuthenticationMethodCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PushConfigView` is `Sync` because it does not support mutation.
unsafe impl Sync for PushConfigView<'_> {}

// SAFETY:
// - `PushConfigView` is `Send` because while its alive a `PushConfigMut` cannot.
// - `PushConfigView` does not use thread-local data.
unsafe impl Send for PushConfigView<'_> {}

impl<'msg> ::protobuf::AsView for PushConfigView<'msg> {
  type Proxied = PushConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, PushConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PushConfigView<'msg> {
  fn into_view<'shorter>(self) -> PushConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PushConfig> for PushConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PushConfig {
    let mut dst = PushConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PushConfig> for PushConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PushConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PushConfig {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PushConfigView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PushConfigMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PushConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PushConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PushConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PushConfigMut<'msg> {
  type Message = PushConfig;
}

impl ::std::fmt::Debug for PushConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PushConfig>> for PushConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PushConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PushConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PushConfig> {
    self.inner
  }

  pub fn to_owned(&self) -> PushConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // push_endpoint: optional string
  pub fn push_endpoint(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_push_endpoint(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // attributes: repeated message google.pubsub.v1.PushConfig.AttributesEntry
  pub fn attributes(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn attributes_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_attributes(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // oidc_token: optional message google.pubsub.v1.PushConfig.OidcToken
  pub fn has_oidc_token(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_oidc_token(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn oidc_token_opt(&self) -> ::protobuf::Optional<super::push_config::OidcTokenView<'_>> {
        ::protobuf::Optional::new(self.oidc_token(), self.has_oidc_token())
  }
  pub fn oidc_token(&self) -> super::push_config::OidcTokenView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::push_config::OidcTokenView::default())
  }
  pub fn oidc_token_mut(&mut self) -> super::push_config::OidcTokenMut<'_> {
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
  pub fn set_oidc_token(&mut self,
    val: impl ::protobuf::IntoProxied<super::push_config::OidcToken>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn authentication_method(&self) -> super::push_config::AuthenticationMethodOneof<'_> {
    match &self.authentication_method_case() {
      super::push_config::AuthenticationMethodCase::OidcToken =>
          super::push_config::AuthenticationMethodOneof::OidcToken(self.oidc_token()),
      _ => super::push_config::AuthenticationMethodOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn authentication_method_case(&self) -> super::push_config::AuthenticationMethodCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::push_config::AuthenticationMethodCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PushConfigMut` does not perform any shared mutation.
unsafe impl Send for PushConfigMut<'_> {}

// SAFETY:
// - `PushConfigMut` does not perform any shared mutation.
unsafe impl Sync for PushConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for PushConfigMut<'msg> {
  type Proxied = PushConfig;
  fn as_view(&self) -> ::protobuf::View<'_, PushConfig> {
    PushConfigView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PushConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PushConfig>
  where
      'msg: 'shorter {
    PushConfigView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PushConfigMut<'msg> {
  type MutProxied = PushConfig;
  fn as_mut(&mut self) -> PushConfigMut<'msg> {
    PushConfigMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PushConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> PushConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PushConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PushConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PushConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PushConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // push_endpoint: optional string
  pub fn push_endpoint(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_push_endpoint(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // attributes: repeated message google.pubsub.v1.PushConfig.AttributesEntry
  pub fn attributes(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn attributes_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_attributes(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // oidc_token: optional message google.pubsub.v1.PushConfig.OidcToken
  pub fn has_oidc_token(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_oidc_token(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn oidc_token_opt(&self) -> ::protobuf::Optional<super::push_config::OidcTokenView<'_>> {
        ::protobuf::Optional::new(self.oidc_token(), self.has_oidc_token())
  }
  pub fn oidc_token(&self) -> super::push_config::OidcTokenView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::push_config::OidcTokenView::default())
  }
  pub fn oidc_token_mut(&mut self) -> super::push_config::OidcTokenMut<'_> {
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
  pub fn set_oidc_token(&mut self,
    val: impl ::protobuf::IntoProxied<super::push_config::OidcToken>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn authentication_method(&self) -> super::push_config::AuthenticationMethodOneof<'_> {
    match &self.authentication_method_case() {
      super::push_config::AuthenticationMethodCase::OidcToken =>
          super::push_config::AuthenticationMethodOneof::OidcToken(self.oidc_token()),
      _ => super::push_config::AuthenticationMethodOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn authentication_method_case(&self) -> super::push_config::AuthenticationMethodCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::push_config::AuthenticationMethodCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl PushConfig

impl ::std::ops::Drop for PushConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PushConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PushConfig {
  type Proxied = Self;
  fn as_view(&self) -> PushConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PushConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PushConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PushConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__PushConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG3^$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__PushConfig_msg_init.0, &[<super::push_config::AttributesEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::push_config::OidcToken as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__PushConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PushConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PushConfig {
  type Msg = PushConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PushConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PushConfig {
  type Msg = PushConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PushConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PushConfigMut<'_> {
  type Msg = PushConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PushConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PushConfigMut<'_> {
  type Msg = PushConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PushConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PushConfigView<'_> {
  type Msg = PushConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PushConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PushConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod push_config {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__PushConfig__OidcToken_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OidcToken {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OidcToken>
}

impl ::protobuf::Message for OidcToken {}

impl ::std::default::Default for OidcToken {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OidcToken {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OidcToken` is `Sync` because it does not implement interior mutability.
//    Neither does `OidcTokenMut`.
unsafe impl Sync for OidcToken {}

// SAFETY:
// - `OidcToken` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for OidcToken {}

impl ::protobuf::Proxied for OidcToken {
  type View<'msg> = OidcTokenView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OidcToken {}

impl ::protobuf::MutProxied for OidcToken {
  type Mut<'msg> = OidcTokenMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OidcTokenView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OidcToken>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OidcTokenView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OidcTokenView<'msg> {
  type Message = OidcToken;
}

impl ::std::fmt::Debug for OidcTokenView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OidcTokenView<'_> {
  fn default() -> OidcTokenView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OidcToken>> for OidcTokenView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OidcToken>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OidcTokenView<'msg> {

  pub fn to_owned(&self) -> OidcToken {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // service_account_email: optional string
  pub fn service_account_email(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // audience: optional string
  pub fn audience(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `OidcTokenView` is `Sync` because it does not support mutation.
unsafe impl Sync for OidcTokenView<'_> {}

// SAFETY:
// - `OidcTokenView` is `Send` because while its alive a `OidcTokenMut` cannot.
// - `OidcTokenView` does not use thread-local data.
unsafe impl Send for OidcTokenView<'_> {}

impl<'msg> ::protobuf::AsView for OidcTokenView<'msg> {
  type Proxied = OidcToken;
  fn as_view(&self) -> ::protobuf::View<'msg, OidcToken> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OidcTokenView<'msg> {
  fn into_view<'shorter>(self) -> OidcTokenView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OidcToken> for OidcTokenView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OidcToken {
    let mut dst = OidcToken::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OidcToken> for OidcTokenMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OidcToken {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for OidcToken {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for OidcTokenView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for OidcTokenMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OidcTokenMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OidcToken>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OidcTokenMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OidcTokenMut<'msg> {
  type Message = OidcToken;
}

impl ::std::fmt::Debug for OidcTokenMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OidcToken>> for OidcTokenMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OidcToken>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OidcTokenMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OidcToken> {
    self.inner
  }

  pub fn to_owned(&self) -> OidcToken {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // service_account_email: optional string
  pub fn service_account_email(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_service_account_email(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // audience: optional string
  pub fn audience(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_audience(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `OidcTokenMut` does not perform any shared mutation.
unsafe impl Send for OidcTokenMut<'_> {}

// SAFETY:
// - `OidcTokenMut` does not perform any shared mutation.
unsafe impl Sync for OidcTokenMut<'_> {}

impl<'msg> ::protobuf::AsView for OidcTokenMut<'msg> {
  type Proxied = OidcToken;
  fn as_view(&self) -> ::protobuf::View<'_, OidcToken> {
    OidcTokenView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OidcTokenMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OidcToken>
  where
      'msg: 'shorter {
    OidcTokenView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for OidcTokenMut<'msg> {
  type MutProxied = OidcToken;
  fn as_mut(&mut self) -> OidcTokenMut<'msg> {
    OidcTokenMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OidcTokenMut<'msg> {
  fn into_mut<'shorter>(self) -> OidcTokenMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OidcToken {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OidcToken> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OidcTokenView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OidcTokenMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // service_account_email: optional string
  pub fn service_account_email(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_service_account_email(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // audience: optional string
  pub fn audience(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_audience(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl OidcToken

impl ::std::ops::Drop for OidcToken {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OidcToken {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OidcToken {
  type Proxied = Self;
  fn as_view(&self) -> OidcTokenView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OidcToken {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OidcTokenMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OidcToken {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::push_config::google__pubsub__v1__PushConfig__OidcToken_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::push_config::google__pubsub__v1__PushConfig__OidcToken_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::push_config::google__pubsub__v1__PushConfig__OidcToken_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OidcToken {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OidcToken {
  type Msg = OidcToken;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OidcToken> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OidcToken {
  type Msg = OidcToken;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OidcToken> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OidcTokenMut<'_> {
  type Msg = OidcToken;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OidcToken> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OidcTokenMut<'_> {
  type Msg = OidcToken;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OidcToken> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OidcTokenView<'_> {
  type Msg = OidcToken;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OidcToken> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OidcTokenMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__PushConfig__AttributesEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct AttributesEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AttributesEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::push_config::google__pubsub__v1__PushConfig__AttributesEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::push_config::google__pubsub__v1__PushConfig__AttributesEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::push_config::google__pubsub__v1__PushConfig__AttributesEntry_msg_init.0)
      }).0
    }
  }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum AuthenticationMethodOneof<'msg> {
  OidcToken(::protobuf::View<'msg, super::super::push_config::OidcToken>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum AuthenticationMethodCase {
  OidcToken = 3,

  not_set = 0
}

impl AuthenticationMethodCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<AuthenticationMethodCase> {
    match v {
      0 => Some(AuthenticationMethodCase::not_set),
      3 => Some(AuthenticationMethodCase::OidcToken),
      _ => None
    }
  }
}
}  // pub mod push_config


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ReceivedMessage_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ReceivedMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ReceivedMessage>
}

impl ::protobuf::Message for ReceivedMessage {}

impl ::std::default::Default for ReceivedMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ReceivedMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ReceivedMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `ReceivedMessageMut`.
unsafe impl Sync for ReceivedMessage {}

// SAFETY:
// - `ReceivedMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ReceivedMessage {}

impl ::protobuf::Proxied for ReceivedMessage {
  type View<'msg> = ReceivedMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ReceivedMessage {}

impl ::protobuf::MutProxied for ReceivedMessage {
  type Mut<'msg> = ReceivedMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReceivedMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ReceivedMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReceivedMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReceivedMessageView<'msg> {
  type Message = ReceivedMessage;
}

impl ::std::fmt::Debug for ReceivedMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ReceivedMessageView<'_> {
  fn default() -> ReceivedMessageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ReceivedMessage>> for ReceivedMessageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ReceivedMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReceivedMessageView<'msg> {

  pub fn to_owned(&self) -> ReceivedMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // ack_id: optional string
  pub fn ack_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // message: optional message google.pubsub.v1.PubsubMessage
  pub fn has_message(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn message_opt(self) -> ::protobuf::Optional<super::PubsubMessageView<'msg>> {
        ::protobuf::Optional::new(self.message(), self.has_message())
  }
  pub fn message(self) -> super::PubsubMessageView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PubsubMessageView::default())
  }

  // delivery_attempt: optional int32
  pub fn delivery_attempt(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ReceivedMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for ReceivedMessageView<'_> {}

// SAFETY:
// - `ReceivedMessageView` is `Send` because while its alive a `ReceivedMessageMut` cannot.
// - `ReceivedMessageView` does not use thread-local data.
unsafe impl Send for ReceivedMessageView<'_> {}

impl<'msg> ::protobuf::AsView for ReceivedMessageView<'msg> {
  type Proxied = ReceivedMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, ReceivedMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReceivedMessageView<'msg> {
  fn into_view<'shorter>(self) -> ReceivedMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ReceivedMessage> for ReceivedMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReceivedMessage {
    let mut dst = ReceivedMessage::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ReceivedMessage> for ReceivedMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReceivedMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ReceivedMessage {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ReceivedMessageView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ReceivedMessageMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReceivedMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ReceivedMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReceivedMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReceivedMessageMut<'msg> {
  type Message = ReceivedMessage;
}

impl ::std::fmt::Debug for ReceivedMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ReceivedMessage>> for ReceivedMessageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ReceivedMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReceivedMessageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ReceivedMessage> {
    self.inner
  }

  pub fn to_owned(&self) -> ReceivedMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // ack_id: optional string
  pub fn ack_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_ack_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // message: optional message google.pubsub.v1.PubsubMessage
  pub fn has_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn message_opt(&self) -> ::protobuf::Optional<super::PubsubMessageView<'_>> {
        ::protobuf::Optional::new(self.message(), self.has_message())
  }
  pub fn message(&self) -> super::PubsubMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PubsubMessageView::default())
  }
  pub fn message_mut(&mut self) -> super::PubsubMessageMut<'_> {
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
  pub fn set_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PubsubMessage>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // delivery_attempt: optional int32
  pub fn delivery_attempt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delivery_attempt(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `ReceivedMessageMut` does not perform any shared mutation.
unsafe impl Send for ReceivedMessageMut<'_> {}

// SAFETY:
// - `ReceivedMessageMut` does not perform any shared mutation.
unsafe impl Sync for ReceivedMessageMut<'_> {}

impl<'msg> ::protobuf::AsView for ReceivedMessageMut<'msg> {
  type Proxied = ReceivedMessage;
  fn as_view(&self) -> ::protobuf::View<'_, ReceivedMessage> {
    ReceivedMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReceivedMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ReceivedMessage>
  where
      'msg: 'shorter {
    ReceivedMessageView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ReceivedMessageMut<'msg> {
  type MutProxied = ReceivedMessage;
  fn as_mut(&mut self) -> ReceivedMessageMut<'msg> {
    ReceivedMessageMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReceivedMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> ReceivedMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ReceivedMessage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ReceivedMessage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ReceivedMessageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ReceivedMessageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // ack_id: optional string
  pub fn ack_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_ack_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // message: optional message google.pubsub.v1.PubsubMessage
  pub fn has_message(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_message(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn message_opt(&self) -> ::protobuf::Optional<super::PubsubMessageView<'_>> {
        ::protobuf::Optional::new(self.message(), self.has_message())
  }
  pub fn message(&self) -> super::PubsubMessageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PubsubMessageView::default())
  }
  pub fn message_mut(&mut self) -> super::PubsubMessageMut<'_> {
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
  pub fn set_message(&mut self,
    val: impl ::protobuf::IntoProxied<super::PubsubMessage>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // delivery_attempt: optional int32
  pub fn delivery_attempt(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delivery_attempt(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}  // impl ReceivedMessage

impl ::std::ops::Drop for ReceivedMessage {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ReceivedMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ReceivedMessage {
  type Proxied = Self;
  fn as_view(&self) -> ReceivedMessageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ReceivedMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReceivedMessageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ReceivedMessage {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ReceivedMessage_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ReceivedMessage_msg_init.0, &[<super::PubsubMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ReceivedMessage_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReceivedMessage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReceivedMessage {
  type Msg = ReceivedMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReceivedMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReceivedMessage {
  type Msg = ReceivedMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReceivedMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReceivedMessageMut<'_> {
  type Msg = ReceivedMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReceivedMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReceivedMessageMut<'_> {
  type Msg = ReceivedMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReceivedMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReceivedMessageView<'_> {
  type Msg = ReceivedMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReceivedMessage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReceivedMessageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__GetSubscriptionRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetSubscriptionRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetSubscriptionRequest>
}

impl ::protobuf::Message for GetSubscriptionRequest {}

impl ::std::default::Default for GetSubscriptionRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetSubscriptionRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetSubscriptionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetSubscriptionRequestMut`.
unsafe impl Sync for GetSubscriptionRequest {}

// SAFETY:
// - `GetSubscriptionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetSubscriptionRequest {}

impl ::protobuf::Proxied for GetSubscriptionRequest {
  type View<'msg> = GetSubscriptionRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetSubscriptionRequest {}

impl ::protobuf::MutProxied for GetSubscriptionRequest {
  type Mut<'msg> = GetSubscriptionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetSubscriptionRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetSubscriptionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetSubscriptionRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetSubscriptionRequestView<'msg> {
  type Message = GetSubscriptionRequest;
}

impl ::std::fmt::Debug for GetSubscriptionRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetSubscriptionRequestView<'_> {
  fn default() -> GetSubscriptionRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetSubscriptionRequest>> for GetSubscriptionRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetSubscriptionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetSubscriptionRequestView<'msg> {

  pub fn to_owned(&self) -> GetSubscriptionRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetSubscriptionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetSubscriptionRequestView<'_> {}

// SAFETY:
// - `GetSubscriptionRequestView` is `Send` because while its alive a `GetSubscriptionRequestMut` cannot.
// - `GetSubscriptionRequestView` does not use thread-local data.
unsafe impl Send for GetSubscriptionRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetSubscriptionRequestView<'msg> {
  type Proxied = GetSubscriptionRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetSubscriptionRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetSubscriptionRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetSubscriptionRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetSubscriptionRequest> for GetSubscriptionRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetSubscriptionRequest {
    let mut dst = GetSubscriptionRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetSubscriptionRequest> for GetSubscriptionRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetSubscriptionRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetSubscriptionRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetSubscriptionRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetSubscriptionRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetSubscriptionRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetSubscriptionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetSubscriptionRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetSubscriptionRequestMut<'msg> {
  type Message = GetSubscriptionRequest;
}

impl ::std::fmt::Debug for GetSubscriptionRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetSubscriptionRequest>> for GetSubscriptionRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetSubscriptionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetSubscriptionRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetSubscriptionRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetSubscriptionRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `GetSubscriptionRequestMut` does not perform any shared mutation.
unsafe impl Send for GetSubscriptionRequestMut<'_> {}

// SAFETY:
// - `GetSubscriptionRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetSubscriptionRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetSubscriptionRequestMut<'msg> {
  type Proxied = GetSubscriptionRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetSubscriptionRequest> {
    GetSubscriptionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetSubscriptionRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetSubscriptionRequest>
  where
      'msg: 'shorter {
    GetSubscriptionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetSubscriptionRequestMut<'msg> {
  type MutProxied = GetSubscriptionRequest;
  fn as_mut(&mut self) -> GetSubscriptionRequestMut<'msg> {
    GetSubscriptionRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetSubscriptionRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetSubscriptionRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetSubscriptionRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetSubscriptionRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetSubscriptionRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetSubscriptionRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl GetSubscriptionRequest

impl ::std::ops::Drop for GetSubscriptionRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetSubscriptionRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetSubscriptionRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetSubscriptionRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetSubscriptionRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetSubscriptionRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetSubscriptionRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__GetSubscriptionRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__GetSubscriptionRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__GetSubscriptionRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetSubscriptionRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetSubscriptionRequest {
  type Msg = GetSubscriptionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSubscriptionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetSubscriptionRequest {
  type Msg = GetSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSubscriptionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetSubscriptionRequestMut<'_> {
  type Msg = GetSubscriptionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSubscriptionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetSubscriptionRequestMut<'_> {
  type Msg = GetSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSubscriptionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetSubscriptionRequestView<'_> {
  type Msg = GetSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSubscriptionRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetSubscriptionRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__UpdateSubscriptionRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpdateSubscriptionRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpdateSubscriptionRequest>
}

impl ::protobuf::Message for UpdateSubscriptionRequest {}

impl ::std::default::Default for UpdateSubscriptionRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpdateSubscriptionRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpdateSubscriptionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `UpdateSubscriptionRequestMut`.
unsafe impl Sync for UpdateSubscriptionRequest {}

// SAFETY:
// - `UpdateSubscriptionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UpdateSubscriptionRequest {}

impl ::protobuf::Proxied for UpdateSubscriptionRequest {
  type View<'msg> = UpdateSubscriptionRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpdateSubscriptionRequest {}

impl ::protobuf::MutProxied for UpdateSubscriptionRequest {
  type Mut<'msg> = UpdateSubscriptionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpdateSubscriptionRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateSubscriptionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateSubscriptionRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpdateSubscriptionRequestView<'msg> {
  type Message = UpdateSubscriptionRequest;
}

impl ::std::fmt::Debug for UpdateSubscriptionRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpdateSubscriptionRequestView<'_> {
  fn default() -> UpdateSubscriptionRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateSubscriptionRequest>> for UpdateSubscriptionRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateSubscriptionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateSubscriptionRequestView<'msg> {

  pub fn to_owned(&self) -> UpdateSubscriptionRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional message google.pubsub.v1.Subscription
  pub fn has_subscription(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn subscription_opt(self) -> ::protobuf::Optional<super::SubscriptionView<'msg>> {
        ::protobuf::Optional::new(self.subscription(), self.has_subscription())
  }
  pub fn subscription(self) -> super::SubscriptionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SubscriptionView::default())
  }

  // update_mask: optional message google.protobuf.FieldMask
  pub fn has_update_mask(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn update_mask_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::FieldMaskView<'msg>> {
        ::protobuf::Optional::new(self.update_mask(), self.has_update_mask())
  }
  pub fn update_mask(self) -> ::protobuf_well_known_types::FieldMaskView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FieldMaskView::default())
  }

}

// SAFETY:
// - `UpdateSubscriptionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for UpdateSubscriptionRequestView<'_> {}

// SAFETY:
// - `UpdateSubscriptionRequestView` is `Send` because while its alive a `UpdateSubscriptionRequestMut` cannot.
// - `UpdateSubscriptionRequestView` does not use thread-local data.
unsafe impl Send for UpdateSubscriptionRequestView<'_> {}

impl<'msg> ::protobuf::AsView for UpdateSubscriptionRequestView<'msg> {
  type Proxied = UpdateSubscriptionRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, UpdateSubscriptionRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateSubscriptionRequestView<'msg> {
  fn into_view<'shorter>(self) -> UpdateSubscriptionRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpdateSubscriptionRequest> for UpdateSubscriptionRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpdateSubscriptionRequest {
    let mut dst = UpdateSubscriptionRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpdateSubscriptionRequest> for UpdateSubscriptionRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpdateSubscriptionRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for UpdateSubscriptionRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UpdateSubscriptionRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UpdateSubscriptionRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpdateSubscriptionRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateSubscriptionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateSubscriptionRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpdateSubscriptionRequestMut<'msg> {
  type Message = UpdateSubscriptionRequest;
}

impl ::std::fmt::Debug for UpdateSubscriptionRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateSubscriptionRequest>> for UpdateSubscriptionRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateSubscriptionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateSubscriptionRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateSubscriptionRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> UpdateSubscriptionRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional message google.pubsub.v1.Subscription
  pub fn has_subscription(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_subscription(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn subscription_opt(&self) -> ::protobuf::Optional<super::SubscriptionView<'_>> {
        ::protobuf::Optional::new(self.subscription(), self.has_subscription())
  }
  pub fn subscription(&self) -> super::SubscriptionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SubscriptionView::default())
  }
  pub fn subscription_mut(&mut self) -> super::SubscriptionMut<'_> {
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
  pub fn set_subscription(&mut self,
    val: impl ::protobuf::IntoProxied<super::Subscription>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // update_mask: optional message google.protobuf.FieldMask
  pub fn has_update_mask(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_update_mask(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn update_mask_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::FieldMaskView<'_>> {
        ::protobuf::Optional::new(self.update_mask(), self.has_update_mask())
  }
  pub fn update_mask(&self) -> ::protobuf_well_known_types::FieldMaskView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FieldMaskView::default())
  }
  pub fn update_mask_mut(&mut self) -> ::protobuf_well_known_types::FieldMaskMut<'_> {
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
  pub fn set_update_mask(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::FieldMask>) {

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
// - `UpdateSubscriptionRequestMut` does not perform any shared mutation.
unsafe impl Send for UpdateSubscriptionRequestMut<'_> {}

// SAFETY:
// - `UpdateSubscriptionRequestMut` does not perform any shared mutation.
unsafe impl Sync for UpdateSubscriptionRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for UpdateSubscriptionRequestMut<'msg> {
  type Proxied = UpdateSubscriptionRequest;
  fn as_view(&self) -> ::protobuf::View<'_, UpdateSubscriptionRequest> {
    UpdateSubscriptionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateSubscriptionRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpdateSubscriptionRequest>
  where
      'msg: 'shorter {
    UpdateSubscriptionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for UpdateSubscriptionRequestMut<'msg> {
  type MutProxied = UpdateSubscriptionRequest;
  fn as_mut(&mut self) -> UpdateSubscriptionRequestMut<'msg> {
    UpdateSubscriptionRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpdateSubscriptionRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> UpdateSubscriptionRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpdateSubscriptionRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpdateSubscriptionRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpdateSubscriptionRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpdateSubscriptionRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional message google.pubsub.v1.Subscription
  pub fn has_subscription(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_subscription(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn subscription_opt(&self) -> ::protobuf::Optional<super::SubscriptionView<'_>> {
        ::protobuf::Optional::new(self.subscription(), self.has_subscription())
  }
  pub fn subscription(&self) -> super::SubscriptionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SubscriptionView::default())
  }
  pub fn subscription_mut(&mut self) -> super::SubscriptionMut<'_> {
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
  pub fn set_subscription(&mut self,
    val: impl ::protobuf::IntoProxied<super::Subscription>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // update_mask: optional message google.protobuf.FieldMask
  pub fn has_update_mask(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_update_mask(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn update_mask_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::FieldMaskView<'_>> {
        ::protobuf::Optional::new(self.update_mask(), self.has_update_mask())
  }
  pub fn update_mask(&self) -> ::protobuf_well_known_types::FieldMaskView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FieldMaskView::default())
  }
  pub fn update_mask_mut(&mut self) -> ::protobuf_well_known_types::FieldMaskMut<'_> {
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
  pub fn set_update_mask(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::FieldMask>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl UpdateSubscriptionRequest

impl ::std::ops::Drop for UpdateSubscriptionRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpdateSubscriptionRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpdateSubscriptionRequest {
  type Proxied = Self;
  fn as_view(&self) -> UpdateSubscriptionRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpdateSubscriptionRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpdateSubscriptionRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpdateSubscriptionRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__UpdateSubscriptionRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__UpdateSubscriptionRequest_msg_init.0, &[<super::Subscription as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::FieldMask as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__UpdateSubscriptionRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdateSubscriptionRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdateSubscriptionRequest {
  type Msg = UpdateSubscriptionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSubscriptionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateSubscriptionRequest {
  type Msg = UpdateSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSubscriptionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdateSubscriptionRequestMut<'_> {
  type Msg = UpdateSubscriptionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSubscriptionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateSubscriptionRequestMut<'_> {
  type Msg = UpdateSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSubscriptionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateSubscriptionRequestView<'_> {
  type Msg = UpdateSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSubscriptionRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdateSubscriptionRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListSubscriptionsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListSubscriptionsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListSubscriptionsRequest>
}

impl ::protobuf::Message for ListSubscriptionsRequest {}

impl ::std::default::Default for ListSubscriptionsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListSubscriptionsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListSubscriptionsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ListSubscriptionsRequestMut`.
unsafe impl Sync for ListSubscriptionsRequest {}

// SAFETY:
// - `ListSubscriptionsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListSubscriptionsRequest {}

impl ::protobuf::Proxied for ListSubscriptionsRequest {
  type View<'msg> = ListSubscriptionsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListSubscriptionsRequest {}

impl ::protobuf::MutProxied for ListSubscriptionsRequest {
  type Mut<'msg> = ListSubscriptionsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListSubscriptionsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSubscriptionsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSubscriptionsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListSubscriptionsRequestView<'msg> {
  type Message = ListSubscriptionsRequest;
}

impl ::std::fmt::Debug for ListSubscriptionsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListSubscriptionsRequestView<'_> {
  fn default() -> ListSubscriptionsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListSubscriptionsRequest>> for ListSubscriptionsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSubscriptionsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSubscriptionsRequestView<'msg> {

  pub fn to_owned(&self) -> ListSubscriptionsRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // project: optional string
  pub fn project(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // page_size: optional int32
  pub fn page_size(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // page_token: optional string
  pub fn page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListSubscriptionsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListSubscriptionsRequestView<'_> {}

// SAFETY:
// - `ListSubscriptionsRequestView` is `Send` because while its alive a `ListSubscriptionsRequestMut` cannot.
// - `ListSubscriptionsRequestView` does not use thread-local data.
unsafe impl Send for ListSubscriptionsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ListSubscriptionsRequestView<'msg> {
  type Proxied = ListSubscriptionsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ListSubscriptionsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSubscriptionsRequestView<'msg> {
  fn into_view<'shorter>(self) -> ListSubscriptionsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSubscriptionsRequest> for ListSubscriptionsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSubscriptionsRequest {
    let mut dst = ListSubscriptionsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSubscriptionsRequest> for ListSubscriptionsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSubscriptionsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListSubscriptionsRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSubscriptionsRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSubscriptionsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListSubscriptionsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSubscriptionsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSubscriptionsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListSubscriptionsRequestMut<'msg> {
  type Message = ListSubscriptionsRequest;
}

impl ::std::fmt::Debug for ListSubscriptionsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListSubscriptionsRequest>> for ListSubscriptionsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSubscriptionsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSubscriptionsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSubscriptionsRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ListSubscriptionsRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // project: optional string
  pub fn project(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_project(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `ListSubscriptionsRequestMut` does not perform any shared mutation.
unsafe impl Send for ListSubscriptionsRequestMut<'_> {}

// SAFETY:
// - `ListSubscriptionsRequestMut` does not perform any shared mutation.
unsafe impl Sync for ListSubscriptionsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ListSubscriptionsRequestMut<'msg> {
  type Proxied = ListSubscriptionsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ListSubscriptionsRequest> {
    ListSubscriptionsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSubscriptionsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListSubscriptionsRequest>
  where
      'msg: 'shorter {
    ListSubscriptionsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListSubscriptionsRequestMut<'msg> {
  type MutProxied = ListSubscriptionsRequest;
  fn as_mut(&mut self) -> ListSubscriptionsRequestMut<'msg> {
    ListSubscriptionsRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListSubscriptionsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ListSubscriptionsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListSubscriptionsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListSubscriptionsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListSubscriptionsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListSubscriptionsRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // project: optional string
  pub fn project(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_project(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl ListSubscriptionsRequest

impl ::std::ops::Drop for ListSubscriptionsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListSubscriptionsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListSubscriptionsRequest {
  type Proxied = Self;
  fn as_view(&self) -> ListSubscriptionsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListSubscriptionsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListSubscriptionsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListSubscriptionsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListSubscriptionsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X(P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListSubscriptionsRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListSubscriptionsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSubscriptionsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSubscriptionsRequest {
  type Msg = ListSubscriptionsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSubscriptionsRequest {
  type Msg = ListSubscriptionsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSubscriptionsRequestMut<'_> {
  type Msg = ListSubscriptionsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSubscriptionsRequestMut<'_> {
  type Msg = ListSubscriptionsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSubscriptionsRequestView<'_> {
  type Msg = ListSubscriptionsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSubscriptionsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListSubscriptionsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListSubscriptionsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListSubscriptionsResponse>
}

impl ::protobuf::Message for ListSubscriptionsResponse {}

impl ::std::default::Default for ListSubscriptionsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListSubscriptionsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListSubscriptionsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ListSubscriptionsResponseMut`.
unsafe impl Sync for ListSubscriptionsResponse {}

// SAFETY:
// - `ListSubscriptionsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListSubscriptionsResponse {}

impl ::protobuf::Proxied for ListSubscriptionsResponse {
  type View<'msg> = ListSubscriptionsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListSubscriptionsResponse {}

impl ::protobuf::MutProxied for ListSubscriptionsResponse {
  type Mut<'msg> = ListSubscriptionsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListSubscriptionsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSubscriptionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSubscriptionsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListSubscriptionsResponseView<'msg> {
  type Message = ListSubscriptionsResponse;
}

impl ::std::fmt::Debug for ListSubscriptionsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListSubscriptionsResponseView<'_> {
  fn default() -> ListSubscriptionsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListSubscriptionsResponse>> for ListSubscriptionsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSubscriptionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSubscriptionsResponseView<'msg> {

  pub fn to_owned(&self) -> ListSubscriptionsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscriptions: repeated message google.pubsub.v1.Subscription
  pub fn subscriptions(self) -> ::protobuf::RepeatedView<'msg, super::Subscription> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Subscription>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // next_page_token: optional string
  pub fn next_page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListSubscriptionsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListSubscriptionsResponseView<'_> {}

// SAFETY:
// - `ListSubscriptionsResponseView` is `Send` because while its alive a `ListSubscriptionsResponseMut` cannot.
// - `ListSubscriptionsResponseView` does not use thread-local data.
unsafe impl Send for ListSubscriptionsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ListSubscriptionsResponseView<'msg> {
  type Proxied = ListSubscriptionsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ListSubscriptionsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSubscriptionsResponseView<'msg> {
  fn into_view<'shorter>(self) -> ListSubscriptionsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSubscriptionsResponse> for ListSubscriptionsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSubscriptionsResponse {
    let mut dst = ListSubscriptionsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSubscriptionsResponse> for ListSubscriptionsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSubscriptionsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListSubscriptionsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSubscriptionsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSubscriptionsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListSubscriptionsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSubscriptionsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSubscriptionsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListSubscriptionsResponseMut<'msg> {
  type Message = ListSubscriptionsResponse;
}

impl ::std::fmt::Debug for ListSubscriptionsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListSubscriptionsResponse>> for ListSubscriptionsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSubscriptionsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSubscriptionsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSubscriptionsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ListSubscriptionsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscriptions: repeated message google.pubsub.v1.Subscription
  pub fn subscriptions(&self) -> ::protobuf::RepeatedView<'_, super::Subscription> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Subscription>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn subscriptions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Subscription> {
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
  pub fn set_subscriptions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Subscription>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ListSubscriptionsResponseMut` does not perform any shared mutation.
unsafe impl Send for ListSubscriptionsResponseMut<'_> {}

// SAFETY:
// - `ListSubscriptionsResponseMut` does not perform any shared mutation.
unsafe impl Sync for ListSubscriptionsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ListSubscriptionsResponseMut<'msg> {
  type Proxied = ListSubscriptionsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ListSubscriptionsResponse> {
    ListSubscriptionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSubscriptionsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListSubscriptionsResponse>
  where
      'msg: 'shorter {
    ListSubscriptionsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListSubscriptionsResponseMut<'msg> {
  type MutProxied = ListSubscriptionsResponse;
  fn as_mut(&mut self) -> ListSubscriptionsResponseMut<'msg> {
    ListSubscriptionsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListSubscriptionsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ListSubscriptionsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListSubscriptionsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListSubscriptionsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListSubscriptionsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListSubscriptionsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscriptions: repeated message google.pubsub.v1.Subscription
  pub fn subscriptions(&self) -> ::protobuf::RepeatedView<'_, super::Subscription> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Subscription>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn subscriptions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Subscription> {
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
  pub fn set_subscriptions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Subscription>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl ListSubscriptionsResponse

impl ::std::ops::Drop for ListSubscriptionsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListSubscriptionsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListSubscriptionsResponse {
  type Proxied = Self;
  fn as_view(&self) -> ListSubscriptionsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListSubscriptionsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListSubscriptionsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListSubscriptionsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListSubscriptionsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListSubscriptionsResponse_msg_init.0, &[<super::Subscription as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListSubscriptionsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSubscriptionsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSubscriptionsResponse {
  type Msg = ListSubscriptionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSubscriptionsResponse {
  type Msg = ListSubscriptionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSubscriptionsResponseMut<'_> {
  type Msg = ListSubscriptionsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSubscriptionsResponseMut<'_> {
  type Msg = ListSubscriptionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSubscriptionsResponseView<'_> {
  type Msg = ListSubscriptionsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSubscriptionsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSubscriptionsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__DeleteSubscriptionRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeleteSubscriptionRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeleteSubscriptionRequest>
}

impl ::protobuf::Message for DeleteSubscriptionRequest {}

impl ::std::default::Default for DeleteSubscriptionRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeleteSubscriptionRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeleteSubscriptionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DeleteSubscriptionRequestMut`.
unsafe impl Sync for DeleteSubscriptionRequest {}

// SAFETY:
// - `DeleteSubscriptionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DeleteSubscriptionRequest {}

impl ::protobuf::Proxied for DeleteSubscriptionRequest {
  type View<'msg> = DeleteSubscriptionRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeleteSubscriptionRequest {}

impl ::protobuf::MutProxied for DeleteSubscriptionRequest {
  type Mut<'msg> = DeleteSubscriptionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeleteSubscriptionRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteSubscriptionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeleteSubscriptionRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeleteSubscriptionRequestView<'msg> {
  type Message = DeleteSubscriptionRequest;
}

impl ::std::fmt::Debug for DeleteSubscriptionRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeleteSubscriptionRequestView<'_> {
  fn default() -> DeleteSubscriptionRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteSubscriptionRequest>> for DeleteSubscriptionRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteSubscriptionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeleteSubscriptionRequestView<'msg> {

  pub fn to_owned(&self) -> DeleteSubscriptionRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `DeleteSubscriptionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for DeleteSubscriptionRequestView<'_> {}

// SAFETY:
// - `DeleteSubscriptionRequestView` is `Send` because while its alive a `DeleteSubscriptionRequestMut` cannot.
// - `DeleteSubscriptionRequestView` does not use thread-local data.
unsafe impl Send for DeleteSubscriptionRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DeleteSubscriptionRequestView<'msg> {
  type Proxied = DeleteSubscriptionRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DeleteSubscriptionRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeleteSubscriptionRequestView<'msg> {
  fn into_view<'shorter>(self) -> DeleteSubscriptionRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeleteSubscriptionRequest> for DeleteSubscriptionRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeleteSubscriptionRequest {
    let mut dst = DeleteSubscriptionRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeleteSubscriptionRequest> for DeleteSubscriptionRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeleteSubscriptionRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DeleteSubscriptionRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeleteSubscriptionRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeleteSubscriptionRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeleteSubscriptionRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSubscriptionRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeleteSubscriptionRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeleteSubscriptionRequestMut<'msg> {
  type Message = DeleteSubscriptionRequest;
}

impl ::std::fmt::Debug for DeleteSubscriptionRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSubscriptionRequest>> for DeleteSubscriptionRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSubscriptionRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeleteSubscriptionRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSubscriptionRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> DeleteSubscriptionRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `DeleteSubscriptionRequestMut` does not perform any shared mutation.
unsafe impl Send for DeleteSubscriptionRequestMut<'_> {}

// SAFETY:
// - `DeleteSubscriptionRequestMut` does not perform any shared mutation.
unsafe impl Sync for DeleteSubscriptionRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DeleteSubscriptionRequestMut<'msg> {
  type Proxied = DeleteSubscriptionRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DeleteSubscriptionRequest> {
    DeleteSubscriptionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeleteSubscriptionRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeleteSubscriptionRequest>
  where
      'msg: 'shorter {
    DeleteSubscriptionRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DeleteSubscriptionRequestMut<'msg> {
  type MutProxied = DeleteSubscriptionRequest;
  fn as_mut(&mut self) -> DeleteSubscriptionRequestMut<'msg> {
    DeleteSubscriptionRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeleteSubscriptionRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DeleteSubscriptionRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeleteSubscriptionRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeleteSubscriptionRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeleteSubscriptionRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeleteSubscriptionRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl DeleteSubscriptionRequest

impl ::std::ops::Drop for DeleteSubscriptionRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeleteSubscriptionRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeleteSubscriptionRequest {
  type Proxied = Self;
  fn as_view(&self) -> DeleteSubscriptionRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeleteSubscriptionRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeleteSubscriptionRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeleteSubscriptionRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__DeleteSubscriptionRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__DeleteSubscriptionRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__DeleteSubscriptionRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeleteSubscriptionRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeleteSubscriptionRequest {
  type Msg = DeleteSubscriptionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSubscriptionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteSubscriptionRequest {
  type Msg = DeleteSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSubscriptionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeleteSubscriptionRequestMut<'_> {
  type Msg = DeleteSubscriptionRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSubscriptionRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteSubscriptionRequestMut<'_> {
  type Msg = DeleteSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSubscriptionRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteSubscriptionRequestView<'_> {
  type Msg = DeleteSubscriptionRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSubscriptionRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeleteSubscriptionRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ModifyPushConfigRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ModifyPushConfigRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ModifyPushConfigRequest>
}

impl ::protobuf::Message for ModifyPushConfigRequest {}

impl ::std::default::Default for ModifyPushConfigRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ModifyPushConfigRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ModifyPushConfigRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ModifyPushConfigRequestMut`.
unsafe impl Sync for ModifyPushConfigRequest {}

// SAFETY:
// - `ModifyPushConfigRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ModifyPushConfigRequest {}

impl ::protobuf::Proxied for ModifyPushConfigRequest {
  type View<'msg> = ModifyPushConfigRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ModifyPushConfigRequest {}

impl ::protobuf::MutProxied for ModifyPushConfigRequest {
  type Mut<'msg> = ModifyPushConfigRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ModifyPushConfigRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ModifyPushConfigRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ModifyPushConfigRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ModifyPushConfigRequestView<'msg> {
  type Message = ModifyPushConfigRequest;
}

impl ::std::fmt::Debug for ModifyPushConfigRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ModifyPushConfigRequestView<'_> {
  fn default() -> ModifyPushConfigRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ModifyPushConfigRequest>> for ModifyPushConfigRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ModifyPushConfigRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ModifyPushConfigRequestView<'msg> {

  pub fn to_owned(&self) -> ModifyPushConfigRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // push_config: optional message google.pubsub.v1.PushConfig
  pub fn has_push_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn push_config_opt(self) -> ::protobuf::Optional<super::PushConfigView<'msg>> {
        ::protobuf::Optional::new(self.push_config(), self.has_push_config())
  }
  pub fn push_config(self) -> super::PushConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PushConfigView::default())
  }

}

// SAFETY:
// - `ModifyPushConfigRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ModifyPushConfigRequestView<'_> {}

// SAFETY:
// - `ModifyPushConfigRequestView` is `Send` because while its alive a `ModifyPushConfigRequestMut` cannot.
// - `ModifyPushConfigRequestView` does not use thread-local data.
unsafe impl Send for ModifyPushConfigRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ModifyPushConfigRequestView<'msg> {
  type Proxied = ModifyPushConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ModifyPushConfigRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ModifyPushConfigRequestView<'msg> {
  fn into_view<'shorter>(self) -> ModifyPushConfigRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ModifyPushConfigRequest> for ModifyPushConfigRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ModifyPushConfigRequest {
    let mut dst = ModifyPushConfigRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ModifyPushConfigRequest> for ModifyPushConfigRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ModifyPushConfigRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ModifyPushConfigRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ModifyPushConfigRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ModifyPushConfigRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ModifyPushConfigRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ModifyPushConfigRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ModifyPushConfigRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ModifyPushConfigRequestMut<'msg> {
  type Message = ModifyPushConfigRequest;
}

impl ::std::fmt::Debug for ModifyPushConfigRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ModifyPushConfigRequest>> for ModifyPushConfigRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ModifyPushConfigRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ModifyPushConfigRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ModifyPushConfigRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ModifyPushConfigRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // push_config: optional message google.pubsub.v1.PushConfig
  pub fn has_push_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_push_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn push_config_opt(&self) -> ::protobuf::Optional<super::PushConfigView<'_>> {
        ::protobuf::Optional::new(self.push_config(), self.has_push_config())
  }
  pub fn push_config(&self) -> super::PushConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PushConfigView::default())
  }
  pub fn push_config_mut(&mut self) -> super::PushConfigMut<'_> {
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
  pub fn set_push_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::PushConfig>) {

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
// - `ModifyPushConfigRequestMut` does not perform any shared mutation.
unsafe impl Send for ModifyPushConfigRequestMut<'_> {}

// SAFETY:
// - `ModifyPushConfigRequestMut` does not perform any shared mutation.
unsafe impl Sync for ModifyPushConfigRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ModifyPushConfigRequestMut<'msg> {
  type Proxied = ModifyPushConfigRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ModifyPushConfigRequest> {
    ModifyPushConfigRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ModifyPushConfigRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ModifyPushConfigRequest>
  where
      'msg: 'shorter {
    ModifyPushConfigRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ModifyPushConfigRequestMut<'msg> {
  type MutProxied = ModifyPushConfigRequest;
  fn as_mut(&mut self) -> ModifyPushConfigRequestMut<'msg> {
    ModifyPushConfigRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ModifyPushConfigRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ModifyPushConfigRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ModifyPushConfigRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ModifyPushConfigRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ModifyPushConfigRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ModifyPushConfigRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // push_config: optional message google.pubsub.v1.PushConfig
  pub fn has_push_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_push_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn push_config_opt(&self) -> ::protobuf::Optional<super::PushConfigView<'_>> {
        ::protobuf::Optional::new(self.push_config(), self.has_push_config())
  }
  pub fn push_config(&self) -> super::PushConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PushConfigView::default())
  }
  pub fn push_config_mut(&mut self) -> super::PushConfigMut<'_> {
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
  pub fn set_push_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::PushConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ModifyPushConfigRequest

impl ::std::ops::Drop for ModifyPushConfigRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ModifyPushConfigRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ModifyPushConfigRequest {
  type Proxied = Self;
  fn as_view(&self) -> ModifyPushConfigRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ModifyPushConfigRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ModifyPushConfigRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ModifyPushConfigRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ModifyPushConfigRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ModifyPushConfigRequest_msg_init.0, &[<super::PushConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ModifyPushConfigRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ModifyPushConfigRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ModifyPushConfigRequest {
  type Msg = ModifyPushConfigRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyPushConfigRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModifyPushConfigRequest {
  type Msg = ModifyPushConfigRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyPushConfigRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ModifyPushConfigRequestMut<'_> {
  type Msg = ModifyPushConfigRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyPushConfigRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModifyPushConfigRequestMut<'_> {
  type Msg = ModifyPushConfigRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyPushConfigRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModifyPushConfigRequestView<'_> {
  type Msg = ModifyPushConfigRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyPushConfigRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ModifyPushConfigRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__PullRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PullRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PullRequest>
}

impl ::protobuf::Message for PullRequest {}

impl ::std::default::Default for PullRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PullRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PullRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PullRequestMut`.
unsafe impl Sync for PullRequest {}

// SAFETY:
// - `PullRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PullRequest {}

impl ::protobuf::Proxied for PullRequest {
  type View<'msg> = PullRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PullRequest {}

impl ::protobuf::MutProxied for PullRequest {
  type Mut<'msg> = PullRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PullRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PullRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PullRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PullRequestView<'msg> {
  type Message = PullRequest;
}

impl ::std::fmt::Debug for PullRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PullRequestView<'_> {
  fn default() -> PullRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PullRequest>> for PullRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PullRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PullRequestView<'msg> {

  pub fn to_owned(&self) -> PullRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // return_immediately: optional bool
  pub fn return_immediately(self) -> bool {
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

  // max_messages: optional int32
  pub fn max_messages(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PullRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PullRequestView<'_> {}

// SAFETY:
// - `PullRequestView` is `Send` because while its alive a `PullRequestMut` cannot.
// - `PullRequestView` does not use thread-local data.
unsafe impl Send for PullRequestView<'_> {}

impl<'msg> ::protobuf::AsView for PullRequestView<'msg> {
  type Proxied = PullRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, PullRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PullRequestView<'msg> {
  fn into_view<'shorter>(self) -> PullRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PullRequest> for PullRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PullRequest {
    let mut dst = PullRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PullRequest> for PullRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PullRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PullRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PullRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PullRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PullRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PullRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PullRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PullRequestMut<'msg> {
  type Message = PullRequest;
}

impl ::std::fmt::Debug for PullRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PullRequest>> for PullRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PullRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PullRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PullRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> PullRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // return_immediately: optional bool
  pub fn return_immediately(&self) -> bool {
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
  pub fn set_return_immediately(&mut self, val: bool) {
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

  // max_messages: optional int32
  pub fn max_messages(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_messages(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `PullRequestMut` does not perform any shared mutation.
unsafe impl Send for PullRequestMut<'_> {}

// SAFETY:
// - `PullRequestMut` does not perform any shared mutation.
unsafe impl Sync for PullRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for PullRequestMut<'msg> {
  type Proxied = PullRequest;
  fn as_view(&self) -> ::protobuf::View<'_, PullRequest> {
    PullRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PullRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PullRequest>
  where
      'msg: 'shorter {
    PullRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PullRequestMut<'msg> {
  type MutProxied = PullRequest;
  fn as_mut(&mut self) -> PullRequestMut<'msg> {
    PullRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PullRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> PullRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PullRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PullRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PullRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PullRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // return_immediately: optional bool
  pub fn return_immediately(&self) -> bool {
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
  pub fn set_return_immediately(&mut self, val: bool) {
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

  // max_messages: optional int32
  pub fn max_messages(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_messages(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}  // impl PullRequest

impl ::std::ops::Drop for PullRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PullRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PullRequest {
  type Proxied = Self;
  fn as_view(&self) -> PullRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PullRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PullRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PullRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__PullRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X/P(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__PullRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__PullRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PullRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PullRequest {
  type Msg = PullRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PullRequest {
  type Msg = PullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PullRequestMut<'_> {
  type Msg = PullRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PullRequestMut<'_> {
  type Msg = PullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PullRequestView<'_> {
  type Msg = PullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PullRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__PullResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PullResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PullResponse>
}

impl ::protobuf::Message for PullResponse {}

impl ::std::default::Default for PullResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PullResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PullResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PullResponseMut`.
unsafe impl Sync for PullResponse {}

// SAFETY:
// - `PullResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PullResponse {}

impl ::protobuf::Proxied for PullResponse {
  type View<'msg> = PullResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PullResponse {}

impl ::protobuf::MutProxied for PullResponse {
  type Mut<'msg> = PullResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PullResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PullResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PullResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PullResponseView<'msg> {
  type Message = PullResponse;
}

impl ::std::fmt::Debug for PullResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PullResponseView<'_> {
  fn default() -> PullResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PullResponse>> for PullResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PullResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PullResponseView<'msg> {

  pub fn to_owned(&self) -> PullResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // received_messages: repeated message google.pubsub.v1.ReceivedMessage
  pub fn received_messages(self) -> ::protobuf::RepeatedView<'msg, super::ReceivedMessage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ReceivedMessage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PullResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PullResponseView<'_> {}

// SAFETY:
// - `PullResponseView` is `Send` because while its alive a `PullResponseMut` cannot.
// - `PullResponseView` does not use thread-local data.
unsafe impl Send for PullResponseView<'_> {}

impl<'msg> ::protobuf::AsView for PullResponseView<'msg> {
  type Proxied = PullResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, PullResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PullResponseView<'msg> {
  fn into_view<'shorter>(self) -> PullResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PullResponse> for PullResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PullResponse {
    let mut dst = PullResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PullResponse> for PullResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PullResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for PullResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PullResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for PullResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PullResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PullResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PullResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PullResponseMut<'msg> {
  type Message = PullResponse;
}

impl ::std::fmt::Debug for PullResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PullResponse>> for PullResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PullResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PullResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PullResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> PullResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // received_messages: repeated message google.pubsub.v1.ReceivedMessage
  pub fn received_messages(&self) -> ::protobuf::RepeatedView<'_, super::ReceivedMessage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ReceivedMessage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn received_messages_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ReceivedMessage> {
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
  pub fn set_received_messages(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ReceivedMessage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `PullResponseMut` does not perform any shared mutation.
unsafe impl Send for PullResponseMut<'_> {}

// SAFETY:
// - `PullResponseMut` does not perform any shared mutation.
unsafe impl Sync for PullResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for PullResponseMut<'msg> {
  type Proxied = PullResponse;
  fn as_view(&self) -> ::protobuf::View<'_, PullResponse> {
    PullResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PullResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PullResponse>
  where
      'msg: 'shorter {
    PullResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for PullResponseMut<'msg> {
  type MutProxied = PullResponse;
  fn as_mut(&mut self) -> PullResponseMut<'msg> {
    PullResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PullResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> PullResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PullResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PullResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PullResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PullResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // received_messages: repeated message google.pubsub.v1.ReceivedMessage
  pub fn received_messages(&self) -> ::protobuf::RepeatedView<'_, super::ReceivedMessage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ReceivedMessage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn received_messages_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ReceivedMessage> {
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
  pub fn set_received_messages(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ReceivedMessage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl PullResponse

impl ::std::ops::Drop for PullResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PullResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PullResponse {
  type Proxied = Self;
  fn as_view(&self) -> PullResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PullResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PullResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PullResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__PullResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__PullResponse_msg_init.0, &[<super::ReceivedMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__PullResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PullResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PullResponse {
  type Msg = PullResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PullResponse {
  type Msg = PullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PullResponseMut<'_> {
  type Msg = PullResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PullResponseMut<'_> {
  type Msg = PullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PullResponseView<'_> {
  type Msg = PullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PullResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PullResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ModifyAckDeadlineRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ModifyAckDeadlineRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ModifyAckDeadlineRequest>
}

impl ::protobuf::Message for ModifyAckDeadlineRequest {}

impl ::std::default::Default for ModifyAckDeadlineRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ModifyAckDeadlineRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ModifyAckDeadlineRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ModifyAckDeadlineRequestMut`.
unsafe impl Sync for ModifyAckDeadlineRequest {}

// SAFETY:
// - `ModifyAckDeadlineRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ModifyAckDeadlineRequest {}

impl ::protobuf::Proxied for ModifyAckDeadlineRequest {
  type View<'msg> = ModifyAckDeadlineRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ModifyAckDeadlineRequest {}

impl ::protobuf::MutProxied for ModifyAckDeadlineRequest {
  type Mut<'msg> = ModifyAckDeadlineRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ModifyAckDeadlineRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ModifyAckDeadlineRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ModifyAckDeadlineRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ModifyAckDeadlineRequestView<'msg> {
  type Message = ModifyAckDeadlineRequest;
}

impl ::std::fmt::Debug for ModifyAckDeadlineRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ModifyAckDeadlineRequestView<'_> {
  fn default() -> ModifyAckDeadlineRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ModifyAckDeadlineRequest>> for ModifyAckDeadlineRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ModifyAckDeadlineRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ModifyAckDeadlineRequestView<'msg> {

  pub fn to_owned(&self) -> ModifyAckDeadlineRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // ack_ids: repeated string
  pub fn ack_ids(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ack_deadline_seconds: optional int32
  pub fn ack_deadline_seconds(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ModifyAckDeadlineRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ModifyAckDeadlineRequestView<'_> {}

// SAFETY:
// - `ModifyAckDeadlineRequestView` is `Send` because while its alive a `ModifyAckDeadlineRequestMut` cannot.
// - `ModifyAckDeadlineRequestView` does not use thread-local data.
unsafe impl Send for ModifyAckDeadlineRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ModifyAckDeadlineRequestView<'msg> {
  type Proxied = ModifyAckDeadlineRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ModifyAckDeadlineRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ModifyAckDeadlineRequestView<'msg> {
  fn into_view<'shorter>(self) -> ModifyAckDeadlineRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ModifyAckDeadlineRequest> for ModifyAckDeadlineRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ModifyAckDeadlineRequest {
    let mut dst = ModifyAckDeadlineRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ModifyAckDeadlineRequest> for ModifyAckDeadlineRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ModifyAckDeadlineRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ModifyAckDeadlineRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ModifyAckDeadlineRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ModifyAckDeadlineRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ModifyAckDeadlineRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ModifyAckDeadlineRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ModifyAckDeadlineRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ModifyAckDeadlineRequestMut<'msg> {
  type Message = ModifyAckDeadlineRequest;
}

impl ::std::fmt::Debug for ModifyAckDeadlineRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ModifyAckDeadlineRequest>> for ModifyAckDeadlineRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ModifyAckDeadlineRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ModifyAckDeadlineRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ModifyAckDeadlineRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ModifyAckDeadlineRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // ack_ids: repeated string
  pub fn ack_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ack_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_ack_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // ack_deadline_seconds: optional int32
  pub fn ack_deadline_seconds(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ack_deadline_seconds(&mut self, val: i32) {
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

}

// SAFETY:
// - `ModifyAckDeadlineRequestMut` does not perform any shared mutation.
unsafe impl Send for ModifyAckDeadlineRequestMut<'_> {}

// SAFETY:
// - `ModifyAckDeadlineRequestMut` does not perform any shared mutation.
unsafe impl Sync for ModifyAckDeadlineRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ModifyAckDeadlineRequestMut<'msg> {
  type Proxied = ModifyAckDeadlineRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ModifyAckDeadlineRequest> {
    ModifyAckDeadlineRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ModifyAckDeadlineRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ModifyAckDeadlineRequest>
  where
      'msg: 'shorter {
    ModifyAckDeadlineRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ModifyAckDeadlineRequestMut<'msg> {
  type MutProxied = ModifyAckDeadlineRequest;
  fn as_mut(&mut self) -> ModifyAckDeadlineRequestMut<'msg> {
    ModifyAckDeadlineRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ModifyAckDeadlineRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ModifyAckDeadlineRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ModifyAckDeadlineRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ModifyAckDeadlineRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ModifyAckDeadlineRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ModifyAckDeadlineRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // ack_ids: repeated string
  pub fn ack_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ack_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_ack_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // ack_deadline_seconds: optional int32
  pub fn ack_deadline_seconds(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ack_deadline_seconds(&mut self, val: i32) {
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

}  // impl ModifyAckDeadlineRequest

impl ::std::ops::Drop for ModifyAckDeadlineRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ModifyAckDeadlineRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ModifyAckDeadlineRequest {
  type Proxied = Self;
  fn as_view(&self) -> ModifyAckDeadlineRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ModifyAckDeadlineRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ModifyAckDeadlineRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ModifyAckDeadlineRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ModifyAckDeadlineRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa(PET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ModifyAckDeadlineRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ModifyAckDeadlineRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ModifyAckDeadlineRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ModifyAckDeadlineRequest {
  type Msg = ModifyAckDeadlineRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyAckDeadlineRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModifyAckDeadlineRequest {
  type Msg = ModifyAckDeadlineRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyAckDeadlineRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ModifyAckDeadlineRequestMut<'_> {
  type Msg = ModifyAckDeadlineRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyAckDeadlineRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModifyAckDeadlineRequestMut<'_> {
  type Msg = ModifyAckDeadlineRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyAckDeadlineRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModifyAckDeadlineRequestView<'_> {
  type Msg = ModifyAckDeadlineRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifyAckDeadlineRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ModifyAckDeadlineRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__AcknowledgeRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AcknowledgeRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AcknowledgeRequest>
}

impl ::protobuf::Message for AcknowledgeRequest {}

impl ::std::default::Default for AcknowledgeRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AcknowledgeRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AcknowledgeRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `AcknowledgeRequestMut`.
unsafe impl Sync for AcknowledgeRequest {}

// SAFETY:
// - `AcknowledgeRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AcknowledgeRequest {}

impl ::protobuf::Proxied for AcknowledgeRequest {
  type View<'msg> = AcknowledgeRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AcknowledgeRequest {}

impl ::protobuf::MutProxied for AcknowledgeRequest {
  type Mut<'msg> = AcknowledgeRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AcknowledgeRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AcknowledgeRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AcknowledgeRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AcknowledgeRequestView<'msg> {
  type Message = AcknowledgeRequest;
}

impl ::std::fmt::Debug for AcknowledgeRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AcknowledgeRequestView<'_> {
  fn default() -> AcknowledgeRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AcknowledgeRequest>> for AcknowledgeRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AcknowledgeRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AcknowledgeRequestView<'msg> {

  pub fn to_owned(&self) -> AcknowledgeRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // ack_ids: repeated string
  pub fn ack_ids(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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
// - `AcknowledgeRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for AcknowledgeRequestView<'_> {}

// SAFETY:
// - `AcknowledgeRequestView` is `Send` because while its alive a `AcknowledgeRequestMut` cannot.
// - `AcknowledgeRequestView` does not use thread-local data.
unsafe impl Send for AcknowledgeRequestView<'_> {}

impl<'msg> ::protobuf::AsView for AcknowledgeRequestView<'msg> {
  type Proxied = AcknowledgeRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, AcknowledgeRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AcknowledgeRequestView<'msg> {
  fn into_view<'shorter>(self) -> AcknowledgeRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AcknowledgeRequest> for AcknowledgeRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AcknowledgeRequest {
    let mut dst = AcknowledgeRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AcknowledgeRequest> for AcknowledgeRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AcknowledgeRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for AcknowledgeRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AcknowledgeRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for AcknowledgeRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AcknowledgeRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AcknowledgeRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AcknowledgeRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AcknowledgeRequestMut<'msg> {
  type Message = AcknowledgeRequest;
}

impl ::std::fmt::Debug for AcknowledgeRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AcknowledgeRequest>> for AcknowledgeRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AcknowledgeRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AcknowledgeRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AcknowledgeRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> AcknowledgeRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // ack_ids: repeated string
  pub fn ack_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn ack_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_ack_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `AcknowledgeRequestMut` does not perform any shared mutation.
unsafe impl Send for AcknowledgeRequestMut<'_> {}

// SAFETY:
// - `AcknowledgeRequestMut` does not perform any shared mutation.
unsafe impl Sync for AcknowledgeRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for AcknowledgeRequestMut<'msg> {
  type Proxied = AcknowledgeRequest;
  fn as_view(&self) -> ::protobuf::View<'_, AcknowledgeRequest> {
    AcknowledgeRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AcknowledgeRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AcknowledgeRequest>
  where
      'msg: 'shorter {
    AcknowledgeRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for AcknowledgeRequestMut<'msg> {
  type MutProxied = AcknowledgeRequest;
  fn as_mut(&mut self) -> AcknowledgeRequestMut<'msg> {
    AcknowledgeRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AcknowledgeRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> AcknowledgeRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AcknowledgeRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AcknowledgeRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AcknowledgeRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AcknowledgeRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // ack_ids: repeated string
  pub fn ack_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn ack_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_ack_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl AcknowledgeRequest

impl ::std::ops::Drop for AcknowledgeRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AcknowledgeRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AcknowledgeRequest {
  type Proxied = Self;
  fn as_view(&self) -> AcknowledgeRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AcknowledgeRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AcknowledgeRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AcknowledgeRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__AcknowledgeRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1PE");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__AcknowledgeRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__AcknowledgeRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AcknowledgeRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AcknowledgeRequest {
  type Msg = AcknowledgeRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AcknowledgeRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AcknowledgeRequest {
  type Msg = AcknowledgeRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AcknowledgeRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AcknowledgeRequestMut<'_> {
  type Msg = AcknowledgeRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AcknowledgeRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AcknowledgeRequestMut<'_> {
  type Msg = AcknowledgeRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AcknowledgeRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AcknowledgeRequestView<'_> {
  type Msg = AcknowledgeRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AcknowledgeRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AcknowledgeRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__StreamingPullRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StreamingPullRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StreamingPullRequest>
}

impl ::protobuf::Message for StreamingPullRequest {}

impl ::std::default::Default for StreamingPullRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StreamingPullRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StreamingPullRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `StreamingPullRequestMut`.
unsafe impl Sync for StreamingPullRequest {}

// SAFETY:
// - `StreamingPullRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for StreamingPullRequest {}

impl ::protobuf::Proxied for StreamingPullRequest {
  type View<'msg> = StreamingPullRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StreamingPullRequest {}

impl ::protobuf::MutProxied for StreamingPullRequest {
  type Mut<'msg> = StreamingPullRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StreamingPullRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StreamingPullRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamingPullRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StreamingPullRequestView<'msg> {
  type Message = StreamingPullRequest;
}

impl ::std::fmt::Debug for StreamingPullRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StreamingPullRequestView<'_> {
  fn default() -> StreamingPullRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StreamingPullRequest>> for StreamingPullRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StreamingPullRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamingPullRequestView<'msg> {

  pub fn to_owned(&self) -> StreamingPullRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // ack_ids: repeated string
  pub fn ack_ids(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // modify_deadline_seconds: repeated int32
  pub fn modify_deadline_seconds(self) -> ::protobuf::RepeatedView<'msg, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // modify_deadline_ack_ids: repeated string
  pub fn modify_deadline_ack_ids(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // stream_ack_deadline_seconds: optional int32
  pub fn stream_ack_deadline_seconds(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // client_id: optional string
  pub fn client_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // max_outstanding_messages: optional int64
  pub fn max_outstanding_messages(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        6, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // max_outstanding_bytes: optional int64
  pub fn max_outstanding_bytes(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        7, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `StreamingPullRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for StreamingPullRequestView<'_> {}

// SAFETY:
// - `StreamingPullRequestView` is `Send` because while its alive a `StreamingPullRequestMut` cannot.
// - `StreamingPullRequestView` does not use thread-local data.
unsafe impl Send for StreamingPullRequestView<'_> {}

impl<'msg> ::protobuf::AsView for StreamingPullRequestView<'msg> {
  type Proxied = StreamingPullRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, StreamingPullRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamingPullRequestView<'msg> {
  fn into_view<'shorter>(self) -> StreamingPullRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StreamingPullRequest> for StreamingPullRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StreamingPullRequest {
    let mut dst = StreamingPullRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StreamingPullRequest> for StreamingPullRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StreamingPullRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for StreamingPullRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for StreamingPullRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for StreamingPullRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StreamingPullRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamingPullRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamingPullRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StreamingPullRequestMut<'msg> {
  type Message = StreamingPullRequest;
}

impl ::std::fmt::Debug for StreamingPullRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StreamingPullRequest>> for StreamingPullRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamingPullRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamingPullRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamingPullRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> StreamingPullRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // ack_ids: repeated string
  pub fn ack_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn ack_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_ack_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // modify_deadline_seconds: repeated int32
  pub fn modify_deadline_seconds(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn modify_deadline_seconds_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_modify_deadline_seconds(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // modify_deadline_ack_ids: repeated string
  pub fn modify_deadline_ack_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn modify_deadline_ack_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_modify_deadline_ack_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // stream_ack_deadline_seconds: optional int32
  pub fn stream_ack_deadline_seconds(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stream_ack_deadline_seconds(&mut self, val: i32) {
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

  // client_id: optional string
  pub fn client_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_client_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // max_outstanding_messages: optional int64
  pub fn max_outstanding_messages(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        6, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_outstanding_messages(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        6, val.into()
      )
    }
  }

  // max_outstanding_bytes: optional int64
  pub fn max_outstanding_bytes(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        7, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_outstanding_bytes(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        7, val.into()
      )
    }
  }

}

// SAFETY:
// - `StreamingPullRequestMut` does not perform any shared mutation.
unsafe impl Send for StreamingPullRequestMut<'_> {}

// SAFETY:
// - `StreamingPullRequestMut` does not perform any shared mutation.
unsafe impl Sync for StreamingPullRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for StreamingPullRequestMut<'msg> {
  type Proxied = StreamingPullRequest;
  fn as_view(&self) -> ::protobuf::View<'_, StreamingPullRequest> {
    StreamingPullRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamingPullRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StreamingPullRequest>
  where
      'msg: 'shorter {
    StreamingPullRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for StreamingPullRequestMut<'msg> {
  type MutProxied = StreamingPullRequest;
  fn as_mut(&mut self) -> StreamingPullRequestMut<'msg> {
    StreamingPullRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StreamingPullRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> StreamingPullRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StreamingPullRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StreamingPullRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StreamingPullRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StreamingPullRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // ack_ids: repeated string
  pub fn ack_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn ack_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_ack_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // modify_deadline_seconds: repeated int32
  pub fn modify_deadline_seconds(&self) -> ::protobuf::RepeatedView<'_, i32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn modify_deadline_seconds_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i32> {
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
  pub fn set_modify_deadline_seconds(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // modify_deadline_ack_ids: repeated string
  pub fn modify_deadline_ack_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn modify_deadline_ack_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_modify_deadline_ack_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // stream_ack_deadline_seconds: optional int32
  pub fn stream_ack_deadline_seconds(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stream_ack_deadline_seconds(&mut self, val: i32) {
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

  // client_id: optional string
  pub fn client_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_client_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // max_outstanding_messages: optional int64
  pub fn max_outstanding_messages(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        6, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_outstanding_messages(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        6, val.into()
      )
    }
  }

  // max_outstanding_bytes: optional int64
  pub fn max_outstanding_bytes(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        7, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_max_outstanding_bytes(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        7, val.into()
      )
    }
  }

}  // impl StreamingPullRequest

impl ::std::ops::Drop for StreamingPullRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StreamingPullRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StreamingPullRequest {
  type Proxied = Self;
  fn as_view(&self) -> StreamingPullRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StreamingPullRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StreamingPullRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StreamingPullRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__StreamingPullRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N1XET<ET(P1X+P+P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__StreamingPullRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__StreamingPullRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StreamingPullRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StreamingPullRequest {
  type Msg = StreamingPullRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamingPullRequest {
  type Msg = StreamingPullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StreamingPullRequestMut<'_> {
  type Msg = StreamingPullRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamingPullRequestMut<'_> {
  type Msg = StreamingPullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamingPullRequestView<'_> {
  type Msg = StreamingPullRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StreamingPullRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__StreamingPullResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StreamingPullResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StreamingPullResponse>
}

impl ::protobuf::Message for StreamingPullResponse {}

impl ::std::default::Default for StreamingPullResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StreamingPullResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StreamingPullResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `StreamingPullResponseMut`.
unsafe impl Sync for StreamingPullResponse {}

// SAFETY:
// - `StreamingPullResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for StreamingPullResponse {}

impl ::protobuf::Proxied for StreamingPullResponse {
  type View<'msg> = StreamingPullResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StreamingPullResponse {}

impl ::protobuf::MutProxied for StreamingPullResponse {
  type Mut<'msg> = StreamingPullResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StreamingPullResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StreamingPullResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamingPullResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StreamingPullResponseView<'msg> {
  type Message = StreamingPullResponse;
}

impl ::std::fmt::Debug for StreamingPullResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StreamingPullResponseView<'_> {
  fn default() -> StreamingPullResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StreamingPullResponse>> for StreamingPullResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StreamingPullResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamingPullResponseView<'msg> {

  pub fn to_owned(&self) -> StreamingPullResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // received_messages: repeated message google.pubsub.v1.ReceivedMessage
  pub fn received_messages(self) -> ::protobuf::RepeatedView<'msg, super::ReceivedMessage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ReceivedMessage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `StreamingPullResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for StreamingPullResponseView<'_> {}

// SAFETY:
// - `StreamingPullResponseView` is `Send` because while its alive a `StreamingPullResponseMut` cannot.
// - `StreamingPullResponseView` does not use thread-local data.
unsafe impl Send for StreamingPullResponseView<'_> {}

impl<'msg> ::protobuf::AsView for StreamingPullResponseView<'msg> {
  type Proxied = StreamingPullResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, StreamingPullResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamingPullResponseView<'msg> {
  fn into_view<'shorter>(self) -> StreamingPullResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StreamingPullResponse> for StreamingPullResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StreamingPullResponse {
    let mut dst = StreamingPullResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StreamingPullResponse> for StreamingPullResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StreamingPullResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for StreamingPullResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for StreamingPullResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for StreamingPullResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StreamingPullResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamingPullResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StreamingPullResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StreamingPullResponseMut<'msg> {
  type Message = StreamingPullResponse;
}

impl ::std::fmt::Debug for StreamingPullResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StreamingPullResponse>> for StreamingPullResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamingPullResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StreamingPullResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StreamingPullResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> StreamingPullResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // received_messages: repeated message google.pubsub.v1.ReceivedMessage
  pub fn received_messages(&self) -> ::protobuf::RepeatedView<'_, super::ReceivedMessage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ReceivedMessage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn received_messages_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ReceivedMessage> {
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
  pub fn set_received_messages(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ReceivedMessage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `StreamingPullResponseMut` does not perform any shared mutation.
unsafe impl Send for StreamingPullResponseMut<'_> {}

// SAFETY:
// - `StreamingPullResponseMut` does not perform any shared mutation.
unsafe impl Sync for StreamingPullResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for StreamingPullResponseMut<'msg> {
  type Proxied = StreamingPullResponse;
  fn as_view(&self) -> ::protobuf::View<'_, StreamingPullResponse> {
    StreamingPullResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StreamingPullResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StreamingPullResponse>
  where
      'msg: 'shorter {
    StreamingPullResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for StreamingPullResponseMut<'msg> {
  type MutProxied = StreamingPullResponse;
  fn as_mut(&mut self) -> StreamingPullResponseMut<'msg> {
    StreamingPullResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StreamingPullResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> StreamingPullResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StreamingPullResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StreamingPullResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StreamingPullResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StreamingPullResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // received_messages: repeated message google.pubsub.v1.ReceivedMessage
  pub fn received_messages(&self) -> ::protobuf::RepeatedView<'_, super::ReceivedMessage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ReceivedMessage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn received_messages_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ReceivedMessage> {
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
  pub fn set_received_messages(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ReceivedMessage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl StreamingPullResponse

impl ::std::ops::Drop for StreamingPullResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StreamingPullResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StreamingPullResponse {
  type Proxied = Self;
  fn as_view(&self) -> StreamingPullResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StreamingPullResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StreamingPullResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StreamingPullResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__StreamingPullResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__StreamingPullResponse_msg_init.0, &[<super::ReceivedMessage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__StreamingPullResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StreamingPullResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StreamingPullResponse {
  type Msg = StreamingPullResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamingPullResponse {
  type Msg = StreamingPullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StreamingPullResponseMut<'_> {
  type Msg = StreamingPullResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamingPullResponseMut<'_> {
  type Msg = StreamingPullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StreamingPullResponseView<'_> {
  type Msg = StreamingPullResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StreamingPullResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StreamingPullResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__CreateSnapshotRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CreateSnapshotRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CreateSnapshotRequest>
}

impl ::protobuf::Message for CreateSnapshotRequest {}

impl ::std::default::Default for CreateSnapshotRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CreateSnapshotRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CreateSnapshotRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `CreateSnapshotRequestMut`.
unsafe impl Sync for CreateSnapshotRequest {}

// SAFETY:
// - `CreateSnapshotRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CreateSnapshotRequest {}

impl ::protobuf::Proxied for CreateSnapshotRequest {
  type View<'msg> = CreateSnapshotRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CreateSnapshotRequest {}

impl ::protobuf::MutProxied for CreateSnapshotRequest {
  type Mut<'msg> = CreateSnapshotRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CreateSnapshotRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CreateSnapshotRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreateSnapshotRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CreateSnapshotRequestView<'msg> {
  type Message = CreateSnapshotRequest;
}

impl ::std::fmt::Debug for CreateSnapshotRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CreateSnapshotRequestView<'_> {
  fn default() -> CreateSnapshotRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CreateSnapshotRequest>> for CreateSnapshotRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CreateSnapshotRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreateSnapshotRequestView<'msg> {

  pub fn to_owned(&self) -> CreateSnapshotRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // labels: repeated message google.pubsub.v1.CreateSnapshotRequest.LabelsEntry
  pub fn labels(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(2)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `CreateSnapshotRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for CreateSnapshotRequestView<'_> {}

// SAFETY:
// - `CreateSnapshotRequestView` is `Send` because while its alive a `CreateSnapshotRequestMut` cannot.
// - `CreateSnapshotRequestView` does not use thread-local data.
unsafe impl Send for CreateSnapshotRequestView<'_> {}

impl<'msg> ::protobuf::AsView for CreateSnapshotRequestView<'msg> {
  type Proxied = CreateSnapshotRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, CreateSnapshotRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreateSnapshotRequestView<'msg> {
  fn into_view<'shorter>(self) -> CreateSnapshotRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CreateSnapshotRequest> for CreateSnapshotRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CreateSnapshotRequest {
    let mut dst = CreateSnapshotRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CreateSnapshotRequest> for CreateSnapshotRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CreateSnapshotRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for CreateSnapshotRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CreateSnapshotRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for CreateSnapshotRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CreateSnapshotRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateSnapshotRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CreateSnapshotRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CreateSnapshotRequestMut<'msg> {
  type Message = CreateSnapshotRequest;
}

impl ::std::fmt::Debug for CreateSnapshotRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CreateSnapshotRequest>> for CreateSnapshotRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateSnapshotRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CreateSnapshotRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CreateSnapshotRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> CreateSnapshotRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // labels: repeated message google.pubsub.v1.CreateSnapshotRequest.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(2)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          2, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `CreateSnapshotRequestMut` does not perform any shared mutation.
unsafe impl Send for CreateSnapshotRequestMut<'_> {}

// SAFETY:
// - `CreateSnapshotRequestMut` does not perform any shared mutation.
unsafe impl Sync for CreateSnapshotRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for CreateSnapshotRequestMut<'msg> {
  type Proxied = CreateSnapshotRequest;
  fn as_view(&self) -> ::protobuf::View<'_, CreateSnapshotRequest> {
    CreateSnapshotRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CreateSnapshotRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CreateSnapshotRequest>
  where
      'msg: 'shorter {
    CreateSnapshotRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for CreateSnapshotRequestMut<'msg> {
  type MutProxied = CreateSnapshotRequest;
  fn as_mut(&mut self) -> CreateSnapshotRequestMut<'msg> {
    CreateSnapshotRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CreateSnapshotRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> CreateSnapshotRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CreateSnapshotRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CreateSnapshotRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CreateSnapshotRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CreateSnapshotRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // labels: repeated message google.pubsub.v1.CreateSnapshotRequest.LabelsEntry
  pub fn labels(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(2)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf::ProtoString>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn labels_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf::ProtoString> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          2, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl CreateSnapshotRequest

impl ::std::ops::Drop for CreateSnapshotRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CreateSnapshotRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CreateSnapshotRequest {
  type Proxied = Self;
  fn as_view(&self) -> CreateSnapshotRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CreateSnapshotRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CreateSnapshotRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CreateSnapshotRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__CreateSnapshotRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__CreateSnapshotRequest_msg_init.0, &[<super::create_snapshot_request::LabelsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__CreateSnapshotRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CreateSnapshotRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CreateSnapshotRequest {
  type Msg = CreateSnapshotRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSnapshotRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateSnapshotRequest {
  type Msg = CreateSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSnapshotRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CreateSnapshotRequestMut<'_> {
  type Msg = CreateSnapshotRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSnapshotRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateSnapshotRequestMut<'_> {
  type Msg = CreateSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSnapshotRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CreateSnapshotRequestView<'_> {
  type Msg = CreateSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CreateSnapshotRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CreateSnapshotRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod create_snapshot_request {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__CreateSnapshotRequest__LabelsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct LabelsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LabelsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::create_snapshot_request::google__pubsub__v1__CreateSnapshotRequest__LabelsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::create_snapshot_request::google__pubsub__v1__CreateSnapshotRequest__LabelsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::create_snapshot_request::google__pubsub__v1__CreateSnapshotRequest__LabelsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod create_snapshot_request


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__UpdateSnapshotRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpdateSnapshotRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpdateSnapshotRequest>
}

impl ::protobuf::Message for UpdateSnapshotRequest {}

impl ::std::default::Default for UpdateSnapshotRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpdateSnapshotRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpdateSnapshotRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `UpdateSnapshotRequestMut`.
unsafe impl Sync for UpdateSnapshotRequest {}

// SAFETY:
// - `UpdateSnapshotRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UpdateSnapshotRequest {}

impl ::protobuf::Proxied for UpdateSnapshotRequest {
  type View<'msg> = UpdateSnapshotRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpdateSnapshotRequest {}

impl ::protobuf::MutProxied for UpdateSnapshotRequest {
  type Mut<'msg> = UpdateSnapshotRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpdateSnapshotRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateSnapshotRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateSnapshotRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpdateSnapshotRequestView<'msg> {
  type Message = UpdateSnapshotRequest;
}

impl ::std::fmt::Debug for UpdateSnapshotRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpdateSnapshotRequestView<'_> {
  fn default() -> UpdateSnapshotRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateSnapshotRequest>> for UpdateSnapshotRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpdateSnapshotRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateSnapshotRequestView<'msg> {

  pub fn to_owned(&self) -> UpdateSnapshotRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // snapshot: optional message google.pubsub.v1.Snapshot
  pub fn has_snapshot(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn snapshot_opt(self) -> ::protobuf::Optional<super::SnapshotView<'msg>> {
        ::protobuf::Optional::new(self.snapshot(), self.has_snapshot())
  }
  pub fn snapshot(self) -> super::SnapshotView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SnapshotView::default())
  }

  // update_mask: optional message google.protobuf.FieldMask
  pub fn has_update_mask(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn update_mask_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::FieldMaskView<'msg>> {
        ::protobuf::Optional::new(self.update_mask(), self.has_update_mask())
  }
  pub fn update_mask(self) -> ::protobuf_well_known_types::FieldMaskView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FieldMaskView::default())
  }

}

// SAFETY:
// - `UpdateSnapshotRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for UpdateSnapshotRequestView<'_> {}

// SAFETY:
// - `UpdateSnapshotRequestView` is `Send` because while its alive a `UpdateSnapshotRequestMut` cannot.
// - `UpdateSnapshotRequestView` does not use thread-local data.
unsafe impl Send for UpdateSnapshotRequestView<'_> {}

impl<'msg> ::protobuf::AsView for UpdateSnapshotRequestView<'msg> {
  type Proxied = UpdateSnapshotRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, UpdateSnapshotRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateSnapshotRequestView<'msg> {
  fn into_view<'shorter>(self) -> UpdateSnapshotRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpdateSnapshotRequest> for UpdateSnapshotRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpdateSnapshotRequest {
    let mut dst = UpdateSnapshotRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpdateSnapshotRequest> for UpdateSnapshotRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpdateSnapshotRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for UpdateSnapshotRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UpdateSnapshotRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for UpdateSnapshotRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpdateSnapshotRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateSnapshotRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateSnapshotRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpdateSnapshotRequestMut<'msg> {
  type Message = UpdateSnapshotRequest;
}

impl ::std::fmt::Debug for UpdateSnapshotRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateSnapshotRequest>> for UpdateSnapshotRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateSnapshotRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateSnapshotRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpdateSnapshotRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> UpdateSnapshotRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // snapshot: optional message google.pubsub.v1.Snapshot
  pub fn has_snapshot(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_snapshot(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn snapshot_opt(&self) -> ::protobuf::Optional<super::SnapshotView<'_>> {
        ::protobuf::Optional::new(self.snapshot(), self.has_snapshot())
  }
  pub fn snapshot(&self) -> super::SnapshotView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SnapshotView::default())
  }
  pub fn snapshot_mut(&mut self) -> super::SnapshotMut<'_> {
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
  pub fn set_snapshot(&mut self,
    val: impl ::protobuf::IntoProxied<super::Snapshot>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // update_mask: optional message google.protobuf.FieldMask
  pub fn has_update_mask(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_update_mask(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn update_mask_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::FieldMaskView<'_>> {
        ::protobuf::Optional::new(self.update_mask(), self.has_update_mask())
  }
  pub fn update_mask(&self) -> ::protobuf_well_known_types::FieldMaskView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FieldMaskView::default())
  }
  pub fn update_mask_mut(&mut self) -> ::protobuf_well_known_types::FieldMaskMut<'_> {
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
  pub fn set_update_mask(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::FieldMask>) {

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
// - `UpdateSnapshotRequestMut` does not perform any shared mutation.
unsafe impl Send for UpdateSnapshotRequestMut<'_> {}

// SAFETY:
// - `UpdateSnapshotRequestMut` does not perform any shared mutation.
unsafe impl Sync for UpdateSnapshotRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for UpdateSnapshotRequestMut<'msg> {
  type Proxied = UpdateSnapshotRequest;
  fn as_view(&self) -> ::protobuf::View<'_, UpdateSnapshotRequest> {
    UpdateSnapshotRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateSnapshotRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpdateSnapshotRequest>
  where
      'msg: 'shorter {
    UpdateSnapshotRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for UpdateSnapshotRequestMut<'msg> {
  type MutProxied = UpdateSnapshotRequest;
  fn as_mut(&mut self) -> UpdateSnapshotRequestMut<'msg> {
    UpdateSnapshotRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpdateSnapshotRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> UpdateSnapshotRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpdateSnapshotRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpdateSnapshotRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpdateSnapshotRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpdateSnapshotRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // snapshot: optional message google.pubsub.v1.Snapshot
  pub fn has_snapshot(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_snapshot(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn snapshot_opt(&self) -> ::protobuf::Optional<super::SnapshotView<'_>> {
        ::protobuf::Optional::new(self.snapshot(), self.has_snapshot())
  }
  pub fn snapshot(&self) -> super::SnapshotView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SnapshotView::default())
  }
  pub fn snapshot_mut(&mut self) -> super::SnapshotMut<'_> {
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
  pub fn set_snapshot(&mut self,
    val: impl ::protobuf::IntoProxied<super::Snapshot>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // update_mask: optional message google.protobuf.FieldMask
  pub fn has_update_mask(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_update_mask(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn update_mask_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::FieldMaskView<'_>> {
        ::protobuf::Optional::new(self.update_mask(), self.has_update_mask())
  }
  pub fn update_mask(&self) -> ::protobuf_well_known_types::FieldMaskView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::FieldMaskView::default())
  }
  pub fn update_mask_mut(&mut self) -> ::protobuf_well_known_types::FieldMaskMut<'_> {
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
  pub fn set_update_mask(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::FieldMask>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl UpdateSnapshotRequest

impl ::std::ops::Drop for UpdateSnapshotRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpdateSnapshotRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpdateSnapshotRequest {
  type Proxied = Self;
  fn as_view(&self) -> UpdateSnapshotRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpdateSnapshotRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpdateSnapshotRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpdateSnapshotRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__UpdateSnapshotRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__UpdateSnapshotRequest_msg_init.0, &[<super::Snapshot as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::FieldMask as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__UpdateSnapshotRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdateSnapshotRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdateSnapshotRequest {
  type Msg = UpdateSnapshotRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSnapshotRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateSnapshotRequest {
  type Msg = UpdateSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSnapshotRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdateSnapshotRequestMut<'_> {
  type Msg = UpdateSnapshotRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSnapshotRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateSnapshotRequestMut<'_> {
  type Msg = UpdateSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSnapshotRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateSnapshotRequestView<'_> {
  type Msg = UpdateSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpdateSnapshotRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdateSnapshotRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__Snapshot_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Snapshot {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Snapshot>
}

impl ::protobuf::Message for Snapshot {}

impl ::std::default::Default for Snapshot {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Snapshot {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Snapshot` is `Sync` because it does not implement interior mutability.
//    Neither does `SnapshotMut`.
unsafe impl Sync for Snapshot {}

// SAFETY:
// - `Snapshot` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Snapshot {}

impl ::protobuf::Proxied for Snapshot {
  type View<'msg> = SnapshotView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Snapshot {}

impl ::protobuf::MutProxied for Snapshot {
  type Mut<'msg> = SnapshotMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SnapshotView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Snapshot>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SnapshotView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SnapshotView<'msg> {
  type Message = Snapshot;
}

impl ::std::fmt::Debug for SnapshotView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SnapshotView<'_> {
  fn default() -> SnapshotView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Snapshot>> for SnapshotView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Snapshot>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SnapshotView<'msg> {

  pub fn to_owned(&self) -> Snapshot {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // topic: optional string
  pub fn topic(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // expire_time: optional message google.protobuf.Timestamp
  pub fn has_expire_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn expire_time_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.expire_time(), self.has_expire_time())
  }
  pub fn expire_time(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // labels: repeated message google.pubsub.v1.Snapshot.LabelsEntry
  pub fn labels(self)
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

}

// SAFETY:
// - `SnapshotView` is `Sync` because it does not support mutation.
unsafe impl Sync for SnapshotView<'_> {}

// SAFETY:
// - `SnapshotView` is `Send` because while its alive a `SnapshotMut` cannot.
// - `SnapshotView` does not use thread-local data.
unsafe impl Send for SnapshotView<'_> {}

impl<'msg> ::protobuf::AsView for SnapshotView<'msg> {
  type Proxied = Snapshot;
  fn as_view(&self) -> ::protobuf::View<'msg, Snapshot> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SnapshotView<'msg> {
  fn into_view<'shorter>(self) -> SnapshotView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Snapshot> for SnapshotView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Snapshot {
    let mut dst = Snapshot::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Snapshot> for SnapshotMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Snapshot {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Snapshot {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SnapshotView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SnapshotMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SnapshotMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Snapshot>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SnapshotMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SnapshotMut<'msg> {
  type Message = Snapshot;
}

impl ::std::fmt::Debug for SnapshotMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Snapshot>> for SnapshotMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Snapshot>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SnapshotMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Snapshot> {
    self.inner
  }

  pub fn to_owned(&self) -> Snapshot {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // expire_time: optional message google.protobuf.Timestamp
  pub fn has_expire_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_expire_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn expire_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.expire_time(), self.has_expire_time())
  }
  pub fn expire_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn expire_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_expire_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // labels: repeated message google.pubsub.v1.Snapshot.LabelsEntry
  pub fn labels(&self)
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
  pub fn labels_mut(&mut self)
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
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `SnapshotMut` does not perform any shared mutation.
unsafe impl Send for SnapshotMut<'_> {}

// SAFETY:
// - `SnapshotMut` does not perform any shared mutation.
unsafe impl Sync for SnapshotMut<'_> {}

impl<'msg> ::protobuf::AsView for SnapshotMut<'msg> {
  type Proxied = Snapshot;
  fn as_view(&self) -> ::protobuf::View<'_, Snapshot> {
    SnapshotView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SnapshotMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Snapshot>
  where
      'msg: 'shorter {
    SnapshotView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SnapshotMut<'msg> {
  type MutProxied = Snapshot;
  fn as_mut(&mut self) -> SnapshotMut<'msg> {
    SnapshotMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SnapshotMut<'msg> {
  fn into_mut<'shorter>(self) -> SnapshotMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Snapshot {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Snapshot> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SnapshotView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SnapshotMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // topic: optional string
  pub fn topic(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_topic(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // expire_time: optional message google.protobuf.Timestamp
  pub fn has_expire_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_expire_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn expire_time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.expire_time(), self.has_expire_time())
  }
  pub fn expire_time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn expire_time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_expire_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // labels: repeated message google.pubsub.v1.Snapshot.LabelsEntry
  pub fn labels(&self)
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
  pub fn labels_mut(&mut self)
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
  pub fn set_labels(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl Snapshot

impl ::std::ops::Drop for Snapshot {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Snapshot {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Snapshot {
  type Proxied = Self;
  fn as_view(&self) -> SnapshotView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Snapshot {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SnapshotMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Snapshot {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__Snapshot_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__Snapshot_msg_init.0, &[<::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::snapshot::LabelsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__Snapshot_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Snapshot {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Snapshot {
  type Msg = Snapshot;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snapshot> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Snapshot {
  type Msg = Snapshot;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snapshot> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SnapshotMut<'_> {
  type Msg = Snapshot;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snapshot> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SnapshotMut<'_> {
  type Msg = Snapshot;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snapshot> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SnapshotView<'_> {
  type Msg = Snapshot;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Snapshot> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SnapshotMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod snapshot {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__Snapshot__LabelsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct LabelsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LabelsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::snapshot::google__pubsub__v1__Snapshot__LabelsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::snapshot::google__pubsub__v1__Snapshot__LabelsEntry_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::snapshot::google__pubsub__v1__Snapshot__LabelsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod snapshot


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__GetSnapshotRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GetSnapshotRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GetSnapshotRequest>
}

impl ::protobuf::Message for GetSnapshotRequest {}

impl ::std::default::Default for GetSnapshotRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GetSnapshotRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GetSnapshotRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetSnapshotRequestMut`.
unsafe impl Sync for GetSnapshotRequest {}

// SAFETY:
// - `GetSnapshotRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetSnapshotRequest {}

impl ::protobuf::Proxied for GetSnapshotRequest {
  type View<'msg> = GetSnapshotRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GetSnapshotRequest {}

impl ::protobuf::MutProxied for GetSnapshotRequest {
  type Mut<'msg> = GetSnapshotRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetSnapshotRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetSnapshotRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetSnapshotRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GetSnapshotRequestView<'msg> {
  type Message = GetSnapshotRequest;
}

impl ::std::fmt::Debug for GetSnapshotRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GetSnapshotRequestView<'_> {
  fn default() -> GetSnapshotRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GetSnapshotRequest>> for GetSnapshotRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GetSnapshotRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetSnapshotRequestView<'msg> {

  pub fn to_owned(&self) -> GetSnapshotRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // snapshot: optional string
  pub fn snapshot(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `GetSnapshotRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetSnapshotRequestView<'_> {}

// SAFETY:
// - `GetSnapshotRequestView` is `Send` because while its alive a `GetSnapshotRequestMut` cannot.
// - `GetSnapshotRequestView` does not use thread-local data.
unsafe impl Send for GetSnapshotRequestView<'_> {}

impl<'msg> ::protobuf::AsView for GetSnapshotRequestView<'msg> {
  type Proxied = GetSnapshotRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, GetSnapshotRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetSnapshotRequestView<'msg> {
  fn into_view<'shorter>(self) -> GetSnapshotRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GetSnapshotRequest> for GetSnapshotRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetSnapshotRequest {
    let mut dst = GetSnapshotRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GetSnapshotRequest> for GetSnapshotRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GetSnapshotRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for GetSnapshotRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetSnapshotRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for GetSnapshotRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetSnapshotRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetSnapshotRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GetSnapshotRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GetSnapshotRequestMut<'msg> {
  type Message = GetSnapshotRequest;
}

impl ::std::fmt::Debug for GetSnapshotRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GetSnapshotRequest>> for GetSnapshotRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GetSnapshotRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GetSnapshotRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GetSnapshotRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> GetSnapshotRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // snapshot: optional string
  pub fn snapshot(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_snapshot(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `GetSnapshotRequestMut` does not perform any shared mutation.
unsafe impl Send for GetSnapshotRequestMut<'_> {}

// SAFETY:
// - `GetSnapshotRequestMut` does not perform any shared mutation.
unsafe impl Sync for GetSnapshotRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for GetSnapshotRequestMut<'msg> {
  type Proxied = GetSnapshotRequest;
  fn as_view(&self) -> ::protobuf::View<'_, GetSnapshotRequest> {
    GetSnapshotRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GetSnapshotRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GetSnapshotRequest>
  where
      'msg: 'shorter {
    GetSnapshotRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for GetSnapshotRequestMut<'msg> {
  type MutProxied = GetSnapshotRequest;
  fn as_mut(&mut self) -> GetSnapshotRequestMut<'msg> {
    GetSnapshotRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GetSnapshotRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> GetSnapshotRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GetSnapshotRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GetSnapshotRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GetSnapshotRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GetSnapshotRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // snapshot: optional string
  pub fn snapshot(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_snapshot(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl GetSnapshotRequest

impl ::std::ops::Drop for GetSnapshotRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GetSnapshotRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GetSnapshotRequest {
  type Proxied = Self;
  fn as_view(&self) -> GetSnapshotRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GetSnapshotRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GetSnapshotRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GetSnapshotRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__GetSnapshotRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__GetSnapshotRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__GetSnapshotRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetSnapshotRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetSnapshotRequest {
  type Msg = GetSnapshotRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSnapshotRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetSnapshotRequest {
  type Msg = GetSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSnapshotRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GetSnapshotRequestMut<'_> {
  type Msg = GetSnapshotRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSnapshotRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetSnapshotRequestMut<'_> {
  type Msg = GetSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSnapshotRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GetSnapshotRequestView<'_> {
  type Msg = GetSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GetSnapshotRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GetSnapshotRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListSnapshotsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListSnapshotsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListSnapshotsRequest>
}

impl ::protobuf::Message for ListSnapshotsRequest {}

impl ::std::default::Default for ListSnapshotsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListSnapshotsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListSnapshotsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ListSnapshotsRequestMut`.
unsafe impl Sync for ListSnapshotsRequest {}

// SAFETY:
// - `ListSnapshotsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListSnapshotsRequest {}

impl ::protobuf::Proxied for ListSnapshotsRequest {
  type View<'msg> = ListSnapshotsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListSnapshotsRequest {}

impl ::protobuf::MutProxied for ListSnapshotsRequest {
  type Mut<'msg> = ListSnapshotsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListSnapshotsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSnapshotsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSnapshotsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListSnapshotsRequestView<'msg> {
  type Message = ListSnapshotsRequest;
}

impl ::std::fmt::Debug for ListSnapshotsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListSnapshotsRequestView<'_> {
  fn default() -> ListSnapshotsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListSnapshotsRequest>> for ListSnapshotsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSnapshotsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSnapshotsRequestView<'msg> {

  pub fn to_owned(&self) -> ListSnapshotsRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // project: optional string
  pub fn project(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // page_size: optional int32
  pub fn page_size(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // page_token: optional string
  pub fn page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListSnapshotsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListSnapshotsRequestView<'_> {}

// SAFETY:
// - `ListSnapshotsRequestView` is `Send` because while its alive a `ListSnapshotsRequestMut` cannot.
// - `ListSnapshotsRequestView` does not use thread-local data.
unsafe impl Send for ListSnapshotsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ListSnapshotsRequestView<'msg> {
  type Proxied = ListSnapshotsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ListSnapshotsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSnapshotsRequestView<'msg> {
  fn into_view<'shorter>(self) -> ListSnapshotsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSnapshotsRequest> for ListSnapshotsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSnapshotsRequest {
    let mut dst = ListSnapshotsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSnapshotsRequest> for ListSnapshotsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSnapshotsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListSnapshotsRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSnapshotsRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSnapshotsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListSnapshotsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSnapshotsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSnapshotsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListSnapshotsRequestMut<'msg> {
  type Message = ListSnapshotsRequest;
}

impl ::std::fmt::Debug for ListSnapshotsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListSnapshotsRequest>> for ListSnapshotsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSnapshotsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSnapshotsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSnapshotsRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> ListSnapshotsRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // project: optional string
  pub fn project(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_project(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `ListSnapshotsRequestMut` does not perform any shared mutation.
unsafe impl Send for ListSnapshotsRequestMut<'_> {}

// SAFETY:
// - `ListSnapshotsRequestMut` does not perform any shared mutation.
unsafe impl Sync for ListSnapshotsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ListSnapshotsRequestMut<'msg> {
  type Proxied = ListSnapshotsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ListSnapshotsRequest> {
    ListSnapshotsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSnapshotsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListSnapshotsRequest>
  where
      'msg: 'shorter {
    ListSnapshotsRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListSnapshotsRequestMut<'msg> {
  type MutProxied = ListSnapshotsRequest;
  fn as_mut(&mut self) -> ListSnapshotsRequestMut<'msg> {
    ListSnapshotsRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListSnapshotsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ListSnapshotsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListSnapshotsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListSnapshotsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListSnapshotsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListSnapshotsRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // project: optional string
  pub fn project(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_project(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // page_size: optional int32
  pub fn page_size(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_page_size(&mut self, val: i32) {
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

  // page_token: optional string
  pub fn page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl ListSnapshotsRequest

impl ::std::ops::Drop for ListSnapshotsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListSnapshotsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListSnapshotsRequest {
  type Proxied = Self;
  fn as_view(&self) -> ListSnapshotsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListSnapshotsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListSnapshotsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListSnapshotsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListSnapshotsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X(P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListSnapshotsRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListSnapshotsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSnapshotsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSnapshotsRequest {
  type Msg = ListSnapshotsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSnapshotsRequest {
  type Msg = ListSnapshotsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSnapshotsRequestMut<'_> {
  type Msg = ListSnapshotsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSnapshotsRequestMut<'_> {
  type Msg = ListSnapshotsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSnapshotsRequestView<'_> {
  type Msg = ListSnapshotsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSnapshotsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__ListSnapshotsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ListSnapshotsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ListSnapshotsResponse>
}

impl ::protobuf::Message for ListSnapshotsResponse {}

impl ::std::default::Default for ListSnapshotsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ListSnapshotsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ListSnapshotsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ListSnapshotsResponseMut`.
unsafe impl Sync for ListSnapshotsResponse {}

// SAFETY:
// - `ListSnapshotsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListSnapshotsResponse {}

impl ::protobuf::Proxied for ListSnapshotsResponse {
  type View<'msg> = ListSnapshotsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ListSnapshotsResponse {}

impl ::protobuf::MutProxied for ListSnapshotsResponse {
  type Mut<'msg> = ListSnapshotsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListSnapshotsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSnapshotsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSnapshotsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ListSnapshotsResponseView<'msg> {
  type Message = ListSnapshotsResponse;
}

impl ::std::fmt::Debug for ListSnapshotsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ListSnapshotsResponseView<'_> {
  fn default() -> ListSnapshotsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ListSnapshotsResponse>> for ListSnapshotsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ListSnapshotsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSnapshotsResponseView<'msg> {

  pub fn to_owned(&self) -> ListSnapshotsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // snapshots: repeated message google.pubsub.v1.Snapshot
  pub fn snapshots(self) -> ::protobuf::RepeatedView<'msg, super::Snapshot> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Snapshot>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // next_page_token: optional string
  pub fn next_page_token(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `ListSnapshotsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListSnapshotsResponseView<'_> {}

// SAFETY:
// - `ListSnapshotsResponseView` is `Send` because while its alive a `ListSnapshotsResponseMut` cannot.
// - `ListSnapshotsResponseView` does not use thread-local data.
unsafe impl Send for ListSnapshotsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ListSnapshotsResponseView<'msg> {
  type Proxied = ListSnapshotsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ListSnapshotsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSnapshotsResponseView<'msg> {
  fn into_view<'shorter>(self) -> ListSnapshotsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSnapshotsResponse> for ListSnapshotsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSnapshotsResponse {
    let mut dst = ListSnapshotsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ListSnapshotsResponse> for ListSnapshotsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ListSnapshotsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for ListSnapshotsResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSnapshotsResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ListSnapshotsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListSnapshotsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSnapshotsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ListSnapshotsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ListSnapshotsResponseMut<'msg> {
  type Message = ListSnapshotsResponse;
}

impl ::std::fmt::Debug for ListSnapshotsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ListSnapshotsResponse>> for ListSnapshotsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSnapshotsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ListSnapshotsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ListSnapshotsResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> ListSnapshotsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // snapshots: repeated message google.pubsub.v1.Snapshot
  pub fn snapshots(&self) -> ::protobuf::RepeatedView<'_, super::Snapshot> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Snapshot>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn snapshots_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Snapshot> {
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
  pub fn set_snapshots(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Snapshot>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ListSnapshotsResponseMut` does not perform any shared mutation.
unsafe impl Send for ListSnapshotsResponseMut<'_> {}

// SAFETY:
// - `ListSnapshotsResponseMut` does not perform any shared mutation.
unsafe impl Sync for ListSnapshotsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ListSnapshotsResponseMut<'msg> {
  type Proxied = ListSnapshotsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ListSnapshotsResponse> {
    ListSnapshotsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ListSnapshotsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ListSnapshotsResponse>
  where
      'msg: 'shorter {
    ListSnapshotsResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for ListSnapshotsResponseMut<'msg> {
  type MutProxied = ListSnapshotsResponse;
  fn as_mut(&mut self) -> ListSnapshotsResponseMut<'msg> {
    ListSnapshotsResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ListSnapshotsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ListSnapshotsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ListSnapshotsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ListSnapshotsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ListSnapshotsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ListSnapshotsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // snapshots: repeated message google.pubsub.v1.Snapshot
  pub fn snapshots(&self) -> ::protobuf::RepeatedView<'_, super::Snapshot> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Snapshot>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn snapshots_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Snapshot> {
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
  pub fn set_snapshots(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Snapshot>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // next_page_token: optional string
  pub fn next_page_token(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_next_page_token(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl ListSnapshotsResponse

impl ::std::ops::Drop for ListSnapshotsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ListSnapshotsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ListSnapshotsResponse {
  type Proxied = Self;
  fn as_view(&self) -> ListSnapshotsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ListSnapshotsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ListSnapshotsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ListSnapshotsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__ListSnapshotsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__ListSnapshotsResponse_msg_init.0, &[<super::Snapshot as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__ListSnapshotsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSnapshotsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSnapshotsResponse {
  type Msg = ListSnapshotsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSnapshotsResponse {
  type Msg = ListSnapshotsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ListSnapshotsResponseMut<'_> {
  type Msg = ListSnapshotsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSnapshotsResponseMut<'_> {
  type Msg = ListSnapshotsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ListSnapshotsResponseView<'_> {
  type Msg = ListSnapshotsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ListSnapshotsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ListSnapshotsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__DeleteSnapshotRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeleteSnapshotRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeleteSnapshotRequest>
}

impl ::protobuf::Message for DeleteSnapshotRequest {}

impl ::std::default::Default for DeleteSnapshotRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeleteSnapshotRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeleteSnapshotRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `DeleteSnapshotRequestMut`.
unsafe impl Sync for DeleteSnapshotRequest {}

// SAFETY:
// - `DeleteSnapshotRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for DeleteSnapshotRequest {}

impl ::protobuf::Proxied for DeleteSnapshotRequest {
  type View<'msg> = DeleteSnapshotRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeleteSnapshotRequest {}

impl ::protobuf::MutProxied for DeleteSnapshotRequest {
  type Mut<'msg> = DeleteSnapshotRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeleteSnapshotRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteSnapshotRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeleteSnapshotRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeleteSnapshotRequestView<'msg> {
  type Message = DeleteSnapshotRequest;
}

impl ::std::fmt::Debug for DeleteSnapshotRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeleteSnapshotRequestView<'_> {
  fn default() -> DeleteSnapshotRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteSnapshotRequest>> for DeleteSnapshotRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeleteSnapshotRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeleteSnapshotRequestView<'msg> {

  pub fn to_owned(&self) -> DeleteSnapshotRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // snapshot: optional string
  pub fn snapshot(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `DeleteSnapshotRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for DeleteSnapshotRequestView<'_> {}

// SAFETY:
// - `DeleteSnapshotRequestView` is `Send` because while its alive a `DeleteSnapshotRequestMut` cannot.
// - `DeleteSnapshotRequestView` does not use thread-local data.
unsafe impl Send for DeleteSnapshotRequestView<'_> {}

impl<'msg> ::protobuf::AsView for DeleteSnapshotRequestView<'msg> {
  type Proxied = DeleteSnapshotRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, DeleteSnapshotRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeleteSnapshotRequestView<'msg> {
  fn into_view<'shorter>(self) -> DeleteSnapshotRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeleteSnapshotRequest> for DeleteSnapshotRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeleteSnapshotRequest {
    let mut dst = DeleteSnapshotRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeleteSnapshotRequest> for DeleteSnapshotRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeleteSnapshotRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for DeleteSnapshotRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeleteSnapshotRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DeleteSnapshotRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeleteSnapshotRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSnapshotRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeleteSnapshotRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeleteSnapshotRequestMut<'msg> {
  type Message = DeleteSnapshotRequest;
}

impl ::std::fmt::Debug for DeleteSnapshotRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSnapshotRequest>> for DeleteSnapshotRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSnapshotRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeleteSnapshotRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeleteSnapshotRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> DeleteSnapshotRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // snapshot: optional string
  pub fn snapshot(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_snapshot(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `DeleteSnapshotRequestMut` does not perform any shared mutation.
unsafe impl Send for DeleteSnapshotRequestMut<'_> {}

// SAFETY:
// - `DeleteSnapshotRequestMut` does not perform any shared mutation.
unsafe impl Sync for DeleteSnapshotRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for DeleteSnapshotRequestMut<'msg> {
  type Proxied = DeleteSnapshotRequest;
  fn as_view(&self) -> ::protobuf::View<'_, DeleteSnapshotRequest> {
    DeleteSnapshotRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeleteSnapshotRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeleteSnapshotRequest>
  where
      'msg: 'shorter {
    DeleteSnapshotRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for DeleteSnapshotRequestMut<'msg> {
  type MutProxied = DeleteSnapshotRequest;
  fn as_mut(&mut self) -> DeleteSnapshotRequestMut<'msg> {
    DeleteSnapshotRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeleteSnapshotRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> DeleteSnapshotRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeleteSnapshotRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeleteSnapshotRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeleteSnapshotRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeleteSnapshotRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // snapshot: optional string
  pub fn snapshot(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_snapshot(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl DeleteSnapshotRequest

impl ::std::ops::Drop for DeleteSnapshotRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeleteSnapshotRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeleteSnapshotRequest {
  type Proxied = Self;
  fn as_view(&self) -> DeleteSnapshotRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeleteSnapshotRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeleteSnapshotRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeleteSnapshotRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__DeleteSnapshotRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__DeleteSnapshotRequest_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__DeleteSnapshotRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeleteSnapshotRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeleteSnapshotRequest {
  type Msg = DeleteSnapshotRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSnapshotRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteSnapshotRequest {
  type Msg = DeleteSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSnapshotRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeleteSnapshotRequestMut<'_> {
  type Msg = DeleteSnapshotRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSnapshotRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteSnapshotRequestMut<'_> {
  type Msg = DeleteSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSnapshotRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeleteSnapshotRequestView<'_> {
  type Msg = DeleteSnapshotRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeleteSnapshotRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeleteSnapshotRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__SeekRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SeekRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SeekRequest>
}

impl ::protobuf::Message for SeekRequest {}

impl ::std::default::Default for SeekRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SeekRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SeekRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `SeekRequestMut`.
unsafe impl Sync for SeekRequest {}

// SAFETY:
// - `SeekRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SeekRequest {}

impl ::protobuf::Proxied for SeekRequest {
  type View<'msg> = SeekRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SeekRequest {}

impl ::protobuf::MutProxied for SeekRequest {
  type Mut<'msg> = SeekRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SeekRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeekRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeekRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SeekRequestView<'msg> {
  type Message = SeekRequest;
}

impl ::std::fmt::Debug for SeekRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SeekRequestView<'_> {
  fn default() -> SeekRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SeekRequest>> for SeekRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeekRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeekRequestView<'msg> {

  pub fn to_owned(&self) -> SeekRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // subscription: optional string
  pub fn subscription(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // time: optional message google.protobuf.Timestamp
  pub fn has_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn time_opt(self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'msg>> {
        ::protobuf::Optional::new(self.time(), self.has_time())
  }
  pub fn time(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // snapshot: optional string
  pub fn has_snapshot(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn snapshot_opt(self) -> ::protobuf::Optional<&'msg ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.snapshot(), self.has_snapshot())
  }
  pub fn snapshot(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  pub fn target(self) -> super::seek_request::TargetOneof<'msg> {
    match self.target_case() {
      super::seek_request::TargetCase::Time =>
          super::seek_request::TargetOneof::Time(self.time()),
      super::seek_request::TargetCase::Snapshot =>
          super::seek_request::TargetOneof::Snapshot(self.snapshot()),
      _ => super::seek_request::TargetOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn target_case(self) -> super::seek_request::TargetCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::seek_request::TargetCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SeekRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for SeekRequestView<'_> {}

// SAFETY:
// - `SeekRequestView` is `Send` because while its alive a `SeekRequestMut` cannot.
// - `SeekRequestView` does not use thread-local data.
unsafe impl Send for SeekRequestView<'_> {}

impl<'msg> ::protobuf::AsView for SeekRequestView<'msg> {
  type Proxied = SeekRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, SeekRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeekRequestView<'msg> {
  fn into_view<'shorter>(self) -> SeekRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SeekRequest> for SeekRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeekRequest {
    let mut dst = SeekRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SeekRequest> for SeekRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeekRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SeekRequest {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SeekRequestView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SeekRequestMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SeekRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeekRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeekRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SeekRequestMut<'msg> {
  type Message = SeekRequest;
}

impl ::std::fmt::Debug for SeekRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SeekRequest>> for SeekRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeekRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeekRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SeekRequest> {
    self.inner
  }

  pub fn to_owned(&self) -> SeekRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // time: optional message google.protobuf.Timestamp
  pub fn has_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.time(), self.has_time())
  }
  pub fn time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // snapshot: optional string
  pub fn has_snapshot(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_snapshot(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn snapshot_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.snapshot(), self.has_snapshot())
  }
  pub fn snapshot(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_snapshot(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  pub fn target(&self) -> super::seek_request::TargetOneof<'_> {
    match &self.target_case() {
      super::seek_request::TargetCase::Time =>
          super::seek_request::TargetOneof::Time(self.time()),
      super::seek_request::TargetCase::Snapshot =>
          super::seek_request::TargetOneof::Snapshot(self.snapshot()),
      _ => super::seek_request::TargetOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn target_case(&self) -> super::seek_request::TargetCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::seek_request::TargetCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SeekRequestMut` does not perform any shared mutation.
unsafe impl Send for SeekRequestMut<'_> {}

// SAFETY:
// - `SeekRequestMut` does not perform any shared mutation.
unsafe impl Sync for SeekRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for SeekRequestMut<'msg> {
  type Proxied = SeekRequest;
  fn as_view(&self) -> ::protobuf::View<'_, SeekRequest> {
    SeekRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeekRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SeekRequest>
  where
      'msg: 'shorter {
    SeekRequestView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SeekRequestMut<'msg> {
  type MutProxied = SeekRequest;
  fn as_mut(&mut self) -> SeekRequestMut<'msg> {
    SeekRequestMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SeekRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> SeekRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SeekRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SeekRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SeekRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SeekRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // subscription: optional string
  pub fn subscription(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_subscription(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // time: optional message google.protobuf.Timestamp
  pub fn has_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn time_opt(&self) -> ::protobuf::Optional<::protobuf_well_known_types::TimestampView<'_>> {
        ::protobuf::Optional::new(self.time(), self.has_time())
  }
  pub fn time(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn time_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // snapshot: optional string
  pub fn has_snapshot(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_snapshot(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn snapshot_opt(&self) -> ::protobuf::Optional<&'_ ::protobuf::ProtoStr> {
        ::protobuf::Optional::new(self.snapshot(), self.has_snapshot())
  }
  pub fn snapshot(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_snapshot(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  pub fn target(&self) -> super::seek_request::TargetOneof<'_> {
    match &self.target_case() {
      super::seek_request::TargetCase::Time =>
          super::seek_request::TargetOneof::Time(self.time()),
      super::seek_request::TargetCase::Snapshot =>
          super::seek_request::TargetOneof::Snapshot(self.snapshot()),
      _ => super::seek_request::TargetOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn target_case(&self) -> super::seek_request::TargetCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::seek_request::TargetCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl SeekRequest

impl ::std::ops::Drop for SeekRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SeekRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SeekRequest {
  type Proxied = Self;
  fn as_view(&self) -> SeekRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SeekRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SeekRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SeekRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__SeekRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X31T^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__SeekRequest_msg_init.0, &[<::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__SeekRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeekRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeekRequest {
  type Msg = SeekRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeekRequest {
  type Msg = SeekRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeekRequestMut<'_> {
  type Msg = SeekRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeekRequestMut<'_> {
  type Msg = SeekRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeekRequestView<'_> {
  type Msg = SeekRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeekRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod seek_request {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TargetOneof<'msg> {
  Time(::protobuf::View<'msg, ::protobuf_well_known_types::Timestamp>) = 2,
  Snapshot(&'msg ::protobuf::ProtoStr) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TargetCase {
  Time = 2,
  Snapshot = 3,

  not_set = 0
}

impl TargetCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TargetCase> {
    match v {
      0 => Some(TargetCase::not_set),
      2 => Some(TargetCase::Time),
      3 => Some(TargetCase::Snapshot),
      _ => None
    }
  }
}
}  // pub mod seek_request


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut google__pubsub__v1__SeekResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SeekResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SeekResponse>
}

impl ::protobuf::Message for SeekResponse {}

impl ::std::default::Default for SeekResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SeekResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SeekResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SeekResponseMut`.
unsafe impl Sync for SeekResponse {}

// SAFETY:
// - `SeekResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SeekResponse {}

impl ::protobuf::Proxied for SeekResponse {
  type View<'msg> = SeekResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SeekResponse {}

impl ::protobuf::MutProxied for SeekResponse {
  type Mut<'msg> = SeekResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SeekResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeekResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeekResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SeekResponseView<'msg> {
  type Message = SeekResponse;
}

impl ::std::fmt::Debug for SeekResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SeekResponseView<'_> {
  fn default() -> SeekResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SeekResponse>> for SeekResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeekResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeekResponseView<'msg> {

  pub fn to_owned(&self) -> SeekResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `SeekResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SeekResponseView<'_> {}

// SAFETY:
// - `SeekResponseView` is `Send` because while its alive a `SeekResponseMut` cannot.
// - `SeekResponseView` does not use thread-local data.
unsafe impl Send for SeekResponseView<'_> {}

impl<'msg> ::protobuf::AsView for SeekResponseView<'msg> {
  type Proxied = SeekResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, SeekResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeekResponseView<'msg> {
  fn into_view<'shorter>(self) -> SeekResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SeekResponse> for SeekResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeekResponse {
    let mut dst = SeekResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SeekResponse> for SeekResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeekResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for SeekResponse {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SeekResponseView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for SeekResponseMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SeekResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeekResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeekResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SeekResponseMut<'msg> {
  type Message = SeekResponse;
}

impl ::std::fmt::Debug for SeekResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SeekResponse>> for SeekResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeekResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeekResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SeekResponse> {
    self.inner
  }

  pub fn to_owned(&self) -> SeekResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `SeekResponseMut` does not perform any shared mutation.
unsafe impl Send for SeekResponseMut<'_> {}

// SAFETY:
// - `SeekResponseMut` does not perform any shared mutation.
unsafe impl Sync for SeekResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for SeekResponseMut<'msg> {
  type Proxied = SeekResponse;
  fn as_view(&self) -> ::protobuf::View<'_, SeekResponse> {
    SeekResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeekResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SeekResponse>
  where
      'msg: 'shorter {
    SeekResponseView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner)
    }
  }
}

impl<'msg> ::protobuf::AsMut for SeekResponseMut<'msg> {
  type MutProxied = SeekResponse;
  fn as_mut(&mut self) -> SeekResponseMut<'msg> {
    SeekResponseMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SeekResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> SeekResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SeekResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SeekResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SeekResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SeekResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl SeekResponse

impl ::std::ops::Drop for SeekResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SeekResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SeekResponse {
  type Proxied = Self;
  fn as_view(&self) -> SeekResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SeekResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SeekResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SeekResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::google__pubsub__v1__SeekResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::google__pubsub__v1__SeekResponse_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::google__pubsub__v1__SeekResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeekResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeekResponse {
  type Msg = SeekResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeekResponse {
  type Msg = SeekResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeekResponseMut<'_> {
  type Msg = SeekResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeekResponseMut<'_> {
  type Msg = SeekResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeekResponseView<'_> {
  type Msg = SeekResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeekResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeekResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



