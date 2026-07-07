const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Locality_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Locality {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Locality>
}

impl ::protobuf::Message for Locality {
  type MessageView<'msg> = LocalityView<'msg>;
  type MessageMut<'msg> = LocalityMut<'msg>;
}

impl ::std::default::Default for Locality {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Locality {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Locality` is `Sync` because it does not implement interior mutability.
//    Neither does `LocalityMut`.
unsafe impl ::std::marker::Sync for Locality {}

// SAFETY:
// - `Locality` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Locality {}

impl ::protobuf::Proxied for Locality {
  type View<'msg> = LocalityView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Locality {}

impl ::protobuf::MutProxied for Locality {
  type Mut<'msg> = LocalityMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocalityView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Locality>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocalityView<'msg> {
  type Message = Locality;
}

impl ::std::fmt::Debug for LocalityView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocalityView<'_> {
  fn default() -> LocalityView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Locality>> for LocalityView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Locality>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityView<'msg> {

  pub fn to_owned(&self) -> Locality {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // region: optional string
  pub fn region(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // zone: optional string
  pub fn zone(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // sub_zone: optional string
  pub fn sub_zone(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `LocalityView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LocalityView<'_> {}

// SAFETY:
// - `LocalityView` is `Send` because while its alive a `LocalityMut` cannot.
// - `LocalityView` does not use thread-local data.
unsafe impl ::std::marker::Send for LocalityView<'_> {}

impl<'msg> ::protobuf::AsView for LocalityView<'msg> {
  type Proxied = Locality;
  fn as_view(&self) -> ::protobuf::View<'msg, Locality> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityView<'msg> {
  fn into_view<'shorter>(self) -> LocalityView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Locality> for LocalityView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Locality {
    let mut dst = Locality::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Locality> for LocalityMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Locality {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Locality {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocalityMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Locality>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocalityMut<'msg> {
  type Message = Locality;
}

impl ::std::fmt::Debug for LocalityMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Locality>> for LocalityMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Locality>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Locality> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Locality {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // region: optional string
  pub fn region(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_region(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // zone: optional string
  pub fn zone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_zone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // sub_zone: optional string
  pub fn sub_zone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_sub_zone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `LocalityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LocalityMut<'_> {}

// SAFETY:
// - `LocalityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LocalityMut<'_> {}

impl<'msg> ::protobuf::AsView for LocalityMut<'msg> {
  type Proxied = Locality;
  fn as_view(&self) -> ::protobuf::View<'_, Locality> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Locality>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LocalityMut<'msg> {
  type MutProxied = Locality;
  fn as_mut(&mut self) -> LocalityMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocalityMut<'msg> {
  fn into_mut<'shorter>(self) -> LocalityMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Locality {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Locality> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocalityView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocalityMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // region: optional string
  pub fn region(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_region(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // zone: optional string
  pub fn zone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_zone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // sub_zone: optional string
  pub fn sub_zone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_sub_zone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl Locality

impl ::std::ops::Drop for Locality {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Locality {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Locality {
  type Proxied = Self;
  fn as_view(&self) -> LocalityView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Locality {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocalityMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Locality {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__Locality_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__Locality_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__Locality_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Locality {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Locality {
  type Msg = Locality;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Locality> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Locality {
  type Msg = Locality;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Locality> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalityMut<'_> {
  type Msg = Locality;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Locality> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityMut<'_> {
  type Msg = Locality;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Locality> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityView<'_> {
  type Msg = Locality;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Locality> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalityMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__BuildVersion_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BuildVersion {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BuildVersion>
}

impl ::protobuf::Message for BuildVersion {
  type MessageView<'msg> = BuildVersionView<'msg>;
  type MessageMut<'msg> = BuildVersionMut<'msg>;
}

impl ::std::default::Default for BuildVersion {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BuildVersion {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BuildVersion` is `Sync` because it does not implement interior mutability.
//    Neither does `BuildVersionMut`.
unsafe impl ::std::marker::Sync for BuildVersion {}

// SAFETY:
// - `BuildVersion` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BuildVersion {}

impl ::protobuf::Proxied for BuildVersion {
  type View<'msg> = BuildVersionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BuildVersion {}

impl ::protobuf::MutProxied for BuildVersion {
  type Mut<'msg> = BuildVersionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BuildVersionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BuildVersion>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BuildVersionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BuildVersionView<'msg> {
  type Message = BuildVersion;
}

impl ::std::fmt::Debug for BuildVersionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BuildVersionView<'_> {
  fn default() -> BuildVersionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BuildVersion>> for BuildVersionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BuildVersion>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BuildVersionView<'msg> {

  pub fn to_owned(&self) -> BuildVersion {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // version: optional message envoy.type.v3.SemanticVersion
  pub fn has_version(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn version_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionView<'msg>> {
    self.has_version().then(|| self.version())
  }
  pub fn version(self) -> crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionView::default())
  }

  // metadata: optional message google.protobuf.Struct
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

}

// SAFETY:
// - `BuildVersionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BuildVersionView<'_> {}

// SAFETY:
// - `BuildVersionView` is `Send` because while its alive a `BuildVersionMut` cannot.
// - `BuildVersionView` does not use thread-local data.
unsafe impl ::std::marker::Send for BuildVersionView<'_> {}

impl<'msg> ::protobuf::AsView for BuildVersionView<'msg> {
  type Proxied = BuildVersion;
  fn as_view(&self) -> ::protobuf::View<'msg, BuildVersion> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BuildVersionView<'msg> {
  fn into_view<'shorter>(self) -> BuildVersionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BuildVersion> for BuildVersionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BuildVersion {
    let mut dst = BuildVersion::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BuildVersion> for BuildVersionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BuildVersion {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BuildVersion {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BuildVersionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BuildVersionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BuildVersionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BuildVersion>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BuildVersionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BuildVersionMut<'msg> {
  type Message = BuildVersion;
}

impl ::std::fmt::Debug for BuildVersionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BuildVersion>> for BuildVersionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BuildVersion>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BuildVersionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BuildVersion> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BuildVersion {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // version: optional message envoy.type.v3.SemanticVersion
  pub fn has_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn version_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionView<'_>> {
    self.has_version().then(|| self.version())
  }
  pub fn version(&self) -> crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionView::default())
  }
  pub fn version_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionMut<'_> {
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
  pub fn set_version(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersion>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // metadata: optional message google.protobuf.Struct
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

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
// - `BuildVersionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BuildVersionMut<'_> {}

// SAFETY:
// - `BuildVersionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BuildVersionMut<'_> {}

impl<'msg> ::protobuf::AsView for BuildVersionMut<'msg> {
  type Proxied = BuildVersion;
  fn as_view(&self) -> ::protobuf::View<'_, BuildVersion> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BuildVersionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BuildVersion>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BuildVersionMut<'msg> {
  type MutProxied = BuildVersion;
  fn as_mut(&mut self) -> BuildVersionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BuildVersionMut<'msg> {
  fn into_mut<'shorter>(self) -> BuildVersionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BuildVersion {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BuildVersion> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BuildVersionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BuildVersionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // version: optional message envoy.type.v3.SemanticVersion
  pub fn has_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn version_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionView<'_>> {
    self.has_version().then(|| self.version())
  }
  pub fn version(&self) -> crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionView::default())
  }
  pub fn version_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersionMut<'_> {
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
  pub fn set_version(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersion>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // metadata: optional message google.protobuf.Struct
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl BuildVersion

impl ::std::ops::Drop for BuildVersion {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BuildVersion {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BuildVersion {
  type Proxied = Self;
  fn as_view(&self) -> BuildVersionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BuildVersion {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BuildVersionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BuildVersion {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__BuildVersion_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__BuildVersion_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::semantic_version::SemanticVersion as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__BuildVersion_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BuildVersion {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BuildVersion {
  type Msg = BuildVersion;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuildVersion> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BuildVersion {
  type Msg = BuildVersion;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuildVersion> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BuildVersionMut<'_> {
  type Msg = BuildVersion;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuildVersion> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BuildVersionMut<'_> {
  type Msg = BuildVersion;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuildVersion> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BuildVersionView<'_> {
  type Msg = BuildVersion;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BuildVersion> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BuildVersionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Extension_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Extension {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Extension>
}

impl ::protobuf::Message for Extension {
  type MessageView<'msg> = ExtensionView<'msg>;
  type MessageMut<'msg> = ExtensionMut<'msg>;
}

impl ::std::default::Default for Extension {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Extension {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Extension` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtensionMut`.
unsafe impl ::std::marker::Sync for Extension {}

// SAFETY:
// - `Extension` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Extension {}

impl ::protobuf::Proxied for Extension {
  type View<'msg> = ExtensionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Extension {}

impl ::protobuf::MutProxied for Extension {
  type Mut<'msg> = ExtensionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtensionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Extension>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtensionView<'msg> {
  type Message = Extension;
}

impl ::std::fmt::Debug for ExtensionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtensionView<'_> {
  fn default() -> ExtensionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Extension>> for ExtensionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Extension>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionView<'msg> {

  pub fn to_owned(&self) -> Extension {
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

  // category: optional string
  pub fn category(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // type_descriptor: optional string
  pub fn type_descriptor(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // version: optional message envoy.config.core.v3.BuildVersion
  pub fn has_version(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn version_opt(self) -> ::std::option::Option<super::BuildVersionView<'msg>> {
    self.has_version().then(|| self.version())
  }
  pub fn version(self) -> super::BuildVersionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BuildVersionView::default())
  }

  // disabled: optional bool
  pub fn disabled(self) -> bool {
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

  // type_urls: repeated string
  pub fn type_urls(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
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
// - `ExtensionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtensionView<'_> {}

// SAFETY:
// - `ExtensionView` is `Send` because while its alive a `ExtensionMut` cannot.
// - `ExtensionView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionView<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionView<'msg> {
  type Proxied = Extension;
  fn as_view(&self) -> ::protobuf::View<'msg, Extension> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionView<'msg> {
  fn into_view<'shorter>(self) -> ExtensionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Extension> for ExtensionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Extension {
    let mut dst = Extension::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Extension> for ExtensionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Extension {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Extension {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtensionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Extension>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtensionMut<'msg> {
  type Message = Extension;
}

impl ::std::fmt::Debug for ExtensionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Extension>> for ExtensionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Extension>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Extension> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Extension {
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

  // category: optional string
  pub fn category(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_category(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // type_descriptor: optional string
  pub fn type_descriptor(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_descriptor(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // version: optional message envoy.config.core.v3.BuildVersion
  pub fn has_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn version_opt(&self) -> ::std::option::Option<super::BuildVersionView<'_>> {
    self.has_version().then(|| self.version())
  }
  pub fn version(&self) -> super::BuildVersionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BuildVersionView::default())
  }
  pub fn version_mut(&mut self) -> super::BuildVersionMut<'_> {
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
  pub fn set_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::BuildVersion>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // disabled: optional bool
  pub fn disabled(&self) -> bool {
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
  pub fn set_disabled(&mut self, val: bool) {
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

  // type_urls: repeated string
  pub fn type_urls(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn type_urls_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_type_urls(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}

// SAFETY:
// - `ExtensionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtensionMut<'_> {}

// SAFETY:
// - `ExtensionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtensionMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionMut<'msg> {
  type Proxied = Extension;
  fn as_view(&self) -> ::protobuf::View<'_, Extension> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Extension>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtensionMut<'msg> {
  type MutProxied = Extension;
  fn as_mut(&mut self) -> ExtensionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtensionMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtensionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Extension {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Extension> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtensionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtensionMut<'_> {
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

  // category: optional string
  pub fn category(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_category(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // type_descriptor: optional string
  pub fn type_descriptor(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_descriptor(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // version: optional message envoy.config.core.v3.BuildVersion
  pub fn has_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn version_opt(&self) -> ::std::option::Option<super::BuildVersionView<'_>> {
    self.has_version().then(|| self.version())
  }
  pub fn version(&self) -> super::BuildVersionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BuildVersionView::default())
  }
  pub fn version_mut(&mut self) -> super::BuildVersionMut<'_> {
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
  pub fn set_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::BuildVersion>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // disabled: optional bool
  pub fn disabled(&self) -> bool {
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
  pub fn set_disabled(&mut self, val: bool) {
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

  // type_urls: repeated string
  pub fn type_urls(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn type_urls_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_type_urls(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}  // impl Extension

impl ::std::ops::Drop for Extension {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Extension {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Extension {
  type Proxied = Self;
  fn as_view(&self) -> ExtensionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Extension {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtensionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Extension {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__Extension_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X1X3/PET");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__Extension_msg_init.0, &[<super::BuildVersion as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__Extension_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Extension {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Extension {
  type Msg = Extension;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Extension {
  type Msg = Extension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionMut<'_> {
  type Msg = Extension;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionMut<'_> {
  type Msg = Extension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionView<'_> {
  type Msg = Extension;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Extension> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Node_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Node {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Node>
}

impl ::protobuf::Message for Node {
  type MessageView<'msg> = NodeView<'msg>;
  type MessageMut<'msg> = NodeMut<'msg>;
}

impl ::std::default::Default for Node {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Node {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Node` is `Sync` because it does not implement interior mutability.
//    Neither does `NodeMut`.
unsafe impl ::std::marker::Sync for Node {}

// SAFETY:
// - `Node` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Node {}

impl ::protobuf::Proxied for Node {
  type View<'msg> = NodeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Node {}

impl ::protobuf::MutProxied for Node {
  type Mut<'msg> = NodeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NodeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Node>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NodeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NodeView<'msg> {
  type Message = Node;
}

impl ::std::fmt::Debug for NodeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NodeView<'_> {
  fn default() -> NodeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Node>> for NodeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Node>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NodeView<'msg> {

  pub fn to_owned(&self) -> Node {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional string
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // cluster: optional string
  pub fn cluster(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // metadata: optional message google.protobuf.Struct
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // dynamic_parameters: repeated message envoy.config.core.v3.Node.DynamicParametersEntry
  pub fn dynamic_parameters(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(10)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // locality: optional message envoy.config.core.v3.Locality
  pub fn has_locality(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn locality_opt(self) -> ::std::option::Option<super::LocalityView<'msg>> {
    self.has_locality().then(|| self.locality())
  }
  pub fn locality(self) -> super::LocalityView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LocalityView::default())
  }

  // user_agent_name: optional string
  pub fn user_agent_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // user_agent_version: optional string
  pub fn has_user_agent_version(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn user_agent_version_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_user_agent_version().then(|| self.user_agent_version())
  }
  pub fn user_agent_version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // user_agent_build_version: optional message envoy.config.core.v3.BuildVersion
  pub fn has_user_agent_build_version(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn user_agent_build_version_opt(self) -> ::std::option::Option<super::BuildVersionView<'msg>> {
    self.has_user_agent_build_version().then(|| self.user_agent_build_version())
  }
  pub fn user_agent_build_version(self) -> super::BuildVersionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BuildVersionView::default())
  }

  // extensions: repeated message envoy.config.core.v3.Extension
  pub fn extensions(self) -> ::protobuf::RepeatedView<'msg, super::Extension> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Extension>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // client_features: repeated string
  pub fn client_features(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // listening_addresses: repeated message envoy.config.core.v3.Address
  pub fn listening_addresses(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::Address>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  pub fn user_agent_version_type(self) -> super::node::UserAgentVersionTypeOneof<'msg> {
    match self.user_agent_version_type_case() {
      super::node::UserAgentVersionTypeCase::UserAgentVersion =>
          super::node::UserAgentVersionTypeOneof::UserAgentVersion(self.user_agent_version()),
      super::node::UserAgentVersionTypeCase::UserAgentBuildVersion =>
          super::node::UserAgentVersionTypeOneof::UserAgentBuildVersion(self.user_agent_build_version()),
      _ => super::node::UserAgentVersionTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn user_agent_version_type_case(self) -> super::node::UserAgentVersionTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(5);
      super::node::UserAgentVersionTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `NodeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for NodeView<'_> {}

// SAFETY:
// - `NodeView` is `Send` because while its alive a `NodeMut` cannot.
// - `NodeView` does not use thread-local data.
unsafe impl ::std::marker::Send for NodeView<'_> {}

impl<'msg> ::protobuf::AsView for NodeView<'msg> {
  type Proxied = Node;
  fn as_view(&self) -> ::protobuf::View<'msg, Node> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NodeView<'msg> {
  fn into_view<'shorter>(self) -> NodeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Node> for NodeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Node {
    let mut dst = Node::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Node> for NodeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Node {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Node {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NodeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NodeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NodeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Node>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NodeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NodeMut<'msg> {
  type Message = Node;
}

impl ::std::fmt::Debug for NodeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Node>> for NodeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Node>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NodeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Node> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Node {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // cluster: optional string
  pub fn cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // metadata: optional message google.protobuf.Struct
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // dynamic_parameters: repeated message envoy.config.core.v3.Node.DynamicParametersEntry
  pub fn dynamic_parameters(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(10)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn dynamic_parameters_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          10, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_dynamic_parameters(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

  // locality: optional message envoy.config.core.v3.Locality
  pub fn has_locality(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_locality(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn locality_opt(&self) -> ::std::option::Option<super::LocalityView<'_>> {
    self.has_locality().then(|| self.locality())
  }
  pub fn locality(&self) -> super::LocalityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LocalityView::default())
  }
  pub fn locality_mut(&mut self) -> super::LocalityMut<'_> {
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
  pub fn set_locality(&mut self,
    val: impl ::protobuf::IntoProxied<super::Locality>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // user_agent_name: optional string
  pub fn user_agent_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_user_agent_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // user_agent_version: optional string
  pub fn has_user_agent_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_user_agent_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn user_agent_version_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_user_agent_version().then(|| self.user_agent_version())
  }
  pub fn user_agent_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_user_agent_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // user_agent_build_version: optional message envoy.config.core.v3.BuildVersion
  pub fn has_user_agent_build_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_user_agent_build_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn user_agent_build_version_opt(&self) -> ::std::option::Option<super::BuildVersionView<'_>> {
    self.has_user_agent_build_version().then(|| self.user_agent_build_version())
  }
  pub fn user_agent_build_version(&self) -> super::BuildVersionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BuildVersionView::default())
  }
  pub fn user_agent_build_version_mut(&mut self) -> super::BuildVersionMut<'_> {
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
  pub fn set_user_agent_build_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::BuildVersion>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // extensions: repeated message envoy.config.core.v3.Extension
  pub fn extensions(&self) -> ::protobuf::RepeatedView<'_, super::Extension> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Extension>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Extension> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Extension>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // client_features: repeated string
  pub fn client_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn client_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_client_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // listening_addresses: repeated message envoy.config.core.v3.Address
  pub fn listening_addresses(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::Address>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn listening_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_listening_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::Address>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

  pub fn user_agent_version_type(&self) -> super::node::UserAgentVersionTypeOneof<'_> {
    match &self.user_agent_version_type_case() {
      super::node::UserAgentVersionTypeCase::UserAgentVersion =>
          super::node::UserAgentVersionTypeOneof::UserAgentVersion(self.user_agent_version()),
      super::node::UserAgentVersionTypeCase::UserAgentBuildVersion =>
          super::node::UserAgentVersionTypeOneof::UserAgentBuildVersion(self.user_agent_build_version()),
      _ => super::node::UserAgentVersionTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn user_agent_version_type_case(&self) -> super::node::UserAgentVersionTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(5);
      super::node::UserAgentVersionTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `NodeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for NodeMut<'_> {}

// SAFETY:
// - `NodeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for NodeMut<'_> {}

impl<'msg> ::protobuf::AsView for NodeMut<'msg> {
  type Proxied = Node;
  fn as_view(&self) -> ::protobuf::View<'_, Node> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NodeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Node>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for NodeMut<'msg> {
  type MutProxied = Node;
  fn as_mut(&mut self) -> NodeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NodeMut<'msg> {
  fn into_mut<'shorter>(self) -> NodeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Node {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Node> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NodeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NodeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // cluster: optional string
  pub fn cluster(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // metadata: optional message google.protobuf.Struct
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn metadata_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // dynamic_parameters: repeated message envoy.config.core.v3.Node.DynamicParametersEntry
  pub fn dynamic_parameters(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(10)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn dynamic_parameters_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          10, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_dynamic_parameters(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, crate::xds::generated::xds::core::v3::context_params::ContextParams>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

  // locality: optional message envoy.config.core.v3.Locality
  pub fn has_locality(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_locality(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn locality_opt(&self) -> ::std::option::Option<super::LocalityView<'_>> {
    self.has_locality().then(|| self.locality())
  }
  pub fn locality(&self) -> super::LocalityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LocalityView::default())
  }
  pub fn locality_mut(&mut self) -> super::LocalityMut<'_> {
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
  pub fn set_locality(&mut self,
    val: impl ::protobuf::IntoProxied<super::Locality>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // user_agent_name: optional string
  pub fn user_agent_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_user_agent_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // user_agent_version: optional string
  pub fn has_user_agent_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_user_agent_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn user_agent_version_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_user_agent_version().then(|| self.user_agent_version())
  }
  pub fn user_agent_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_user_agent_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // user_agent_build_version: optional message envoy.config.core.v3.BuildVersion
  pub fn has_user_agent_build_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_user_agent_build_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn user_agent_build_version_opt(&self) -> ::std::option::Option<super::BuildVersionView<'_>> {
    self.has_user_agent_build_version().then(|| self.user_agent_build_version())
  }
  pub fn user_agent_build_version(&self) -> super::BuildVersionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::BuildVersionView::default())
  }
  pub fn user_agent_build_version_mut(&mut self) -> super::BuildVersionMut<'_> {
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
  pub fn set_user_agent_build_version(&mut self,
    val: impl ::protobuf::IntoProxied<super::BuildVersion>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // extensions: repeated message envoy.config.core.v3.Extension
  pub fn extensions(&self) -> ::protobuf::RepeatedView<'_, super::Extension> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Extension>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Extension> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Extension>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // client_features: repeated string
  pub fn client_features(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        8
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn client_features_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        8,
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
  pub fn set_client_features(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        src);
    }
  }

  // listening_addresses: repeated message envoy.config.core.v3.Address
  pub fn listening_addresses(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::Address>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn listening_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_listening_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::Address>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

  pub fn user_agent_version_type(&self) -> super::node::UserAgentVersionTypeOneof<'_> {
    match &self.user_agent_version_type_case() {
      super::node::UserAgentVersionTypeCase::UserAgentVersion =>
          super::node::UserAgentVersionTypeOneof::UserAgentVersion(self.user_agent_version()),
      super::node::UserAgentVersionTypeCase::UserAgentBuildVersion =>
          super::node::UserAgentVersionTypeOneof::UserAgentBuildVersion(self.user_agent_build_version()),
      _ => super::node::UserAgentVersionTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn user_agent_version_type_case(&self) -> super::node::UserAgentVersionTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(5);
      super::node::UserAgentVersionTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Node

impl ::std::ops::Drop for Node {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Node {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Node {
  type Proxied = Self;
  fn as_view(&self) -> NodeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Node {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NodeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Node {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__Node_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X33a1X1T3GETGG^)|*");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__Node_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Locality as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::BuildVersion as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Extension as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::node::DynamicParametersEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__Node_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Node {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Node {
  type Msg = Node;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Node> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Node {
  type Msg = Node;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Node> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NodeMut<'_> {
  type Msg = Node;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Node> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NodeMut<'_> {
  type Msg = Node;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Node> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NodeView<'_> {
  type Msg = Node;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Node> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NodeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod node {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Node__DynamicParametersEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct DynamicParametersEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicParametersEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::node::envoy__config__core__v3__Node__DynamicParametersEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::node::envoy__config__core__v3__Node__DynamicParametersEntry_msg_init.0, &[<crate::xds::generated::xds::core::v3::context_params::ContextParams as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::node::envoy__config__core__v3__Node__DynamicParametersEntry_msg_init.0)
      }).0
    }
  }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum UserAgentVersionTypeOneof<'msg> {
  UserAgentVersion(&'msg ::protobuf::ProtoStr) = 7,
  UserAgentBuildVersion(::protobuf::View<'msg, super::super::BuildVersion>) = 8,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum UserAgentVersionTypeCase {
  UserAgentVersion = 7,
  UserAgentBuildVersion = 8,

  not_set = 0
}

impl UserAgentVersionTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<UserAgentVersionTypeCase> {
    match v {
      0 => Some(UserAgentVersionTypeCase::not_set),
      7 => Some(UserAgentVersionTypeCase::UserAgentVersion),
      8 => Some(UserAgentVersionTypeCase::UserAgentBuildVersion),
      _ => None
    }
  }
}
}  // pub mod node


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Metadata_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Metadata {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Metadata>
}

impl ::protobuf::Message for Metadata {
  type MessageView<'msg> = MetadataView<'msg>;
  type MessageMut<'msg> = MetadataMut<'msg>;
}

impl ::std::default::Default for Metadata {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Metadata {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Metadata` is `Sync` because it does not implement interior mutability.
//    Neither does `MetadataMut`.
unsafe impl ::std::marker::Sync for Metadata {}

// SAFETY:
// - `Metadata` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Metadata {}

impl ::protobuf::Proxied for Metadata {
  type View<'msg> = MetadataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Metadata {}

impl ::protobuf::MutProxied for Metadata {
  type Mut<'msg> = MetadataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MetadataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Metadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MetadataView<'msg> {
  type Message = Metadata;
}

impl ::std::fmt::Debug for MetadataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MetadataView<'_> {
  fn default() -> MetadataView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Metadata>> for MetadataView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Metadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataView<'msg> {

  pub fn to_owned(&self) -> Metadata {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // filter_metadata: repeated message envoy.config.core.v3.Metadata.FilterMetadataEntry
  pub fn filter_metadata(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // typed_filter_metadata: repeated message envoy.config.core.v3.Metadata.TypedFilterMetadataEntry
  pub fn typed_filter_metadata(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `MetadataView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MetadataView<'_> {}

// SAFETY:
// - `MetadataView` is `Send` because while its alive a `MetadataMut` cannot.
// - `MetadataView` does not use thread-local data.
unsafe impl ::std::marker::Send for MetadataView<'_> {}

impl<'msg> ::protobuf::AsView for MetadataView<'msg> {
  type Proxied = Metadata;
  fn as_view(&self) -> ::protobuf::View<'msg, Metadata> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataView<'msg> {
  fn into_view<'shorter>(self) -> MetadataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Metadata> for MetadataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Metadata {
    let mut dst = Metadata::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Metadata> for MetadataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Metadata {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Metadata {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MetadataMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MetadataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Metadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MetadataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MetadataMut<'msg> {
  type Message = Metadata;
}

impl ::std::fmt::Debug for MetadataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Metadata>> for MetadataMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Metadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MetadataMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Metadata> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Metadata {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // filter_metadata: repeated message envoy.config.core.v3.Metadata.FilterMetadataEntry
  pub fn filter_metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn filter_metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_filter_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // typed_filter_metadata: repeated message envoy.config.core.v3.Metadata.TypedFilterMetadataEntry
  pub fn typed_filter_metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn typed_filter_metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_typed_filter_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `MetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MetadataMut<'_> {}

// SAFETY:
// - `MetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MetadataMut<'_> {}

impl<'msg> ::protobuf::AsView for MetadataMut<'msg> {
  type Proxied = Metadata;
  fn as_view(&self) -> ::protobuf::View<'_, Metadata> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MetadataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Metadata>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MetadataMut<'msg> {
  type MutProxied = Metadata;
  fn as_mut(&mut self) -> MetadataMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MetadataMut<'msg> {
  fn into_mut<'shorter>(self) -> MetadataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Metadata {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Metadata> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MetadataView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MetadataMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // filter_metadata: repeated message envoy.config.core.v3.Metadata.FilterMetadataEntry
  pub fn filter_metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn filter_metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Struct> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_filter_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Struct>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // typed_filter_metadata: repeated message envoy.config.core.v3.Metadata.TypedFilterMetadataEntry
  pub fn typed_filter_metadata(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(1)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn typed_filter_metadata_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          1, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_typed_filter_metadata(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl Metadata

impl ::std::ops::Drop for Metadata {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Metadata {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Metadata {
  type Proxied = Self;
  fn as_view(&self) -> MetadataView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Metadata {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MetadataMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Metadata {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__Metadata_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__Metadata_msg_init.0, &[<super::metadata::FilterMetadataEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::metadata::TypedFilterMetadataEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__Metadata_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Metadata {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Metadata {
  type Msg = Metadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Metadata {
  type Msg = Metadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MetadataMut<'_> {
  type Msg = Metadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataMut<'_> {
  type Msg = Metadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MetadataView<'_> {
  type Msg = Metadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Metadata> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MetadataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod metadata {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Metadata__FilterMetadataEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct FilterMetadataEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FilterMetadataEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::metadata::envoy__config__core__v3__Metadata__FilterMetadataEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::metadata::envoy__config__core__v3__Metadata__FilterMetadataEntry_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::metadata::envoy__config__core__v3__Metadata__FilterMetadataEntry_msg_init.0)
      }).0
    }
  }
}
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Metadata__TypedFilterMetadataEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct TypedFilterMetadataEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TypedFilterMetadataEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::metadata::envoy__config__core__v3__Metadata__TypedFilterMetadataEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::metadata::envoy__config__core__v3__Metadata__TypedFilterMetadataEntry_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::metadata::envoy__config__core__v3__Metadata__TypedFilterMetadataEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod metadata


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RuntimeUInt32_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RuntimeUInt32 {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RuntimeUInt32>
}

impl ::protobuf::Message for RuntimeUInt32 {
  type MessageView<'msg> = RuntimeUInt32View<'msg>;
  type MessageMut<'msg> = RuntimeUInt32Mut<'msg>;
}

impl ::std::default::Default for RuntimeUInt32 {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RuntimeUInt32 {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RuntimeUInt32` is `Sync` because it does not implement interior mutability.
//    Neither does `RuntimeUInt32Mut`.
unsafe impl ::std::marker::Sync for RuntimeUInt32 {}

// SAFETY:
// - `RuntimeUInt32` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeUInt32 {}

impl ::protobuf::Proxied for RuntimeUInt32 {
  type View<'msg> = RuntimeUInt32View<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RuntimeUInt32 {}

impl ::protobuf::MutProxied for RuntimeUInt32 {
  type Mut<'msg> = RuntimeUInt32Mut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RuntimeUInt32View<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeUInt32>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeUInt32View<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RuntimeUInt32View<'msg> {
  type Message = RuntimeUInt32;
}

impl ::std::fmt::Debug for RuntimeUInt32View<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RuntimeUInt32View<'_> {
  fn default() -> RuntimeUInt32View<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeUInt32>> for RuntimeUInt32View<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeUInt32>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeUInt32View<'msg> {

  pub fn to_owned(&self) -> RuntimeUInt32 {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // default_value: optional uint32
  pub fn default_value(self) -> u32 {
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

  // runtime_key: optional string
  pub fn runtime_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `RuntimeUInt32View` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RuntimeUInt32View<'_> {}

// SAFETY:
// - `RuntimeUInt32View` is `Send` because while its alive a `RuntimeUInt32Mut` cannot.
// - `RuntimeUInt32View` does not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeUInt32View<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeUInt32View<'msg> {
  type Proxied = RuntimeUInt32;
  fn as_view(&self) -> ::protobuf::View<'msg, RuntimeUInt32> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeUInt32View<'msg> {
  fn into_view<'shorter>(self) -> RuntimeUInt32View<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeUInt32> for RuntimeUInt32View<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeUInt32 {
    let mut dst = RuntimeUInt32::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeUInt32> for RuntimeUInt32Mut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeUInt32 {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RuntimeUInt32 {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeUInt32View<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeUInt32Mut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RuntimeUInt32Mut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeUInt32>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeUInt32Mut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RuntimeUInt32Mut<'msg> {
  type Message = RuntimeUInt32;
}

impl ::std::fmt::Debug for RuntimeUInt32Mut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeUInt32>> for RuntimeUInt32Mut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeUInt32>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeUInt32Mut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeUInt32> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RuntimeUInt32 {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // default_value: optional uint32
  pub fn default_value(&self) -> u32 {
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
  pub fn set_default_value(&mut self, val: u32) {
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

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `RuntimeUInt32Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RuntimeUInt32Mut<'_> {}

// SAFETY:
// - `RuntimeUInt32Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RuntimeUInt32Mut<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeUInt32Mut<'msg> {
  type Proxied = RuntimeUInt32;
  fn as_view(&self) -> ::protobuf::View<'_, RuntimeUInt32> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeUInt32Mut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RuntimeUInt32>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RuntimeUInt32Mut<'msg> {
  type MutProxied = RuntimeUInt32;
  fn as_mut(&mut self) -> RuntimeUInt32Mut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RuntimeUInt32Mut<'msg> {
  fn into_mut<'shorter>(self) -> RuntimeUInt32Mut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RuntimeUInt32 {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RuntimeUInt32> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RuntimeUInt32View<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RuntimeUInt32Mut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // default_value: optional uint32
  pub fn default_value(&self) -> u32 {
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
  pub fn set_default_value(&mut self, val: u32) {
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

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl RuntimeUInt32

impl ::std::ops::Drop for RuntimeUInt32 {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RuntimeUInt32 {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RuntimeUInt32 {
  type Proxied = Self;
  fn as_view(&self) -> RuntimeUInt32View<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RuntimeUInt32 {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RuntimeUInt32Mut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RuntimeUInt32 {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__RuntimeUInt32_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$a)P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__RuntimeUInt32_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__RuntimeUInt32_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeUInt32 {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeUInt32 {
  type Msg = RuntimeUInt32;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeUInt32> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeUInt32 {
  type Msg = RuntimeUInt32;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeUInt32> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeUInt32Mut<'_> {
  type Msg = RuntimeUInt32;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeUInt32> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeUInt32Mut<'_> {
  type Msg = RuntimeUInt32;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeUInt32> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeUInt32View<'_> {
  type Msg = RuntimeUInt32;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeUInt32> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeUInt32Mut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RuntimePercent_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RuntimePercent {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RuntimePercent>
}

impl ::protobuf::Message for RuntimePercent {
  type MessageView<'msg> = RuntimePercentView<'msg>;
  type MessageMut<'msg> = RuntimePercentMut<'msg>;
}

impl ::std::default::Default for RuntimePercent {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RuntimePercent {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RuntimePercent` is `Sync` because it does not implement interior mutability.
//    Neither does `RuntimePercentMut`.
unsafe impl ::std::marker::Sync for RuntimePercent {}

// SAFETY:
// - `RuntimePercent` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RuntimePercent {}

impl ::protobuf::Proxied for RuntimePercent {
  type View<'msg> = RuntimePercentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RuntimePercent {}

impl ::protobuf::MutProxied for RuntimePercent {
  type Mut<'msg> = RuntimePercentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RuntimePercentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimePercent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimePercentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RuntimePercentView<'msg> {
  type Message = RuntimePercent;
}

impl ::std::fmt::Debug for RuntimePercentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RuntimePercentView<'_> {
  fn default() -> RuntimePercentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimePercent>> for RuntimePercentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimePercent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimePercentView<'msg> {

  pub fn to_owned(&self) -> RuntimePercent {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // default_value: optional message envoy.type.v3.Percent
  pub fn has_default_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn default_value_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  // runtime_key: optional string
  pub fn runtime_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `RuntimePercentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RuntimePercentView<'_> {}

// SAFETY:
// - `RuntimePercentView` is `Send` because while its alive a `RuntimePercentMut` cannot.
// - `RuntimePercentView` does not use thread-local data.
unsafe impl ::std::marker::Send for RuntimePercentView<'_> {}

impl<'msg> ::protobuf::AsView for RuntimePercentView<'msg> {
  type Proxied = RuntimePercent;
  fn as_view(&self) -> ::protobuf::View<'msg, RuntimePercent> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimePercentView<'msg> {
  fn into_view<'shorter>(self) -> RuntimePercentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimePercent> for RuntimePercentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimePercent {
    let mut dst = RuntimePercent::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimePercent> for RuntimePercentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimePercent {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RuntimePercent {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimePercentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimePercentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RuntimePercentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimePercent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimePercentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RuntimePercentMut<'msg> {
  type Message = RuntimePercent;
}

impl ::std::fmt::Debug for RuntimePercentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimePercent>> for RuntimePercentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimePercent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimePercentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimePercent> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RuntimePercent {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // default_value: optional message envoy.type.v3.Percent
  pub fn has_default_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_default_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn default_value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn default_value_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_default_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `RuntimePercentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RuntimePercentMut<'_> {}

// SAFETY:
// - `RuntimePercentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RuntimePercentMut<'_> {}

impl<'msg> ::protobuf::AsView for RuntimePercentMut<'msg> {
  type Proxied = RuntimePercent;
  fn as_view(&self) -> ::protobuf::View<'_, RuntimePercent> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimePercentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RuntimePercent>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RuntimePercentMut<'msg> {
  type MutProxied = RuntimePercent;
  fn as_mut(&mut self) -> RuntimePercentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RuntimePercentMut<'msg> {
  fn into_mut<'shorter>(self) -> RuntimePercentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RuntimePercent {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RuntimePercent> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RuntimePercentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RuntimePercentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // default_value: optional message envoy.type.v3.Percent
  pub fn has_default_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_default_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn default_value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn default_value_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_default_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl RuntimePercent

impl ::std::ops::Drop for RuntimePercent {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RuntimePercent {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RuntimePercent {
  type Proxied = Self;
  fn as_view(&self) -> RuntimePercentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RuntimePercent {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RuntimePercentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RuntimePercent {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__RuntimePercent_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__RuntimePercent_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__RuntimePercent_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimePercent {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimePercent {
  type Msg = RuntimePercent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimePercent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimePercent {
  type Msg = RuntimePercent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimePercent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimePercentMut<'_> {
  type Msg = RuntimePercent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimePercent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimePercentMut<'_> {
  type Msg = RuntimePercent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimePercent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimePercentView<'_> {
  type Msg = RuntimePercent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimePercent> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimePercentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RuntimeDouble_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RuntimeDouble {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RuntimeDouble>
}

impl ::protobuf::Message for RuntimeDouble {
  type MessageView<'msg> = RuntimeDoubleView<'msg>;
  type MessageMut<'msg> = RuntimeDoubleMut<'msg>;
}

impl ::std::default::Default for RuntimeDouble {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RuntimeDouble {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RuntimeDouble` is `Sync` because it does not implement interior mutability.
//    Neither does `RuntimeDoubleMut`.
unsafe impl ::std::marker::Sync for RuntimeDouble {}

// SAFETY:
// - `RuntimeDouble` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeDouble {}

impl ::protobuf::Proxied for RuntimeDouble {
  type View<'msg> = RuntimeDoubleView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RuntimeDouble {}

impl ::protobuf::MutProxied for RuntimeDouble {
  type Mut<'msg> = RuntimeDoubleMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RuntimeDoubleView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeDouble>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeDoubleView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RuntimeDoubleView<'msg> {
  type Message = RuntimeDouble;
}

impl ::std::fmt::Debug for RuntimeDoubleView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RuntimeDoubleView<'_> {
  fn default() -> RuntimeDoubleView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeDouble>> for RuntimeDoubleView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeDouble>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeDoubleView<'msg> {

  pub fn to_owned(&self) -> RuntimeDouble {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // default_value: optional double
  pub fn default_value(self) -> f64 {
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

  // runtime_key: optional string
  pub fn runtime_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `RuntimeDoubleView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RuntimeDoubleView<'_> {}

// SAFETY:
// - `RuntimeDoubleView` is `Send` because while its alive a `RuntimeDoubleMut` cannot.
// - `RuntimeDoubleView` does not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeDoubleView<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeDoubleView<'msg> {
  type Proxied = RuntimeDouble;
  fn as_view(&self) -> ::protobuf::View<'msg, RuntimeDouble> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeDoubleView<'msg> {
  fn into_view<'shorter>(self) -> RuntimeDoubleView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeDouble> for RuntimeDoubleView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeDouble {
    let mut dst = RuntimeDouble::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeDouble> for RuntimeDoubleMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeDouble {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RuntimeDouble {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeDoubleView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeDoubleMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RuntimeDoubleMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeDouble>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeDoubleMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RuntimeDoubleMut<'msg> {
  type Message = RuntimeDouble;
}

impl ::std::fmt::Debug for RuntimeDoubleMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeDouble>> for RuntimeDoubleMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeDouble>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeDoubleMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeDouble> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RuntimeDouble {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // default_value: optional double
  pub fn default_value(&self) -> f64 {
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
  pub fn set_default_value(&mut self, val: f64) {
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

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `RuntimeDoubleMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RuntimeDoubleMut<'_> {}

// SAFETY:
// - `RuntimeDoubleMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RuntimeDoubleMut<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeDoubleMut<'msg> {
  type Proxied = RuntimeDouble;
  fn as_view(&self) -> ::protobuf::View<'_, RuntimeDouble> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeDoubleMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RuntimeDouble>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RuntimeDoubleMut<'msg> {
  type MutProxied = RuntimeDouble;
  fn as_mut(&mut self) -> RuntimeDoubleMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RuntimeDoubleMut<'msg> {
  fn into_mut<'shorter>(self) -> RuntimeDoubleMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RuntimeDouble {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RuntimeDouble> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RuntimeDoubleView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RuntimeDoubleMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // default_value: optional double
  pub fn default_value(&self) -> f64 {
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
  pub fn set_default_value(&mut self, val: f64) {
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

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl RuntimeDouble

impl ::std::ops::Drop for RuntimeDouble {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RuntimeDouble {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RuntimeDouble {
  type Proxied = Self;
  fn as_view(&self) -> RuntimeDoubleView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RuntimeDouble {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RuntimeDoubleMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RuntimeDouble {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__RuntimeDouble_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__RuntimeDouble_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__RuntimeDouble_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeDouble {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeDouble {
  type Msg = RuntimeDouble;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeDouble> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeDouble {
  type Msg = RuntimeDouble;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeDouble> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeDoubleMut<'_> {
  type Msg = RuntimeDouble;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeDouble> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeDoubleMut<'_> {
  type Msg = RuntimeDouble;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeDouble> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeDoubleView<'_> {
  type Msg = RuntimeDouble;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeDouble> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeDoubleMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RuntimeFeatureFlag_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RuntimeFeatureFlag {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RuntimeFeatureFlag>
}

impl ::protobuf::Message for RuntimeFeatureFlag {
  type MessageView<'msg> = RuntimeFeatureFlagView<'msg>;
  type MessageMut<'msg> = RuntimeFeatureFlagMut<'msg>;
}

impl ::std::default::Default for RuntimeFeatureFlag {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RuntimeFeatureFlag {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RuntimeFeatureFlag` is `Sync` because it does not implement interior mutability.
//    Neither does `RuntimeFeatureFlagMut`.
unsafe impl ::std::marker::Sync for RuntimeFeatureFlag {}

// SAFETY:
// - `RuntimeFeatureFlag` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeFeatureFlag {}

impl ::protobuf::Proxied for RuntimeFeatureFlag {
  type View<'msg> = RuntimeFeatureFlagView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RuntimeFeatureFlag {}

impl ::protobuf::MutProxied for RuntimeFeatureFlag {
  type Mut<'msg> = RuntimeFeatureFlagMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RuntimeFeatureFlagView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeFeatureFlag>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeFeatureFlagView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RuntimeFeatureFlagView<'msg> {
  type Message = RuntimeFeatureFlag;
}

impl ::std::fmt::Debug for RuntimeFeatureFlagView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RuntimeFeatureFlagView<'_> {
  fn default() -> RuntimeFeatureFlagView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeFeatureFlag>> for RuntimeFeatureFlagView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeFeatureFlag>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeFeatureFlagView<'msg> {

  pub fn to_owned(&self) -> RuntimeFeatureFlag {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // default_value: optional message google.protobuf.BoolValue
  pub fn has_default_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn default_value_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // runtime_key: optional string
  pub fn runtime_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `RuntimeFeatureFlagView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RuntimeFeatureFlagView<'_> {}

// SAFETY:
// - `RuntimeFeatureFlagView` is `Send` because while its alive a `RuntimeFeatureFlagMut` cannot.
// - `RuntimeFeatureFlagView` does not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeFeatureFlagView<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeFeatureFlagView<'msg> {
  type Proxied = RuntimeFeatureFlag;
  fn as_view(&self) -> ::protobuf::View<'msg, RuntimeFeatureFlag> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeFeatureFlagView<'msg> {
  fn into_view<'shorter>(self) -> RuntimeFeatureFlagView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeFeatureFlag> for RuntimeFeatureFlagView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeFeatureFlag {
    let mut dst = RuntimeFeatureFlag::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeFeatureFlag> for RuntimeFeatureFlagMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeFeatureFlag {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RuntimeFeatureFlag {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeFeatureFlagView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeFeatureFlagMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RuntimeFeatureFlagMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFeatureFlag>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeFeatureFlagMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RuntimeFeatureFlagMut<'msg> {
  type Message = RuntimeFeatureFlag;
}

impl ::std::fmt::Debug for RuntimeFeatureFlagMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFeatureFlag>> for RuntimeFeatureFlagMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFeatureFlag>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeFeatureFlagMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFeatureFlag> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RuntimeFeatureFlag {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // default_value: optional message google.protobuf.BoolValue
  pub fn has_default_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_default_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn default_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn default_value_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_default_value(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `RuntimeFeatureFlagMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RuntimeFeatureFlagMut<'_> {}

// SAFETY:
// - `RuntimeFeatureFlagMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RuntimeFeatureFlagMut<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeFeatureFlagMut<'msg> {
  type Proxied = RuntimeFeatureFlag;
  fn as_view(&self) -> ::protobuf::View<'_, RuntimeFeatureFlag> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeFeatureFlagMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RuntimeFeatureFlag>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RuntimeFeatureFlagMut<'msg> {
  type MutProxied = RuntimeFeatureFlag;
  fn as_mut(&mut self) -> RuntimeFeatureFlagMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RuntimeFeatureFlagMut<'msg> {
  fn into_mut<'shorter>(self) -> RuntimeFeatureFlagMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RuntimeFeatureFlag {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RuntimeFeatureFlag> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RuntimeFeatureFlagView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RuntimeFeatureFlagMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // default_value: optional message google.protobuf.BoolValue
  pub fn has_default_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_default_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn default_value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn default_value_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_default_value(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl RuntimeFeatureFlag

impl ::std::ops::Drop for RuntimeFeatureFlag {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RuntimeFeatureFlag {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RuntimeFeatureFlag {
  type Proxied = Self;
  fn as_view(&self) -> RuntimeFeatureFlagView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RuntimeFeatureFlag {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RuntimeFeatureFlagMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RuntimeFeatureFlag {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__RuntimeFeatureFlag_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__RuntimeFeatureFlag_msg_init.0, &[<::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__RuntimeFeatureFlag_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeFeatureFlag {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeFeatureFlag {
  type Msg = RuntimeFeatureFlag;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFeatureFlag> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeFeatureFlag {
  type Msg = RuntimeFeatureFlag;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFeatureFlag> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeFeatureFlagMut<'_> {
  type Msg = RuntimeFeatureFlag;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFeatureFlag> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeFeatureFlagMut<'_> {
  type Msg = RuntimeFeatureFlag;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFeatureFlag> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeFeatureFlagView<'_> {
  type Msg = RuntimeFeatureFlag;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFeatureFlag> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeFeatureFlagMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__KeyValue_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct KeyValue {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<KeyValue>
}

impl ::protobuf::Message for KeyValue {
  type MessageView<'msg> = KeyValueView<'msg>;
  type MessageMut<'msg> = KeyValueMut<'msg>;
}

impl ::std::default::Default for KeyValue {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for KeyValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `KeyValue` is `Sync` because it does not implement interior mutability.
//    Neither does `KeyValueMut`.
unsafe impl ::std::marker::Sync for KeyValue {}

// SAFETY:
// - `KeyValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for KeyValue {}

impl ::protobuf::Proxied for KeyValue {
  type View<'msg> = KeyValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for KeyValue {}

impl ::protobuf::MutProxied for KeyValue {
  type Mut<'msg> = KeyValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct KeyValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for KeyValueView<'msg> {
  type Message = KeyValue;
}

impl ::std::fmt::Debug for KeyValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for KeyValueView<'_> {
  fn default() -> KeyValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValue>> for KeyValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyValueView<'msg> {

  pub fn to_owned(&self) -> KeyValue {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional string
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // value: optional bytes
  pub fn value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `KeyValueView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for KeyValueView<'_> {}

// SAFETY:
// - `KeyValueView` is `Send` because while its alive a `KeyValueMut` cannot.
// - `KeyValueView` does not use thread-local data.
unsafe impl ::std::marker::Send for KeyValueView<'_> {}

impl<'msg> ::protobuf::AsView for KeyValueView<'msg> {
  type Proxied = KeyValue;
  fn as_view(&self) -> ::protobuf::View<'msg, KeyValue> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyValueView<'msg> {
  fn into_view<'shorter>(self) -> KeyValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<KeyValue> for KeyValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeyValue {
    let mut dst = KeyValue::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<KeyValue> for KeyValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeyValue {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for KeyValue {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyValueView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyValueMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct KeyValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for KeyValueMut<'msg> {
  type Message = KeyValue;
}

impl ::std::fmt::Debug for KeyValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValue>> for KeyValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValue> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> KeyValue {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional bytes
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `KeyValueMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for KeyValueMut<'_> {}

// SAFETY:
// - `KeyValueMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for KeyValueMut<'_> {}

impl<'msg> ::protobuf::AsView for KeyValueMut<'msg> {
  type Proxied = KeyValue;
  fn as_view(&self) -> ::protobuf::View<'_, KeyValue> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, KeyValue>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for KeyValueMut<'msg> {
  type MutProxied = KeyValue;
  fn as_mut(&mut self) -> KeyValueMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for KeyValueMut<'msg> {
  fn into_mut<'shorter>(self) -> KeyValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl KeyValue {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, KeyValue> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> KeyValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> KeyValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional bytes
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl KeyValue

impl ::std::ops::Drop for KeyValue {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for KeyValue {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for KeyValue {
  type Proxied = Self;
  fn as_view(&self) -> KeyValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for KeyValue {
  type MutProxied = Self;
  fn as_mut(&mut self) -> KeyValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for KeyValue {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__KeyValue_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__KeyValue_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__KeyValue_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeyValue {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeyValue {
  type Msg = KeyValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValue {
  type Msg = KeyValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeyValueMut<'_> {
  type Msg = KeyValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValueMut<'_> {
  type Msg = KeyValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValueView<'_> {
  type Msg = KeyValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValue> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeyValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__KeyValuePair_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct KeyValuePair {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<KeyValuePair>
}

impl ::protobuf::Message for KeyValuePair {
  type MessageView<'msg> = KeyValuePairView<'msg>;
  type MessageMut<'msg> = KeyValuePairMut<'msg>;
}

impl ::std::default::Default for KeyValuePair {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for KeyValuePair {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `KeyValuePair` is `Sync` because it does not implement interior mutability.
//    Neither does `KeyValuePairMut`.
unsafe impl ::std::marker::Sync for KeyValuePair {}

// SAFETY:
// - `KeyValuePair` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for KeyValuePair {}

impl ::protobuf::Proxied for KeyValuePair {
  type View<'msg> = KeyValuePairView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for KeyValuePair {}

impl ::protobuf::MutProxied for KeyValuePair {
  type Mut<'msg> = KeyValuePairMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct KeyValuePairView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValuePair>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyValuePairView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for KeyValuePairView<'msg> {
  type Message = KeyValuePair;
}

impl ::std::fmt::Debug for KeyValuePairView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for KeyValuePairView<'_> {
  fn default() -> KeyValuePairView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValuePair>> for KeyValuePairView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValuePair>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyValuePairView<'msg> {

  pub fn to_owned(&self) -> KeyValuePair {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional string
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // value: optional message google.protobuf.Value
  pub fn has_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_opt(self) -> ::std::option::Option<::protobuf_well_known_types::ValueView<'msg>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(self) -> ::protobuf_well_known_types::ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::ValueView::default())
  }

}

// SAFETY:
// - `KeyValuePairView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for KeyValuePairView<'_> {}

// SAFETY:
// - `KeyValuePairView` is `Send` because while its alive a `KeyValuePairMut` cannot.
// - `KeyValuePairView` does not use thread-local data.
unsafe impl ::std::marker::Send for KeyValuePairView<'_> {}

impl<'msg> ::protobuf::AsView for KeyValuePairView<'msg> {
  type Proxied = KeyValuePair;
  fn as_view(&self) -> ::protobuf::View<'msg, KeyValuePair> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyValuePairView<'msg> {
  fn into_view<'shorter>(self) -> KeyValuePairView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<KeyValuePair> for KeyValuePairView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeyValuePair {
    let mut dst = KeyValuePair::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<KeyValuePair> for KeyValuePairMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeyValuePair {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for KeyValuePair {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyValuePairView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyValuePairMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct KeyValuePairMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValuePair>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyValuePairMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for KeyValuePairMut<'msg> {
  type Message = KeyValuePair;
}

impl ::std::fmt::Debug for KeyValuePairMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValuePair>> for KeyValuePairMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValuePair>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyValuePairMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValuePair> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> KeyValuePair {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional message google.protobuf.Value
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
  pub fn value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::ValueView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf_well_known_types::ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::ValueView::default())
  }
  pub fn value_mut(&mut self) -> ::protobuf_well_known_types::ValueMut<'_> {
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
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Value>) {

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
// - `KeyValuePairMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for KeyValuePairMut<'_> {}

// SAFETY:
// - `KeyValuePairMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for KeyValuePairMut<'_> {}

impl<'msg> ::protobuf::AsView for KeyValuePairMut<'msg> {
  type Proxied = KeyValuePair;
  fn as_view(&self) -> ::protobuf::View<'_, KeyValuePair> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyValuePairMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, KeyValuePair>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for KeyValuePairMut<'msg> {
  type MutProxied = KeyValuePair;
  fn as_mut(&mut self) -> KeyValuePairMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for KeyValuePairMut<'msg> {
  fn into_mut<'shorter>(self) -> KeyValuePairMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl KeyValuePair {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, KeyValuePair> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> KeyValuePairView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> KeyValuePairMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional message google.protobuf.Value
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
  pub fn value_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::ValueView<'_>> {
    self.has_value().then(|| self.value())
  }
  pub fn value(&self) -> ::protobuf_well_known_types::ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::ValueView::default())
  }
  pub fn value_mut(&mut self) -> ::protobuf_well_known_types::ValueMut<'_> {
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
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl KeyValuePair

impl ::std::ops::Drop for KeyValuePair {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for KeyValuePair {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for KeyValuePair {
  type Proxied = Self;
  fn as_view(&self) -> KeyValuePairView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for KeyValuePair {
  type MutProxied = Self;
  fn as_mut(&mut self) -> KeyValuePairMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for KeyValuePair {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__KeyValuePair_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__KeyValuePair_msg_init.0, &[<::protobuf_well_known_types::Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__KeyValuePair_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeyValuePair {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeyValuePair {
  type Msg = KeyValuePair;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValuePair> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValuePair {
  type Msg = KeyValuePair;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValuePair> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeyValuePairMut<'_> {
  type Msg = KeyValuePair;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValuePair> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValuePairMut<'_> {
  type Msg = KeyValuePair;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValuePair> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValuePairView<'_> {
  type Msg = KeyValuePair;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValuePair> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeyValuePairMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__KeyValueAppend_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct KeyValueAppend {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<KeyValueAppend>
}

impl ::protobuf::Message for KeyValueAppend {
  type MessageView<'msg> = KeyValueAppendView<'msg>;
  type MessageMut<'msg> = KeyValueAppendMut<'msg>;
}

impl ::std::default::Default for KeyValueAppend {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for KeyValueAppend {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `KeyValueAppend` is `Sync` because it does not implement interior mutability.
//    Neither does `KeyValueAppendMut`.
unsafe impl ::std::marker::Sync for KeyValueAppend {}

// SAFETY:
// - `KeyValueAppend` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for KeyValueAppend {}

impl ::protobuf::Proxied for KeyValueAppend {
  type View<'msg> = KeyValueAppendView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for KeyValueAppend {}

impl ::protobuf::MutProxied for KeyValueAppend {
  type Mut<'msg> = KeyValueAppendMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct KeyValueAppendView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValueAppend>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyValueAppendView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for KeyValueAppendView<'msg> {
  type Message = KeyValueAppend;
}

impl ::std::fmt::Debug for KeyValueAppendView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for KeyValueAppendView<'_> {
  fn default() -> KeyValueAppendView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValueAppend>> for KeyValueAppendView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValueAppend>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyValueAppendView<'msg> {

  pub fn to_owned(&self) -> KeyValueAppend {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // record: optional message envoy.config.core.v3.KeyValuePair
  pub fn has_record(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn record_opt(self) -> ::std::option::Option<super::KeyValuePairView<'msg>> {
    self.has_record().then(|| self.record())
  }
  pub fn record(self) -> super::KeyValuePairView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeyValuePairView::default())
  }

  // entry: optional message envoy.config.core.v3.KeyValue
  pub fn has_entry(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn entry_opt(self) -> ::std::option::Option<super::KeyValueView<'msg>> {
    self.has_entry().then(|| self.entry())
  }
  pub fn entry(self) -> super::KeyValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeyValueView::default())
  }

  // action: optional enum envoy.config.core.v3.KeyValueAppend.KeyValueAppendAction
  pub fn action(self) -> super::key_value_append::KeyValueAppendAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::key_value_append::KeyValueAppendAction::AppendIfExistsOrAdd).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `KeyValueAppendView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for KeyValueAppendView<'_> {}

// SAFETY:
// - `KeyValueAppendView` is `Send` because while its alive a `KeyValueAppendMut` cannot.
// - `KeyValueAppendView` does not use thread-local data.
unsafe impl ::std::marker::Send for KeyValueAppendView<'_> {}

impl<'msg> ::protobuf::AsView for KeyValueAppendView<'msg> {
  type Proxied = KeyValueAppend;
  fn as_view(&self) -> ::protobuf::View<'msg, KeyValueAppend> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyValueAppendView<'msg> {
  fn into_view<'shorter>(self) -> KeyValueAppendView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<KeyValueAppend> for KeyValueAppendView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeyValueAppend {
    let mut dst = KeyValueAppend::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<KeyValueAppend> for KeyValueAppendMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeyValueAppend {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for KeyValueAppend {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyValueAppendView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyValueAppendMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct KeyValueAppendMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValueAppend>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyValueAppendMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for KeyValueAppendMut<'msg> {
  type Message = KeyValueAppend;
}

impl ::std::fmt::Debug for KeyValueAppendMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValueAppend>> for KeyValueAppendMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValueAppend>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyValueAppendMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValueAppend> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> KeyValueAppend {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // record: optional message envoy.config.core.v3.KeyValuePair
  pub fn has_record(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_record(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn record_opt(&self) -> ::std::option::Option<super::KeyValuePairView<'_>> {
    self.has_record().then(|| self.record())
  }
  pub fn record(&self) -> super::KeyValuePairView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeyValuePairView::default())
  }
  pub fn record_mut(&mut self) -> super::KeyValuePairMut<'_> {
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
  pub fn set_record(&mut self,
    val: impl ::protobuf::IntoProxied<super::KeyValuePair>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // entry: optional message envoy.config.core.v3.KeyValue
  pub fn has_entry(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entry(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entry_opt(&self) -> ::std::option::Option<super::KeyValueView<'_>> {
    self.has_entry().then(|| self.entry())
  }
  pub fn entry(&self) -> super::KeyValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeyValueView::default())
  }
  pub fn entry_mut(&mut self) -> super::KeyValueMut<'_> {
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
  pub fn set_entry(&mut self,
    val: impl ::protobuf::IntoProxied<super::KeyValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // action: optional enum envoy.config.core.v3.KeyValueAppend.KeyValueAppendAction
  pub fn action(&self) -> super::key_value_append::KeyValueAppendAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::key_value_append::KeyValueAppendAction::AppendIfExistsOrAdd).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_action(&mut self, val: super::key_value_append::KeyValueAppendAction) {
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
// - `KeyValueAppendMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for KeyValueAppendMut<'_> {}

// SAFETY:
// - `KeyValueAppendMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for KeyValueAppendMut<'_> {}

impl<'msg> ::protobuf::AsView for KeyValueAppendMut<'msg> {
  type Proxied = KeyValueAppend;
  fn as_view(&self) -> ::protobuf::View<'_, KeyValueAppend> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyValueAppendMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, KeyValueAppend>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for KeyValueAppendMut<'msg> {
  type MutProxied = KeyValueAppend;
  fn as_mut(&mut self) -> KeyValueAppendMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for KeyValueAppendMut<'msg> {
  fn into_mut<'shorter>(self) -> KeyValueAppendMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl KeyValueAppend {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, KeyValueAppend> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> KeyValueAppendView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> KeyValueAppendMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // record: optional message envoy.config.core.v3.KeyValuePair
  pub fn has_record(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_record(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn record_opt(&self) -> ::std::option::Option<super::KeyValuePairView<'_>> {
    self.has_record().then(|| self.record())
  }
  pub fn record(&self) -> super::KeyValuePairView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeyValuePairView::default())
  }
  pub fn record_mut(&mut self) -> super::KeyValuePairMut<'_> {
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
  pub fn set_record(&mut self,
    val: impl ::protobuf::IntoProxied<super::KeyValuePair>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // entry: optional message envoy.config.core.v3.KeyValue
  pub fn has_entry(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entry(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entry_opt(&self) -> ::std::option::Option<super::KeyValueView<'_>> {
    self.has_entry().then(|| self.entry())
  }
  pub fn entry(&self) -> super::KeyValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeyValueView::default())
  }
  pub fn entry_mut(&mut self) -> super::KeyValueMut<'_> {
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
  pub fn set_entry(&mut self,
    val: impl ::protobuf::IntoProxied<super::KeyValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // action: optional enum envoy.config.core.v3.KeyValueAppend.KeyValueAppendAction
  pub fn action(&self) -> super::key_value_append::KeyValueAppendAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::key_value_append::KeyValueAppendAction::AppendIfExistsOrAdd).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_action(&mut self, val: super::key_value_append::KeyValueAppendAction) {
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

}  // impl KeyValueAppend

impl ::std::ops::Drop for KeyValueAppend {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for KeyValueAppend {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for KeyValueAppend {
  type Proxied = Self;
  fn as_view(&self) -> KeyValueAppendView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for KeyValueAppend {
  type MutProxied = Self;
  fn as_mut(&mut self) -> KeyValueAppendMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for KeyValueAppend {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__KeyValueAppend_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3.P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__KeyValueAppend_msg_init.0, &[<super::KeyValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::KeyValuePair as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__KeyValueAppend_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeyValueAppend {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeyValueAppend {
  type Msg = KeyValueAppend;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueAppend> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValueAppend {
  type Msg = KeyValueAppend;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueAppend> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeyValueAppendMut<'_> {
  type Msg = KeyValueAppend;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueAppend> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValueAppendMut<'_> {
  type Msg = KeyValueAppend;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueAppend> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValueAppendView<'_> {
  type Msg = KeyValueAppend;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueAppend> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeyValueAppendMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod key_value_append {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeyValueAppendAction(i32);

#[allow(non_upper_case_globals)]
impl KeyValueAppendAction {
  pub const AppendIfExistsOrAdd: KeyValueAppendAction = KeyValueAppendAction(0);
  pub const AddIfAbsent: KeyValueAppendAction = KeyValueAppendAction(1);
  pub const OverwriteIfExistsOrAdd: KeyValueAppendAction = KeyValueAppendAction(2);
  pub const OverwriteIfExists: KeyValueAppendAction = KeyValueAppendAction(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "AppendIfExistsOrAdd",
      1 => "AddIfAbsent",
      2 => "OverwriteIfExistsOrAdd",
      3 => "OverwriteIfExists",
      _ => return None
    })
  }
}

impl ::std::convert::From<KeyValueAppendAction> for i32 {
  fn from(val: KeyValueAppendAction) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for KeyValueAppendAction {
  fn from(val: i32) -> KeyValueAppendAction {
    Self(val)
  }
}

impl ::std::default::Default for KeyValueAppendAction {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for KeyValueAppendAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "KeyValueAppendAction::{}", constant_name)
    } else {
      write!(f, "KeyValueAppendAction::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for KeyValueAppendAction {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for KeyValueAppendAction {}

impl ::protobuf::Proxied for KeyValueAppendAction {
  type View<'a> = KeyValueAppendAction;
}

impl ::protobuf::AsView for KeyValueAppendAction {
  type Proxied = KeyValueAppendAction;

  fn as_view(&self) -> KeyValueAppendAction {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyValueAppendAction {
  fn into_view<'shorter>(self) -> KeyValueAppendAction where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for KeyValueAppendAction {
  const NAME: &'static str = "KeyValueAppendAction";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for KeyValueAppendAction {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod key_value_append


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__KeyValueMutation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct KeyValueMutation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<KeyValueMutation>
}

impl ::protobuf::Message for KeyValueMutation {
  type MessageView<'msg> = KeyValueMutationView<'msg>;
  type MessageMut<'msg> = KeyValueMutationMut<'msg>;
}

impl ::std::default::Default for KeyValueMutation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for KeyValueMutation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `KeyValueMutation` is `Sync` because it does not implement interior mutability.
//    Neither does `KeyValueMutationMut`.
unsafe impl ::std::marker::Sync for KeyValueMutation {}

// SAFETY:
// - `KeyValueMutation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for KeyValueMutation {}

impl ::protobuf::Proxied for KeyValueMutation {
  type View<'msg> = KeyValueMutationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for KeyValueMutation {}

impl ::protobuf::MutProxied for KeyValueMutation {
  type Mut<'msg> = KeyValueMutationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct KeyValueMutationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValueMutation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyValueMutationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for KeyValueMutationView<'msg> {
  type Message = KeyValueMutation;
}

impl ::std::fmt::Debug for KeyValueMutationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for KeyValueMutationView<'_> {
  fn default() -> KeyValueMutationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValueMutation>> for KeyValueMutationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, KeyValueMutation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyValueMutationView<'msg> {

  pub fn to_owned(&self) -> KeyValueMutation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // append: optional message envoy.config.core.v3.KeyValueAppend
  pub fn has_append(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn append_opt(self) -> ::std::option::Option<super::KeyValueAppendView<'msg>> {
    self.has_append().then(|| self.append())
  }
  pub fn append(self) -> super::KeyValueAppendView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeyValueAppendView::default())
  }

  // remove: optional string
  pub fn remove(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `KeyValueMutationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for KeyValueMutationView<'_> {}

// SAFETY:
// - `KeyValueMutationView` is `Send` because while its alive a `KeyValueMutationMut` cannot.
// - `KeyValueMutationView` does not use thread-local data.
unsafe impl ::std::marker::Send for KeyValueMutationView<'_> {}

impl<'msg> ::protobuf::AsView for KeyValueMutationView<'msg> {
  type Proxied = KeyValueMutation;
  fn as_view(&self) -> ::protobuf::View<'msg, KeyValueMutation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyValueMutationView<'msg> {
  fn into_view<'shorter>(self) -> KeyValueMutationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<KeyValueMutation> for KeyValueMutationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeyValueMutation {
    let mut dst = KeyValueMutation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<KeyValueMutation> for KeyValueMutationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> KeyValueMutation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for KeyValueMutation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyValueMutationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyValueMutationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct KeyValueMutationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValueMutation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyValueMutationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for KeyValueMutationMut<'msg> {
  type Message = KeyValueMutation;
}

impl ::std::fmt::Debug for KeyValueMutationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValueMutation>> for KeyValueMutationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValueMutation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyValueMutationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, KeyValueMutation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> KeyValueMutation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // append: optional message envoy.config.core.v3.KeyValueAppend
  pub fn has_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn append_opt(&self) -> ::std::option::Option<super::KeyValueAppendView<'_>> {
    self.has_append().then(|| self.append())
  }
  pub fn append(&self) -> super::KeyValueAppendView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeyValueAppendView::default())
  }
  pub fn append_mut(&mut self) -> super::KeyValueAppendMut<'_> {
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
  pub fn set_append(&mut self,
    val: impl ::protobuf::IntoProxied<super::KeyValueAppend>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // remove: optional string
  pub fn remove(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_remove(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `KeyValueMutationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for KeyValueMutationMut<'_> {}

// SAFETY:
// - `KeyValueMutationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for KeyValueMutationMut<'_> {}

impl<'msg> ::protobuf::AsView for KeyValueMutationMut<'msg> {
  type Proxied = KeyValueMutation;
  fn as_view(&self) -> ::protobuf::View<'_, KeyValueMutation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyValueMutationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, KeyValueMutation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for KeyValueMutationMut<'msg> {
  type MutProxied = KeyValueMutation;
  fn as_mut(&mut self) -> KeyValueMutationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for KeyValueMutationMut<'msg> {
  fn into_mut<'shorter>(self) -> KeyValueMutationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl KeyValueMutation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, KeyValueMutation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> KeyValueMutationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> KeyValueMutationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // append: optional message envoy.config.core.v3.KeyValueAppend
  pub fn has_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn append_opt(&self) -> ::std::option::Option<super::KeyValueAppendView<'_>> {
    self.has_append().then(|| self.append())
  }
  pub fn append(&self) -> super::KeyValueAppendView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::KeyValueAppendView::default())
  }
  pub fn append_mut(&mut self) -> super::KeyValueAppendMut<'_> {
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
  pub fn set_append(&mut self,
    val: impl ::protobuf::IntoProxied<super::KeyValueAppend>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // remove: optional string
  pub fn remove(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_remove(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl KeyValueMutation

impl ::std::ops::Drop for KeyValueMutation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for KeyValueMutation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for KeyValueMutation {
  type Proxied = Self;
  fn as_view(&self) -> KeyValueMutationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for KeyValueMutation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> KeyValueMutationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for KeyValueMutation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__KeyValueMutation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__KeyValueMutation_msg_init.0, &[<super::KeyValueAppend as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__KeyValueMutation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeyValueMutation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeyValueMutation {
  type Msg = KeyValueMutation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueMutation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValueMutation {
  type Msg = KeyValueMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueMutation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeyValueMutationMut<'_> {
  type Msg = KeyValueMutation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueMutation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValueMutationMut<'_> {
  type Msg = KeyValueMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueMutation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyValueMutationView<'_> {
  type Msg = KeyValueMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<KeyValueMutation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeyValueMutationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__QueryParameter_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct QueryParameter {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<QueryParameter>
}

impl ::protobuf::Message for QueryParameter {
  type MessageView<'msg> = QueryParameterView<'msg>;
  type MessageMut<'msg> = QueryParameterMut<'msg>;
}

impl ::std::default::Default for QueryParameter {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for QueryParameter {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `QueryParameter` is `Sync` because it does not implement interior mutability.
//    Neither does `QueryParameterMut`.
unsafe impl ::std::marker::Sync for QueryParameter {}

// SAFETY:
// - `QueryParameter` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for QueryParameter {}

impl ::protobuf::Proxied for QueryParameter {
  type View<'msg> = QueryParameterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for QueryParameter {}

impl ::protobuf::MutProxied for QueryParameter {
  type Mut<'msg> = QueryParameterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct QueryParameterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QueryParameter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QueryParameterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for QueryParameterView<'msg> {
  type Message = QueryParameter;
}

impl ::std::fmt::Debug for QueryParameterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for QueryParameterView<'_> {
  fn default() -> QueryParameterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, QueryParameter>> for QueryParameterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QueryParameter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QueryParameterView<'msg> {

  pub fn to_owned(&self) -> QueryParameter {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional string
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // value: optional string
  pub fn value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `QueryParameterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for QueryParameterView<'_> {}

// SAFETY:
// - `QueryParameterView` is `Send` because while its alive a `QueryParameterMut` cannot.
// - `QueryParameterView` does not use thread-local data.
unsafe impl ::std::marker::Send for QueryParameterView<'_> {}

impl<'msg> ::protobuf::AsView for QueryParameterView<'msg> {
  type Proxied = QueryParameter;
  fn as_view(&self) -> ::protobuf::View<'msg, QueryParameter> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QueryParameterView<'msg> {
  fn into_view<'shorter>(self) -> QueryParameterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<QueryParameter> for QueryParameterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QueryParameter {
    let mut dst = QueryParameter::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<QueryParameter> for QueryParameterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QueryParameter {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for QueryParameter {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QueryParameterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QueryParameterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct QueryParameterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QueryParameter>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QueryParameterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for QueryParameterMut<'msg> {
  type Message = QueryParameter;
}

impl ::std::fmt::Debug for QueryParameterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, QueryParameter>> for QueryParameterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QueryParameter>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QueryParameterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, QueryParameter> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> QueryParameter {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional string
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `QueryParameterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for QueryParameterMut<'_> {}

// SAFETY:
// - `QueryParameterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for QueryParameterMut<'_> {}

impl<'msg> ::protobuf::AsView for QueryParameterMut<'msg> {
  type Proxied = QueryParameter;
  fn as_view(&self) -> ::protobuf::View<'_, QueryParameter> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QueryParameterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, QueryParameter>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for QueryParameterMut<'msg> {
  type MutProxied = QueryParameter;
  fn as_mut(&mut self) -> QueryParameterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for QueryParameterMut<'msg> {
  fn into_mut<'shorter>(self) -> QueryParameterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl QueryParameter {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, QueryParameter> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> QueryParameterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> QueryParameterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional string
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl QueryParameter

impl ::std::ops::Drop for QueryParameter {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for QueryParameter {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for QueryParameter {
  type Proxied = Self;
  fn as_view(&self) -> QueryParameterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for QueryParameter {
  type MutProxied = Self;
  fn as_mut(&mut self) -> QueryParameterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for QueryParameter {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__QueryParameter_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__QueryParameter_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__QueryParameter_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QueryParameter {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QueryParameter {
  type Msg = QueryParameter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QueryParameter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QueryParameter {
  type Msg = QueryParameter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QueryParameter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QueryParameterMut<'_> {
  type Msg = QueryParameter;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QueryParameter> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QueryParameterMut<'_> {
  type Msg = QueryParameter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QueryParameter> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QueryParameterView<'_> {
  type Msg = QueryParameter;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QueryParameter> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QueryParameterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HeaderValue_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderValue {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderValue>
}

impl ::protobuf::Message for HeaderValue {
  type MessageView<'msg> = HeaderValueView<'msg>;
  type MessageMut<'msg> = HeaderValueMut<'msg>;
}

impl ::std::default::Default for HeaderValue {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderValue` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderValueMut`.
unsafe impl ::std::marker::Sync for HeaderValue {}

// SAFETY:
// - `HeaderValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderValue {}

impl ::protobuf::Proxied for HeaderValue {
  type View<'msg> = HeaderValueView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderValue {}

impl ::protobuf::MutProxied for HeaderValue {
  type Mut<'msg> = HeaderValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderValueView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderValueView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderValueView<'msg> {
  type Message = HeaderValue;
}

impl ::std::fmt::Debug for HeaderValueView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderValueView<'_> {
  fn default() -> HeaderValueView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderValue>> for HeaderValueView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderValueView<'msg> {

  pub fn to_owned(&self) -> HeaderValue {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key: optional string
  pub fn key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // value: optional string
  pub fn value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // raw_value: optional bytes
  pub fn raw_value(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `HeaderValueView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderValueView<'_> {}

// SAFETY:
// - `HeaderValueView` is `Send` because while its alive a `HeaderValueMut` cannot.
// - `HeaderValueView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderValueView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderValueView<'msg> {
  type Proxied = HeaderValue;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderValue> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderValueView<'msg> {
  fn into_view<'shorter>(self) -> HeaderValueView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderValue> for HeaderValueView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderValue {
    let mut dst = HeaderValue::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderValue> for HeaderValueMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderValue {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderValue {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderValueView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderValueMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderValueMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValue>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderValueMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderValueMut<'msg> {
  type Message = HeaderValue;
}

impl ::std::fmt::Debug for HeaderValueMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValue>> for HeaderValueMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValue>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderValueMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValue> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderValue {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional string
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // raw_value: optional bytes
  pub fn raw_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_raw_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `HeaderValueMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderValueMut<'_> {}

// SAFETY:
// - `HeaderValueMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderValueMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderValueMut<'msg> {
  type Proxied = HeaderValue;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderValue> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderValueMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderValue>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderValueMut<'msg> {
  type MutProxied = HeaderValue;
  fn as_mut(&mut self) -> HeaderValueMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderValueMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderValueMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderValue {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderValue> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderValueView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderValueMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key: optional string
  pub fn key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // value: optional string
  pub fn value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // raw_value: optional bytes
  pub fn raw_value(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_raw_value(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl HeaderValue

impl ::std::ops::Drop for HeaderValue {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderValue {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderValue {
  type Proxied = Self;
  fn as_view(&self) -> HeaderValueView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderValue {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderValueMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderValue {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__HeaderValue_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__HeaderValue_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__HeaderValue_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderValue {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderValue {
  type Msg = HeaderValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderValue {
  type Msg = HeaderValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderValueMut<'_> {
  type Msg = HeaderValue;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValue> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderValueMut<'_> {
  type Msg = HeaderValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValue> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderValueView<'_> {
  type Msg = HeaderValue;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValue> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderValueMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HeaderValueOption_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderValueOption {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderValueOption>
}

impl ::protobuf::Message for HeaderValueOption {
  type MessageView<'msg> = HeaderValueOptionView<'msg>;
  type MessageMut<'msg> = HeaderValueOptionMut<'msg>;
}

impl ::std::default::Default for HeaderValueOption {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderValueOption {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderValueOption` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderValueOptionMut`.
unsafe impl ::std::marker::Sync for HeaderValueOption {}

// SAFETY:
// - `HeaderValueOption` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderValueOption {}

impl ::protobuf::Proxied for HeaderValueOption {
  type View<'msg> = HeaderValueOptionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderValueOption {}

impl ::protobuf::MutProxied for HeaderValueOption {
  type Mut<'msg> = HeaderValueOptionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderValueOptionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderValueOption>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderValueOptionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderValueOptionView<'msg> {
  type Message = HeaderValueOption;
}

impl ::std::fmt::Debug for HeaderValueOptionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderValueOptionView<'_> {
  fn default() -> HeaderValueOptionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderValueOption>> for HeaderValueOptionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderValueOption>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderValueOptionView<'msg> {

  pub fn to_owned(&self) -> HeaderValueOption {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // header: optional message envoy.config.core.v3.HeaderValue
  pub fn has_header(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn header_opt(self) -> ::std::option::Option<super::HeaderValueView<'msg>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(self) -> super::HeaderValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderValueView::default())
  }

  // append: optional message google.protobuf.BoolValue
  pub fn has_append(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn append_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_append().then(|| self.append())
  }
  pub fn append(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // append_action: optional enum envoy.config.core.v3.HeaderValueOption.HeaderAppendAction
  pub fn append_action(self) -> super::header_value_option::HeaderAppendAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::header_value_option::HeaderAppendAction::AppendIfExistsOrAdd).into()
      ).try_into().unwrap()
    }
  }

  // keep_empty_value: optional bool
  pub fn keep_empty_value(self) -> bool {
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

}

// SAFETY:
// - `HeaderValueOptionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderValueOptionView<'_> {}

// SAFETY:
// - `HeaderValueOptionView` is `Send` because while its alive a `HeaderValueOptionMut` cannot.
// - `HeaderValueOptionView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderValueOptionView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderValueOptionView<'msg> {
  type Proxied = HeaderValueOption;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderValueOption> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderValueOptionView<'msg> {
  fn into_view<'shorter>(self) -> HeaderValueOptionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderValueOption> for HeaderValueOptionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderValueOption {
    let mut dst = HeaderValueOption::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderValueOption> for HeaderValueOptionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderValueOption {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderValueOption {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderValueOptionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderValueOptionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderValueOptionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValueOption>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderValueOptionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderValueOptionMut<'msg> {
  type Message = HeaderValueOption;
}

impl ::std::fmt::Debug for HeaderValueOptionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValueOption>> for HeaderValueOptionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValueOption>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderValueOptionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderValueOption> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderValueOption {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // header: optional message envoy.config.core.v3.HeaderValue
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<super::HeaderValueView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> super::HeaderValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderValueView::default())
  }
  pub fn header_mut(&mut self) -> super::HeaderValueMut<'_> {
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
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // append: optional message google.protobuf.BoolValue
  pub fn has_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn append_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_append().then(|| self.append())
  }
  pub fn append(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn append_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_append(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // append_action: optional enum envoy.config.core.v3.HeaderValueOption.HeaderAppendAction
  pub fn append_action(&self) -> super::header_value_option::HeaderAppendAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::header_value_option::HeaderAppendAction::AppendIfExistsOrAdd).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_append_action(&mut self, val: super::header_value_option::HeaderAppendAction) {
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

  // keep_empty_value: optional bool
  pub fn keep_empty_value(&self) -> bool {
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
  pub fn set_keep_empty_value(&mut self, val: bool) {
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

}

// SAFETY:
// - `HeaderValueOptionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderValueOptionMut<'_> {}

// SAFETY:
// - `HeaderValueOptionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderValueOptionMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderValueOptionMut<'msg> {
  type Proxied = HeaderValueOption;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderValueOption> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderValueOptionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderValueOption>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderValueOptionMut<'msg> {
  type MutProxied = HeaderValueOption;
  fn as_mut(&mut self) -> HeaderValueOptionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderValueOptionMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderValueOptionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderValueOption {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderValueOption> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderValueOptionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderValueOptionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // header: optional message envoy.config.core.v3.HeaderValue
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<super::HeaderValueView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> super::HeaderValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HeaderValueView::default())
  }
  pub fn header_mut(&mut self) -> super::HeaderValueMut<'_> {
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
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<super::HeaderValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // append: optional message google.protobuf.BoolValue
  pub fn has_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn append_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_append().then(|| self.append())
  }
  pub fn append(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn append_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_append(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // append_action: optional enum envoy.config.core.v3.HeaderValueOption.HeaderAppendAction
  pub fn append_action(&self) -> super::header_value_option::HeaderAppendAction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::header_value_option::HeaderAppendAction::AppendIfExistsOrAdd).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_append_action(&mut self, val: super::header_value_option::HeaderAppendAction) {
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

  // keep_empty_value: optional bool
  pub fn keep_empty_value(&self) -> bool {
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
  pub fn set_keep_empty_value(&mut self, val: bool) {
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

}  // impl HeaderValueOption

impl ::std::ops::Drop for HeaderValueOption {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderValueOption {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderValueOption {
  type Proxied = Self;
  fn as_view(&self) -> HeaderValueOptionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderValueOption {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderValueOptionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderValueOption {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__HeaderValueOption_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33.P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__HeaderValueOption_msg_init.0, &[<super::HeaderValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__HeaderValueOption_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderValueOption {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderValueOption {
  type Msg = HeaderValueOption;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueOption> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderValueOption {
  type Msg = HeaderValueOption;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueOption> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderValueOptionMut<'_> {
  type Msg = HeaderValueOption;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueOption> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderValueOptionMut<'_> {
  type Msg = HeaderValueOption;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueOption> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderValueOptionView<'_> {
  type Msg = HeaderValueOption;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderValueOption> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderValueOptionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod header_value_option {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HeaderAppendAction(i32);

#[allow(non_upper_case_globals)]
impl HeaderAppendAction {
  pub const AppendIfExistsOrAdd: HeaderAppendAction = HeaderAppendAction(0);
  pub const AddIfAbsent: HeaderAppendAction = HeaderAppendAction(1);
  pub const OverwriteIfExistsOrAdd: HeaderAppendAction = HeaderAppendAction(2);
  pub const OverwriteIfExists: HeaderAppendAction = HeaderAppendAction(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "AppendIfExistsOrAdd",
      1 => "AddIfAbsent",
      2 => "OverwriteIfExistsOrAdd",
      3 => "OverwriteIfExists",
      _ => return None
    })
  }
}

impl ::std::convert::From<HeaderAppendAction> for i32 {
  fn from(val: HeaderAppendAction) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for HeaderAppendAction {
  fn from(val: i32) -> HeaderAppendAction {
    Self(val)
  }
}

impl ::std::default::Default for HeaderAppendAction {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for HeaderAppendAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "HeaderAppendAction::{}", constant_name)
    } else {
      write!(f, "HeaderAppendAction::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for HeaderAppendAction {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for HeaderAppendAction {}

impl ::protobuf::Proxied for HeaderAppendAction {
  type View<'a> = HeaderAppendAction;
}

impl ::protobuf::AsView for HeaderAppendAction {
  type Proxied = HeaderAppendAction;

  fn as_view(&self) -> HeaderAppendAction {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderAppendAction {
  fn into_view<'shorter>(self) -> HeaderAppendAction where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for HeaderAppendAction {
  const NAME: &'static str = "HeaderAppendAction";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for HeaderAppendAction {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod header_value_option


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__HeaderMap_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderMap {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderMap>
}

impl ::protobuf::Message for HeaderMap {
  type MessageView<'msg> = HeaderMapView<'msg>;
  type MessageMut<'msg> = HeaderMapMut<'msg>;
}

impl ::std::default::Default for HeaderMap {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderMap {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderMap` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderMapMut`.
unsafe impl ::std::marker::Sync for HeaderMap {}

// SAFETY:
// - `HeaderMap` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderMap {}

impl ::protobuf::Proxied for HeaderMap {
  type View<'msg> = HeaderMapView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderMap {}

impl ::protobuf::MutProxied for HeaderMap {
  type Mut<'msg> = HeaderMapMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderMapView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMap>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderMapView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderMapView<'msg> {
  type Message = HeaderMap;
}

impl ::std::fmt::Debug for HeaderMapView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderMapView<'_> {
  fn default() -> HeaderMapView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMap>> for HeaderMapView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMap>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderMapView<'msg> {

  pub fn to_owned(&self) -> HeaderMap {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // headers: repeated message envoy.config.core.v3.HeaderValue
  pub fn headers(self) -> ::protobuf::RepeatedView<'msg, super::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `HeaderMapView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderMapView<'_> {}

// SAFETY:
// - `HeaderMapView` is `Send` because while its alive a `HeaderMapMut` cannot.
// - `HeaderMapView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderMapView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderMapView<'msg> {
  type Proxied = HeaderMap;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderMap> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderMapView<'msg> {
  fn into_view<'shorter>(self) -> HeaderMapView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderMap> for HeaderMapView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderMap {
    let mut dst = HeaderMap::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderMap> for HeaderMapMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderMap {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderMap {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderMapView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderMapMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderMapMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMap>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderMapMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderMapMut<'msg> {
  type Message = HeaderMap;
}

impl ::std::fmt::Debug for HeaderMapMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMap>> for HeaderMapMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMap>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderMapMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMap> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderMap {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // headers: repeated message envoy.config.core.v3.HeaderValue
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, super::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HeaderValue> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HeaderValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `HeaderMapMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderMapMut<'_> {}

// SAFETY:
// - `HeaderMapMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderMapMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderMapMut<'msg> {
  type Proxied = HeaderMap;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderMap> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderMapMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderMap>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderMapMut<'msg> {
  type MutProxied = HeaderMap;
  fn as_mut(&mut self) -> HeaderMapMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderMapMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderMapMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderMap {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderMap> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderMapView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderMapMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // headers: repeated message envoy.config.core.v3.HeaderValue
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, super::HeaderValue> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::HeaderValue>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::HeaderValue> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::HeaderValue>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl HeaderMap

impl ::std::ops::Drop for HeaderMap {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderMap {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderMap {
  type Proxied = Self;
  fn as_view(&self) -> HeaderMapView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderMap {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderMapMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderMap {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__HeaderMap_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__HeaderMap_msg_init.0, &[<super::HeaderValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__HeaderMap_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderMap {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderMap {
  type Msg = HeaderMap;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMap> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMap {
  type Msg = HeaderMap;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMap> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderMapMut<'_> {
  type Msg = HeaderMap;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMap> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMapMut<'_> {
  type Msg = HeaderMap;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMap> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMapView<'_> {
  type Msg = HeaderMap;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMap> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderMapMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__WatchedDirectory_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct WatchedDirectory {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<WatchedDirectory>
}

impl ::protobuf::Message for WatchedDirectory {
  type MessageView<'msg> = WatchedDirectoryView<'msg>;
  type MessageMut<'msg> = WatchedDirectoryMut<'msg>;
}

impl ::std::default::Default for WatchedDirectory {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for WatchedDirectory {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `WatchedDirectory` is `Sync` because it does not implement interior mutability.
//    Neither does `WatchedDirectoryMut`.
unsafe impl ::std::marker::Sync for WatchedDirectory {}

// SAFETY:
// - `WatchedDirectory` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for WatchedDirectory {}

impl ::protobuf::Proxied for WatchedDirectory {
  type View<'msg> = WatchedDirectoryView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for WatchedDirectory {}

impl ::protobuf::MutProxied for WatchedDirectory {
  type Mut<'msg> = WatchedDirectoryMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WatchedDirectoryView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WatchedDirectory>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WatchedDirectoryView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for WatchedDirectoryView<'msg> {
  type Message = WatchedDirectory;
}

impl ::std::fmt::Debug for WatchedDirectoryView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for WatchedDirectoryView<'_> {
  fn default() -> WatchedDirectoryView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, WatchedDirectory>> for WatchedDirectoryView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WatchedDirectory>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WatchedDirectoryView<'msg> {

  pub fn to_owned(&self) -> WatchedDirectory {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // path: optional string
  pub fn path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `WatchedDirectoryView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for WatchedDirectoryView<'_> {}

// SAFETY:
// - `WatchedDirectoryView` is `Send` because while its alive a `WatchedDirectoryMut` cannot.
// - `WatchedDirectoryView` does not use thread-local data.
unsafe impl ::std::marker::Send for WatchedDirectoryView<'_> {}

impl<'msg> ::protobuf::AsView for WatchedDirectoryView<'msg> {
  type Proxied = WatchedDirectory;
  fn as_view(&self) -> ::protobuf::View<'msg, WatchedDirectory> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WatchedDirectoryView<'msg> {
  fn into_view<'shorter>(self) -> WatchedDirectoryView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<WatchedDirectory> for WatchedDirectoryView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WatchedDirectory {
    let mut dst = WatchedDirectory::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<WatchedDirectory> for WatchedDirectoryMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WatchedDirectory {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for WatchedDirectory {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WatchedDirectoryView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WatchedDirectoryMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WatchedDirectoryMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WatchedDirectory>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WatchedDirectoryMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for WatchedDirectoryMut<'msg> {
  type Message = WatchedDirectory;
}

impl ::std::fmt::Debug for WatchedDirectoryMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, WatchedDirectory>> for WatchedDirectoryMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WatchedDirectory>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WatchedDirectoryMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, WatchedDirectory> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> WatchedDirectory {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `WatchedDirectoryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for WatchedDirectoryMut<'_> {}

// SAFETY:
// - `WatchedDirectoryMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for WatchedDirectoryMut<'_> {}

impl<'msg> ::protobuf::AsView for WatchedDirectoryMut<'msg> {
  type Proxied = WatchedDirectory;
  fn as_view(&self) -> ::protobuf::View<'_, WatchedDirectory> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WatchedDirectoryMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, WatchedDirectory>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for WatchedDirectoryMut<'msg> {
  type MutProxied = WatchedDirectory;
  fn as_mut(&mut self) -> WatchedDirectoryMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for WatchedDirectoryMut<'msg> {
  fn into_mut<'shorter>(self) -> WatchedDirectoryMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl WatchedDirectory {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, WatchedDirectory> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> WatchedDirectoryView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> WatchedDirectoryMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // path: optional string
  pub fn path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl WatchedDirectory

impl ::std::ops::Drop for WatchedDirectory {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for WatchedDirectory {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for WatchedDirectory {
  type Proxied = Self;
  fn as_view(&self) -> WatchedDirectoryView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for WatchedDirectory {
  type MutProxied = Self;
  fn as_mut(&mut self) -> WatchedDirectoryMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for WatchedDirectory {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__WatchedDirectory_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__WatchedDirectory_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__WatchedDirectory_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WatchedDirectory {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WatchedDirectory {
  type Msg = WatchedDirectory;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchedDirectory> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchedDirectory {
  type Msg = WatchedDirectory;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchedDirectory> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WatchedDirectoryMut<'_> {
  type Msg = WatchedDirectory;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchedDirectory> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchedDirectoryMut<'_> {
  type Msg = WatchedDirectory;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchedDirectory> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchedDirectoryView<'_> {
  type Msg = WatchedDirectory;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchedDirectory> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WatchedDirectoryMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__DataSource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DataSource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DataSource>
}

impl ::protobuf::Message for DataSource {
  type MessageView<'msg> = DataSourceView<'msg>;
  type MessageMut<'msg> = DataSourceMut<'msg>;
}

impl ::std::default::Default for DataSource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DataSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DataSource` is `Sync` because it does not implement interior mutability.
//    Neither does `DataSourceMut`.
unsafe impl ::std::marker::Sync for DataSource {}

// SAFETY:
// - `DataSource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DataSource {}

impl ::protobuf::Proxied for DataSource {
  type View<'msg> = DataSourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DataSource {}

impl ::protobuf::MutProxied for DataSource {
  type Mut<'msg> = DataSourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DataSourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DataSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DataSourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DataSourceView<'msg> {
  type Message = DataSource;
}

impl ::std::fmt::Debug for DataSourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DataSourceView<'_> {
  fn default() -> DataSourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DataSource>> for DataSourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DataSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DataSourceView<'msg> {

  pub fn to_owned(&self) -> DataSource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // filename: optional string
  pub fn has_filename(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn filename_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_filename().then(|| self.filename())
  }
  pub fn filename(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // inline_bytes: optional bytes
  pub fn has_inline_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn inline_bytes_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_inline_bytes().then(|| self.inline_bytes())
  }
  pub fn inline_bytes(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // inline_string: optional string
  pub fn has_inline_string(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn inline_string_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_inline_string().then(|| self.inline_string())
  }
  pub fn inline_string(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // environment_variable: optional string
  pub fn has_environment_variable(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn environment_variable_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_environment_variable().then(|| self.environment_variable())
  }
  pub fn environment_variable(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn watched_directory_opt(self) -> ::std::option::Option<super::WatchedDirectoryView<'msg>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(self) -> super::WatchedDirectoryView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchedDirectoryView::default())
  }

  pub fn specifier(self) -> super::data_source::SpecifierOneof<'msg> {
    match self.specifier_case() {
      super::data_source::SpecifierCase::Filename =>
          super::data_source::SpecifierOneof::Filename(self.filename()),
      super::data_source::SpecifierCase::InlineBytes =>
          super::data_source::SpecifierOneof::InlineBytes(self.inline_bytes()),
      super::data_source::SpecifierCase::InlineString =>
          super::data_source::SpecifierOneof::InlineString(self.inline_string()),
      super::data_source::SpecifierCase::EnvironmentVariable =>
          super::data_source::SpecifierOneof::EnvironmentVariable(self.environment_variable()),
      _ => super::data_source::SpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn specifier_case(self) -> super::data_source::SpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::data_source::SpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DataSourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DataSourceView<'_> {}

// SAFETY:
// - `DataSourceView` is `Send` because while its alive a `DataSourceMut` cannot.
// - `DataSourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for DataSourceView<'_> {}

impl<'msg> ::protobuf::AsView for DataSourceView<'msg> {
  type Proxied = DataSource;
  fn as_view(&self) -> ::protobuf::View<'msg, DataSource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DataSourceView<'msg> {
  fn into_view<'shorter>(self) -> DataSourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DataSource> for DataSourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DataSource {
    let mut dst = DataSource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DataSource> for DataSourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DataSource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DataSource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DataSourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DataSourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DataSourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DataSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DataSourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DataSourceMut<'msg> {
  type Message = DataSource;
}

impl ::std::fmt::Debug for DataSourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DataSource>> for DataSourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DataSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DataSourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DataSource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DataSource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // filename: optional string
  pub fn has_filename(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_filename(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn filename_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_filename().then(|| self.filename())
  }
  pub fn filename(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_filename(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // inline_bytes: optional bytes
  pub fn has_inline_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_inline_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn inline_bytes_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_inline_bytes().then(|| self.inline_bytes())
  }
  pub fn inline_bytes(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_inline_bytes(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // inline_string: optional string
  pub fn has_inline_string(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_inline_string(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn inline_string_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_inline_string().then(|| self.inline_string())
  }
  pub fn inline_string(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_inline_string(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // environment_variable: optional string
  pub fn has_environment_variable(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_environment_variable(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn environment_variable_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_environment_variable().then(|| self.environment_variable())
  }
  pub fn environment_variable(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_environment_variable(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_watched_directory(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn watched_directory_opt(&self) -> ::std::option::Option<super::WatchedDirectoryView<'_>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(&self) -> super::WatchedDirectoryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchedDirectoryView::default())
  }
  pub fn watched_directory_mut(&mut self) -> super::WatchedDirectoryMut<'_> {
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
  pub fn set_watched_directory(&mut self,
    val: impl ::protobuf::IntoProxied<super::WatchedDirectory>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn specifier(&self) -> super::data_source::SpecifierOneof<'_> {
    match &self.specifier_case() {
      super::data_source::SpecifierCase::Filename =>
          super::data_source::SpecifierOneof::Filename(self.filename()),
      super::data_source::SpecifierCase::InlineBytes =>
          super::data_source::SpecifierOneof::InlineBytes(self.inline_bytes()),
      super::data_source::SpecifierCase::InlineString =>
          super::data_source::SpecifierOneof::InlineString(self.inline_string()),
      super::data_source::SpecifierCase::EnvironmentVariable =>
          super::data_source::SpecifierOneof::EnvironmentVariable(self.environment_variable()),
      _ => super::data_source::SpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn specifier_case(&self) -> super::data_source::SpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::data_source::SpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DataSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DataSourceMut<'_> {}

// SAFETY:
// - `DataSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DataSourceMut<'_> {}

impl<'msg> ::protobuf::AsView for DataSourceMut<'msg> {
  type Proxied = DataSource;
  fn as_view(&self) -> ::protobuf::View<'_, DataSource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DataSourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DataSource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DataSourceMut<'msg> {
  type MutProxied = DataSource;
  fn as_mut(&mut self) -> DataSourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DataSourceMut<'msg> {
  fn into_mut<'shorter>(self) -> DataSourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DataSource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DataSource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DataSourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DataSourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // filename: optional string
  pub fn has_filename(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_filename(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn filename_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_filename().then(|| self.filename())
  }
  pub fn filename(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_filename(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // inline_bytes: optional bytes
  pub fn has_inline_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_inline_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn inline_bytes_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_inline_bytes().then(|| self.inline_bytes())
  }
  pub fn inline_bytes(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_inline_bytes(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // inline_string: optional string
  pub fn has_inline_string(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_inline_string(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn inline_string_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_inline_string().then(|| self.inline_string())
  }
  pub fn inline_string(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_inline_string(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // environment_variable: optional string
  pub fn has_environment_variable(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_environment_variable(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn environment_variable_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_environment_variable().then(|| self.environment_variable())
  }
  pub fn environment_variable(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_environment_variable(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_watched_directory(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn watched_directory_opt(&self) -> ::std::option::Option<super::WatchedDirectoryView<'_>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(&self) -> super::WatchedDirectoryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchedDirectoryView::default())
  }
  pub fn watched_directory_mut(&mut self) -> super::WatchedDirectoryMut<'_> {
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
  pub fn set_watched_directory(&mut self,
    val: impl ::protobuf::IntoProxied<super::WatchedDirectory>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn specifier(&self) -> super::data_source::SpecifierOneof<'_> {
    match &self.specifier_case() {
      super::data_source::SpecifierCase::Filename =>
          super::data_source::SpecifierOneof::Filename(self.filename()),
      super::data_source::SpecifierCase::InlineBytes =>
          super::data_source::SpecifierOneof::InlineBytes(self.inline_bytes()),
      super::data_source::SpecifierCase::InlineString =>
          super::data_source::SpecifierOneof::InlineString(self.inline_string()),
      super::data_source::SpecifierCase::EnvironmentVariable =>
          super::data_source::SpecifierOneof::EnvironmentVariable(self.environment_variable()),
      _ => super::data_source::SpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn specifier_case(&self) -> super::data_source::SpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::data_source::SpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl DataSource

impl ::std::ops::Drop for DataSource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DataSource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DataSource {
  type Proxied = Self;
  fn as_view(&self) -> DataSourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DataSource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DataSourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DataSource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__DataSource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T01T1T3^!|#|$|%");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__DataSource_msg_init.0, &[<super::WatchedDirectory as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__DataSource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DataSource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DataSource {
  type Msg = DataSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DataSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DataSource {
  type Msg = DataSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DataSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DataSourceMut<'_> {
  type Msg = DataSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DataSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DataSourceMut<'_> {
  type Msg = DataSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DataSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DataSourceView<'_> {
  type Msg = DataSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DataSource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DataSourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod data_source {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum SpecifierOneof<'msg> {
  Filename(&'msg ::protobuf::ProtoStr) = 1,
  InlineBytes(&'msg [u8]) = 2,
  InlineString(&'msg ::protobuf::ProtoStr) = 3,
  EnvironmentVariable(&'msg ::protobuf::ProtoStr) = 4,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum SpecifierCase {
  Filename = 1,
  InlineBytes = 2,
  InlineString = 3,
  EnvironmentVariable = 4,

  not_set = 0
}

impl SpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<SpecifierCase> {
    match v {
      0 => Some(SpecifierCase::not_set),
      1 => Some(SpecifierCase::Filename),
      2 => Some(SpecifierCase::InlineBytes),
      3 => Some(SpecifierCase::InlineString),
      4 => Some(SpecifierCase::EnvironmentVariable),
      _ => None
    }
  }
}
}  // pub mod data_source


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RetryPolicy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RetryPolicy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RetryPolicy>
}

impl ::protobuf::Message for RetryPolicy {
  type MessageView<'msg> = RetryPolicyView<'msg>;
  type MessageMut<'msg> = RetryPolicyMut<'msg>;
}

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
unsafe impl ::std::marker::Sync for RetryPolicy {}

// SAFETY:
// - `RetryPolicy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RetryPolicy {}

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

  // retry_back_off: optional message envoy.config.core.v3.BackoffStrategy
  pub fn has_retry_back_off(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn retry_back_off_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyView<'msg>> {
    self.has_retry_back_off().then(|| self.retry_back_off())
  }
  pub fn retry_back_off(self) -> crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyView::default())
  }

  // num_retries: optional message google.protobuf.UInt32Value
  pub fn has_num_retries(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn num_retries_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_num_retries().then(|| self.num_retries())
  }
  pub fn num_retries(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // retry_on: optional string
  pub fn retry_on(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // retry_priority: optional message envoy.config.core.v3.RetryPolicy.RetryPriority
  pub fn has_retry_priority(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn retry_priority_opt(self) -> ::std::option::Option<super::retry_policy::RetryPriorityView<'msg>> {
    self.has_retry_priority().then(|| self.retry_priority())
  }
  pub fn retry_priority(self) -> super::retry_policy::RetryPriorityView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::retry_policy::RetryPriorityView::default())
  }

  // retry_host_predicate: repeated message envoy.config.core.v3.RetryPolicy.RetryHostPredicate
  pub fn retry_host_predicate(self) -> ::protobuf::RepeatedView<'msg, super::retry_policy::RetryHostPredicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::retry_policy::RetryHostPredicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // host_selection_retry_max_attempts: optional int64
  pub fn host_selection_retry_max_attempts(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        5, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `RetryPolicyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RetryPolicyView<'_> {}

// SAFETY:
// - `RetryPolicyView` is `Send` because while its alive a `RetryPolicyMut` cannot.
// - `RetryPolicyView` does not use thread-local data.
unsafe impl ::std::marker::Send for RetryPolicyView<'_> {}

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

impl ::protobuf::__internal::EntityType for RetryPolicy {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RetryPolicyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RetryPolicyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
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
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RetryPolicy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // retry_back_off: optional message envoy.config.core.v3.BackoffStrategy
  pub fn has_retry_back_off(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_retry_back_off(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn retry_back_off_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyView<'_>> {
    self.has_retry_back_off().then(|| self.retry_back_off())
  }
  pub fn retry_back_off(&self) -> crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyView::default())
  }
  pub fn retry_back_off_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyMut<'_> {
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
  pub fn set_retry_back_off(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // num_retries: optional message google.protobuf.UInt32Value
  pub fn has_num_retries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_num_retries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn num_retries_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_num_retries().then(|| self.num_retries())
  }
  pub fn num_retries(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn num_retries_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_num_retries(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // retry_on: optional string
  pub fn retry_on(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_retry_on(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // retry_priority: optional message envoy.config.core.v3.RetryPolicy.RetryPriority
  pub fn has_retry_priority(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_retry_priority(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn retry_priority_opt(&self) -> ::std::option::Option<super::retry_policy::RetryPriorityView<'_>> {
    self.has_retry_priority().then(|| self.retry_priority())
  }
  pub fn retry_priority(&self) -> super::retry_policy::RetryPriorityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::retry_policy::RetryPriorityView::default())
  }
  pub fn retry_priority_mut(&mut self) -> super::retry_policy::RetryPriorityMut<'_> {
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
  pub fn set_retry_priority(&mut self,
    val: impl ::protobuf::IntoProxied<super::retry_policy::RetryPriority>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // retry_host_predicate: repeated message envoy.config.core.v3.RetryPolicy.RetryHostPredicate
  pub fn retry_host_predicate(&self) -> ::protobuf::RepeatedView<'_, super::retry_policy::RetryHostPredicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::retry_policy::RetryHostPredicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn retry_host_predicate_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::retry_policy::RetryHostPredicate> {
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
  pub fn set_retry_host_predicate(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::retry_policy::RetryHostPredicate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // host_selection_retry_max_attempts: optional int64
  pub fn host_selection_retry_max_attempts(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        5, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_host_selection_retry_max_attempts(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `RetryPolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RetryPolicyMut<'_> {}

// SAFETY:
// - `RetryPolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RetryPolicyMut<'_> {}

impl<'msg> ::protobuf::AsView for RetryPolicyMut<'msg> {
  type Proxied = RetryPolicy;
  fn as_view(&self) -> ::protobuf::View<'_, RetryPolicy> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetryPolicyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RetryPolicy>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RetryPolicyMut<'msg> {
  type MutProxied = RetryPolicy;
  fn as_mut(&mut self) -> RetryPolicyMut<'msg> {
    self.inner.reborrow().into()
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

  // retry_back_off: optional message envoy.config.core.v3.BackoffStrategy
  pub fn has_retry_back_off(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_retry_back_off(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn retry_back_off_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyView<'_>> {
    self.has_retry_back_off().then(|| self.retry_back_off())
  }
  pub fn retry_back_off(&self) -> crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyView::default())
  }
  pub fn retry_back_off_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategyMut<'_> {
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
  pub fn set_retry_back_off(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // num_retries: optional message google.protobuf.UInt32Value
  pub fn has_num_retries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_num_retries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn num_retries_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_num_retries().then(|| self.num_retries())
  }
  pub fn num_retries(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn num_retries_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_num_retries(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // retry_on: optional string
  pub fn retry_on(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_retry_on(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // retry_priority: optional message envoy.config.core.v3.RetryPolicy.RetryPriority
  pub fn has_retry_priority(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_retry_priority(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn retry_priority_opt(&self) -> ::std::option::Option<super::retry_policy::RetryPriorityView<'_>> {
    self.has_retry_priority().then(|| self.retry_priority())
  }
  pub fn retry_priority(&self) -> super::retry_policy::RetryPriorityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::retry_policy::RetryPriorityView::default())
  }
  pub fn retry_priority_mut(&mut self) -> super::retry_policy::RetryPriorityMut<'_> {
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
  pub fn set_retry_priority(&mut self,
    val: impl ::protobuf::IntoProxied<super::retry_policy::RetryPriority>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // retry_host_predicate: repeated message envoy.config.core.v3.RetryPolicy.RetryHostPredicate
  pub fn retry_host_predicate(&self) -> ::protobuf::RepeatedView<'_, super::retry_policy::RetryHostPredicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::retry_policy::RetryHostPredicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn retry_host_predicate_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::retry_policy::RetryHostPredicate> {
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
  pub fn set_retry_host_predicate(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::retry_policy::RetryHostPredicate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // host_selection_retry_max_attempts: optional int64
  pub fn host_selection_retry_max_attempts(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        5, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_host_selection_retry_max_attempts(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        5, val.into()
      )
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
        super::envoy__config__core__v3__RetryPolicy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$331X3G+P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__RetryPolicy_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::backoff::BackoffStrategy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::retry_policy::RetryPriority as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::retry_policy::RetryHostPredicate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__RetryPolicy_msg_init.0)
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

pub mod retry_policy {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RetryPolicy__RetryPriority_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RetryPriority {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RetryPriority>
}

impl ::protobuf::Message for RetryPriority {
  type MessageView<'msg> = RetryPriorityView<'msg>;
  type MessageMut<'msg> = RetryPriorityMut<'msg>;
}

impl ::std::default::Default for RetryPriority {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RetryPriority {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RetryPriority` is `Sync` because it does not implement interior mutability.
//    Neither does `RetryPriorityMut`.
unsafe impl ::std::marker::Sync for RetryPriority {}

// SAFETY:
// - `RetryPriority` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RetryPriority {}

impl ::protobuf::Proxied for RetryPriority {
  type View<'msg> = RetryPriorityView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RetryPriority {}

impl ::protobuf::MutProxied for RetryPriority {
  type Mut<'msg> = RetryPriorityMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RetryPriorityView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RetryPriority>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RetryPriorityView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RetryPriorityView<'msg> {
  type Message = RetryPriority;
}

impl ::std::fmt::Debug for RetryPriorityView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RetryPriorityView<'_> {
  fn default() -> RetryPriorityView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RetryPriority>> for RetryPriorityView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RetryPriority>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RetryPriorityView<'msg> {

  pub fn to_owned(&self) -> RetryPriority {
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

  pub fn config_type(self) -> super::super::retry_policy::retry_priority::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::super::retry_policy::retry_priority::ConfigTypeCase::TypedConfig =>
          super::super::retry_policy::retry_priority::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::retry_policy::retry_priority::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::super::retry_policy::retry_priority::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::retry_policy::retry_priority::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RetryPriorityView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RetryPriorityView<'_> {}

// SAFETY:
// - `RetryPriorityView` is `Send` because while its alive a `RetryPriorityMut` cannot.
// - `RetryPriorityView` does not use thread-local data.
unsafe impl ::std::marker::Send for RetryPriorityView<'_> {}

impl<'msg> ::protobuf::AsView for RetryPriorityView<'msg> {
  type Proxied = RetryPriority;
  fn as_view(&self) -> ::protobuf::View<'msg, RetryPriority> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetryPriorityView<'msg> {
  fn into_view<'shorter>(self) -> RetryPriorityView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RetryPriority> for RetryPriorityView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RetryPriority {
    let mut dst = RetryPriority::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RetryPriority> for RetryPriorityMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RetryPriority {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RetryPriority {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RetryPriorityView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RetryPriorityMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RetryPriorityMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryPriority>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RetryPriorityMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RetryPriorityMut<'msg> {
  type Message = RetryPriority;
}

impl ::std::fmt::Debug for RetryPriorityMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RetryPriority>> for RetryPriorityMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryPriority>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RetryPriorityMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryPriority> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RetryPriority {
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

  pub fn config_type(&self) -> super::super::retry_policy::retry_priority::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::retry_policy::retry_priority::ConfigTypeCase::TypedConfig =>
          super::super::retry_policy::retry_priority::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::retry_policy::retry_priority::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::retry_policy::retry_priority::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::retry_policy::retry_priority::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RetryPriorityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RetryPriorityMut<'_> {}

// SAFETY:
// - `RetryPriorityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RetryPriorityMut<'_> {}

impl<'msg> ::protobuf::AsView for RetryPriorityMut<'msg> {
  type Proxied = RetryPriority;
  fn as_view(&self) -> ::protobuf::View<'_, RetryPriority> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetryPriorityMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RetryPriority>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RetryPriorityMut<'msg> {
  type MutProxied = RetryPriority;
  fn as_mut(&mut self) -> RetryPriorityMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RetryPriorityMut<'msg> {
  fn into_mut<'shorter>(self) -> RetryPriorityMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RetryPriority {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RetryPriority> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RetryPriorityView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RetryPriorityMut<'_> {
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

  pub fn config_type(&self) -> super::super::retry_policy::retry_priority::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::retry_policy::retry_priority::ConfigTypeCase::TypedConfig =>
          super::super::retry_policy::retry_priority::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::retry_policy::retry_priority::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::retry_policy::retry_priority::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::retry_policy::retry_priority::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl RetryPriority

impl ::std::ops::Drop for RetryPriority {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RetryPriority {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RetryPriority {
  type Proxied = Self;
  fn as_view(&self) -> RetryPriorityView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RetryPriority {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RetryPriorityMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RetryPriority {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::retry_policy::envoy__config__core__v3__RetryPolicy__RetryPriority_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3^#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::retry_policy::envoy__config__core__v3__RetryPolicy__RetryPriority_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::retry_policy::envoy__config__core__v3__RetryPolicy__RetryPriority_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RetryPriority {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RetryPriority {
  type Msg = RetryPriority;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPriority> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryPriority {
  type Msg = RetryPriority;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPriority> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RetryPriorityMut<'_> {
  type Msg = RetryPriority;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPriority> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryPriorityMut<'_> {
  type Msg = RetryPriority;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPriority> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryPriorityView<'_> {
  type Msg = RetryPriority;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryPriority> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RetryPriorityMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod retry_priority {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigTypeOneof<'msg> {
  TypedConfig(::protobuf::View<'msg, ::protobuf_well_known_types::Any>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigTypeCase {
  TypedConfig = 2,

  not_set = 0
}

impl ConfigTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigTypeCase> {
    match v {
      0 => Some(ConfigTypeCase::not_set),
      2 => Some(ConfigTypeCase::TypedConfig),
      _ => None
    }
  }
}
}  // pub mod retry_priority

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RetryPolicy__RetryHostPredicate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RetryHostPredicate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RetryHostPredicate>
}

impl ::protobuf::Message for RetryHostPredicate {
  type MessageView<'msg> = RetryHostPredicateView<'msg>;
  type MessageMut<'msg> = RetryHostPredicateMut<'msg>;
}

impl ::std::default::Default for RetryHostPredicate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RetryHostPredicate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RetryHostPredicate` is `Sync` because it does not implement interior mutability.
//    Neither does `RetryHostPredicateMut`.
unsafe impl ::std::marker::Sync for RetryHostPredicate {}

// SAFETY:
// - `RetryHostPredicate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RetryHostPredicate {}

impl ::protobuf::Proxied for RetryHostPredicate {
  type View<'msg> = RetryHostPredicateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RetryHostPredicate {}

impl ::protobuf::MutProxied for RetryHostPredicate {
  type Mut<'msg> = RetryHostPredicateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RetryHostPredicateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RetryHostPredicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RetryHostPredicateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RetryHostPredicateView<'msg> {
  type Message = RetryHostPredicate;
}

impl ::std::fmt::Debug for RetryHostPredicateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RetryHostPredicateView<'_> {
  fn default() -> RetryHostPredicateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RetryHostPredicate>> for RetryHostPredicateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RetryHostPredicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RetryHostPredicateView<'msg> {

  pub fn to_owned(&self) -> RetryHostPredicate {
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

  pub fn config_type(self) -> super::super::retry_policy::retry_host_predicate::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::super::retry_policy::retry_host_predicate::ConfigTypeCase::TypedConfig =>
          super::super::retry_policy::retry_host_predicate::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::retry_policy::retry_host_predicate::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::super::retry_policy::retry_host_predicate::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::retry_policy::retry_host_predicate::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RetryHostPredicateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RetryHostPredicateView<'_> {}

// SAFETY:
// - `RetryHostPredicateView` is `Send` because while its alive a `RetryHostPredicateMut` cannot.
// - `RetryHostPredicateView` does not use thread-local data.
unsafe impl ::std::marker::Send for RetryHostPredicateView<'_> {}

impl<'msg> ::protobuf::AsView for RetryHostPredicateView<'msg> {
  type Proxied = RetryHostPredicate;
  fn as_view(&self) -> ::protobuf::View<'msg, RetryHostPredicate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetryHostPredicateView<'msg> {
  fn into_view<'shorter>(self) -> RetryHostPredicateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RetryHostPredicate> for RetryHostPredicateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RetryHostPredicate {
    let mut dst = RetryHostPredicate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RetryHostPredicate> for RetryHostPredicateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RetryHostPredicate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RetryHostPredicate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RetryHostPredicateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RetryHostPredicateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RetryHostPredicateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryHostPredicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RetryHostPredicateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RetryHostPredicateMut<'msg> {
  type Message = RetryHostPredicate;
}

impl ::std::fmt::Debug for RetryHostPredicateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RetryHostPredicate>> for RetryHostPredicateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryHostPredicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RetryHostPredicateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RetryHostPredicate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RetryHostPredicate {
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

  pub fn config_type(&self) -> super::super::retry_policy::retry_host_predicate::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::retry_policy::retry_host_predicate::ConfigTypeCase::TypedConfig =>
          super::super::retry_policy::retry_host_predicate::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::retry_policy::retry_host_predicate::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::retry_policy::retry_host_predicate::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::retry_policy::retry_host_predicate::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RetryHostPredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RetryHostPredicateMut<'_> {}

// SAFETY:
// - `RetryHostPredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RetryHostPredicateMut<'_> {}

impl<'msg> ::protobuf::AsView for RetryHostPredicateMut<'msg> {
  type Proxied = RetryHostPredicate;
  fn as_view(&self) -> ::protobuf::View<'_, RetryHostPredicate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetryHostPredicateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RetryHostPredicate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RetryHostPredicateMut<'msg> {
  type MutProxied = RetryHostPredicate;
  fn as_mut(&mut self) -> RetryHostPredicateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RetryHostPredicateMut<'msg> {
  fn into_mut<'shorter>(self) -> RetryHostPredicateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RetryHostPredicate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RetryHostPredicate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RetryHostPredicateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RetryHostPredicateMut<'_> {
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

  pub fn config_type(&self) -> super::super::retry_policy::retry_host_predicate::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::super::retry_policy::retry_host_predicate::ConfigTypeCase::TypedConfig =>
          super::super::retry_policy::retry_host_predicate::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::super::retry_policy::retry_host_predicate::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::super::retry_policy::retry_host_predicate::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::retry_policy::retry_host_predicate::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl RetryHostPredicate

impl ::std::ops::Drop for RetryHostPredicate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RetryHostPredicate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RetryHostPredicate {
  type Proxied = Self;
  fn as_view(&self) -> RetryHostPredicateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RetryHostPredicate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RetryHostPredicateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RetryHostPredicate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::retry_policy::envoy__config__core__v3__RetryPolicy__RetryHostPredicate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3^#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::retry_policy::envoy__config__core__v3__RetryPolicy__RetryHostPredicate_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::retry_policy::envoy__config__core__v3__RetryPolicy__RetryHostPredicate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RetryHostPredicate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RetryHostPredicate {
  type Msg = RetryHostPredicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryHostPredicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryHostPredicate {
  type Msg = RetryHostPredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryHostPredicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RetryHostPredicateMut<'_> {
  type Msg = RetryHostPredicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryHostPredicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryHostPredicateMut<'_> {
  type Msg = RetryHostPredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryHostPredicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RetryHostPredicateView<'_> {
  type Msg = RetryHostPredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RetryHostPredicate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RetryHostPredicateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod retry_host_predicate {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigTypeOneof<'msg> {
  TypedConfig(::protobuf::View<'msg, ::protobuf_well_known_types::Any>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigTypeCase {
  TypedConfig = 2,

  not_set = 0
}

impl ConfigTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigTypeCase> {
    match v {
      0 => Some(ConfigTypeCase::not_set),
      2 => Some(ConfigTypeCase::TypedConfig),
      _ => None
    }
  }
}
}  // pub mod retry_host_predicate


}  // pub mod retry_policy


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RemoteDataSource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RemoteDataSource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RemoteDataSource>
}

impl ::protobuf::Message for RemoteDataSource {
  type MessageView<'msg> = RemoteDataSourceView<'msg>;
  type MessageMut<'msg> = RemoteDataSourceMut<'msg>;
}

impl ::std::default::Default for RemoteDataSource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RemoteDataSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RemoteDataSource` is `Sync` because it does not implement interior mutability.
//    Neither does `RemoteDataSourceMut`.
unsafe impl ::std::marker::Sync for RemoteDataSource {}

// SAFETY:
// - `RemoteDataSource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RemoteDataSource {}

impl ::protobuf::Proxied for RemoteDataSource {
  type View<'msg> = RemoteDataSourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RemoteDataSource {}

impl ::protobuf::MutProxied for RemoteDataSource {
  type Mut<'msg> = RemoteDataSourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RemoteDataSourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RemoteDataSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RemoteDataSourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RemoteDataSourceView<'msg> {
  type Message = RemoteDataSource;
}

impl ::std::fmt::Debug for RemoteDataSourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RemoteDataSourceView<'_> {
  fn default() -> RemoteDataSourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RemoteDataSource>> for RemoteDataSourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RemoteDataSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RemoteDataSourceView<'msg> {

  pub fn to_owned(&self) -> RemoteDataSource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // http_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_http_uri(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn http_uri_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'msg>> {
    self.has_http_uri().then(|| self.http_uri())
  }
  pub fn http_uri(self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }

  // sha256: optional string
  pub fn sha256(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn retry_policy_opt(self) -> ::std::option::Option<super::RetryPolicyView<'msg>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(self) -> super::RetryPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RetryPolicyView::default())
  }

}

// SAFETY:
// - `RemoteDataSourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RemoteDataSourceView<'_> {}

// SAFETY:
// - `RemoteDataSourceView` is `Send` because while its alive a `RemoteDataSourceMut` cannot.
// - `RemoteDataSourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for RemoteDataSourceView<'_> {}

impl<'msg> ::protobuf::AsView for RemoteDataSourceView<'msg> {
  type Proxied = RemoteDataSource;
  fn as_view(&self) -> ::protobuf::View<'msg, RemoteDataSource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RemoteDataSourceView<'msg> {
  fn into_view<'shorter>(self) -> RemoteDataSourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RemoteDataSource> for RemoteDataSourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RemoteDataSource {
    let mut dst = RemoteDataSource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RemoteDataSource> for RemoteDataSourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RemoteDataSource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RemoteDataSource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RemoteDataSourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RemoteDataSourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RemoteDataSourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RemoteDataSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RemoteDataSourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RemoteDataSourceMut<'msg> {
  type Message = RemoteDataSource;
}

impl ::std::fmt::Debug for RemoteDataSourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RemoteDataSource>> for RemoteDataSourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RemoteDataSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RemoteDataSourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RemoteDataSource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RemoteDataSource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // http_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_http_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_uri_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_>> {
    self.has_http_uri().then(|| self.http_uri())
  }
  pub fn http_uri(&self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }
  pub fn http_uri_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriMut<'_> {
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
  pub fn set_http_uri(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // sha256: optional string
  pub fn sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<super::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> super::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> super::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::RetryPolicy>) {

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
// - `RemoteDataSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RemoteDataSourceMut<'_> {}

// SAFETY:
// - `RemoteDataSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RemoteDataSourceMut<'_> {}

impl<'msg> ::protobuf::AsView for RemoteDataSourceMut<'msg> {
  type Proxied = RemoteDataSource;
  fn as_view(&self) -> ::protobuf::View<'_, RemoteDataSource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RemoteDataSourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RemoteDataSource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RemoteDataSourceMut<'msg> {
  type MutProxied = RemoteDataSource;
  fn as_mut(&mut self) -> RemoteDataSourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RemoteDataSourceMut<'msg> {
  fn into_mut<'shorter>(self) -> RemoteDataSourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RemoteDataSource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RemoteDataSource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RemoteDataSourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RemoteDataSourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // http_uri: optional message envoy.config.core.v3.HttpUri
  pub fn has_http_uri(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_http_uri(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn http_uri_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_>> {
    self.has_http_uri().then(|| self.http_uri())
  }
  pub fn http_uri(&self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriView::default())
  }
  pub fn http_uri_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::http_uri::HttpUriMut<'_> {
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
  pub fn set_http_uri(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // sha256: optional string
  pub fn sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // retry_policy: optional message envoy.config.core.v3.RetryPolicy
  pub fn has_retry_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_retry_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn retry_policy_opt(&self) -> ::std::option::Option<super::RetryPolicyView<'_>> {
    self.has_retry_policy().then(|| self.retry_policy())
  }
  pub fn retry_policy(&self) -> super::RetryPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RetryPolicyView::default())
  }
  pub fn retry_policy_mut(&mut self) -> super::RetryPolicyMut<'_> {
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
  pub fn set_retry_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::RetryPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl RemoteDataSource

impl ::std::ops::Drop for RemoteDataSource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RemoteDataSource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RemoteDataSource {
  type Proxied = Self;
  fn as_view(&self) -> RemoteDataSourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RemoteDataSource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RemoteDataSourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RemoteDataSource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__RemoteDataSource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__RemoteDataSource_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::http_uri::HttpUri as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::RetryPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__RemoteDataSource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RemoteDataSource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RemoteDataSource {
  type Msg = RemoteDataSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoteDataSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RemoteDataSource {
  type Msg = RemoteDataSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoteDataSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RemoteDataSourceMut<'_> {
  type Msg = RemoteDataSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoteDataSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RemoteDataSourceMut<'_> {
  type Msg = RemoteDataSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoteDataSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RemoteDataSourceView<'_> {
  type Msg = RemoteDataSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoteDataSource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RemoteDataSourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__AsyncDataSource_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AsyncDataSource {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AsyncDataSource>
}

impl ::protobuf::Message for AsyncDataSource {
  type MessageView<'msg> = AsyncDataSourceView<'msg>;
  type MessageMut<'msg> = AsyncDataSourceMut<'msg>;
}

impl ::std::default::Default for AsyncDataSource {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AsyncDataSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AsyncDataSource` is `Sync` because it does not implement interior mutability.
//    Neither does `AsyncDataSourceMut`.
unsafe impl ::std::marker::Sync for AsyncDataSource {}

// SAFETY:
// - `AsyncDataSource` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AsyncDataSource {}

impl ::protobuf::Proxied for AsyncDataSource {
  type View<'msg> = AsyncDataSourceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AsyncDataSource {}

impl ::protobuf::MutProxied for AsyncDataSource {
  type Mut<'msg> = AsyncDataSourceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AsyncDataSourceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AsyncDataSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AsyncDataSourceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AsyncDataSourceView<'msg> {
  type Message = AsyncDataSource;
}

impl ::std::fmt::Debug for AsyncDataSourceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AsyncDataSourceView<'_> {
  fn default() -> AsyncDataSourceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AsyncDataSource>> for AsyncDataSourceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AsyncDataSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AsyncDataSourceView<'msg> {

  pub fn to_owned(&self) -> AsyncDataSource {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // local: optional message envoy.config.core.v3.DataSource
  pub fn has_local(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn local_opt(self) -> ::std::option::Option<super::DataSourceView<'msg>> {
    self.has_local().then(|| self.local())
  }
  pub fn local(self) -> super::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DataSourceView::default())
  }

  // remote: optional message envoy.config.core.v3.RemoteDataSource
  pub fn has_remote(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn remote_opt(self) -> ::std::option::Option<super::RemoteDataSourceView<'msg>> {
    self.has_remote().then(|| self.remote())
  }
  pub fn remote(self) -> super::RemoteDataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RemoteDataSourceView::default())
  }

  pub fn specifier(self) -> super::async_data_source::SpecifierOneof<'msg> {
    match self.specifier_case() {
      super::async_data_source::SpecifierCase::Local =>
          super::async_data_source::SpecifierOneof::Local(self.local()),
      super::async_data_source::SpecifierCase::Remote =>
          super::async_data_source::SpecifierOneof::Remote(self.remote()),
      _ => super::async_data_source::SpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn specifier_case(self) -> super::async_data_source::SpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::async_data_source::SpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AsyncDataSourceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AsyncDataSourceView<'_> {}

// SAFETY:
// - `AsyncDataSourceView` is `Send` because while its alive a `AsyncDataSourceMut` cannot.
// - `AsyncDataSourceView` does not use thread-local data.
unsafe impl ::std::marker::Send for AsyncDataSourceView<'_> {}

impl<'msg> ::protobuf::AsView for AsyncDataSourceView<'msg> {
  type Proxied = AsyncDataSource;
  fn as_view(&self) -> ::protobuf::View<'msg, AsyncDataSource> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AsyncDataSourceView<'msg> {
  fn into_view<'shorter>(self) -> AsyncDataSourceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AsyncDataSource> for AsyncDataSourceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AsyncDataSource {
    let mut dst = AsyncDataSource::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AsyncDataSource> for AsyncDataSourceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AsyncDataSource {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AsyncDataSource {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AsyncDataSourceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AsyncDataSourceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AsyncDataSourceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AsyncDataSource>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AsyncDataSourceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AsyncDataSourceMut<'msg> {
  type Message = AsyncDataSource;
}

impl ::std::fmt::Debug for AsyncDataSourceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AsyncDataSource>> for AsyncDataSourceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AsyncDataSource>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AsyncDataSourceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AsyncDataSource> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AsyncDataSource {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // local: optional message envoy.config.core.v3.DataSource
  pub fn has_local(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_local(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn local_opt(&self) -> ::std::option::Option<super::DataSourceView<'_>> {
    self.has_local().then(|| self.local())
  }
  pub fn local(&self) -> super::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DataSourceView::default())
  }
  pub fn local_mut(&mut self) -> super::DataSourceMut<'_> {
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
  pub fn set_local(&mut self,
    val: impl ::protobuf::IntoProxied<super::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // remote: optional message envoy.config.core.v3.RemoteDataSource
  pub fn has_remote(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_remote(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn remote_opt(&self) -> ::std::option::Option<super::RemoteDataSourceView<'_>> {
    self.has_remote().then(|| self.remote())
  }
  pub fn remote(&self) -> super::RemoteDataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RemoteDataSourceView::default())
  }
  pub fn remote_mut(&mut self) -> super::RemoteDataSourceMut<'_> {
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
  pub fn set_remote(&mut self,
    val: impl ::protobuf::IntoProxied<super::RemoteDataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn specifier(&self) -> super::async_data_source::SpecifierOneof<'_> {
    match &self.specifier_case() {
      super::async_data_source::SpecifierCase::Local =>
          super::async_data_source::SpecifierOneof::Local(self.local()),
      super::async_data_source::SpecifierCase::Remote =>
          super::async_data_source::SpecifierOneof::Remote(self.remote()),
      _ => super::async_data_source::SpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn specifier_case(&self) -> super::async_data_source::SpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::async_data_source::SpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AsyncDataSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AsyncDataSourceMut<'_> {}

// SAFETY:
// - `AsyncDataSourceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AsyncDataSourceMut<'_> {}

impl<'msg> ::protobuf::AsView for AsyncDataSourceMut<'msg> {
  type Proxied = AsyncDataSource;
  fn as_view(&self) -> ::protobuf::View<'_, AsyncDataSource> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AsyncDataSourceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AsyncDataSource>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AsyncDataSourceMut<'msg> {
  type MutProxied = AsyncDataSource;
  fn as_mut(&mut self) -> AsyncDataSourceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AsyncDataSourceMut<'msg> {
  fn into_mut<'shorter>(self) -> AsyncDataSourceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AsyncDataSource {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AsyncDataSource> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AsyncDataSourceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AsyncDataSourceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // local: optional message envoy.config.core.v3.DataSource
  pub fn has_local(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_local(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn local_opt(&self) -> ::std::option::Option<super::DataSourceView<'_>> {
    self.has_local().then(|| self.local())
  }
  pub fn local(&self) -> super::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::DataSourceView::default())
  }
  pub fn local_mut(&mut self) -> super::DataSourceMut<'_> {
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
  pub fn set_local(&mut self,
    val: impl ::protobuf::IntoProxied<super::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // remote: optional message envoy.config.core.v3.RemoteDataSource
  pub fn has_remote(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_remote(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn remote_opt(&self) -> ::std::option::Option<super::RemoteDataSourceView<'_>> {
    self.has_remote().then(|| self.remote())
  }
  pub fn remote(&self) -> super::RemoteDataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RemoteDataSourceView::default())
  }
  pub fn remote_mut(&mut self) -> super::RemoteDataSourceMut<'_> {
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
  pub fn set_remote(&mut self,
    val: impl ::protobuf::IntoProxied<super::RemoteDataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn specifier(&self) -> super::async_data_source::SpecifierOneof<'_> {
    match &self.specifier_case() {
      super::async_data_source::SpecifierCase::Local =>
          super::async_data_source::SpecifierOneof::Local(self.local()),
      super::async_data_source::SpecifierCase::Remote =>
          super::async_data_source::SpecifierOneof::Remote(self.remote()),
      _ => super::async_data_source::SpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn specifier_case(&self) -> super::async_data_source::SpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::async_data_source::SpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl AsyncDataSource

impl ::std::ops::Drop for AsyncDataSource {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AsyncDataSource {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AsyncDataSource {
  type Proxied = Self;
  fn as_view(&self) -> AsyncDataSourceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AsyncDataSource {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AsyncDataSourceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AsyncDataSource {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__AsyncDataSource_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__AsyncDataSource_msg_init.0, &[<super::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::RemoteDataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__AsyncDataSource_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AsyncDataSource {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AsyncDataSource {
  type Msg = AsyncDataSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AsyncDataSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AsyncDataSource {
  type Msg = AsyncDataSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AsyncDataSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AsyncDataSourceMut<'_> {
  type Msg = AsyncDataSource;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AsyncDataSource> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AsyncDataSourceMut<'_> {
  type Msg = AsyncDataSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AsyncDataSource> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AsyncDataSourceView<'_> {
  type Msg = AsyncDataSource;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AsyncDataSource> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AsyncDataSourceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod async_data_source {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum SpecifierOneof<'msg> {
  Local(::protobuf::View<'msg, super::super::DataSource>) = 1,
  Remote(::protobuf::View<'msg, super::super::RemoteDataSource>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum SpecifierCase {
  Local = 1,
  Remote = 2,

  not_set = 0
}

impl SpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<SpecifierCase> {
    match v {
      0 => Some(SpecifierCase::not_set),
      1 => Some(SpecifierCase::Local),
      2 => Some(SpecifierCase::Remote),
      _ => None
    }
  }
}
}  // pub mod async_data_source


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__TransportSocket_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TransportSocket {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TransportSocket>
}

impl ::protobuf::Message for TransportSocket {
  type MessageView<'msg> = TransportSocketView<'msg>;
  type MessageMut<'msg> = TransportSocketMut<'msg>;
}

impl ::std::default::Default for TransportSocket {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TransportSocket {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TransportSocket` is `Sync` because it does not implement interior mutability.
//    Neither does `TransportSocketMut`.
unsafe impl ::std::marker::Sync for TransportSocket {}

// SAFETY:
// - `TransportSocket` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TransportSocket {}

impl ::protobuf::Proxied for TransportSocket {
  type View<'msg> = TransportSocketView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TransportSocket {}

impl ::protobuf::MutProxied for TransportSocket {
  type Mut<'msg> = TransportSocketMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TransportSocketView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TransportSocket>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TransportSocketView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TransportSocketView<'msg> {
  type Message = TransportSocket;
}

impl ::std::fmt::Debug for TransportSocketView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TransportSocketView<'_> {
  fn default() -> TransportSocketView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TransportSocket>> for TransportSocketView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TransportSocket>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TransportSocketView<'msg> {

  pub fn to_owned(&self) -> TransportSocket {
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

  pub fn config_type(self) -> super::transport_socket::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::transport_socket::ConfigTypeCase::TypedConfig =>
          super::transport_socket::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::transport_socket::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::transport_socket::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::transport_socket::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `TransportSocketView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TransportSocketView<'_> {}

// SAFETY:
// - `TransportSocketView` is `Send` because while its alive a `TransportSocketMut` cannot.
// - `TransportSocketView` does not use thread-local data.
unsafe impl ::std::marker::Send for TransportSocketView<'_> {}

impl<'msg> ::protobuf::AsView for TransportSocketView<'msg> {
  type Proxied = TransportSocket;
  fn as_view(&self) -> ::protobuf::View<'msg, TransportSocket> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TransportSocketView<'msg> {
  fn into_view<'shorter>(self) -> TransportSocketView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TransportSocket> for TransportSocketView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TransportSocket {
    let mut dst = TransportSocket::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TransportSocket> for TransportSocketMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TransportSocket {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TransportSocket {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TransportSocketView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TransportSocketMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TransportSocketMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TransportSocket>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TransportSocketMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TransportSocketMut<'msg> {
  type Message = TransportSocket;
}

impl ::std::fmt::Debug for TransportSocketMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TransportSocket>> for TransportSocketMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TransportSocket>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TransportSocketMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TransportSocket> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TransportSocket {
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

  pub fn config_type(&self) -> super::transport_socket::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::transport_socket::ConfigTypeCase::TypedConfig =>
          super::transport_socket::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::transport_socket::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::transport_socket::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::transport_socket::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `TransportSocketMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TransportSocketMut<'_> {}

// SAFETY:
// - `TransportSocketMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TransportSocketMut<'_> {}

impl<'msg> ::protobuf::AsView for TransportSocketMut<'msg> {
  type Proxied = TransportSocket;
  fn as_view(&self) -> ::protobuf::View<'_, TransportSocket> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TransportSocketMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TransportSocket>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TransportSocketMut<'msg> {
  type MutProxied = TransportSocket;
  fn as_mut(&mut self) -> TransportSocketMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TransportSocketMut<'msg> {
  fn into_mut<'shorter>(self) -> TransportSocketMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TransportSocket {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TransportSocket> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TransportSocketView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TransportSocketMut<'_> {
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

  pub fn config_type(&self) -> super::transport_socket::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::transport_socket::ConfigTypeCase::TypedConfig =>
          super::transport_socket::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::transport_socket::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::transport_socket::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::transport_socket::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl TransportSocket

impl ::std::ops::Drop for TransportSocket {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TransportSocket {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TransportSocket {
  type Proxied = Self;
  fn as_view(&self) -> TransportSocketView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TransportSocket {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TransportSocketMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TransportSocket {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__TransportSocket_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa3^$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__TransportSocket_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__TransportSocket_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TransportSocket {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TransportSocket {
  type Msg = TransportSocket;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocket> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TransportSocket {
  type Msg = TransportSocket;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocket> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TransportSocketMut<'_> {
  type Msg = TransportSocket;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocket> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TransportSocketMut<'_> {
  type Msg = TransportSocket;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocket> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TransportSocketView<'_> {
  type Msg = TransportSocket;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocket> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TransportSocketMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod transport_socket {

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
}  // pub mod transport_socket


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__RuntimeFractionalPercent_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RuntimeFractionalPercent {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RuntimeFractionalPercent>
}

impl ::protobuf::Message for RuntimeFractionalPercent {
  type MessageView<'msg> = RuntimeFractionalPercentView<'msg>;
  type MessageMut<'msg> = RuntimeFractionalPercentMut<'msg>;
}

impl ::std::default::Default for RuntimeFractionalPercent {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RuntimeFractionalPercent {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RuntimeFractionalPercent` is `Sync` because it does not implement interior mutability.
//    Neither does `RuntimeFractionalPercentMut`.
unsafe impl ::std::marker::Sync for RuntimeFractionalPercent {}

// SAFETY:
// - `RuntimeFractionalPercent` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeFractionalPercent {}

impl ::protobuf::Proxied for RuntimeFractionalPercent {
  type View<'msg> = RuntimeFractionalPercentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RuntimeFractionalPercent {}

impl ::protobuf::MutProxied for RuntimeFractionalPercent {
  type Mut<'msg> = RuntimeFractionalPercentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RuntimeFractionalPercentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeFractionalPercent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeFractionalPercentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RuntimeFractionalPercentView<'msg> {
  type Message = RuntimeFractionalPercent;
}

impl ::std::fmt::Debug for RuntimeFractionalPercentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RuntimeFractionalPercentView<'_> {
  fn default() -> RuntimeFractionalPercentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeFractionalPercent>> for RuntimeFractionalPercentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeFractionalPercent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeFractionalPercentView<'msg> {

  pub fn to_owned(&self) -> RuntimeFractionalPercent {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // default_value: optional message envoy.type.v3.FractionalPercent
  pub fn has_default_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn default_value_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }

  // runtime_key: optional string
  pub fn runtime_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `RuntimeFractionalPercentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RuntimeFractionalPercentView<'_> {}

// SAFETY:
// - `RuntimeFractionalPercentView` is `Send` because while its alive a `RuntimeFractionalPercentMut` cannot.
// - `RuntimeFractionalPercentView` does not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeFractionalPercentView<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeFractionalPercentView<'msg> {
  type Proxied = RuntimeFractionalPercent;
  fn as_view(&self) -> ::protobuf::View<'msg, RuntimeFractionalPercent> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeFractionalPercentView<'msg> {
  fn into_view<'shorter>(self) -> RuntimeFractionalPercentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeFractionalPercent> for RuntimeFractionalPercentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeFractionalPercent {
    let mut dst = RuntimeFractionalPercent::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeFractionalPercent> for RuntimeFractionalPercentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeFractionalPercent {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RuntimeFractionalPercent {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeFractionalPercentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeFractionalPercentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RuntimeFractionalPercentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFractionalPercent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeFractionalPercentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RuntimeFractionalPercentMut<'msg> {
  type Message = RuntimeFractionalPercent;
}

impl ::std::fmt::Debug for RuntimeFractionalPercentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFractionalPercent>> for RuntimeFractionalPercentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFractionalPercent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeFractionalPercentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeFractionalPercent> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RuntimeFractionalPercent {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // default_value: optional message envoy.type.v3.FractionalPercent
  pub fn has_default_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_default_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn default_value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn default_value_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_default_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `RuntimeFractionalPercentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RuntimeFractionalPercentMut<'_> {}

// SAFETY:
// - `RuntimeFractionalPercentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RuntimeFractionalPercentMut<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeFractionalPercentMut<'msg> {
  type Proxied = RuntimeFractionalPercent;
  fn as_view(&self) -> ::protobuf::View<'_, RuntimeFractionalPercent> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeFractionalPercentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RuntimeFractionalPercent>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RuntimeFractionalPercentMut<'msg> {
  type MutProxied = RuntimeFractionalPercent;
  fn as_mut(&mut self) -> RuntimeFractionalPercentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RuntimeFractionalPercentMut<'msg> {
  fn into_mut<'shorter>(self) -> RuntimeFractionalPercentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RuntimeFractionalPercent {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RuntimeFractionalPercent> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RuntimeFractionalPercentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RuntimeFractionalPercentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // default_value: optional message envoy.type.v3.FractionalPercent
  pub fn has_default_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_default_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn default_value_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_default_value().then(|| self.default_value())
  }
  pub fn default_value(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn default_value_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_default_value(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // runtime_key: optional string
  pub fn runtime_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_runtime_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl RuntimeFractionalPercent

impl ::std::ops::Drop for RuntimeFractionalPercent {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RuntimeFractionalPercent {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RuntimeFractionalPercent {
  type Proxied = Self;
  fn as_view(&self) -> RuntimeFractionalPercentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RuntimeFractionalPercent {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RuntimeFractionalPercentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RuntimeFractionalPercent {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__RuntimeFractionalPercent_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__RuntimeFractionalPercent_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__RuntimeFractionalPercent_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeFractionalPercent {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeFractionalPercent {
  type Msg = RuntimeFractionalPercent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFractionalPercent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeFractionalPercent {
  type Msg = RuntimeFractionalPercent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFractionalPercent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeFractionalPercentMut<'_> {
  type Msg = RuntimeFractionalPercent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFractionalPercent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeFractionalPercentMut<'_> {
  type Msg = RuntimeFractionalPercent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFractionalPercent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeFractionalPercentView<'_> {
  type Msg = RuntimeFractionalPercent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeFractionalPercent> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeFractionalPercentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__ControlPlane_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ControlPlane {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ControlPlane>
}

impl ::protobuf::Message for ControlPlane {
  type MessageView<'msg> = ControlPlaneView<'msg>;
  type MessageMut<'msg> = ControlPlaneMut<'msg>;
}

impl ::std::default::Default for ControlPlane {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ControlPlane {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ControlPlane` is `Sync` because it does not implement interior mutability.
//    Neither does `ControlPlaneMut`.
unsafe impl ::std::marker::Sync for ControlPlane {}

// SAFETY:
// - `ControlPlane` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ControlPlane {}

impl ::protobuf::Proxied for ControlPlane {
  type View<'msg> = ControlPlaneView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ControlPlane {}

impl ::protobuf::MutProxied for ControlPlane {
  type Mut<'msg> = ControlPlaneMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ControlPlaneView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ControlPlane>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ControlPlaneView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ControlPlaneView<'msg> {
  type Message = ControlPlane;
}

impl ::std::fmt::Debug for ControlPlaneView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ControlPlaneView<'_> {
  fn default() -> ControlPlaneView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ControlPlane>> for ControlPlaneView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ControlPlane>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ControlPlaneView<'msg> {

  pub fn to_owned(&self) -> ControlPlane {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // identifier: optional string
  pub fn identifier(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ControlPlaneView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ControlPlaneView<'_> {}

// SAFETY:
// - `ControlPlaneView` is `Send` because while its alive a `ControlPlaneMut` cannot.
// - `ControlPlaneView` does not use thread-local data.
unsafe impl ::std::marker::Send for ControlPlaneView<'_> {}

impl<'msg> ::protobuf::AsView for ControlPlaneView<'msg> {
  type Proxied = ControlPlane;
  fn as_view(&self) -> ::protobuf::View<'msg, ControlPlane> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ControlPlaneView<'msg> {
  fn into_view<'shorter>(self) -> ControlPlaneView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ControlPlane> for ControlPlaneView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ControlPlane {
    let mut dst = ControlPlane::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ControlPlane> for ControlPlaneMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ControlPlane {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ControlPlane {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ControlPlaneView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ControlPlaneMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ControlPlaneMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ControlPlane>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ControlPlaneMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ControlPlaneMut<'msg> {
  type Message = ControlPlane;
}

impl ::std::fmt::Debug for ControlPlaneMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ControlPlane>> for ControlPlaneMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ControlPlane>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ControlPlaneMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ControlPlane> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ControlPlane {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // identifier: optional string
  pub fn identifier(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_identifier(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}

// SAFETY:
// - `ControlPlaneMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ControlPlaneMut<'_> {}

// SAFETY:
// - `ControlPlaneMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ControlPlaneMut<'_> {}

impl<'msg> ::protobuf::AsView for ControlPlaneMut<'msg> {
  type Proxied = ControlPlane;
  fn as_view(&self) -> ::protobuf::View<'_, ControlPlane> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ControlPlaneMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ControlPlane>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ControlPlaneMut<'msg> {
  type MutProxied = ControlPlane;
  fn as_mut(&mut self) -> ControlPlaneMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ControlPlaneMut<'msg> {
  fn into_mut<'shorter>(self) -> ControlPlaneMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ControlPlane {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ControlPlane> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ControlPlaneView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ControlPlaneMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // identifier: optional string
  pub fn identifier(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_identifier(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

}  // impl ControlPlane

impl ::std::ops::Drop for ControlPlane {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ControlPlane {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ControlPlane {
  type Proxied = Self;
  fn as_view(&self) -> ControlPlaneView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ControlPlane {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ControlPlaneMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ControlPlane {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__ControlPlane_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__ControlPlane_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__ControlPlane_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ControlPlane {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ControlPlane {
  type Msg = ControlPlane;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ControlPlane> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ControlPlane {
  type Msg = ControlPlane;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ControlPlane> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ControlPlaneMut<'_> {
  type Msg = ControlPlane;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ControlPlane> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ControlPlaneMut<'_> {
  type Msg = ControlPlane;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ControlPlane> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ControlPlaneView<'_> {
  type Msg = ControlPlane;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ControlPlane> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ControlPlaneMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoutingPriority(i32);

#[allow(non_upper_case_globals)]
impl RoutingPriority {
  pub const Default: RoutingPriority = RoutingPriority(0);
  pub const High: RoutingPriority = RoutingPriority(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Default",
      1 => "High",
      _ => return None
    })
  }
}

impl ::std::convert::From<RoutingPriority> for i32 {
  fn from(val: RoutingPriority) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for RoutingPriority {
  fn from(val: i32) -> RoutingPriority {
    Self(val)
  }
}

impl ::std::default::Default for RoutingPriority {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for RoutingPriority {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "RoutingPriority::{}", constant_name)
    } else {
      write!(f, "RoutingPriority::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for RoutingPriority {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for RoutingPriority {}

impl ::protobuf::Proxied for RoutingPriority {
  type View<'a> = RoutingPriority;
}

impl ::protobuf::AsView for RoutingPriority {
  type Proxied = RoutingPriority;

  fn as_view(&self) -> RoutingPriority {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoutingPriority {
  fn into_view<'shorter>(self) -> RoutingPriority where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for RoutingPriority {
  const NAME: &'static str = "RoutingPriority";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for RoutingPriority {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestMethod(i32);

#[allow(non_upper_case_globals)]
impl RequestMethod {
  pub const MethodUnspecified: RequestMethod = RequestMethod(0);
  pub const Get: RequestMethod = RequestMethod(1);
  pub const Head: RequestMethod = RequestMethod(2);
  pub const Post: RequestMethod = RequestMethod(3);
  pub const Put: RequestMethod = RequestMethod(4);
  pub const Delete: RequestMethod = RequestMethod(5);
  pub const Connect: RequestMethod = RequestMethod(6);
  pub const Options: RequestMethod = RequestMethod(7);
  pub const Trace: RequestMethod = RequestMethod(8);
  pub const Patch: RequestMethod = RequestMethod(9);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "MethodUnspecified",
      1 => "Get",
      2 => "Head",
      3 => "Post",
      4 => "Put",
      5 => "Delete",
      6 => "Connect",
      7 => "Options",
      8 => "Trace",
      9 => "Patch",
      _ => return None
    })
  }
}

impl ::std::convert::From<RequestMethod> for i32 {
  fn from(val: RequestMethod) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for RequestMethod {
  fn from(val: i32) -> RequestMethod {
    Self(val)
  }
}

impl ::std::default::Default for RequestMethod {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for RequestMethod {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "RequestMethod::{}", constant_name)
    } else {
      write!(f, "RequestMethod::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for RequestMethod {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for RequestMethod {}

impl ::protobuf::Proxied for RequestMethod {
  type View<'a> = RequestMethod;
}

impl ::protobuf::AsView for RequestMethod {
  type Proxied = RequestMethod;

  fn as_view(&self) -> RequestMethod {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RequestMethod {
  fn into_view<'shorter>(self) -> RequestMethod where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for RequestMethod {
  const NAME: &'static str = "RequestMethod";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9)
  }
}

impl ::protobuf::__internal::EntityType for RequestMethod {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TrafficDirection(i32);

#[allow(non_upper_case_globals)]
impl TrafficDirection {
  pub const Unspecified: TrafficDirection = TrafficDirection(0);
  pub const Inbound: TrafficDirection = TrafficDirection(1);
  pub const Outbound: TrafficDirection = TrafficDirection(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Inbound",
      2 => "Outbound",
      _ => return None
    })
  }
}

impl ::std::convert::From<TrafficDirection> for i32 {
  fn from(val: TrafficDirection) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for TrafficDirection {
  fn from(val: i32) -> TrafficDirection {
    Self(val)
  }
}

impl ::std::default::Default for TrafficDirection {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for TrafficDirection {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "TrafficDirection::{}", constant_name)
    } else {
      write!(f, "TrafficDirection::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for TrafficDirection {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for TrafficDirection {}

impl ::protobuf::Proxied for TrafficDirection {
  type View<'a> = TrafficDirection;
}

impl ::protobuf::AsView for TrafficDirection {
  type Proxied = TrafficDirection;

  fn as_view(&self) -> TrafficDirection {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TrafficDirection {
  fn into_view<'shorter>(self) -> TrafficDirection where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for TrafficDirection {
  const NAME: &'static str = "TrafficDirection";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for TrafficDirection {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


