const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__GenericSecret_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GenericSecret {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GenericSecret>
}

impl ::protobuf::Message for GenericSecret {
  type MessageView<'msg> = GenericSecretView<'msg>;
  type MessageMut<'msg> = GenericSecretMut<'msg>;
}

impl ::std::default::Default for GenericSecret {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GenericSecret {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GenericSecret` is `Sync` because it does not implement interior mutability.
//    Neither does `GenericSecretMut`.
unsafe impl ::std::marker::Sync for GenericSecret {}

// SAFETY:
// - `GenericSecret` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GenericSecret {}

impl ::protobuf::Proxied for GenericSecret {
  type View<'msg> = GenericSecretView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GenericSecret {}

impl ::protobuf::MutProxied for GenericSecret {
  type Mut<'msg> = GenericSecretMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GenericSecretView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GenericSecret>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GenericSecretView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GenericSecretView<'msg> {
  type Message = GenericSecret;
}

impl ::std::fmt::Debug for GenericSecretView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GenericSecretView<'_> {
  fn default() -> GenericSecretView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GenericSecret>> for GenericSecretView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GenericSecret>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GenericSecretView<'msg> {

  pub fn to_owned(&self) -> GenericSecret {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // secret: optional message envoy.config.core.v3.DataSource
  pub fn has_secret(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn secret_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_secret().then(|| self.secret())
  }
  pub fn secret(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // secrets: repeated message envoy.extensions.transport_sockets.tls.v3.GenericSecret.SecretsEntry
  pub fn secrets(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `GenericSecretView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GenericSecretView<'_> {}

// SAFETY:
// - `GenericSecretView` is `Send` because while its alive a `GenericSecretMut` cannot.
// - `GenericSecretView` does not use thread-local data.
unsafe impl ::std::marker::Send for GenericSecretView<'_> {}

impl<'msg> ::protobuf::AsView for GenericSecretView<'msg> {
  type Proxied = GenericSecret;
  fn as_view(&self) -> ::protobuf::View<'msg, GenericSecret> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GenericSecretView<'msg> {
  fn into_view<'shorter>(self) -> GenericSecretView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GenericSecret> for GenericSecretView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GenericSecret {
    let mut dst = GenericSecret::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GenericSecret> for GenericSecretMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GenericSecret {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GenericSecret {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GenericSecretView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GenericSecretMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GenericSecretMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GenericSecret>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GenericSecretMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GenericSecretMut<'msg> {
  type Message = GenericSecret;
}

impl ::std::fmt::Debug for GenericSecretMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GenericSecret>> for GenericSecretMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GenericSecret>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GenericSecretMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GenericSecret> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GenericSecret {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // secret: optional message envoy.config.core.v3.DataSource
  pub fn has_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn secret_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_secret().then(|| self.secret())
  }
  pub fn secret(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn secret_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_secret(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // secrets: repeated message envoy.extensions.transport_sockets.tls.v3.GenericSecret.SecretsEntry
  pub fn secrets(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn secrets_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_secrets(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `GenericSecretMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GenericSecretMut<'_> {}

// SAFETY:
// - `GenericSecretMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GenericSecretMut<'_> {}

impl<'msg> ::protobuf::AsView for GenericSecretMut<'msg> {
  type Proxied = GenericSecret;
  fn as_view(&self) -> ::protobuf::View<'_, GenericSecret> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GenericSecretMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GenericSecret>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GenericSecretMut<'msg> {
  type MutProxied = GenericSecret;
  fn as_mut(&mut self) -> GenericSecretMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GenericSecretMut<'msg> {
  fn into_mut<'shorter>(self) -> GenericSecretMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GenericSecret {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GenericSecret> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GenericSecretView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GenericSecretMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // secret: optional message envoy.config.core.v3.DataSource
  pub fn has_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn secret_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_secret().then(|| self.secret())
  }
  pub fn secret(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn secret_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_secret(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // secrets: repeated message envoy.extensions.transport_sockets.tls.v3.GenericSecret.SecretsEntry
  pub fn secrets(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn secrets_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_secrets(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::base::DataSource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl GenericSecret

impl ::std::ops::Drop for GenericSecret {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GenericSecret {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GenericSecret {
  type Proxied = Self;
  fn as_view(&self) -> GenericSecretView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GenericSecret {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GenericSecretMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GenericSecret {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__GenericSecret_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__GenericSecret_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::generic_secret::SecretsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__GenericSecret_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GenericSecret {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GenericSecret {
  type Msg = GenericSecret;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericSecret> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GenericSecret {
  type Msg = GenericSecret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericSecret> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GenericSecretMut<'_> {
  type Msg = GenericSecret;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericSecret> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GenericSecretMut<'_> {
  type Msg = GenericSecret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericSecret> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GenericSecretView<'_> {
  type Msg = GenericSecret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericSecret> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GenericSecretMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod generic_secret {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__GenericSecret__SecretsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct SecretsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SecretsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::generic_secret::envoy__extensions__transport_0sockets__tls__v3__GenericSecret__SecretsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::generic_secret::envoy__extensions__transport_0sockets__tls__v3__GenericSecret__SecretsEntry_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::generic_secret::envoy__extensions__transport_0sockets__tls__v3__GenericSecret__SecretsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod generic_secret


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__SdsSecretConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SdsSecretConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SdsSecretConfig>
}

impl ::protobuf::Message for SdsSecretConfig {
  type MessageView<'msg> = SdsSecretConfigView<'msg>;
  type MessageMut<'msg> = SdsSecretConfigMut<'msg>;
}

impl ::std::default::Default for SdsSecretConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SdsSecretConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SdsSecretConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `SdsSecretConfigMut`.
unsafe impl ::std::marker::Sync for SdsSecretConfig {}

// SAFETY:
// - `SdsSecretConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SdsSecretConfig {}

impl ::protobuf::Proxied for SdsSecretConfig {
  type View<'msg> = SdsSecretConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SdsSecretConfig {}

impl ::protobuf::MutProxied for SdsSecretConfig {
  type Mut<'msg> = SdsSecretConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SdsSecretConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SdsSecretConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SdsSecretConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SdsSecretConfigView<'msg> {
  type Message = SdsSecretConfig;
}

impl ::std::fmt::Debug for SdsSecretConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SdsSecretConfigView<'_> {
  fn default() -> SdsSecretConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SdsSecretConfig>> for SdsSecretConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SdsSecretConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SdsSecretConfigView<'msg> {

  pub fn to_owned(&self) -> SdsSecretConfig {
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

  // sds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_sds_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn sds_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_sds_config().then(|| self.sds_config())
  }
  pub fn sds_config(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

}

// SAFETY:
// - `SdsSecretConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SdsSecretConfigView<'_> {}

// SAFETY:
// - `SdsSecretConfigView` is `Send` because while its alive a `SdsSecretConfigMut` cannot.
// - `SdsSecretConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for SdsSecretConfigView<'_> {}

impl<'msg> ::protobuf::AsView for SdsSecretConfigView<'msg> {
  type Proxied = SdsSecretConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, SdsSecretConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SdsSecretConfigView<'msg> {
  fn into_view<'shorter>(self) -> SdsSecretConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SdsSecretConfig> for SdsSecretConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SdsSecretConfig {
    let mut dst = SdsSecretConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SdsSecretConfig> for SdsSecretConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SdsSecretConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SdsSecretConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SdsSecretConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SdsSecretConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SdsSecretConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SdsSecretConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SdsSecretConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SdsSecretConfigMut<'msg> {
  type Message = SdsSecretConfig;
}

impl ::std::fmt::Debug for SdsSecretConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SdsSecretConfig>> for SdsSecretConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SdsSecretConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SdsSecretConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SdsSecretConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SdsSecretConfig {
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

  // sds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_sds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_sds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn sds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_sds_config().then(|| self.sds_config())
  }
  pub fn sds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn sds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_sds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

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
// - `SdsSecretConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SdsSecretConfigMut<'_> {}

// SAFETY:
// - `SdsSecretConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SdsSecretConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for SdsSecretConfigMut<'msg> {
  type Proxied = SdsSecretConfig;
  fn as_view(&self) -> ::protobuf::View<'_, SdsSecretConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SdsSecretConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SdsSecretConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SdsSecretConfigMut<'msg> {
  type MutProxied = SdsSecretConfig;
  fn as_mut(&mut self) -> SdsSecretConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SdsSecretConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> SdsSecretConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SdsSecretConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SdsSecretConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SdsSecretConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SdsSecretConfigMut<'_> {
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

  // sds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_sds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_sds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn sds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_sds_config().then(|| self.sds_config())
  }
  pub fn sds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn sds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_sds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl SdsSecretConfig

impl ::std::ops::Drop for SdsSecretConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SdsSecretConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SdsSecretConfig {
  type Proxied = Self;
  fn as_view(&self) -> SdsSecretConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SdsSecretConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SdsSecretConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SdsSecretConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__SdsSecretConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__SdsSecretConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__SdsSecretConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SdsSecretConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SdsSecretConfig {
  type Msg = SdsSecretConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SdsSecretConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SdsSecretConfig {
  type Msg = SdsSecretConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SdsSecretConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SdsSecretConfigMut<'_> {
  type Msg = SdsSecretConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SdsSecretConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SdsSecretConfigMut<'_> {
  type Msg = SdsSecretConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SdsSecretConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SdsSecretConfigView<'_> {
  type Msg = SdsSecretConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SdsSecretConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SdsSecretConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__Secret_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Secret {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Secret>
}

impl ::protobuf::Message for Secret {
  type MessageView<'msg> = SecretView<'msg>;
  type MessageMut<'msg> = SecretMut<'msg>;
}

impl ::std::default::Default for Secret {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Secret {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Secret` is `Sync` because it does not implement interior mutability.
//    Neither does `SecretMut`.
unsafe impl ::std::marker::Sync for Secret {}

// SAFETY:
// - `Secret` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Secret {}

impl ::protobuf::Proxied for Secret {
  type View<'msg> = SecretView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Secret {}

impl ::protobuf::MutProxied for Secret {
  type Mut<'msg> = SecretMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SecretView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Secret>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SecretView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SecretView<'msg> {
  type Message = Secret;
}

impl ::std::fmt::Debug for SecretView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SecretView<'_> {
  fn default() -> SecretView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Secret>> for SecretView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Secret>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SecretView<'msg> {

  pub fn to_owned(&self) -> Secret {
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

  // tls_certificate: optional message envoy.extensions.transport_sockets.tls.v3.TlsCertificate
  pub fn has_tls_certificate(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn tls_certificate_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateView<'msg>> {
    self.has_tls_certificate().then(|| self.tls_certificate())
  }
  pub fn tls_certificate(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateView::default())
  }

  // session_ticket_keys: optional message envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys
  pub fn has_session_ticket_keys(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn session_ticket_keys_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'msg>> {
    self.has_session_ticket_keys().then(|| self.session_ticket_keys())
  }
  pub fn session_ticket_keys(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView::default())
  }

  // validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
  pub fn has_validation_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn validation_context_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'msg>> {
    self.has_validation_context().then(|| self.validation_context())
  }
  pub fn validation_context(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView::default())
  }

  // generic_secret: optional message envoy.extensions.transport_sockets.tls.v3.GenericSecret
  pub fn has_generic_secret(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn generic_secret_opt(self) -> ::std::option::Option<super::GenericSecretView<'msg>> {
    self.has_generic_secret().then(|| self.generic_secret())
  }
  pub fn generic_secret(self) -> super::GenericSecretView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GenericSecretView::default())
  }

  pub fn r#type(self) -> super::secret::TypeOneof<'msg> {
    match self.r#type_case() {
      super::secret::TypeCase::TlsCertificate =>
          super::secret::TypeOneof::TlsCertificate(self.tls_certificate()),
      super::secret::TypeCase::SessionTicketKeys =>
          super::secret::TypeOneof::SessionTicketKeys(self.session_ticket_keys()),
      super::secret::TypeCase::ValidationContext =>
          super::secret::TypeOneof::ValidationContext(self.validation_context()),
      super::secret::TypeCase::GenericSecret =>
          super::secret::TypeOneof::GenericSecret(self.generic_secret()),
      _ => super::secret::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(self) -> super::secret::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::secret::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SecretView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SecretView<'_> {}

// SAFETY:
// - `SecretView` is `Send` because while its alive a `SecretMut` cannot.
// - `SecretView` does not use thread-local data.
unsafe impl ::std::marker::Send for SecretView<'_> {}

impl<'msg> ::protobuf::AsView for SecretView<'msg> {
  type Proxied = Secret;
  fn as_view(&self) -> ::protobuf::View<'msg, Secret> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SecretView<'msg> {
  fn into_view<'shorter>(self) -> SecretView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Secret> for SecretView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Secret {
    let mut dst = Secret::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Secret> for SecretMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Secret {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Secret {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SecretView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SecretMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SecretMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Secret>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SecretMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SecretMut<'msg> {
  type Message = Secret;
}

impl ::std::fmt::Debug for SecretMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Secret>> for SecretMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Secret>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SecretMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Secret> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Secret {
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

  // tls_certificate: optional message envoy.extensions.transport_sockets.tls.v3.TlsCertificate
  pub fn has_tls_certificate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_tls_certificate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn tls_certificate_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateView<'_>> {
    self.has_tls_certificate().then(|| self.tls_certificate())
  }
  pub fn tls_certificate(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateView::default())
  }
  pub fn tls_certificate_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateMut<'_> {
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
  pub fn set_tls_certificate(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // session_ticket_keys: optional message envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys
  pub fn has_session_ticket_keys(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_session_ticket_keys(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn session_ticket_keys_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'_>> {
    self.has_session_ticket_keys().then(|| self.session_ticket_keys())
  }
  pub fn session_ticket_keys(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView::default())
  }
  pub fn session_ticket_keys_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysMut<'_> {
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
  pub fn set_session_ticket_keys(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeys>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
  pub fn has_validation_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_validation_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn validation_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_>> {
    self.has_validation_context().then(|| self.validation_context())
  }
  pub fn validation_context(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView::default())
  }
  pub fn validation_context_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextMut<'_> {
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
  pub fn set_validation_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // generic_secret: optional message envoy.extensions.transport_sockets.tls.v3.GenericSecret
  pub fn has_generic_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_generic_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn generic_secret_opt(&self) -> ::std::option::Option<super::GenericSecretView<'_>> {
    self.has_generic_secret().then(|| self.generic_secret())
  }
  pub fn generic_secret(&self) -> super::GenericSecretView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GenericSecretView::default())
  }
  pub fn generic_secret_mut(&mut self) -> super::GenericSecretMut<'_> {
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
  pub fn set_generic_secret(&mut self,
    val: impl ::protobuf::IntoProxied<super::GenericSecret>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn r#type(&self) -> super::secret::TypeOneof<'_> {
    match &self.r#type_case() {
      super::secret::TypeCase::TlsCertificate =>
          super::secret::TypeOneof::TlsCertificate(self.tls_certificate()),
      super::secret::TypeCase::SessionTicketKeys =>
          super::secret::TypeOneof::SessionTicketKeys(self.session_ticket_keys()),
      super::secret::TypeCase::ValidationContext =>
          super::secret::TypeOneof::ValidationContext(self.validation_context()),
      super::secret::TypeCase::GenericSecret =>
          super::secret::TypeOneof::GenericSecret(self.generic_secret()),
      _ => super::secret::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::secret::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::secret::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SecretMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SecretMut<'_> {}

// SAFETY:
// - `SecretMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SecretMut<'_> {}

impl<'msg> ::protobuf::AsView for SecretMut<'msg> {
  type Proxied = Secret;
  fn as_view(&self) -> ::protobuf::View<'_, Secret> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SecretMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Secret>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SecretMut<'msg> {
  type MutProxied = Secret;
  fn as_mut(&mut self) -> SecretMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SecretMut<'msg> {
  fn into_mut<'shorter>(self) -> SecretMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Secret {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Secret> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SecretView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SecretMut<'_> {
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

  // tls_certificate: optional message envoy.extensions.transport_sockets.tls.v3.TlsCertificate
  pub fn has_tls_certificate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_tls_certificate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn tls_certificate_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateView<'_>> {
    self.has_tls_certificate().then(|| self.tls_certificate())
  }
  pub fn tls_certificate(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateView::default())
  }
  pub fn tls_certificate_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificateMut<'_> {
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
  pub fn set_tls_certificate(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // session_ticket_keys: optional message envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys
  pub fn has_session_ticket_keys(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_session_ticket_keys(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn session_ticket_keys_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'_>> {
    self.has_session_ticket_keys().then(|| self.session_ticket_keys())
  }
  pub fn session_ticket_keys(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView::default())
  }
  pub fn session_ticket_keys_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysMut<'_> {
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
  pub fn set_session_ticket_keys(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeys>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
  pub fn has_validation_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_validation_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn validation_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_>> {
    self.has_validation_context().then(|| self.validation_context())
  }
  pub fn validation_context(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView::default())
  }
  pub fn validation_context_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextMut<'_> {
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
  pub fn set_validation_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // generic_secret: optional message envoy.extensions.transport_sockets.tls.v3.GenericSecret
  pub fn has_generic_secret(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_generic_secret(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn generic_secret_opt(&self) -> ::std::option::Option<super::GenericSecretView<'_>> {
    self.has_generic_secret().then(|| self.generic_secret())
  }
  pub fn generic_secret(&self) -> super::GenericSecretView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GenericSecretView::default())
  }
  pub fn generic_secret_mut(&mut self) -> super::GenericSecretMut<'_> {
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
  pub fn set_generic_secret(&mut self,
    val: impl ::protobuf::IntoProxied<super::GenericSecret>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn r#type(&self) -> super::secret::TypeOneof<'_> {
    match &self.r#type_case() {
      super::secret::TypeCase::TlsCertificate =>
          super::secret::TypeOneof::TlsCertificate(self.tls_certificate()),
      super::secret::TypeCase::SessionTicketKeys =>
          super::secret::TypeOneof::SessionTicketKeys(self.session_ticket_keys()),
      super::secret::TypeCase::ValidationContext =>
          super::secret::TypeOneof::ValidationContext(self.validation_context()),
      super::secret::TypeCase::GenericSecret =>
          super::secret::TypeOneof::GenericSecret(self.generic_secret()),
      _ => super::secret::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::secret::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::secret::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Secret

impl ::std::ops::Drop for Secret {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Secret {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Secret {
  type Proxied = Self;
  fn as_view(&self) -> SecretView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Secret {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SecretMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Secret {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__Secret_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3333^#|$|%|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__Secret_msg_init.0, &[<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeys as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::GenericSecret as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__Secret_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Secret {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Secret {
  type Msg = Secret;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Secret> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Secret {
  type Msg = Secret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Secret> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SecretMut<'_> {
  type Msg = Secret;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Secret> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SecretMut<'_> {
  type Msg = Secret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Secret> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SecretView<'_> {
  type Msg = Secret;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Secret> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SecretMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod secret {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TypeOneof<'msg> {
  TlsCertificate(::protobuf::View<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate>) = 2,
  SessionTicketKeys(::protobuf::View<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeys>) = 3,
  ValidationContext(::protobuf::View<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext>) = 4,
  GenericSecret(::protobuf::View<'msg, super::super::GenericSecret>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TypeCase {
  TlsCertificate = 2,
  SessionTicketKeys = 3,
  ValidationContext = 4,
  GenericSecret = 5,

  not_set = 0
}

impl TypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TypeCase> {
    match v {
      0 => Some(TypeCase::not_set),
      2 => Some(TypeCase::TlsCertificate),
      3 => Some(TypeCase::SessionTicketKeys),
      4 => Some(TypeCase::ValidationContext),
      5 => Some(TypeCase::GenericSecret),
      _ => None
    }
  }
}
}  // pub mod secret


