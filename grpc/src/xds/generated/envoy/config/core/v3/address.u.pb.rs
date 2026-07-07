const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Pipe_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Pipe {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Pipe>
}

impl ::protobuf::Message for Pipe {
  type MessageView<'msg> = PipeView<'msg>;
  type MessageMut<'msg> = PipeMut<'msg>;
}

impl ::std::default::Default for Pipe {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Pipe {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Pipe` is `Sync` because it does not implement interior mutability.
//    Neither does `PipeMut`.
unsafe impl ::std::marker::Sync for Pipe {}

// SAFETY:
// - `Pipe` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Pipe {}

impl ::protobuf::Proxied for Pipe {
  type View<'msg> = PipeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Pipe {}

impl ::protobuf::MutProxied for Pipe {
  type Mut<'msg> = PipeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PipeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Pipe>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PipeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PipeView<'msg> {
  type Message = Pipe;
}

impl ::std::fmt::Debug for PipeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PipeView<'_> {
  fn default() -> PipeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Pipe>> for PipeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Pipe>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PipeView<'msg> {

  pub fn to_owned(&self) -> Pipe {
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

  // mode: optional uint32
  pub fn mode(self) -> u32 {
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
// - `PipeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PipeView<'_> {}

// SAFETY:
// - `PipeView` is `Send` because while its alive a `PipeMut` cannot.
// - `PipeView` does not use thread-local data.
unsafe impl ::std::marker::Send for PipeView<'_> {}

impl<'msg> ::protobuf::AsView for PipeView<'msg> {
  type Proxied = Pipe;
  fn as_view(&self) -> ::protobuf::View<'msg, Pipe> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PipeView<'msg> {
  fn into_view<'shorter>(self) -> PipeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Pipe> for PipeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Pipe {
    let mut dst = Pipe::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Pipe> for PipeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Pipe {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Pipe {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PipeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PipeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PipeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Pipe>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PipeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PipeMut<'msg> {
  type Message = Pipe;
}

impl ::std::fmt::Debug for PipeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Pipe>> for PipeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Pipe>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PipeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Pipe> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Pipe {
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

  // mode: optional uint32
  pub fn mode(&self) -> u32 {
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
  pub fn set_mode(&mut self, val: u32) {
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
// - `PipeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PipeMut<'_> {}

// SAFETY:
// - `PipeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PipeMut<'_> {}

impl<'msg> ::protobuf::AsView for PipeMut<'msg> {
  type Proxied = Pipe;
  fn as_view(&self) -> ::protobuf::View<'_, Pipe> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PipeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Pipe>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PipeMut<'msg> {
  type MutProxied = Pipe;
  fn as_mut(&mut self) -> PipeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PipeMut<'msg> {
  fn into_mut<'shorter>(self) -> PipeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Pipe {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Pipe> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PipeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PipeMut<'_> {
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

  // mode: optional uint32
  pub fn mode(&self) -> u32 {
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
  pub fn set_mode(&mut self, val: u32) {
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

}  // impl Pipe

impl ::std::ops::Drop for Pipe {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Pipe {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Pipe {
  type Proxied = Self;
  fn as_view(&self) -> PipeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Pipe {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PipeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Pipe {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__Pipe_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__Pipe_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__Pipe_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Pipe {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Pipe {
  type Msg = Pipe;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pipe> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Pipe {
  type Msg = Pipe;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pipe> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PipeMut<'_> {
  type Msg = Pipe;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pipe> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PipeMut<'_> {
  type Msg = Pipe;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pipe> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PipeView<'_> {
  type Msg = Pipe;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Pipe> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PipeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__EnvoyInternalAddress_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EnvoyInternalAddress {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EnvoyInternalAddress>
}

impl ::protobuf::Message for EnvoyInternalAddress {
  type MessageView<'msg> = EnvoyInternalAddressView<'msg>;
  type MessageMut<'msg> = EnvoyInternalAddressMut<'msg>;
}

impl ::std::default::Default for EnvoyInternalAddress {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EnvoyInternalAddress {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EnvoyInternalAddress` is `Sync` because it does not implement interior mutability.
//    Neither does `EnvoyInternalAddressMut`.
unsafe impl ::std::marker::Sync for EnvoyInternalAddress {}

// SAFETY:
// - `EnvoyInternalAddress` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EnvoyInternalAddress {}

impl ::protobuf::Proxied for EnvoyInternalAddress {
  type View<'msg> = EnvoyInternalAddressView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EnvoyInternalAddress {}

impl ::protobuf::MutProxied for EnvoyInternalAddress {
  type Mut<'msg> = EnvoyInternalAddressMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EnvoyInternalAddressView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnvoyInternalAddress>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnvoyInternalAddressView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EnvoyInternalAddressView<'msg> {
  type Message = EnvoyInternalAddress;
}

impl ::std::fmt::Debug for EnvoyInternalAddressView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EnvoyInternalAddressView<'_> {
  fn default() -> EnvoyInternalAddressView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EnvoyInternalAddress>> for EnvoyInternalAddressView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnvoyInternalAddress>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnvoyInternalAddressView<'msg> {

  pub fn to_owned(&self) -> EnvoyInternalAddress {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // server_listener_name: optional string
  pub fn has_server_listener_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn server_listener_name_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_server_listener_name().then(|| self.server_listener_name())
  }
  pub fn server_listener_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // endpoint_id: optional string
  pub fn endpoint_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn address_name_specifier(self) -> super::envoy_internal_address::AddressNameSpecifierOneof<'msg> {
    match self.address_name_specifier_case() {
      super::envoy_internal_address::AddressNameSpecifierCase::ServerListenerName =>
          super::envoy_internal_address::AddressNameSpecifierOneof::ServerListenerName(self.server_listener_name()),
      _ => super::envoy_internal_address::AddressNameSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn address_name_specifier_case(self) -> super::envoy_internal_address::AddressNameSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::envoy_internal_address::AddressNameSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `EnvoyInternalAddressView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EnvoyInternalAddressView<'_> {}

// SAFETY:
// - `EnvoyInternalAddressView` is `Send` because while its alive a `EnvoyInternalAddressMut` cannot.
// - `EnvoyInternalAddressView` does not use thread-local data.
unsafe impl ::std::marker::Send for EnvoyInternalAddressView<'_> {}

impl<'msg> ::protobuf::AsView for EnvoyInternalAddressView<'msg> {
  type Proxied = EnvoyInternalAddress;
  fn as_view(&self) -> ::protobuf::View<'msg, EnvoyInternalAddress> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnvoyInternalAddressView<'msg> {
  fn into_view<'shorter>(self) -> EnvoyInternalAddressView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EnvoyInternalAddress> for EnvoyInternalAddressView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnvoyInternalAddress {
    let mut dst = EnvoyInternalAddress::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EnvoyInternalAddress> for EnvoyInternalAddressMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnvoyInternalAddress {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EnvoyInternalAddress {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnvoyInternalAddressView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EnvoyInternalAddressMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EnvoyInternalAddressMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyInternalAddress>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnvoyInternalAddressMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EnvoyInternalAddressMut<'msg> {
  type Message = EnvoyInternalAddress;
}

impl ::std::fmt::Debug for EnvoyInternalAddressMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyInternalAddress>> for EnvoyInternalAddressMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyInternalAddress>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EnvoyInternalAddressMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EnvoyInternalAddress> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EnvoyInternalAddress {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // server_listener_name: optional string
  pub fn has_server_listener_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_server_listener_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn server_listener_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_server_listener_name().then(|| self.server_listener_name())
  }
  pub fn server_listener_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_server_listener_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // endpoint_id: optional string
  pub fn endpoint_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_endpoint_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn address_name_specifier(&self) -> super::envoy_internal_address::AddressNameSpecifierOneof<'_> {
    match &self.address_name_specifier_case() {
      super::envoy_internal_address::AddressNameSpecifierCase::ServerListenerName =>
          super::envoy_internal_address::AddressNameSpecifierOneof::ServerListenerName(self.server_listener_name()),
      _ => super::envoy_internal_address::AddressNameSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn address_name_specifier_case(&self) -> super::envoy_internal_address::AddressNameSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::envoy_internal_address::AddressNameSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `EnvoyInternalAddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EnvoyInternalAddressMut<'_> {}

// SAFETY:
// - `EnvoyInternalAddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EnvoyInternalAddressMut<'_> {}

impl<'msg> ::protobuf::AsView for EnvoyInternalAddressMut<'msg> {
  type Proxied = EnvoyInternalAddress;
  fn as_view(&self) -> ::protobuf::View<'_, EnvoyInternalAddress> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnvoyInternalAddressMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EnvoyInternalAddress>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EnvoyInternalAddressMut<'msg> {
  type MutProxied = EnvoyInternalAddress;
  fn as_mut(&mut self) -> EnvoyInternalAddressMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EnvoyInternalAddressMut<'msg> {
  fn into_mut<'shorter>(self) -> EnvoyInternalAddressMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EnvoyInternalAddress {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EnvoyInternalAddress> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EnvoyInternalAddressView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EnvoyInternalAddressMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // server_listener_name: optional string
  pub fn has_server_listener_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_server_listener_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn server_listener_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_server_listener_name().then(|| self.server_listener_name())
  }
  pub fn server_listener_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_server_listener_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // endpoint_id: optional string
  pub fn endpoint_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_endpoint_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn address_name_specifier(&self) -> super::envoy_internal_address::AddressNameSpecifierOneof<'_> {
    match &self.address_name_specifier_case() {
      super::envoy_internal_address::AddressNameSpecifierCase::ServerListenerName =>
          super::envoy_internal_address::AddressNameSpecifierOneof::ServerListenerName(self.server_listener_name()),
      _ => super::envoy_internal_address::AddressNameSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn address_name_specifier_case(&self) -> super::envoy_internal_address::AddressNameSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::envoy_internal_address::AddressNameSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl EnvoyInternalAddress

impl ::std::ops::Drop for EnvoyInternalAddress {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EnvoyInternalAddress {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EnvoyInternalAddress {
  type Proxied = Self;
  fn as_view(&self) -> EnvoyInternalAddressView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EnvoyInternalAddress {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EnvoyInternalAddressMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EnvoyInternalAddress {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__EnvoyInternalAddress_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M11P^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__EnvoyInternalAddress_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__EnvoyInternalAddress_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnvoyInternalAddress {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnvoyInternalAddress {
  type Msg = EnvoyInternalAddress;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyInternalAddress> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvoyInternalAddress {
  type Msg = EnvoyInternalAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyInternalAddress> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnvoyInternalAddressMut<'_> {
  type Msg = EnvoyInternalAddress;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyInternalAddress> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvoyInternalAddressMut<'_> {
  type Msg = EnvoyInternalAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyInternalAddress> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnvoyInternalAddressView<'_> {
  type Msg = EnvoyInternalAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnvoyInternalAddress> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnvoyInternalAddressMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod envoy_internal_address {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum AddressNameSpecifierOneof<'msg> {
  ServerListenerName(&'msg ::protobuf::ProtoStr) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum AddressNameSpecifierCase {
  ServerListenerName = 1,

  not_set = 0
}

impl AddressNameSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<AddressNameSpecifierCase> {
    match v {
      0 => Some(AddressNameSpecifierCase::not_set),
      1 => Some(AddressNameSpecifierCase::ServerListenerName),
      _ => None
    }
  }
}
}  // pub mod envoy_internal_address


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__SocketAddress_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SocketAddress {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SocketAddress>
}

impl ::protobuf::Message for SocketAddress {
  type MessageView<'msg> = SocketAddressView<'msg>;
  type MessageMut<'msg> = SocketAddressMut<'msg>;
}

impl ::std::default::Default for SocketAddress {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SocketAddress {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SocketAddress` is `Sync` because it does not implement interior mutability.
//    Neither does `SocketAddressMut`.
unsafe impl ::std::marker::Sync for SocketAddress {}

// SAFETY:
// - `SocketAddress` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SocketAddress {}

impl ::protobuf::Proxied for SocketAddress {
  type View<'msg> = SocketAddressView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SocketAddress {}

impl ::protobuf::MutProxied for SocketAddress {
  type Mut<'msg> = SocketAddressMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SocketAddressView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketAddress>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketAddressView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SocketAddressView<'msg> {
  type Message = SocketAddress;
}

impl ::std::fmt::Debug for SocketAddressView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SocketAddressView<'_> {
  fn default() -> SocketAddressView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SocketAddress>> for SocketAddressView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SocketAddress>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketAddressView<'msg> {

  pub fn to_owned(&self) -> SocketAddress {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // protocol: optional enum envoy.config.core.v3.SocketAddress.Protocol
  pub fn protocol(self) -> super::socket_address::Protocol {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::socket_address::Protocol::Tcp).into()
      ).try_into().unwrap()
    }
  }

  // address: optional string
  pub fn address(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // port_value: optional uint32
  pub fn has_port_value(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn port_value_opt(self) -> ::std::option::Option<u32> {
    self.has_port_value().then(|| self.port_value())
  }
  pub fn port_value(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // named_port: optional string
  pub fn has_named_port(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn named_port_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_named_port().then(|| self.named_port())
  }
  pub fn named_port(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // resolver_name: optional string
  pub fn resolver_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // ipv4_compat: optional bool
  pub fn ipv4_compat(self) -> bool {
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

  // network_namespace_filepath: optional string
  pub fn network_namespace_filepath(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn port_specifier(self) -> super::socket_address::PortSpecifierOneof<'msg> {
    match self.port_specifier_case() {
      super::socket_address::PortSpecifierCase::PortValue =>
          super::socket_address::PortSpecifierOneof::PortValue(self.port_value()),
      super::socket_address::PortSpecifierCase::NamedPort =>
          super::socket_address::PortSpecifierOneof::NamedPort(self.named_port()),
      _ => super::socket_address::PortSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn port_specifier_case(self) -> super::socket_address::PortSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::socket_address::PortSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SocketAddressView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SocketAddressView<'_> {}

// SAFETY:
// - `SocketAddressView` is `Send` because while its alive a `SocketAddressMut` cannot.
// - `SocketAddressView` does not use thread-local data.
unsafe impl ::std::marker::Send for SocketAddressView<'_> {}

impl<'msg> ::protobuf::AsView for SocketAddressView<'msg> {
  type Proxied = SocketAddress;
  fn as_view(&self) -> ::protobuf::View<'msg, SocketAddress> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketAddressView<'msg> {
  fn into_view<'shorter>(self) -> SocketAddressView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketAddress> for SocketAddressView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketAddress {
    let mut dst = SocketAddress::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SocketAddress> for SocketAddressMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SocketAddress {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SocketAddress {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketAddressView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SocketAddressMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SocketAddressMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketAddress>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SocketAddressMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SocketAddressMut<'msg> {
  type Message = SocketAddress;
}

impl ::std::fmt::Debug for SocketAddressMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SocketAddress>> for SocketAddressMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketAddress>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SocketAddressMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SocketAddress> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SocketAddress {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // protocol: optional enum envoy.config.core.v3.SocketAddress.Protocol
  pub fn protocol(&self) -> super::socket_address::Protocol {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::socket_address::Protocol::Tcp).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_protocol(&mut self, val: super::socket_address::Protocol) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // address: optional string
  pub fn address(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_address(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // port_value: optional uint32
  pub fn has_port_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_port_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn port_value_opt(&self) -> ::std::option::Option<u32> {
    self.has_port_value().then(|| self.port_value())
  }
  pub fn port_value(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_port_value(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // named_port: optional string
  pub fn has_named_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_named_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn named_port_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_named_port().then(|| self.named_port())
  }
  pub fn named_port(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_named_port(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // resolver_name: optional string
  pub fn resolver_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resolver_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // ipv4_compat: optional bool
  pub fn ipv4_compat(&self) -> bool {
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
  pub fn set_ipv4_compat(&mut self, val: bool) {
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

  // network_namespace_filepath: optional string
  pub fn network_namespace_filepath(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_network_namespace_filepath(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  pub fn port_specifier(&self) -> super::socket_address::PortSpecifierOneof<'_> {
    match &self.port_specifier_case() {
      super::socket_address::PortSpecifierCase::PortValue =>
          super::socket_address::PortSpecifierOneof::PortValue(self.port_value()),
      super::socket_address::PortSpecifierCase::NamedPort =>
          super::socket_address::PortSpecifierOneof::NamedPort(self.named_port()),
      _ => super::socket_address::PortSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn port_specifier_case(&self) -> super::socket_address::PortSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::socket_address::PortSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SocketAddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SocketAddressMut<'_> {}

// SAFETY:
// - `SocketAddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SocketAddressMut<'_> {}

impl<'msg> ::protobuf::AsView for SocketAddressMut<'msg> {
  type Proxied = SocketAddress;
  fn as_view(&self) -> ::protobuf::View<'_, SocketAddress> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SocketAddressMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SocketAddress>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SocketAddressMut<'msg> {
  type MutProxied = SocketAddress;
  fn as_mut(&mut self) -> SocketAddressMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SocketAddressMut<'msg> {
  fn into_mut<'shorter>(self) -> SocketAddressMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SocketAddress {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SocketAddress> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SocketAddressView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SocketAddressMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // protocol: optional enum envoy.config.core.v3.SocketAddress.Protocol
  pub fn protocol(&self) -> super::socket_address::Protocol {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::socket_address::Protocol::Tcp).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_protocol(&mut self, val: super::socket_address::Protocol) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // address: optional string
  pub fn address(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_address(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // port_value: optional uint32
  pub fn has_port_value(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_port_value(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn port_value_opt(&self) -> ::std::option::Option<u32> {
    self.has_port_value().then(|| self.port_value())
  }
  pub fn port_value(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_port_value(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // named_port: optional string
  pub fn has_named_port(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_named_port(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn named_port_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_named_port().then(|| self.named_port())
  }
  pub fn named_port(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_named_port(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // resolver_name: optional string
  pub fn resolver_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_resolver_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // ipv4_compat: optional bool
  pub fn ipv4_compat(&self) -> bool {
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
  pub fn set_ipv4_compat(&mut self, val: bool) {
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

  // network_namespace_filepath: optional string
  pub fn network_namespace_filepath(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_network_namespace_filepath(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  pub fn port_specifier(&self) -> super::socket_address::PortSpecifierOneof<'_> {
    match &self.port_specifier_case() {
      super::socket_address::PortSpecifierCase::PortValue =>
          super::socket_address::PortSpecifierOneof::PortValue(self.port_value()),
      super::socket_address::PortSpecifierCase::NamedPort =>
          super::socket_address::PortSpecifierOneof::NamedPort(self.named_port()),
      _ => super::socket_address::PortSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn port_specifier_case(&self) -> super::socket_address::PortSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::socket_address::PortSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl SocketAddress

impl ::std::ops::Drop for SocketAddress {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SocketAddress {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SocketAddress {
  type Proxied = Self;
  fn as_view(&self) -> SocketAddressView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SocketAddress {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SocketAddressMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SocketAddress {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__SocketAddress_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P1X)1T1X/P1X^$|%");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__SocketAddress_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__SocketAddress_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketAddress {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketAddress {
  type Msg = SocketAddress;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketAddress> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketAddress {
  type Msg = SocketAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketAddress> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SocketAddressMut<'_> {
  type Msg = SocketAddress;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketAddress> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketAddressMut<'_> {
  type Msg = SocketAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketAddress> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SocketAddressView<'_> {
  type Msg = SocketAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SocketAddress> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SocketAddressMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod socket_address {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Protocol(i32);

#[allow(non_upper_case_globals)]
impl Protocol {
  pub const Tcp: Protocol = Protocol(0);
  pub const Udp: Protocol = Protocol(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Tcp",
      1 => "Udp",
      _ => return None
    })
  }
}

impl ::std::convert::From<Protocol> for i32 {
  fn from(val: Protocol) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Protocol {
  fn from(val: i32) -> Protocol {
    Self(val)
  }
}

impl ::std::default::Default for Protocol {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Protocol {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Protocol::{}", constant_name)
    } else {
      write!(f, "Protocol::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Protocol {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Protocol {}

impl ::protobuf::Proxied for Protocol {
  type View<'a> = Protocol;
}

impl ::protobuf::AsView for Protocol {
  type Proxied = Protocol;

  fn as_view(&self) -> Protocol {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Protocol {
  fn into_view<'shorter>(self) -> Protocol where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Protocol {
  const NAME: &'static str = "Protocol";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for Protocol {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum PortSpecifierOneof<'msg> {
  PortValue(u32) = 3,
  NamedPort(&'msg ::protobuf::ProtoStr) = 4,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum PortSpecifierCase {
  PortValue = 3,
  NamedPort = 4,

  not_set = 0
}

impl PortSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<PortSpecifierCase> {
    match v {
      0 => Some(PortSpecifierCase::not_set),
      3 => Some(PortSpecifierCase::PortValue),
      4 => Some(PortSpecifierCase::NamedPort),
      _ => None
    }
  }
}
}  // pub mod socket_address


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__TcpKeepalive_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TcpKeepalive {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TcpKeepalive>
}

impl ::protobuf::Message for TcpKeepalive {
  type MessageView<'msg> = TcpKeepaliveView<'msg>;
  type MessageMut<'msg> = TcpKeepaliveMut<'msg>;
}

impl ::std::default::Default for TcpKeepalive {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TcpKeepalive {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TcpKeepalive` is `Sync` because it does not implement interior mutability.
//    Neither does `TcpKeepaliveMut`.
unsafe impl ::std::marker::Sync for TcpKeepalive {}

// SAFETY:
// - `TcpKeepalive` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TcpKeepalive {}

impl ::protobuf::Proxied for TcpKeepalive {
  type View<'msg> = TcpKeepaliveView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TcpKeepalive {}

impl ::protobuf::MutProxied for TcpKeepalive {
  type Mut<'msg> = TcpKeepaliveMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TcpKeepaliveView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TcpKeepalive>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TcpKeepaliveView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TcpKeepaliveView<'msg> {
  type Message = TcpKeepalive;
}

impl ::std::fmt::Debug for TcpKeepaliveView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TcpKeepaliveView<'_> {
  fn default() -> TcpKeepaliveView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TcpKeepalive>> for TcpKeepaliveView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TcpKeepalive>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TcpKeepaliveView<'msg> {

  pub fn to_owned(&self) -> TcpKeepalive {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // keepalive_probes: optional message google.protobuf.UInt32Value
  pub fn has_keepalive_probes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn keepalive_probes_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_keepalive_probes().then(|| self.keepalive_probes())
  }
  pub fn keepalive_probes(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // keepalive_time: optional message google.protobuf.UInt32Value
  pub fn has_keepalive_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn keepalive_time_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_keepalive_time().then(|| self.keepalive_time())
  }
  pub fn keepalive_time(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // keepalive_interval: optional message google.protobuf.UInt32Value
  pub fn has_keepalive_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn keepalive_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_keepalive_interval().then(|| self.keepalive_interval())
  }
  pub fn keepalive_interval(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `TcpKeepaliveView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TcpKeepaliveView<'_> {}

// SAFETY:
// - `TcpKeepaliveView` is `Send` because while its alive a `TcpKeepaliveMut` cannot.
// - `TcpKeepaliveView` does not use thread-local data.
unsafe impl ::std::marker::Send for TcpKeepaliveView<'_> {}

impl<'msg> ::protobuf::AsView for TcpKeepaliveView<'msg> {
  type Proxied = TcpKeepalive;
  fn as_view(&self) -> ::protobuf::View<'msg, TcpKeepalive> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TcpKeepaliveView<'msg> {
  fn into_view<'shorter>(self) -> TcpKeepaliveView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TcpKeepalive> for TcpKeepaliveView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TcpKeepalive {
    let mut dst = TcpKeepalive::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TcpKeepalive> for TcpKeepaliveMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TcpKeepalive {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TcpKeepalive {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TcpKeepaliveView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TcpKeepaliveMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TcpKeepaliveMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TcpKeepalive>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TcpKeepaliveMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TcpKeepaliveMut<'msg> {
  type Message = TcpKeepalive;
}

impl ::std::fmt::Debug for TcpKeepaliveMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TcpKeepalive>> for TcpKeepaliveMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TcpKeepalive>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TcpKeepaliveMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TcpKeepalive> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TcpKeepalive {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // keepalive_probes: optional message google.protobuf.UInt32Value
  pub fn has_keepalive_probes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_keepalive_probes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn keepalive_probes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_keepalive_probes().then(|| self.keepalive_probes())
  }
  pub fn keepalive_probes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn keepalive_probes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_keepalive_probes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // keepalive_time: optional message google.protobuf.UInt32Value
  pub fn has_keepalive_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_keepalive_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn keepalive_time_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_keepalive_time().then(|| self.keepalive_time())
  }
  pub fn keepalive_time(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn keepalive_time_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_keepalive_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // keepalive_interval: optional message google.protobuf.UInt32Value
  pub fn has_keepalive_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_keepalive_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn keepalive_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_keepalive_interval().then(|| self.keepalive_interval())
  }
  pub fn keepalive_interval(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn keepalive_interval_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_keepalive_interval(&mut self,
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
// - `TcpKeepaliveMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TcpKeepaliveMut<'_> {}

// SAFETY:
// - `TcpKeepaliveMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TcpKeepaliveMut<'_> {}

impl<'msg> ::protobuf::AsView for TcpKeepaliveMut<'msg> {
  type Proxied = TcpKeepalive;
  fn as_view(&self) -> ::protobuf::View<'_, TcpKeepalive> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TcpKeepaliveMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TcpKeepalive>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TcpKeepaliveMut<'msg> {
  type MutProxied = TcpKeepalive;
  fn as_mut(&mut self) -> TcpKeepaliveMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TcpKeepaliveMut<'msg> {
  fn into_mut<'shorter>(self) -> TcpKeepaliveMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TcpKeepalive {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TcpKeepalive> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TcpKeepaliveView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TcpKeepaliveMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // keepalive_probes: optional message google.protobuf.UInt32Value
  pub fn has_keepalive_probes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_keepalive_probes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn keepalive_probes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_keepalive_probes().then(|| self.keepalive_probes())
  }
  pub fn keepalive_probes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn keepalive_probes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_keepalive_probes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // keepalive_time: optional message google.protobuf.UInt32Value
  pub fn has_keepalive_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_keepalive_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn keepalive_time_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_keepalive_time().then(|| self.keepalive_time())
  }
  pub fn keepalive_time(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn keepalive_time_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_keepalive_time(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // keepalive_interval: optional message google.protobuf.UInt32Value
  pub fn has_keepalive_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_keepalive_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn keepalive_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_keepalive_interval().then(|| self.keepalive_interval())
  }
  pub fn keepalive_interval(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn keepalive_interval_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_keepalive_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl TcpKeepalive

impl ::std::ops::Drop for TcpKeepalive {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TcpKeepalive {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TcpKeepalive {
  type Proxied = Self;
  fn as_view(&self) -> TcpKeepaliveView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TcpKeepalive {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TcpKeepaliveMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TcpKeepalive {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__TcpKeepalive_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__TcpKeepalive_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__TcpKeepalive_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TcpKeepalive {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TcpKeepalive {
  type Msg = TcpKeepalive;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpKeepalive> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TcpKeepalive {
  type Msg = TcpKeepalive;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpKeepalive> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TcpKeepaliveMut<'_> {
  type Msg = TcpKeepalive;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpKeepalive> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TcpKeepaliveMut<'_> {
  type Msg = TcpKeepalive;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpKeepalive> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TcpKeepaliveView<'_> {
  type Msg = TcpKeepalive;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TcpKeepalive> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TcpKeepaliveMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__ExtraSourceAddress_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtraSourceAddress {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtraSourceAddress>
}

impl ::protobuf::Message for ExtraSourceAddress {
  type MessageView<'msg> = ExtraSourceAddressView<'msg>;
  type MessageMut<'msg> = ExtraSourceAddressMut<'msg>;
}

impl ::std::default::Default for ExtraSourceAddress {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtraSourceAddress {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtraSourceAddress` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtraSourceAddressMut`.
unsafe impl ::std::marker::Sync for ExtraSourceAddress {}

// SAFETY:
// - `ExtraSourceAddress` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtraSourceAddress {}

impl ::protobuf::Proxied for ExtraSourceAddress {
  type View<'msg> = ExtraSourceAddressView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtraSourceAddress {}

impl ::protobuf::MutProxied for ExtraSourceAddress {
  type Mut<'msg> = ExtraSourceAddressMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtraSourceAddressView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtraSourceAddress>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtraSourceAddressView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtraSourceAddressView<'msg> {
  type Message = ExtraSourceAddress;
}

impl ::std::fmt::Debug for ExtraSourceAddressView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtraSourceAddressView<'_> {
  fn default() -> ExtraSourceAddressView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtraSourceAddress>> for ExtraSourceAddressView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtraSourceAddress>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtraSourceAddressView<'msg> {

  pub fn to_owned(&self) -> ExtraSourceAddress {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // address: optional message envoy.config.core.v3.SocketAddress
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<super::SocketAddressView<'msg>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> super::SocketAddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SocketAddressView::default())
  }

  // socket_options: optional message envoy.config.core.v3.SocketOptionsOverride
  pub fn has_socket_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn socket_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'msg>> {
    self.has_socket_options().then(|| self.socket_options())
  }
  pub fn socket_options(self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView::default())
  }

}

// SAFETY:
// - `ExtraSourceAddressView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtraSourceAddressView<'_> {}

// SAFETY:
// - `ExtraSourceAddressView` is `Send` because while its alive a `ExtraSourceAddressMut` cannot.
// - `ExtraSourceAddressView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtraSourceAddressView<'_> {}

impl<'msg> ::protobuf::AsView for ExtraSourceAddressView<'msg> {
  type Proxied = ExtraSourceAddress;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtraSourceAddress> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtraSourceAddressView<'msg> {
  fn into_view<'shorter>(self) -> ExtraSourceAddressView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtraSourceAddress> for ExtraSourceAddressView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtraSourceAddress {
    let mut dst = ExtraSourceAddress::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtraSourceAddress> for ExtraSourceAddressMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtraSourceAddress {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtraSourceAddress {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtraSourceAddressView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtraSourceAddressMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtraSourceAddressMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtraSourceAddress>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtraSourceAddressMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtraSourceAddressMut<'msg> {
  type Message = ExtraSourceAddress;
}

impl ::std::fmt::Debug for ExtraSourceAddressMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtraSourceAddress>> for ExtraSourceAddressMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtraSourceAddress>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtraSourceAddressMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtraSourceAddress> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtraSourceAddress {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // address: optional message envoy.config.core.v3.SocketAddress
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
  pub fn address_opt(&self) -> ::std::option::Option<super::SocketAddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> super::SocketAddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SocketAddressView::default())
  }
  pub fn address_mut(&mut self) -> super::SocketAddressMut<'_> {
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
    val: impl ::protobuf::IntoProxied<super::SocketAddress>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // socket_options: optional message envoy.config.core.v3.SocketOptionsOverride
  pub fn has_socket_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_socket_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn socket_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'_>> {
    self.has_socket_options().then(|| self.socket_options())
  }
  pub fn socket_options(&self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView::default())
  }
  pub fn socket_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideMut<'_> {
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
  pub fn set_socket_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverride>) {

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
// - `ExtraSourceAddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtraSourceAddressMut<'_> {}

// SAFETY:
// - `ExtraSourceAddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtraSourceAddressMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtraSourceAddressMut<'msg> {
  type Proxied = ExtraSourceAddress;
  fn as_view(&self) -> ::protobuf::View<'_, ExtraSourceAddress> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtraSourceAddressMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtraSourceAddress>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtraSourceAddressMut<'msg> {
  type MutProxied = ExtraSourceAddress;
  fn as_mut(&mut self) -> ExtraSourceAddressMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtraSourceAddressMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtraSourceAddressMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtraSourceAddress {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtraSourceAddress> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtraSourceAddressView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtraSourceAddressMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // address: optional message envoy.config.core.v3.SocketAddress
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
  pub fn address_opt(&self) -> ::std::option::Option<super::SocketAddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> super::SocketAddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SocketAddressView::default())
  }
  pub fn address_mut(&mut self) -> super::SocketAddressMut<'_> {
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
    val: impl ::protobuf::IntoProxied<super::SocketAddress>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // socket_options: optional message envoy.config.core.v3.SocketOptionsOverride
  pub fn has_socket_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_socket_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn socket_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'_>> {
    self.has_socket_options().then(|| self.socket_options())
  }
  pub fn socket_options(&self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideView::default())
  }
  pub fn socket_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverrideMut<'_> {
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
  pub fn set_socket_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverride>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ExtraSourceAddress

impl ::std::ops::Drop for ExtraSourceAddress {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtraSourceAddress {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtraSourceAddress {
  type Proxied = Self;
  fn as_view(&self) -> ExtraSourceAddressView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtraSourceAddress {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtraSourceAddressMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtraSourceAddress {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__ExtraSourceAddress_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__ExtraSourceAddress_msg_init.0, &[<super::SocketAddress as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::socket_option::SocketOptionsOverride as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__ExtraSourceAddress_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtraSourceAddress {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtraSourceAddress {
  type Msg = ExtraSourceAddress;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtraSourceAddress> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtraSourceAddress {
  type Msg = ExtraSourceAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtraSourceAddress> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtraSourceAddressMut<'_> {
  type Msg = ExtraSourceAddress;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtraSourceAddress> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtraSourceAddressMut<'_> {
  type Msg = ExtraSourceAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtraSourceAddress> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtraSourceAddressView<'_> {
  type Msg = ExtraSourceAddress;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtraSourceAddress> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtraSourceAddressMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__BindConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BindConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BindConfig>
}

impl ::protobuf::Message for BindConfig {
  type MessageView<'msg> = BindConfigView<'msg>;
  type MessageMut<'msg> = BindConfigMut<'msg>;
}

impl ::std::default::Default for BindConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BindConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BindConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `BindConfigMut`.
unsafe impl ::std::marker::Sync for BindConfig {}

// SAFETY:
// - `BindConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BindConfig {}

impl ::protobuf::Proxied for BindConfig {
  type View<'msg> = BindConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BindConfig {}

impl ::protobuf::MutProxied for BindConfig {
  type Mut<'msg> = BindConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BindConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BindConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BindConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BindConfigView<'msg> {
  type Message = BindConfig;
}

impl ::std::fmt::Debug for BindConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BindConfigView<'_> {
  fn default() -> BindConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BindConfig>> for BindConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BindConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BindConfigView<'msg> {

  pub fn to_owned(&self) -> BindConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // source_address: optional message envoy.config.core.v3.SocketAddress
  pub fn has_source_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn source_address_opt(self) -> ::std::option::Option<super::SocketAddressView<'msg>> {
    self.has_source_address().then(|| self.source_address())
  }
  pub fn source_address(self) -> super::SocketAddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SocketAddressView::default())
  }

  // freebind: optional message google.protobuf.BoolValue
  pub fn has_freebind(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn freebind_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_freebind().then(|| self.freebind())
  }
  pub fn freebind(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // extra_source_addresses: repeated message envoy.config.core.v3.ExtraSourceAddress
  pub fn extra_source_addresses(self) -> ::protobuf::RepeatedView<'msg, super::ExtraSourceAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ExtraSourceAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // additional_source_addresses: repeated message envoy.config.core.v3.SocketAddress
  pub fn additional_source_addresses(self) -> ::protobuf::RepeatedView<'msg, super::SocketAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SocketAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // local_address_selector: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_local_address_selector(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn local_address_selector_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_local_address_selector().then(|| self.local_address_selector())
  }
  pub fn local_address_selector(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

}

// SAFETY:
// - `BindConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BindConfigView<'_> {}

// SAFETY:
// - `BindConfigView` is `Send` because while its alive a `BindConfigMut` cannot.
// - `BindConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for BindConfigView<'_> {}

impl<'msg> ::protobuf::AsView for BindConfigView<'msg> {
  type Proxied = BindConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, BindConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BindConfigView<'msg> {
  fn into_view<'shorter>(self) -> BindConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BindConfig> for BindConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BindConfig {
    let mut dst = BindConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BindConfig> for BindConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BindConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BindConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BindConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BindConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BindConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BindConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BindConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BindConfigMut<'msg> {
  type Message = BindConfig;
}

impl ::std::fmt::Debug for BindConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BindConfig>> for BindConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BindConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BindConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BindConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BindConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // source_address: optional message envoy.config.core.v3.SocketAddress
  pub fn has_source_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_source_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn source_address_opt(&self) -> ::std::option::Option<super::SocketAddressView<'_>> {
    self.has_source_address().then(|| self.source_address())
  }
  pub fn source_address(&self) -> super::SocketAddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SocketAddressView::default())
  }
  pub fn source_address_mut(&mut self) -> super::SocketAddressMut<'_> {
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
  pub fn set_source_address(&mut self,
    val: impl ::protobuf::IntoProxied<super::SocketAddress>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // freebind: optional message google.protobuf.BoolValue
  pub fn has_freebind(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_freebind(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn freebind_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_freebind().then(|| self.freebind())
  }
  pub fn freebind(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn freebind_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_freebind(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn socket_options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
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
  pub fn set_socket_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // extra_source_addresses: repeated message envoy.config.core.v3.ExtraSourceAddress
  pub fn extra_source_addresses(&self) -> ::protobuf::RepeatedView<'_, super::ExtraSourceAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ExtraSourceAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn extra_source_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ExtraSourceAddress> {
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
  pub fn set_extra_source_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ExtraSourceAddress>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // additional_source_addresses: repeated message envoy.config.core.v3.SocketAddress
  pub fn additional_source_addresses(&self) -> ::protobuf::RepeatedView<'_, super::SocketAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SocketAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_source_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::SocketAddress> {
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
  pub fn set_additional_source_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::SocketAddress>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // local_address_selector: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_local_address_selector(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_local_address_selector(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn local_address_selector_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_local_address_selector().then(|| self.local_address_selector())
  }
  pub fn local_address_selector(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn local_address_selector_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_local_address_selector(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

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
// - `BindConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BindConfigMut<'_> {}

// SAFETY:
// - `BindConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BindConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for BindConfigMut<'msg> {
  type Proxied = BindConfig;
  fn as_view(&self) -> ::protobuf::View<'_, BindConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BindConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BindConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BindConfigMut<'msg> {
  type MutProxied = BindConfig;
  fn as_mut(&mut self) -> BindConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BindConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> BindConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BindConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BindConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BindConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BindConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // source_address: optional message envoy.config.core.v3.SocketAddress
  pub fn has_source_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_source_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn source_address_opt(&self) -> ::std::option::Option<super::SocketAddressView<'_>> {
    self.has_source_address().then(|| self.source_address())
  }
  pub fn source_address(&self) -> super::SocketAddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SocketAddressView::default())
  }
  pub fn source_address_mut(&mut self) -> super::SocketAddressMut<'_> {
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
  pub fn set_source_address(&mut self,
    val: impl ::protobuf::IntoProxied<super::SocketAddress>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // freebind: optional message google.protobuf.BoolValue
  pub fn has_freebind(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_freebind(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn freebind_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_freebind().then(|| self.freebind())
  }
  pub fn freebind(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn freebind_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_freebind(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn socket_options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
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
  pub fn set_socket_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // extra_source_addresses: repeated message envoy.config.core.v3.ExtraSourceAddress
  pub fn extra_source_addresses(&self) -> ::protobuf::RepeatedView<'_, super::ExtraSourceAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ExtraSourceAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn extra_source_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ExtraSourceAddress> {
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
  pub fn set_extra_source_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ExtraSourceAddress>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // additional_source_addresses: repeated message envoy.config.core.v3.SocketAddress
  pub fn additional_source_addresses(&self) -> ::protobuf::RepeatedView<'_, super::SocketAddress> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SocketAddress>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn additional_source_addresses_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::SocketAddress> {
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
  pub fn set_additional_source_addresses(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::SocketAddress>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // local_address_selector: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_local_address_selector(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_local_address_selector(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn local_address_selector_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_local_address_selector().then(|| self.local_address_selector())
  }
  pub fn local_address_selector(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn local_address_selector_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_local_address_selector(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}  // impl BindConfig

impl ::std::ops::Drop for BindConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BindConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BindConfig {
  type Proxied = Self;
  fn as_view(&self) -> BindConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BindConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BindConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BindConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__BindConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33GGG3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__BindConfig_msg_init.0, &[<super::SocketAddress as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SocketAddress as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ExtraSourceAddress as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__BindConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BindConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BindConfig {
  type Msg = BindConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BindConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BindConfig {
  type Msg = BindConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BindConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BindConfigMut<'_> {
  type Msg = BindConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BindConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BindConfigMut<'_> {
  type Msg = BindConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BindConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BindConfigView<'_> {
  type Msg = BindConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BindConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BindConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__Address_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Address {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Address>
}

impl ::protobuf::Message for Address {
  type MessageView<'msg> = AddressView<'msg>;
  type MessageMut<'msg> = AddressMut<'msg>;
}

impl ::std::default::Default for Address {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Address {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Address` is `Sync` because it does not implement interior mutability.
//    Neither does `AddressMut`.
unsafe impl ::std::marker::Sync for Address {}

// SAFETY:
// - `Address` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Address {}

impl ::protobuf::Proxied for Address {
  type View<'msg> = AddressView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Address {}

impl ::protobuf::MutProxied for Address {
  type Mut<'msg> = AddressMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AddressView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Address>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AddressView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AddressView<'msg> {
  type Message = Address;
}

impl ::std::fmt::Debug for AddressView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AddressView<'_> {
  fn default() -> AddressView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Address>> for AddressView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Address>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AddressView<'msg> {

  pub fn to_owned(&self) -> Address {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // socket_address: optional message envoy.config.core.v3.SocketAddress
  pub fn has_socket_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn socket_address_opt(self) -> ::std::option::Option<super::SocketAddressView<'msg>> {
    self.has_socket_address().then(|| self.socket_address())
  }
  pub fn socket_address(self) -> super::SocketAddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SocketAddressView::default())
  }

  // pipe: optional message envoy.config.core.v3.Pipe
  pub fn has_pipe(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn pipe_opt(self) -> ::std::option::Option<super::PipeView<'msg>> {
    self.has_pipe().then(|| self.pipe())
  }
  pub fn pipe(self) -> super::PipeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PipeView::default())
  }

  // envoy_internal_address: optional message envoy.config.core.v3.EnvoyInternalAddress
  pub fn has_envoy_internal_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn envoy_internal_address_opt(self) -> ::std::option::Option<super::EnvoyInternalAddressView<'msg>> {
    self.has_envoy_internal_address().then(|| self.envoy_internal_address())
  }
  pub fn envoy_internal_address(self) -> super::EnvoyInternalAddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EnvoyInternalAddressView::default())
  }

  pub fn address(self) -> super::address::AddressOneof<'msg> {
    match self.address_case() {
      super::address::AddressCase::SocketAddress =>
          super::address::AddressOneof::SocketAddress(self.socket_address()),
      super::address::AddressCase::Pipe =>
          super::address::AddressOneof::Pipe(self.pipe()),
      super::address::AddressCase::EnvoyInternalAddress =>
          super::address::AddressOneof::EnvoyInternalAddress(self.envoy_internal_address()),
      _ => super::address::AddressOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn address_case(self) -> super::address::AddressCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::address::AddressCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AddressView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AddressView<'_> {}

// SAFETY:
// - `AddressView` is `Send` because while its alive a `AddressMut` cannot.
// - `AddressView` does not use thread-local data.
unsafe impl ::std::marker::Send for AddressView<'_> {}

impl<'msg> ::protobuf::AsView for AddressView<'msg> {
  type Proxied = Address;
  fn as_view(&self) -> ::protobuf::View<'msg, Address> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AddressView<'msg> {
  fn into_view<'shorter>(self) -> AddressView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Address> for AddressView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Address {
    let mut dst = Address::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Address> for AddressMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Address {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Address {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AddressView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AddressMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AddressMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Address>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AddressMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AddressMut<'msg> {
  type Message = Address;
}

impl ::std::fmt::Debug for AddressMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Address>> for AddressMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Address>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AddressMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Address> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Address {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // socket_address: optional message envoy.config.core.v3.SocketAddress
  pub fn has_socket_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_socket_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn socket_address_opt(&self) -> ::std::option::Option<super::SocketAddressView<'_>> {
    self.has_socket_address().then(|| self.socket_address())
  }
  pub fn socket_address(&self) -> super::SocketAddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SocketAddressView::default())
  }
  pub fn socket_address_mut(&mut self) -> super::SocketAddressMut<'_> {
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
  pub fn set_socket_address(&mut self,
    val: impl ::protobuf::IntoProxied<super::SocketAddress>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // pipe: optional message envoy.config.core.v3.Pipe
  pub fn has_pipe(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_pipe(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn pipe_opt(&self) -> ::std::option::Option<super::PipeView<'_>> {
    self.has_pipe().then(|| self.pipe())
  }
  pub fn pipe(&self) -> super::PipeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PipeView::default())
  }
  pub fn pipe_mut(&mut self) -> super::PipeMut<'_> {
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
  pub fn set_pipe(&mut self,
    val: impl ::protobuf::IntoProxied<super::Pipe>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // envoy_internal_address: optional message envoy.config.core.v3.EnvoyInternalAddress
  pub fn has_envoy_internal_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_envoy_internal_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn envoy_internal_address_opt(&self) -> ::std::option::Option<super::EnvoyInternalAddressView<'_>> {
    self.has_envoy_internal_address().then(|| self.envoy_internal_address())
  }
  pub fn envoy_internal_address(&self) -> super::EnvoyInternalAddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EnvoyInternalAddressView::default())
  }
  pub fn envoy_internal_address_mut(&mut self) -> super::EnvoyInternalAddressMut<'_> {
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
  pub fn set_envoy_internal_address(&mut self,
    val: impl ::protobuf::IntoProxied<super::EnvoyInternalAddress>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn address(&self) -> super::address::AddressOneof<'_> {
    match &self.address_case() {
      super::address::AddressCase::SocketAddress =>
          super::address::AddressOneof::SocketAddress(self.socket_address()),
      super::address::AddressCase::Pipe =>
          super::address::AddressOneof::Pipe(self.pipe()),
      super::address::AddressCase::EnvoyInternalAddress =>
          super::address::AddressOneof::EnvoyInternalAddress(self.envoy_internal_address()),
      _ => super::address::AddressOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn address_case(&self) -> super::address::AddressCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::address::AddressCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `AddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AddressMut<'_> {}

// SAFETY:
// - `AddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AddressMut<'_> {}

impl<'msg> ::protobuf::AsView for AddressMut<'msg> {
  type Proxied = Address;
  fn as_view(&self) -> ::protobuf::View<'_, Address> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AddressMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Address>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AddressMut<'msg> {
  type MutProxied = Address;
  fn as_mut(&mut self) -> AddressMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AddressMut<'msg> {
  fn into_mut<'shorter>(self) -> AddressMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Address {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Address> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AddressView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AddressMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // socket_address: optional message envoy.config.core.v3.SocketAddress
  pub fn has_socket_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_socket_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn socket_address_opt(&self) -> ::std::option::Option<super::SocketAddressView<'_>> {
    self.has_socket_address().then(|| self.socket_address())
  }
  pub fn socket_address(&self) -> super::SocketAddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SocketAddressView::default())
  }
  pub fn socket_address_mut(&mut self) -> super::SocketAddressMut<'_> {
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
  pub fn set_socket_address(&mut self,
    val: impl ::protobuf::IntoProxied<super::SocketAddress>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // pipe: optional message envoy.config.core.v3.Pipe
  pub fn has_pipe(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_pipe(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn pipe_opt(&self) -> ::std::option::Option<super::PipeView<'_>> {
    self.has_pipe().then(|| self.pipe())
  }
  pub fn pipe(&self) -> super::PipeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PipeView::default())
  }
  pub fn pipe_mut(&mut self) -> super::PipeMut<'_> {
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
  pub fn set_pipe(&mut self,
    val: impl ::protobuf::IntoProxied<super::Pipe>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // envoy_internal_address: optional message envoy.config.core.v3.EnvoyInternalAddress
  pub fn has_envoy_internal_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_envoy_internal_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn envoy_internal_address_opt(&self) -> ::std::option::Option<super::EnvoyInternalAddressView<'_>> {
    self.has_envoy_internal_address().then(|| self.envoy_internal_address())
  }
  pub fn envoy_internal_address(&self) -> super::EnvoyInternalAddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EnvoyInternalAddressView::default())
  }
  pub fn envoy_internal_address_mut(&mut self) -> super::EnvoyInternalAddressMut<'_> {
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
  pub fn set_envoy_internal_address(&mut self,
    val: impl ::protobuf::IntoProxied<super::EnvoyInternalAddress>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn address(&self) -> super::address::AddressOneof<'_> {
    match &self.address_case() {
      super::address::AddressCase::SocketAddress =>
          super::address::AddressOneof::SocketAddress(self.socket_address()),
      super::address::AddressCase::Pipe =>
          super::address::AddressOneof::Pipe(self.pipe()),
      super::address::AddressCase::EnvoyInternalAddress =>
          super::address::AddressOneof::EnvoyInternalAddress(self.envoy_internal_address()),
      _ => super::address::AddressOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn address_case(&self) -> super::address::AddressCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::address::AddressCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Address

impl ::std::ops::Drop for Address {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Address {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Address {
  type Proxied = Self;
  fn as_view(&self) -> AddressView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Address {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AddressMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Address {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__Address_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^!|#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__Address_msg_init.0, &[<super::SocketAddress as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Pipe as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::EnvoyInternalAddress as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__Address_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Address {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Address {
  type Msg = Address;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Address {
  type Msg = Address;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AddressMut<'_> {
  type Msg = Address;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AddressMut<'_> {
  type Msg = Address;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AddressView<'_> {
  type Msg = Address;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AddressMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod address {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum AddressOneof<'msg> {
  SocketAddress(::protobuf::View<'msg, super::super::SocketAddress>) = 1,
  Pipe(::protobuf::View<'msg, super::super::Pipe>) = 2,
  EnvoyInternalAddress(::protobuf::View<'msg, super::super::EnvoyInternalAddress>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum AddressCase {
  SocketAddress = 1,
  Pipe = 2,
  EnvoyInternalAddress = 3,

  not_set = 0
}

impl AddressCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<AddressCase> {
    match v {
      0 => Some(AddressCase::not_set),
      1 => Some(AddressCase::SocketAddress),
      2 => Some(AddressCase::Pipe),
      3 => Some(AddressCase::EnvoyInternalAddress),
      _ => None
    }
  }
}
}  // pub mod address


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__core__v3__CidrRange_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CidrRange {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CidrRange>
}

impl ::protobuf::Message for CidrRange {
  type MessageView<'msg> = CidrRangeView<'msg>;
  type MessageMut<'msg> = CidrRangeMut<'msg>;
}

impl ::std::default::Default for CidrRange {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CidrRange {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CidrRange` is `Sync` because it does not implement interior mutability.
//    Neither does `CidrRangeMut`.
unsafe impl ::std::marker::Sync for CidrRange {}

// SAFETY:
// - `CidrRange` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CidrRange {}

impl ::protobuf::Proxied for CidrRange {
  type View<'msg> = CidrRangeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CidrRange {}

impl ::protobuf::MutProxied for CidrRange {
  type Mut<'msg> = CidrRangeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CidrRangeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CidrRange>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CidrRangeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CidrRangeView<'msg> {
  type Message = CidrRange;
}

impl ::std::fmt::Debug for CidrRangeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CidrRangeView<'_> {
  fn default() -> CidrRangeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CidrRange>> for CidrRangeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CidrRange>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CidrRangeView<'msg> {

  pub fn to_owned(&self) -> CidrRange {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // address_prefix: optional string
  pub fn address_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // prefix_len: optional message google.protobuf.UInt32Value
  pub fn has_prefix_len(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn prefix_len_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_prefix_len().then(|| self.prefix_len())
  }
  pub fn prefix_len(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `CidrRangeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CidrRangeView<'_> {}

// SAFETY:
// - `CidrRangeView` is `Send` because while its alive a `CidrRangeMut` cannot.
// - `CidrRangeView` does not use thread-local data.
unsafe impl ::std::marker::Send for CidrRangeView<'_> {}

impl<'msg> ::protobuf::AsView for CidrRangeView<'msg> {
  type Proxied = CidrRange;
  fn as_view(&self) -> ::protobuf::View<'msg, CidrRange> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CidrRangeView<'msg> {
  fn into_view<'shorter>(self) -> CidrRangeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CidrRange> for CidrRangeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CidrRange {
    let mut dst = CidrRange::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CidrRange> for CidrRangeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CidrRange {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CidrRange {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CidrRangeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CidrRangeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CidrRangeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CidrRange>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CidrRangeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CidrRangeMut<'msg> {
  type Message = CidrRange;
}

impl ::std::fmt::Debug for CidrRangeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CidrRange>> for CidrRangeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CidrRange>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CidrRangeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CidrRange> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CidrRange {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // address_prefix: optional string
  pub fn address_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_address_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // prefix_len: optional message google.protobuf.UInt32Value
  pub fn has_prefix_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_prefix_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn prefix_len_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_prefix_len().then(|| self.prefix_len())
  }
  pub fn prefix_len(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn prefix_len_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_prefix_len(&mut self,
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
// - `CidrRangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CidrRangeMut<'_> {}

// SAFETY:
// - `CidrRangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CidrRangeMut<'_> {}

impl<'msg> ::protobuf::AsView for CidrRangeMut<'msg> {
  type Proxied = CidrRange;
  fn as_view(&self) -> ::protobuf::View<'_, CidrRange> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CidrRangeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CidrRange>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CidrRangeMut<'msg> {
  type MutProxied = CidrRange;
  fn as_mut(&mut self) -> CidrRangeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CidrRangeMut<'msg> {
  fn into_mut<'shorter>(self) -> CidrRangeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CidrRange {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CidrRange> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CidrRangeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CidrRangeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // address_prefix: optional string
  pub fn address_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_address_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // prefix_len: optional message google.protobuf.UInt32Value
  pub fn has_prefix_len(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_prefix_len(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn prefix_len_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_prefix_len().then(|| self.prefix_len())
  }
  pub fn prefix_len(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn prefix_len_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_prefix_len(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl CidrRange

impl ::std::ops::Drop for CidrRange {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CidrRange {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CidrRange {
  type Proxied = Self;
  fn as_view(&self) -> CidrRangeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CidrRange {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CidrRangeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CidrRange {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__core__v3__CidrRange_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__core__v3__CidrRange_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__core__v3__CidrRange_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CidrRange {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CidrRange {
  type Msg = CidrRange;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CidrRange {
  type Msg = CidrRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CidrRangeMut<'_> {
  type Msg = CidrRange;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CidrRangeMut<'_> {
  type Msg = CidrRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CidrRangeView<'_> {
  type Msg = CidrRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CidrRange> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CidrRangeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



