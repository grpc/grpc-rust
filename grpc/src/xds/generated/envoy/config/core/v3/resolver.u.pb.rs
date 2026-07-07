const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__DnsResolverOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DnsResolverOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DnsResolverOptions>
}

impl ::protobuf::Message for DnsResolverOptions {
  type MessageView<'msg> = DnsResolverOptionsView<'msg>;
  type MessageMut<'msg> = DnsResolverOptionsMut<'msg>;
}

impl ::std::default::Default for DnsResolverOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DnsResolverOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DnsResolverOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `DnsResolverOptionsMut`.
unsafe impl ::std::marker::Sync for DnsResolverOptions {}

// SAFETY:
// - `DnsResolverOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DnsResolverOptions {}

impl ::protobuf::Proxied for DnsResolverOptions {
  type View<'msg> = DnsResolverOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DnsResolverOptions {}

impl ::protobuf::MutProxied for DnsResolverOptions {
  type Mut<'msg> = DnsResolverOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DnsResolverOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DnsResolverOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DnsResolverOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DnsResolverOptionsView<'msg> {
  type Message = DnsResolverOptions;
}

impl ::std::fmt::Debug for DnsResolverOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DnsResolverOptionsView<'_> {
  fn default() -> DnsResolverOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DnsResolverOptions>> for DnsResolverOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DnsResolverOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DnsResolverOptionsView<'msg> {

  pub fn to_owned(&self) -> DnsResolverOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // use_tcp_for_dns_lookups: optional bool
  pub fn use_tcp_for_dns_lookups(self) -> bool {
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

  // no_default_search_domain: optional bool
  pub fn no_default_search_domain(self) -> bool {
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
// - `DnsResolverOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DnsResolverOptionsView<'_> {}

// SAFETY:
// - `DnsResolverOptionsView` is `Send` because while its alive a `DnsResolverOptionsMut` cannot.
// - `DnsResolverOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for DnsResolverOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for DnsResolverOptionsView<'msg> {
  type Proxied = DnsResolverOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, DnsResolverOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DnsResolverOptionsView<'msg> {
  fn into_view<'shorter>(self) -> DnsResolverOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DnsResolverOptions> for DnsResolverOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DnsResolverOptions {
    let mut dst = DnsResolverOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DnsResolverOptions> for DnsResolverOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DnsResolverOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DnsResolverOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DnsResolverOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DnsResolverOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DnsResolverOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DnsResolverOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DnsResolverOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DnsResolverOptionsMut<'msg> {
  type Message = DnsResolverOptions;
}

impl ::std::fmt::Debug for DnsResolverOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DnsResolverOptions>> for DnsResolverOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DnsResolverOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DnsResolverOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DnsResolverOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DnsResolverOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // use_tcp_for_dns_lookups: optional bool
  pub fn use_tcp_for_dns_lookups(&self) -> bool {
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
  pub fn set_use_tcp_for_dns_lookups(&mut self, val: bool) {
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

  // no_default_search_domain: optional bool
  pub fn no_default_search_domain(&self) -> bool {
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
  pub fn set_no_default_search_domain(&mut self, val: bool) {
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
// - `DnsResolverOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DnsResolverOptionsMut<'_> {}

// SAFETY:
// - `DnsResolverOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DnsResolverOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for DnsResolverOptionsMut<'msg> {
  type Proxied = DnsResolverOptions;
  fn as_view(&self) -> ::protobuf::View<'_, DnsResolverOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DnsResolverOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DnsResolverOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DnsResolverOptionsMut<'msg> {
  type MutProxied = DnsResolverOptions;
  fn as_mut(&mut self) -> DnsResolverOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DnsResolverOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> DnsResolverOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DnsResolverOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DnsResolverOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DnsResolverOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DnsResolverOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // use_tcp_for_dns_lookups: optional bool
  pub fn use_tcp_for_dns_lookups(&self) -> bool {
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
  pub fn set_use_tcp_for_dns_lookups(&mut self, val: bool) {
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

  // no_default_search_domain: optional bool
  pub fn no_default_search_domain(&self) -> bool {
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
  pub fn set_no_default_search_domain(&mut self, val: bool) {
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

}  // impl DnsResolverOptions

impl ::std::ops::Drop for DnsResolverOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DnsResolverOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DnsResolverOptions {
  type Proxied = Self;
  fn as_view(&self) -> DnsResolverOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DnsResolverOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DnsResolverOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DnsResolverOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__DnsResolverOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__DnsResolverOptions_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__DnsResolverOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DnsResolverOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DnsResolverOptions {
  type Msg = DnsResolverOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolverOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DnsResolverOptions {
  type Msg = DnsResolverOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolverOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DnsResolverOptionsMut<'_> {
  type Msg = DnsResolverOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolverOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DnsResolverOptionsMut<'_> {
  type Msg = DnsResolverOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolverOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DnsResolverOptionsView<'_> {
  type Msg = DnsResolverOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolverOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DnsResolverOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__DnsResolutionConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DnsResolutionConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DnsResolutionConfig>
}

impl ::protobuf::Message for DnsResolutionConfig {
  type MessageView<'msg> = DnsResolutionConfigView<'msg>;
  type MessageMut<'msg> = DnsResolutionConfigMut<'msg>;
}

impl ::std::default::Default for DnsResolutionConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DnsResolutionConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DnsResolutionConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `DnsResolutionConfigMut`.
unsafe impl ::std::marker::Sync for DnsResolutionConfig {}

// SAFETY:
// - `DnsResolutionConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DnsResolutionConfig {}

impl ::protobuf::Proxied for DnsResolutionConfig {
  type View<'msg> = DnsResolutionConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DnsResolutionConfig {}

impl ::protobuf::MutProxied for DnsResolutionConfig {
  type Mut<'msg> = DnsResolutionConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DnsResolutionConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DnsResolutionConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DnsResolutionConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DnsResolutionConfigView<'msg> {
  type Message = DnsResolutionConfig;
}

impl ::std::fmt::Debug for DnsResolutionConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DnsResolutionConfigView<'_> {
  fn default() -> DnsResolutionConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DnsResolutionConfig>> for DnsResolutionConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DnsResolutionConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DnsResolutionConfigView<'msg> {

  pub fn to_owned(&self) -> DnsResolutionConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // resolvers: repeated message envoy.config.core.v3.Address
  pub fn resolvers(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::Address>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // dns_resolver_options: optional message envoy.config.core.v3.DnsResolverOptions
  pub fn has_dns_resolver_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn dns_resolver_options_opt(self) -> ::std::option::Option<super::DnsResolverOptionsView<'msg>> {
    self.has_dns_resolver_options().then(|| self.dns_resolver_options())
  }
  pub fn dns_resolver_options(self) -> super::DnsResolverOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DnsResolverOptionsView::default())
  }

}

// SAFETY:
// - `DnsResolutionConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DnsResolutionConfigView<'_> {}

// SAFETY:
// - `DnsResolutionConfigView` is `Send` because while its alive a `DnsResolutionConfigMut` cannot.
// - `DnsResolutionConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for DnsResolutionConfigView<'_> {}

impl<'msg> ::protobuf::AsView for DnsResolutionConfigView<'msg> {
  type Proxied = DnsResolutionConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, DnsResolutionConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DnsResolutionConfigView<'msg> {
  fn into_view<'shorter>(self) -> DnsResolutionConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DnsResolutionConfig> for DnsResolutionConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DnsResolutionConfig {
    let mut dst = DnsResolutionConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DnsResolutionConfig> for DnsResolutionConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DnsResolutionConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DnsResolutionConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DnsResolutionConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DnsResolutionConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DnsResolutionConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DnsResolutionConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DnsResolutionConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DnsResolutionConfigMut<'msg> {
  type Message = DnsResolutionConfig;
}

impl ::std::fmt::Debug for DnsResolutionConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DnsResolutionConfig>> for DnsResolutionConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DnsResolutionConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DnsResolutionConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DnsResolutionConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DnsResolutionConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // resolvers: repeated message envoy.config.core.v3.Address
  pub fn resolvers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::Address>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resolvers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
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
  pub fn set_resolvers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::Address>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dns_resolver_options: optional message envoy.config.core.v3.DnsResolverOptions
  pub fn has_dns_resolver_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dns_resolver_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dns_resolver_options_opt(&self) -> ::std::option::Option<super::DnsResolverOptionsView<'_>> {
    self.has_dns_resolver_options().then(|| self.dns_resolver_options())
  }
  pub fn dns_resolver_options(&self) -> super::DnsResolverOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DnsResolverOptionsView::default())
  }
  pub fn dns_resolver_options_mut(&mut self) -> super::DnsResolverOptionsMut<'_> {
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
  pub fn set_dns_resolver_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::DnsResolverOptions>) {

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
// - `DnsResolutionConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DnsResolutionConfigMut<'_> {}

// SAFETY:
// - `DnsResolutionConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DnsResolutionConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for DnsResolutionConfigMut<'msg> {
  type Proxied = DnsResolutionConfig;
  fn as_view(&self) -> ::protobuf::View<'_, DnsResolutionConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DnsResolutionConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DnsResolutionConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DnsResolutionConfigMut<'msg> {
  type MutProxied = DnsResolutionConfig;
  fn as_mut(&mut self) -> DnsResolutionConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DnsResolutionConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> DnsResolutionConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DnsResolutionConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DnsResolutionConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DnsResolutionConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DnsResolutionConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // resolvers: repeated message envoy.config.core.v3.Address
  pub fn resolvers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::Address>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn resolvers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
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
  pub fn set_resolvers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::Address>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // dns_resolver_options: optional message envoy.config.core.v3.DnsResolverOptions
  pub fn has_dns_resolver_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_dns_resolver_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn dns_resolver_options_opt(&self) -> ::std::option::Option<super::DnsResolverOptionsView<'_>> {
    self.has_dns_resolver_options().then(|| self.dns_resolver_options())
  }
  pub fn dns_resolver_options(&self) -> super::DnsResolverOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DnsResolverOptionsView::default())
  }
  pub fn dns_resolver_options_mut(&mut self) -> super::DnsResolverOptionsMut<'_> {
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
  pub fn set_dns_resolver_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::DnsResolverOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl DnsResolutionConfig

impl ::std::ops::Drop for DnsResolutionConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DnsResolutionConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DnsResolutionConfig {
  type Proxied = Self;
  fn as_view(&self) -> DnsResolutionConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DnsResolutionConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DnsResolutionConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DnsResolutionConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__DnsResolutionConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__DnsResolutionConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::DnsResolverOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__DnsResolutionConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DnsResolutionConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DnsResolutionConfig {
  type Msg = DnsResolutionConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolutionConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DnsResolutionConfig {
  type Msg = DnsResolutionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolutionConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DnsResolutionConfigMut<'_> {
  type Msg = DnsResolutionConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolutionConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DnsResolutionConfigMut<'_> {
  type Msg = DnsResolutionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolutionConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DnsResolutionConfigView<'_> {
  type Msg = DnsResolutionConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DnsResolutionConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DnsResolutionConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



