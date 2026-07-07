const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__UpstreamTlsContext_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpstreamTlsContext {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpstreamTlsContext>
}

impl ::protobuf::Message for UpstreamTlsContext {
  type MessageView<'msg> = UpstreamTlsContextView<'msg>;
  type MessageMut<'msg> = UpstreamTlsContextMut<'msg>;
}

impl ::std::default::Default for UpstreamTlsContext {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpstreamTlsContext {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpstreamTlsContext` is `Sync` because it does not implement interior mutability.
//    Neither does `UpstreamTlsContextMut`.
unsafe impl ::std::marker::Sync for UpstreamTlsContext {}

// SAFETY:
// - `UpstreamTlsContext` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamTlsContext {}

impl ::protobuf::Proxied for UpstreamTlsContext {
  type View<'msg> = UpstreamTlsContextView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpstreamTlsContext {}

impl ::protobuf::MutProxied for UpstreamTlsContext {
  type Mut<'msg> = UpstreamTlsContextMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpstreamTlsContextView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamTlsContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamTlsContextView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpstreamTlsContextView<'msg> {
  type Message = UpstreamTlsContext;
}

impl ::std::fmt::Debug for UpstreamTlsContextView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpstreamTlsContextView<'_> {
  fn default() -> UpstreamTlsContextView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamTlsContext>> for UpstreamTlsContextView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamTlsContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamTlsContextView<'msg> {

  pub fn to_owned(&self) -> UpstreamTlsContext {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // common_tls_context: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext
  pub fn has_common_tls_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn common_tls_context_opt(self) -> ::std::option::Option<super::CommonTlsContextView<'msg>> {
    self.has_common_tls_context().then(|| self.common_tls_context())
  }
  pub fn common_tls_context(self) -> super::CommonTlsContextView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonTlsContextView::default())
  }

  // sni: optional string
  pub fn sni(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // auto_host_sni: optional bool
  pub fn auto_host_sni(self) -> bool {
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

  // auto_sni_san_validation: optional bool
  pub fn auto_sni_san_validation(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }

  // allow_renegotiation: optional bool
  pub fn allow_renegotiation(self) -> bool {
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

  // max_session_keys: optional message google.protobuf.UInt32Value
  pub fn has_max_session_keys(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn max_session_keys_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_session_keys().then(|| self.max_session_keys())
  }
  pub fn max_session_keys(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // enforce_rsa_key_usage: optional message google.protobuf.BoolValue
  pub fn has_enforce_rsa_key_usage(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn enforce_rsa_key_usage_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_enforce_rsa_key_usage().then(|| self.enforce_rsa_key_usage())
  }
  pub fn enforce_rsa_key_usage(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

}

// SAFETY:
// - `UpstreamTlsContextView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpstreamTlsContextView<'_> {}

// SAFETY:
// - `UpstreamTlsContextView` is `Send` because while its alive a `UpstreamTlsContextMut` cannot.
// - `UpstreamTlsContextView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamTlsContextView<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamTlsContextView<'msg> {
  type Proxied = UpstreamTlsContext;
  fn as_view(&self) -> ::protobuf::View<'msg, UpstreamTlsContext> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamTlsContextView<'msg> {
  fn into_view<'shorter>(self) -> UpstreamTlsContextView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamTlsContext> for UpstreamTlsContextView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamTlsContext {
    let mut dst = UpstreamTlsContext::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamTlsContext> for UpstreamTlsContextMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamTlsContext {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UpstreamTlsContext {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamTlsContextView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamTlsContextMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpstreamTlsContextMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamTlsContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamTlsContextMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpstreamTlsContextMut<'msg> {
  type Message = UpstreamTlsContext;
}

impl ::std::fmt::Debug for UpstreamTlsContextMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamTlsContext>> for UpstreamTlsContextMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamTlsContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamTlsContextMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamTlsContext> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UpstreamTlsContext {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // common_tls_context: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext
  pub fn has_common_tls_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_common_tls_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn common_tls_context_opt(&self) -> ::std::option::Option<super::CommonTlsContextView<'_>> {
    self.has_common_tls_context().then(|| self.common_tls_context())
  }
  pub fn common_tls_context(&self) -> super::CommonTlsContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonTlsContextView::default())
  }
  pub fn common_tls_context_mut(&mut self) -> super::CommonTlsContextMut<'_> {
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
  pub fn set_common_tls_context(&mut self,
    val: impl ::protobuf::IntoProxied<super::CommonTlsContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // sni: optional string
  pub fn sni(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_sni(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // auto_host_sni: optional bool
  pub fn auto_host_sni(&self) -> bool {
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
  pub fn set_auto_host_sni(&mut self, val: bool) {
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

  // auto_sni_san_validation: optional bool
  pub fn auto_sni_san_validation(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_auto_sni_san_validation(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // allow_renegotiation: optional bool
  pub fn allow_renegotiation(&self) -> bool {
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
  pub fn set_allow_renegotiation(&mut self, val: bool) {
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

  // max_session_keys: optional message google.protobuf.UInt32Value
  pub fn has_max_session_keys(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_max_session_keys(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn max_session_keys_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_session_keys().then(|| self.max_session_keys())
  }
  pub fn max_session_keys(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_session_keys_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_session_keys(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // enforce_rsa_key_usage: optional message google.protobuf.BoolValue
  pub fn has_enforce_rsa_key_usage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_enforce_rsa_key_usage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn enforce_rsa_key_usage_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enforce_rsa_key_usage().then(|| self.enforce_rsa_key_usage())
  }
  pub fn enforce_rsa_key_usage(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enforce_rsa_key_usage_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enforce_rsa_key_usage(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}

// SAFETY:
// - `UpstreamTlsContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpstreamTlsContextMut<'_> {}

// SAFETY:
// - `UpstreamTlsContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpstreamTlsContextMut<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamTlsContextMut<'msg> {
  type Proxied = UpstreamTlsContext;
  fn as_view(&self) -> ::protobuf::View<'_, UpstreamTlsContext> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamTlsContextMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpstreamTlsContext>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpstreamTlsContextMut<'msg> {
  type MutProxied = UpstreamTlsContext;
  fn as_mut(&mut self) -> UpstreamTlsContextMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpstreamTlsContextMut<'msg> {
  fn into_mut<'shorter>(self) -> UpstreamTlsContextMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpstreamTlsContext {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpstreamTlsContext> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpstreamTlsContextView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpstreamTlsContextMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // common_tls_context: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext
  pub fn has_common_tls_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_common_tls_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn common_tls_context_opt(&self) -> ::std::option::Option<super::CommonTlsContextView<'_>> {
    self.has_common_tls_context().then(|| self.common_tls_context())
  }
  pub fn common_tls_context(&self) -> super::CommonTlsContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonTlsContextView::default())
  }
  pub fn common_tls_context_mut(&mut self) -> super::CommonTlsContextMut<'_> {
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
  pub fn set_common_tls_context(&mut self,
    val: impl ::protobuf::IntoProxied<super::CommonTlsContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // sni: optional string
  pub fn sni(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_sni(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // auto_host_sni: optional bool
  pub fn auto_host_sni(&self) -> bool {
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
  pub fn set_auto_host_sni(&mut self, val: bool) {
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

  // auto_sni_san_validation: optional bool
  pub fn auto_sni_san_validation(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_auto_sni_san_validation(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // allow_renegotiation: optional bool
  pub fn allow_renegotiation(&self) -> bool {
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
  pub fn set_allow_renegotiation(&mut self, val: bool) {
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

  // max_session_keys: optional message google.protobuf.UInt32Value
  pub fn has_max_session_keys(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_max_session_keys(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn max_session_keys_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_session_keys().then(|| self.max_session_keys())
  }
  pub fn max_session_keys(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_session_keys_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_session_keys(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // enforce_rsa_key_usage: optional message google.protobuf.BoolValue
  pub fn has_enforce_rsa_key_usage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_enforce_rsa_key_usage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn enforce_rsa_key_usage_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_enforce_rsa_key_usage().then(|| self.enforce_rsa_key_usage())
  }
  pub fn enforce_rsa_key_usage(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn enforce_rsa_key_usage_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_enforce_rsa_key_usage(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}  // impl UpstreamTlsContext

impl ::std::ops::Drop for UpstreamTlsContext {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpstreamTlsContext {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpstreamTlsContext {
  type Proxied = Self;
  fn as_view(&self) -> UpstreamTlsContextView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpstreamTlsContext {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpstreamTlsContextMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpstreamTlsContext {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__UpstreamTlsContext_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X/P33/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__UpstreamTlsContext_msg_init.0, &[<super::CommonTlsContext as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__UpstreamTlsContext_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamTlsContext {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamTlsContext {
  type Msg = UpstreamTlsContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamTlsContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamTlsContext {
  type Msg = UpstreamTlsContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamTlsContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamTlsContextMut<'_> {
  type Msg = UpstreamTlsContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamTlsContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamTlsContextMut<'_> {
  type Msg = UpstreamTlsContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamTlsContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamTlsContextView<'_> {
  type Msg = UpstreamTlsContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamTlsContext> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamTlsContextMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__DownstreamTlsContext_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DownstreamTlsContext {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DownstreamTlsContext>
}

impl ::protobuf::Message for DownstreamTlsContext {
  type MessageView<'msg> = DownstreamTlsContextView<'msg>;
  type MessageMut<'msg> = DownstreamTlsContextMut<'msg>;
}

impl ::std::default::Default for DownstreamTlsContext {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DownstreamTlsContext {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DownstreamTlsContext` is `Sync` because it does not implement interior mutability.
//    Neither does `DownstreamTlsContextMut`.
unsafe impl ::std::marker::Sync for DownstreamTlsContext {}

// SAFETY:
// - `DownstreamTlsContext` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DownstreamTlsContext {}

impl ::protobuf::Proxied for DownstreamTlsContext {
  type View<'msg> = DownstreamTlsContextView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DownstreamTlsContext {}

impl ::protobuf::MutProxied for DownstreamTlsContext {
  type Mut<'msg> = DownstreamTlsContextMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DownstreamTlsContextView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DownstreamTlsContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DownstreamTlsContextView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DownstreamTlsContextView<'msg> {
  type Message = DownstreamTlsContext;
}

impl ::std::fmt::Debug for DownstreamTlsContextView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DownstreamTlsContextView<'_> {
  fn default() -> DownstreamTlsContextView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DownstreamTlsContext>> for DownstreamTlsContextView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DownstreamTlsContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DownstreamTlsContextView<'msg> {

  pub fn to_owned(&self) -> DownstreamTlsContext {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // common_tls_context: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext
  pub fn has_common_tls_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn common_tls_context_opt(self) -> ::std::option::Option<super::CommonTlsContextView<'msg>> {
    self.has_common_tls_context().then(|| self.common_tls_context())
  }
  pub fn common_tls_context(self) -> super::CommonTlsContextView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonTlsContextView::default())
  }

  // require_client_certificate: optional message google.protobuf.BoolValue
  pub fn has_require_client_certificate(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn require_client_certificate_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_require_client_certificate().then(|| self.require_client_certificate())
  }
  pub fn require_client_certificate(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // require_sni: optional message google.protobuf.BoolValue
  pub fn has_require_sni(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn require_sni_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_require_sni().then(|| self.require_sni())
  }
  pub fn require_sni(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // session_ticket_keys: optional message envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys
  pub fn has_session_ticket_keys(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn session_ticket_keys_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'msg>> {
    self.has_session_ticket_keys().then(|| self.session_ticket_keys())
  }
  pub fn session_ticket_keys(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView::default())
  }

  // session_ticket_keys_sds_secret_config: optional message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn has_session_ticket_keys_sds_secret_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn session_ticket_keys_sds_secret_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'msg>> {
    self.has_session_ticket_keys_sds_secret_config().then(|| self.session_ticket_keys_sds_secret_config())
  }
  pub fn session_ticket_keys_sds_secret_config(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView::default())
  }

  // disable_stateless_session_resumption: optional bool
  pub fn has_disable_stateless_session_resumption(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn disable_stateless_session_resumption_opt(self) -> ::std::option::Option<bool> {
    self.has_disable_stateless_session_resumption().then(|| self.disable_stateless_session_resumption())
  }
  pub fn disable_stateless_session_resumption(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }

  // disable_stateful_session_resumption: optional bool
  pub fn disable_stateful_session_resumption(self) -> bool {
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

  // session_timeout: optional message google.protobuf.Duration
  pub fn has_session_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn session_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_session_timeout().then(|| self.session_timeout())
  }
  pub fn session_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // ocsp_staple_policy: optional enum envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext.OcspStaplePolicy
  pub fn ocsp_staple_policy(self) -> super::downstream_tls_context::OcspStaplePolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::downstream_tls_context::OcspStaplePolicy::LenientStapling).into()
      ).try_into().unwrap()
    }
  }

  // full_scan_certs_on_sni_mismatch: optional message google.protobuf.BoolValue
  pub fn has_full_scan_certs_on_sni_mismatch(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn full_scan_certs_on_sni_mismatch_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_full_scan_certs_on_sni_mismatch().then(|| self.full_scan_certs_on_sni_mismatch())
  }
  pub fn full_scan_certs_on_sni_mismatch(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // prefer_client_ciphers: optional bool
  pub fn prefer_client_ciphers(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }

  pub fn session_ticket_keys_type(self) -> super::downstream_tls_context::SessionTicketKeysTypeOneof<'msg> {
    match self.session_ticket_keys_type_case() {
      super::downstream_tls_context::SessionTicketKeysTypeCase::SessionTicketKeys =>
          super::downstream_tls_context::SessionTicketKeysTypeOneof::SessionTicketKeys(self.session_ticket_keys()),
      super::downstream_tls_context::SessionTicketKeysTypeCase::SessionTicketKeysSdsSecretConfig =>
          super::downstream_tls_context::SessionTicketKeysTypeOneof::SessionTicketKeysSdsSecretConfig(self.session_ticket_keys_sds_secret_config()),
      super::downstream_tls_context::SessionTicketKeysTypeCase::DisableStatelessSessionResumption =>
          super::downstream_tls_context::SessionTicketKeysTypeOneof::DisableStatelessSessionResumption(self.disable_stateless_session_resumption()),
      _ => super::downstream_tls_context::SessionTicketKeysTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn session_ticket_keys_type_case(self) -> super::downstream_tls_context::SessionTicketKeysTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::downstream_tls_context::SessionTicketKeysTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DownstreamTlsContextView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DownstreamTlsContextView<'_> {}

// SAFETY:
// - `DownstreamTlsContextView` is `Send` because while its alive a `DownstreamTlsContextMut` cannot.
// - `DownstreamTlsContextView` does not use thread-local data.
unsafe impl ::std::marker::Send for DownstreamTlsContextView<'_> {}

impl<'msg> ::protobuf::AsView for DownstreamTlsContextView<'msg> {
  type Proxied = DownstreamTlsContext;
  fn as_view(&self) -> ::protobuf::View<'msg, DownstreamTlsContext> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DownstreamTlsContextView<'msg> {
  fn into_view<'shorter>(self) -> DownstreamTlsContextView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DownstreamTlsContext> for DownstreamTlsContextView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DownstreamTlsContext {
    let mut dst = DownstreamTlsContext::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DownstreamTlsContext> for DownstreamTlsContextMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DownstreamTlsContext {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DownstreamTlsContext {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DownstreamTlsContextView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DownstreamTlsContextMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DownstreamTlsContextMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DownstreamTlsContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DownstreamTlsContextMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DownstreamTlsContextMut<'msg> {
  type Message = DownstreamTlsContext;
}

impl ::std::fmt::Debug for DownstreamTlsContextMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DownstreamTlsContext>> for DownstreamTlsContextMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DownstreamTlsContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DownstreamTlsContextMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DownstreamTlsContext> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DownstreamTlsContext {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // common_tls_context: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext
  pub fn has_common_tls_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_common_tls_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn common_tls_context_opt(&self) -> ::std::option::Option<super::CommonTlsContextView<'_>> {
    self.has_common_tls_context().then(|| self.common_tls_context())
  }
  pub fn common_tls_context(&self) -> super::CommonTlsContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonTlsContextView::default())
  }
  pub fn common_tls_context_mut(&mut self) -> super::CommonTlsContextMut<'_> {
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
  pub fn set_common_tls_context(&mut self,
    val: impl ::protobuf::IntoProxied<super::CommonTlsContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // require_client_certificate: optional message google.protobuf.BoolValue
  pub fn has_require_client_certificate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_require_client_certificate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn require_client_certificate_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_require_client_certificate().then(|| self.require_client_certificate())
  }
  pub fn require_client_certificate(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn require_client_certificate_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_require_client_certificate(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // require_sni: optional message google.protobuf.BoolValue
  pub fn has_require_sni(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_require_sni(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn require_sni_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_require_sni().then(|| self.require_sni())
  }
  pub fn require_sni(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn require_sni_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_require_sni(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // session_ticket_keys: optional message envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys
  pub fn has_session_ticket_keys(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_session_ticket_keys(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn session_ticket_keys_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'_>> {
    self.has_session_ticket_keys().then(|| self.session_ticket_keys())
  }
  pub fn session_ticket_keys(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView::default())
  }
  pub fn session_ticket_keys_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysMut<'_> {
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
  pub fn set_session_ticket_keys(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeys>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // session_ticket_keys_sds_secret_config: optional message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn has_session_ticket_keys_sds_secret_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_session_ticket_keys_sds_secret_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn session_ticket_keys_sds_secret_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_>> {
    self.has_session_ticket_keys_sds_secret_config().then(|| self.session_ticket_keys_sds_secret_config())
  }
  pub fn session_ticket_keys_sds_secret_config(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView::default())
  }
  pub fn session_ticket_keys_sds_secret_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigMut<'_> {
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
  pub fn set_session_ticket_keys_sds_secret_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // disable_stateless_session_resumption: optional bool
  pub fn has_disable_stateless_session_resumption(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_disable_stateless_session_resumption(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn disable_stateless_session_resumption_opt(&self) -> ::std::option::Option<bool> {
    self.has_disable_stateless_session_resumption().then(|| self.disable_stateless_session_resumption())
  }
  pub fn disable_stateless_session_resumption(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_stateless_session_resumption(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // disable_stateful_session_resumption: optional bool
  pub fn disable_stateful_session_resumption(&self) -> bool {
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
  pub fn set_disable_stateful_session_resumption(&mut self, val: bool) {
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

  // session_timeout: optional message google.protobuf.Duration
  pub fn has_session_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_session_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn session_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_session_timeout().then(|| self.session_timeout())
  }
  pub fn session_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn session_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_session_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // ocsp_staple_policy: optional enum envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext.OcspStaplePolicy
  pub fn ocsp_staple_policy(&self) -> super::downstream_tls_context::OcspStaplePolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::downstream_tls_context::OcspStaplePolicy::LenientStapling).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ocsp_staple_policy(&mut self, val: super::downstream_tls_context::OcspStaplePolicy) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        7, val.into()
      )
    }
  }

  // full_scan_certs_on_sni_mismatch: optional message google.protobuf.BoolValue
  pub fn has_full_scan_certs_on_sni_mismatch(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_full_scan_certs_on_sni_mismatch(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn full_scan_certs_on_sni_mismatch_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_full_scan_certs_on_sni_mismatch().then(|| self.full_scan_certs_on_sni_mismatch())
  }
  pub fn full_scan_certs_on_sni_mismatch(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn full_scan_certs_on_sni_mismatch_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_full_scan_certs_on_sni_mismatch(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // prefer_client_ciphers: optional bool
  pub fn prefer_client_ciphers(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_prefer_client_ciphers(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        10, val.into()
      )
    }
  }

  pub fn session_ticket_keys_type(&self) -> super::downstream_tls_context::SessionTicketKeysTypeOneof<'_> {
    match &self.session_ticket_keys_type_case() {
      super::downstream_tls_context::SessionTicketKeysTypeCase::SessionTicketKeys =>
          super::downstream_tls_context::SessionTicketKeysTypeOneof::SessionTicketKeys(self.session_ticket_keys()),
      super::downstream_tls_context::SessionTicketKeysTypeCase::SessionTicketKeysSdsSecretConfig =>
          super::downstream_tls_context::SessionTicketKeysTypeOneof::SessionTicketKeysSdsSecretConfig(self.session_ticket_keys_sds_secret_config()),
      super::downstream_tls_context::SessionTicketKeysTypeCase::DisableStatelessSessionResumption =>
          super::downstream_tls_context::SessionTicketKeysTypeOneof::DisableStatelessSessionResumption(self.disable_stateless_session_resumption()),
      _ => super::downstream_tls_context::SessionTicketKeysTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn session_ticket_keys_type_case(&self) -> super::downstream_tls_context::SessionTicketKeysTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::downstream_tls_context::SessionTicketKeysTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `DownstreamTlsContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DownstreamTlsContextMut<'_> {}

// SAFETY:
// - `DownstreamTlsContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DownstreamTlsContextMut<'_> {}

impl<'msg> ::protobuf::AsView for DownstreamTlsContextMut<'msg> {
  type Proxied = DownstreamTlsContext;
  fn as_view(&self) -> ::protobuf::View<'_, DownstreamTlsContext> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DownstreamTlsContextMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DownstreamTlsContext>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DownstreamTlsContextMut<'msg> {
  type MutProxied = DownstreamTlsContext;
  fn as_mut(&mut self) -> DownstreamTlsContextMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DownstreamTlsContextMut<'msg> {
  fn into_mut<'shorter>(self) -> DownstreamTlsContextMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DownstreamTlsContext {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DownstreamTlsContext> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DownstreamTlsContextView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DownstreamTlsContextMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // common_tls_context: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext
  pub fn has_common_tls_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_common_tls_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn common_tls_context_opt(&self) -> ::std::option::Option<super::CommonTlsContextView<'_>> {
    self.has_common_tls_context().then(|| self.common_tls_context())
  }
  pub fn common_tls_context(&self) -> super::CommonTlsContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CommonTlsContextView::default())
  }
  pub fn common_tls_context_mut(&mut self) -> super::CommonTlsContextMut<'_> {
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
  pub fn set_common_tls_context(&mut self,
    val: impl ::protobuf::IntoProxied<super::CommonTlsContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // require_client_certificate: optional message google.protobuf.BoolValue
  pub fn has_require_client_certificate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_require_client_certificate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn require_client_certificate_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_require_client_certificate().then(|| self.require_client_certificate())
  }
  pub fn require_client_certificate(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn require_client_certificate_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_require_client_certificate(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // require_sni: optional message google.protobuf.BoolValue
  pub fn has_require_sni(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_require_sni(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn require_sni_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_require_sni().then(|| self.require_sni())
  }
  pub fn require_sni(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn require_sni_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_require_sni(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // session_ticket_keys: optional message envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys
  pub fn has_session_ticket_keys(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_session_ticket_keys(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn session_ticket_keys_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'_>> {
    self.has_session_ticket_keys().then(|| self.session_ticket_keys())
  }
  pub fn session_ticket_keys(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysView::default())
  }
  pub fn session_ticket_keys_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeysMut<'_> {
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
  pub fn set_session_ticket_keys(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeys>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // session_ticket_keys_sds_secret_config: optional message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn has_session_ticket_keys_sds_secret_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_session_ticket_keys_sds_secret_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn session_ticket_keys_sds_secret_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_>> {
    self.has_session_ticket_keys_sds_secret_config().then(|| self.session_ticket_keys_sds_secret_config())
  }
  pub fn session_ticket_keys_sds_secret_config(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView::default())
  }
  pub fn session_ticket_keys_sds_secret_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigMut<'_> {
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
  pub fn set_session_ticket_keys_sds_secret_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // disable_stateless_session_resumption: optional bool
  pub fn has_disable_stateless_session_resumption(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_disable_stateless_session_resumption(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn disable_stateless_session_resumption_opt(&self) -> ::std::option::Option<bool> {
    self.has_disable_stateless_session_resumption().then(|| self.disable_stateless_session_resumption())
  }
  pub fn disable_stateless_session_resumption(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_disable_stateless_session_resumption(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // disable_stateful_session_resumption: optional bool
  pub fn disable_stateful_session_resumption(&self) -> bool {
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
  pub fn set_disable_stateful_session_resumption(&mut self, val: bool) {
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

  // session_timeout: optional message google.protobuf.Duration
  pub fn has_session_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_session_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn session_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_session_timeout().then(|| self.session_timeout())
  }
  pub fn session_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn session_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_session_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // ocsp_staple_policy: optional enum envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext.OcspStaplePolicy
  pub fn ocsp_staple_policy(&self) -> super::downstream_tls_context::OcspStaplePolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::downstream_tls_context::OcspStaplePolicy::LenientStapling).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ocsp_staple_policy(&mut self, val: super::downstream_tls_context::OcspStaplePolicy) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        7, val.into()
      )
    }
  }

  // full_scan_certs_on_sni_mismatch: optional message google.protobuf.BoolValue
  pub fn has_full_scan_certs_on_sni_mismatch(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_full_scan_certs_on_sni_mismatch(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn full_scan_certs_on_sni_mismatch_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_full_scan_certs_on_sni_mismatch().then(|| self.full_scan_certs_on_sni_mismatch())
  }
  pub fn full_scan_certs_on_sni_mismatch(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn full_scan_certs_on_sni_mismatch_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_full_scan_certs_on_sni_mismatch(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // prefer_client_ciphers: optional bool
  pub fn prefer_client_ciphers(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_prefer_client_ciphers(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        10, val.into()
      )
    }
  }

  pub fn session_ticket_keys_type(&self) -> super::downstream_tls_context::SessionTicketKeysTypeOneof<'_> {
    match &self.session_ticket_keys_type_case() {
      super::downstream_tls_context::SessionTicketKeysTypeCase::SessionTicketKeys =>
          super::downstream_tls_context::SessionTicketKeysTypeOneof::SessionTicketKeys(self.session_ticket_keys()),
      super::downstream_tls_context::SessionTicketKeysTypeCase::SessionTicketKeysSdsSecretConfig =>
          super::downstream_tls_context::SessionTicketKeysTypeOneof::SessionTicketKeysSdsSecretConfig(self.session_ticket_keys_sds_secret_config()),
      super::downstream_tls_context::SessionTicketKeysTypeCase::DisableStatelessSessionResumption =>
          super::downstream_tls_context::SessionTicketKeysTypeOneof::DisableStatelessSessionResumption(self.disable_stateless_session_resumption()),
      _ => super::downstream_tls_context::SessionTicketKeysTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn session_ticket_keys_type_case(&self) -> super::downstream_tls_context::SessionTicketKeysTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(3);
      super::downstream_tls_context::SessionTicketKeysTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl DownstreamTlsContext

impl ::std::ops::Drop for DownstreamTlsContext {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DownstreamTlsContext {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DownstreamTlsContext {
  type Proxied = Self;
  fn as_view(&self) -> DownstreamTlsContextView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DownstreamTlsContext {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DownstreamTlsContextMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DownstreamTlsContext {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__DownstreamTlsContext_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333333/.P3/P/P^%|&|)");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__DownstreamTlsContext_msg_init.0, &[<super::CommonTlsContext as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeys as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__DownstreamTlsContext_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DownstreamTlsContext {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DownstreamTlsContext {
  type Msg = DownstreamTlsContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DownstreamTlsContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DownstreamTlsContext {
  type Msg = DownstreamTlsContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DownstreamTlsContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DownstreamTlsContextMut<'_> {
  type Msg = DownstreamTlsContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DownstreamTlsContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DownstreamTlsContextMut<'_> {
  type Msg = DownstreamTlsContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DownstreamTlsContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DownstreamTlsContextView<'_> {
  type Msg = DownstreamTlsContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DownstreamTlsContext> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DownstreamTlsContextMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod downstream_tls_context {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OcspStaplePolicy(i32);

#[allow(non_upper_case_globals)]
impl OcspStaplePolicy {
  pub const LenientStapling: OcspStaplePolicy = OcspStaplePolicy(0);
  pub const StrictStapling: OcspStaplePolicy = OcspStaplePolicy(1);
  pub const MustStaple: OcspStaplePolicy = OcspStaplePolicy(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "LenientStapling",
      1 => "StrictStapling",
      2 => "MustStaple",
      _ => return None
    })
  }
}

impl ::std::convert::From<OcspStaplePolicy> for i32 {
  fn from(val: OcspStaplePolicy) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for OcspStaplePolicy {
  fn from(val: i32) -> OcspStaplePolicy {
    Self(val)
  }
}

impl ::std::default::Default for OcspStaplePolicy {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for OcspStaplePolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "OcspStaplePolicy::{}", constant_name)
    } else {
      write!(f, "OcspStaplePolicy::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for OcspStaplePolicy {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for OcspStaplePolicy {}

impl ::protobuf::Proxied for OcspStaplePolicy {
  type View<'a> = OcspStaplePolicy;
}

impl ::protobuf::AsView for OcspStaplePolicy {
  type Proxied = OcspStaplePolicy;

  fn as_view(&self) -> OcspStaplePolicy {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OcspStaplePolicy {
  fn into_view<'shorter>(self) -> OcspStaplePolicy where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for OcspStaplePolicy {
  const NAME: &'static str = "OcspStaplePolicy";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for OcspStaplePolicy {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum SessionTicketKeysTypeOneof<'msg> {
  SessionTicketKeys(::protobuf::View<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsSessionTicketKeys>) = 4,
  SessionTicketKeysSdsSecretConfig(::protobuf::View<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>) = 5,
  DisableStatelessSessionResumption(bool) = 7,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum SessionTicketKeysTypeCase {
  SessionTicketKeys = 4,
  SessionTicketKeysSdsSecretConfig = 5,
  DisableStatelessSessionResumption = 7,

  not_set = 0
}

impl SessionTicketKeysTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<SessionTicketKeysTypeCase> {
    match v {
      0 => Some(SessionTicketKeysTypeCase::not_set),
      4 => Some(SessionTicketKeysTypeCase::SessionTicketKeys),
      5 => Some(SessionTicketKeysTypeCase::SessionTicketKeysSdsSecretConfig),
      7 => Some(SessionTicketKeysTypeCase::DisableStatelessSessionResumption),
      _ => None
    }
  }
}
}  // pub mod downstream_tls_context


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__TlsKeyLog_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TlsKeyLog {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TlsKeyLog>
}

impl ::protobuf::Message for TlsKeyLog {
  type MessageView<'msg> = TlsKeyLogView<'msg>;
  type MessageMut<'msg> = TlsKeyLogMut<'msg>;
}

impl ::std::default::Default for TlsKeyLog {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TlsKeyLog {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TlsKeyLog` is `Sync` because it does not implement interior mutability.
//    Neither does `TlsKeyLogMut`.
unsafe impl ::std::marker::Sync for TlsKeyLog {}

// SAFETY:
// - `TlsKeyLog` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TlsKeyLog {}

impl ::protobuf::Proxied for TlsKeyLog {
  type View<'msg> = TlsKeyLogView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TlsKeyLog {}

impl ::protobuf::MutProxied for TlsKeyLog {
  type Mut<'msg> = TlsKeyLogMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TlsKeyLogView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsKeyLog>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsKeyLogView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TlsKeyLogView<'msg> {
  type Message = TlsKeyLog;
}

impl ::std::fmt::Debug for TlsKeyLogView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TlsKeyLogView<'_> {
  fn default() -> TlsKeyLogView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TlsKeyLog>> for TlsKeyLogView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsKeyLog>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsKeyLogView<'msg> {

  pub fn to_owned(&self) -> TlsKeyLog {
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

  // local_address_range: repeated message envoy.config.core.v3.CidrRange
  pub fn local_address_range(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // remote_address_range: repeated message envoy.config.core.v3.CidrRange
  pub fn remote_address_range(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `TlsKeyLogView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TlsKeyLogView<'_> {}

// SAFETY:
// - `TlsKeyLogView` is `Send` because while its alive a `TlsKeyLogMut` cannot.
// - `TlsKeyLogView` does not use thread-local data.
unsafe impl ::std::marker::Send for TlsKeyLogView<'_> {}

impl<'msg> ::protobuf::AsView for TlsKeyLogView<'msg> {
  type Proxied = TlsKeyLog;
  fn as_view(&self) -> ::protobuf::View<'msg, TlsKeyLog> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsKeyLogView<'msg> {
  fn into_view<'shorter>(self) -> TlsKeyLogView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsKeyLog> for TlsKeyLogView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsKeyLog {
    let mut dst = TlsKeyLog::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsKeyLog> for TlsKeyLogMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsKeyLog {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TlsKeyLog {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsKeyLogView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsKeyLogMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TlsKeyLogMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsKeyLog>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsKeyLogMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TlsKeyLogMut<'msg> {
  type Message = TlsKeyLog;
}

impl ::std::fmt::Debug for TlsKeyLogMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TlsKeyLog>> for TlsKeyLogMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsKeyLog>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsKeyLogMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsKeyLog> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TlsKeyLog {
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

  // local_address_range: repeated message envoy.config.core.v3.CidrRange
  pub fn local_address_range(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn local_address_range_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_local_address_range(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // remote_address_range: repeated message envoy.config.core.v3.CidrRange
  pub fn remote_address_range(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn remote_address_range_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_remote_address_range(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `TlsKeyLogMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TlsKeyLogMut<'_> {}

// SAFETY:
// - `TlsKeyLogMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TlsKeyLogMut<'_> {}

impl<'msg> ::protobuf::AsView for TlsKeyLogMut<'msg> {
  type Proxied = TlsKeyLog;
  fn as_view(&self) -> ::protobuf::View<'_, TlsKeyLog> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsKeyLogMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TlsKeyLog>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TlsKeyLogMut<'msg> {
  type MutProxied = TlsKeyLog;
  fn as_mut(&mut self) -> TlsKeyLogMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TlsKeyLogMut<'msg> {
  fn into_mut<'shorter>(self) -> TlsKeyLogMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TlsKeyLog {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TlsKeyLog> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TlsKeyLogView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TlsKeyLogMut<'_> {
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

  // local_address_range: repeated message envoy.config.core.v3.CidrRange
  pub fn local_address_range(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn local_address_range_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_local_address_range(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // remote_address_range: repeated message envoy.config.core.v3.CidrRange
  pub fn remote_address_range(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::CidrRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn remote_address_range_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::CidrRange> {
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
  pub fn set_remote_address_range(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::CidrRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl TlsKeyLog

impl ::std::ops::Drop for TlsKeyLog {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TlsKeyLog {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TlsKeyLog {
  type Proxied = Self;
  fn as_view(&self) -> TlsKeyLogView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TlsKeyLog {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TlsKeyLogMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TlsKeyLog {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__TlsKeyLog_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XGG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__TlsKeyLog_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::CidrRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__TlsKeyLog_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsKeyLog {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsKeyLog {
  type Msg = TlsKeyLog;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsKeyLog> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsKeyLog {
  type Msg = TlsKeyLog;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsKeyLog> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsKeyLogMut<'_> {
  type Msg = TlsKeyLog;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsKeyLog> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsKeyLogMut<'_> {
  type Msg = TlsKeyLog;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsKeyLog> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsKeyLogView<'_> {
  type Msg = TlsKeyLog;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsKeyLog> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsKeyLogMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CommonTlsContext {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CommonTlsContext>
}

impl ::protobuf::Message for CommonTlsContext {
  type MessageView<'msg> = CommonTlsContextView<'msg>;
  type MessageMut<'msg> = CommonTlsContextMut<'msg>;
}

impl ::std::default::Default for CommonTlsContext {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CommonTlsContext {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CommonTlsContext` is `Sync` because it does not implement interior mutability.
//    Neither does `CommonTlsContextMut`.
unsafe impl ::std::marker::Sync for CommonTlsContext {}

// SAFETY:
// - `CommonTlsContext` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CommonTlsContext {}

impl ::protobuf::Proxied for CommonTlsContext {
  type View<'msg> = CommonTlsContextView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CommonTlsContext {}

impl ::protobuf::MutProxied for CommonTlsContext {
  type Mut<'msg> = CommonTlsContextMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CommonTlsContextView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommonTlsContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommonTlsContextView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CommonTlsContextView<'msg> {
  type Message = CommonTlsContext;
}

impl ::std::fmt::Debug for CommonTlsContextView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CommonTlsContextView<'_> {
  fn default() -> CommonTlsContextView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CommonTlsContext>> for CommonTlsContextView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommonTlsContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommonTlsContextView<'msg> {

  pub fn to_owned(&self) -> CommonTlsContext {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // tls_params: optional message envoy.extensions.transport_sockets.tls.v3.TlsParameters
  pub fn has_tls_params(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn tls_params_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersView<'msg>> {
    self.has_tls_params().then(|| self.tls_params())
  }
  pub fn tls_params(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersView::default())
  }

  // tls_certificates: repeated message envoy.extensions.transport_sockets.tls.v3.TlsCertificate
  pub fn tls_certificates(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // tls_certificate_sds_secret_configs: repeated message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn tls_certificate_sds_secret_configs(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // tls_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance
  pub fn has_tls_certificate_provider_instance(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn tls_certificate_provider_instance_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceView<'msg>> {
    self.has_tls_certificate_provider_instance().then(|| self.tls_certificate_provider_instance())
  }
  pub fn tls_certificate_provider_instance(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceView::default())
  }

  // custom_tls_certificate_selector: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_tls_certificate_selector(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn custom_tls_certificate_selector_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom_tls_certificate_selector().then(|| self.custom_tls_certificate_selector())
  }
  pub fn custom_tls_certificate_selector(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // tls_certificate_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider
  pub fn has_tls_certificate_certificate_provider(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn tls_certificate_certificate_provider_opt(self) -> ::std::option::Option<super::common_tls_context::CertificateProviderView<'msg>> {
    self.has_tls_certificate_certificate_provider().then(|| self.tls_certificate_certificate_provider())
  }
  pub fn tls_certificate_certificate_provider(self) -> super::common_tls_context::CertificateProviderView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderView::default())
  }

  // tls_certificate_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_tls_certificate_certificate_provider_instance(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn tls_certificate_certificate_provider_instance_opt(self) -> ::std::option::Option<super::common_tls_context::CertificateProviderInstanceView<'msg>> {
    self.has_tls_certificate_certificate_provider_instance().then(|| self.tls_certificate_certificate_provider_instance())
  }
  pub fn tls_certificate_certificate_provider_instance(self) -> super::common_tls_context::CertificateProviderInstanceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderInstanceView::default())
  }

  // validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
  pub fn has_validation_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn validation_context_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'msg>> {
    self.has_validation_context().then(|| self.validation_context())
  }
  pub fn validation_context(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView::default())
  }

  // validation_context_sds_secret_config: optional message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn has_validation_context_sds_secret_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn validation_context_sds_secret_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'msg>> {
    self.has_validation_context_sds_secret_config().then(|| self.validation_context_sds_secret_config())
  }
  pub fn validation_context_sds_secret_config(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView::default())
  }

  // combined_validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CombinedCertificateValidationContext
  pub fn has_combined_validation_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn combined_validation_context_opt(self) -> ::std::option::Option<super::common_tls_context::CombinedCertificateValidationContextView<'msg>> {
    self.has_combined_validation_context().then(|| self.combined_validation_context())
  }
  pub fn combined_validation_context(self) -> super::common_tls_context::CombinedCertificateValidationContextView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CombinedCertificateValidationContextView::default())
  }

  // validation_context_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider
  pub fn has_validation_context_certificate_provider(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn validation_context_certificate_provider_opt(self) -> ::std::option::Option<super::common_tls_context::CertificateProviderView<'msg>> {
    self.has_validation_context_certificate_provider().then(|| self.validation_context_certificate_provider())
  }
  pub fn validation_context_certificate_provider(self) -> super::common_tls_context::CertificateProviderView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderView::default())
  }

  // validation_context_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_validation_context_certificate_provider_instance(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn validation_context_certificate_provider_instance_opt(self) -> ::std::option::Option<super::common_tls_context::CertificateProviderInstanceView<'msg>> {
    self.has_validation_context_certificate_provider_instance().then(|| self.validation_context_certificate_provider_instance())
  }
  pub fn validation_context_certificate_provider_instance(self) -> super::common_tls_context::CertificateProviderInstanceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderInstanceView::default())
  }

  // alpn_protocols: repeated string
  pub fn alpn_protocols(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // custom_handshaker: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_handshaker(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn custom_handshaker_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom_handshaker().then(|| self.custom_handshaker())
  }
  pub fn custom_handshaker(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // key_log: optional message envoy.extensions.transport_sockets.tls.v3.TlsKeyLog
  pub fn has_key_log(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn key_log_opt(self) -> ::std::option::Option<super::TlsKeyLogView<'msg>> {
    self.has_key_log().then(|| self.key_log())
  }
  pub fn key_log(self) -> super::TlsKeyLogView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TlsKeyLogView::default())
  }

  pub fn validation_context_type(self) -> super::common_tls_context::ValidationContextTypeOneof<'msg> {
    match self.validation_context_type_case() {
      super::common_tls_context::ValidationContextTypeCase::ValidationContext =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContext(self.validation_context()),
      super::common_tls_context::ValidationContextTypeCase::ValidationContextSdsSecretConfig =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContextSdsSecretConfig(self.validation_context_sds_secret_config()),
      super::common_tls_context::ValidationContextTypeCase::CombinedValidationContext =>
          super::common_tls_context::ValidationContextTypeOneof::CombinedValidationContext(self.combined_validation_context()),
      super::common_tls_context::ValidationContextTypeCase::ValidationContextCertificateProvider =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContextCertificateProvider(self.validation_context_certificate_provider()),
      super::common_tls_context::ValidationContextTypeCase::ValidationContextCertificateProviderInstance =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContextCertificateProviderInstance(self.validation_context_certificate_provider_instance()),
      _ => super::common_tls_context::ValidationContextTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn validation_context_type_case(self) -> super::common_tls_context::ValidationContextTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::common_tls_context::ValidationContextTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CommonTlsContextView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CommonTlsContextView<'_> {}

// SAFETY:
// - `CommonTlsContextView` is `Send` because while its alive a `CommonTlsContextMut` cannot.
// - `CommonTlsContextView` does not use thread-local data.
unsafe impl ::std::marker::Send for CommonTlsContextView<'_> {}

impl<'msg> ::protobuf::AsView for CommonTlsContextView<'msg> {
  type Proxied = CommonTlsContext;
  fn as_view(&self) -> ::protobuf::View<'msg, CommonTlsContext> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommonTlsContextView<'msg> {
  fn into_view<'shorter>(self) -> CommonTlsContextView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CommonTlsContext> for CommonTlsContextView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommonTlsContext {
    let mut dst = CommonTlsContext::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CommonTlsContext> for CommonTlsContextMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommonTlsContext {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CommonTlsContext {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CommonTlsContextView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CommonTlsContextMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CommonTlsContextMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommonTlsContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommonTlsContextMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CommonTlsContextMut<'msg> {
  type Message = CommonTlsContext;
}

impl ::std::fmt::Debug for CommonTlsContextMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CommonTlsContext>> for CommonTlsContextMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommonTlsContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommonTlsContextMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CommonTlsContext> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CommonTlsContext {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // tls_params: optional message envoy.extensions.transport_sockets.tls.v3.TlsParameters
  pub fn has_tls_params(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_tls_params(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn tls_params_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersView<'_>> {
    self.has_tls_params().then(|| self.tls_params())
  }
  pub fn tls_params(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersView::default())
  }
  pub fn tls_params_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersMut<'_> {
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
  pub fn set_tls_params(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParameters>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // tls_certificates: repeated message envoy.extensions.transport_sockets.tls.v3.TlsCertificate
  pub fn tls_certificates(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn tls_certificates_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate> {
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
  pub fn set_tls_certificates(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // tls_certificate_sds_secret_configs: repeated message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn tls_certificate_sds_secret_configs(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn tls_certificate_sds_secret_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig> {
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
  pub fn set_tls_certificate_sds_secret_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // tls_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance
  pub fn has_tls_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_tls_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn tls_certificate_provider_instance_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceView<'_>> {
    self.has_tls_certificate_provider_instance().then(|| self.tls_certificate_provider_instance())
  }
  pub fn tls_certificate_provider_instance(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceView::default())
  }
  pub fn tls_certificate_provider_instance_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tls_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // custom_tls_certificate_selector: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_tls_certificate_selector(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_custom_tls_certificate_selector(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn custom_tls_certificate_selector_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_tls_certificate_selector().then(|| self.custom_tls_certificate_selector())
  }
  pub fn custom_tls_certificate_selector(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_tls_certificate_selector_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_tls_certificate_selector(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // tls_certificate_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider
  pub fn has_tls_certificate_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_tls_certificate_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn tls_certificate_certificate_provider_opt(&self) -> ::std::option::Option<super::common_tls_context::CertificateProviderView<'_>> {
    self.has_tls_certificate_certificate_provider().then(|| self.tls_certificate_certificate_provider())
  }
  pub fn tls_certificate_certificate_provider(&self) -> super::common_tls_context::CertificateProviderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderView::default())
  }
  pub fn tls_certificate_certificate_provider_mut(&mut self) -> super::common_tls_context::CertificateProviderMut<'_> {
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
  pub fn set_tls_certificate_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CertificateProvider>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // tls_certificate_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_tls_certificate_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_tls_certificate_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn tls_certificate_certificate_provider_instance_opt(&self) -> ::std::option::Option<super::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_tls_certificate_certificate_provider_instance().then(|| self.tls_certificate_certificate_provider_instance())
  }
  pub fn tls_certificate_certificate_provider_instance(&self) -> super::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn tls_certificate_certificate_provider_instance_mut(&mut self) -> super::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_tls_certificate_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CertificateProviderInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
  pub fn has_validation_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_validation_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn validation_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_>> {
    self.has_validation_context().then(|| self.validation_context())
  }
  pub fn validation_context(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView::default())
  }
  pub fn validation_context_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextMut<'_> {
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
  pub fn set_validation_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // validation_context_sds_secret_config: optional message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn has_validation_context_sds_secret_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_validation_context_sds_secret_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn validation_context_sds_secret_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_>> {
    self.has_validation_context_sds_secret_config().then(|| self.validation_context_sds_secret_config())
  }
  pub fn validation_context_sds_secret_config(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView::default())
  }
  pub fn validation_context_sds_secret_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigMut<'_> {
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
  pub fn set_validation_context_sds_secret_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // combined_validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CombinedCertificateValidationContext
  pub fn has_combined_validation_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_combined_validation_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn combined_validation_context_opt(&self) -> ::std::option::Option<super::common_tls_context::CombinedCertificateValidationContextView<'_>> {
    self.has_combined_validation_context().then(|| self.combined_validation_context())
  }
  pub fn combined_validation_context(&self) -> super::common_tls_context::CombinedCertificateValidationContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CombinedCertificateValidationContextView::default())
  }
  pub fn combined_validation_context_mut(&mut self) -> super::common_tls_context::CombinedCertificateValidationContextMut<'_> {
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
  pub fn set_combined_validation_context(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CombinedCertificateValidationContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // validation_context_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider
  pub fn has_validation_context_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_validation_context_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn validation_context_certificate_provider_opt(&self) -> ::std::option::Option<super::common_tls_context::CertificateProviderView<'_>> {
    self.has_validation_context_certificate_provider().then(|| self.validation_context_certificate_provider())
  }
  pub fn validation_context_certificate_provider(&self) -> super::common_tls_context::CertificateProviderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderView::default())
  }
  pub fn validation_context_certificate_provider_mut(&mut self) -> super::common_tls_context::CertificateProviderMut<'_> {
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
  pub fn set_validation_context_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CertificateProvider>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // validation_context_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_validation_context_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_validation_context_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn validation_context_certificate_provider_instance_opt(&self) -> ::std::option::Option<super::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_validation_context_certificate_provider_instance().then(|| self.validation_context_certificate_provider_instance())
  }
  pub fn validation_context_certificate_provider_instance(&self) -> super::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn validation_context_certificate_provider_instance_mut(&mut self) -> super::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_validation_context_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CertificateProviderInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // alpn_protocols: repeated string
  pub fn alpn_protocols(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn alpn_protocols_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_alpn_protocols(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // custom_handshaker: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_handshaker(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_custom_handshaker(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn custom_handshaker_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_handshaker().then(|| self.custom_handshaker())
  }
  pub fn custom_handshaker(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_handshaker_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_handshaker(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // key_log: optional message envoy.extensions.transport_sockets.tls.v3.TlsKeyLog
  pub fn has_key_log(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_key_log(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn key_log_opt(&self) -> ::std::option::Option<super::TlsKeyLogView<'_>> {
    self.has_key_log().then(|| self.key_log())
  }
  pub fn key_log(&self) -> super::TlsKeyLogView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TlsKeyLogView::default())
  }
  pub fn key_log_mut(&mut self) -> super::TlsKeyLogMut<'_> {
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
  pub fn set_key_log(&mut self,
    val: impl ::protobuf::IntoProxied<super::TlsKeyLog>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  pub fn validation_context_type(&self) -> super::common_tls_context::ValidationContextTypeOneof<'_> {
    match &self.validation_context_type_case() {
      super::common_tls_context::ValidationContextTypeCase::ValidationContext =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContext(self.validation_context()),
      super::common_tls_context::ValidationContextTypeCase::ValidationContextSdsSecretConfig =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContextSdsSecretConfig(self.validation_context_sds_secret_config()),
      super::common_tls_context::ValidationContextTypeCase::CombinedValidationContext =>
          super::common_tls_context::ValidationContextTypeOneof::CombinedValidationContext(self.combined_validation_context()),
      super::common_tls_context::ValidationContextTypeCase::ValidationContextCertificateProvider =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContextCertificateProvider(self.validation_context_certificate_provider()),
      super::common_tls_context::ValidationContextTypeCase::ValidationContextCertificateProviderInstance =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContextCertificateProviderInstance(self.validation_context_certificate_provider_instance()),
      _ => super::common_tls_context::ValidationContextTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn validation_context_type_case(&self) -> super::common_tls_context::ValidationContextTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::common_tls_context::ValidationContextTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CommonTlsContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CommonTlsContextMut<'_> {}

// SAFETY:
// - `CommonTlsContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CommonTlsContextMut<'_> {}

impl<'msg> ::protobuf::AsView for CommonTlsContextMut<'msg> {
  type Proxied = CommonTlsContext;
  fn as_view(&self) -> ::protobuf::View<'_, CommonTlsContext> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommonTlsContextMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CommonTlsContext>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CommonTlsContextMut<'msg> {
  type MutProxied = CommonTlsContext;
  fn as_mut(&mut self) -> CommonTlsContextMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CommonTlsContextMut<'msg> {
  fn into_mut<'shorter>(self) -> CommonTlsContextMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CommonTlsContext {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CommonTlsContext> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CommonTlsContextView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CommonTlsContextMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // tls_params: optional message envoy.extensions.transport_sockets.tls.v3.TlsParameters
  pub fn has_tls_params(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_tls_params(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn tls_params_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersView<'_>> {
    self.has_tls_params().then(|| self.tls_params())
  }
  pub fn tls_params(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersView::default())
  }
  pub fn tls_params_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParametersMut<'_> {
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
  pub fn set_tls_params(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParameters>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // tls_certificates: repeated message envoy.extensions.transport_sockets.tls.v3.TlsCertificate
  pub fn tls_certificates(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn tls_certificates_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate> {
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
  pub fn set_tls_certificates(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // tls_certificate_sds_secret_configs: repeated message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn tls_certificate_sds_secret_configs(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn tls_certificate_sds_secret_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig> {
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
  pub fn set_tls_certificate_sds_secret_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // tls_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance
  pub fn has_tls_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_tls_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn tls_certificate_provider_instance_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceView<'_>> {
    self.has_tls_certificate_provider_instance().then(|| self.tls_certificate_provider_instance())
  }
  pub fn tls_certificate_provider_instance(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceView::default())
  }
  pub fn tls_certificate_provider_instance_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstanceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tls_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // custom_tls_certificate_selector: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_tls_certificate_selector(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_custom_tls_certificate_selector(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn custom_tls_certificate_selector_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_tls_certificate_selector().then(|| self.custom_tls_certificate_selector())
  }
  pub fn custom_tls_certificate_selector(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_tls_certificate_selector_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_tls_certificate_selector(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // tls_certificate_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider
  pub fn has_tls_certificate_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_tls_certificate_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn tls_certificate_certificate_provider_opt(&self) -> ::std::option::Option<super::common_tls_context::CertificateProviderView<'_>> {
    self.has_tls_certificate_certificate_provider().then(|| self.tls_certificate_certificate_provider())
  }
  pub fn tls_certificate_certificate_provider(&self) -> super::common_tls_context::CertificateProviderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderView::default())
  }
  pub fn tls_certificate_certificate_provider_mut(&mut self) -> super::common_tls_context::CertificateProviderMut<'_> {
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
  pub fn set_tls_certificate_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CertificateProvider>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // tls_certificate_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_tls_certificate_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_tls_certificate_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn tls_certificate_certificate_provider_instance_opt(&self) -> ::std::option::Option<super::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_tls_certificate_certificate_provider_instance().then(|| self.tls_certificate_certificate_provider_instance())
  }
  pub fn tls_certificate_certificate_provider_instance(&self) -> super::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn tls_certificate_certificate_provider_instance_mut(&mut self) -> super::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_tls_certificate_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CertificateProviderInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
  pub fn has_validation_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_validation_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn validation_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_>> {
    self.has_validation_context().then(|| self.validation_context())
  }
  pub fn validation_context(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView::default())
  }
  pub fn validation_context_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextMut<'_> {
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
  pub fn set_validation_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // validation_context_sds_secret_config: optional message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn has_validation_context_sds_secret_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_validation_context_sds_secret_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn validation_context_sds_secret_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_>> {
    self.has_validation_context_sds_secret_config().then(|| self.validation_context_sds_secret_config())
  }
  pub fn validation_context_sds_secret_config(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView::default())
  }
  pub fn validation_context_sds_secret_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigMut<'_> {
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
  pub fn set_validation_context_sds_secret_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // combined_validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CombinedCertificateValidationContext
  pub fn has_combined_validation_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_combined_validation_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn combined_validation_context_opt(&self) -> ::std::option::Option<super::common_tls_context::CombinedCertificateValidationContextView<'_>> {
    self.has_combined_validation_context().then(|| self.combined_validation_context())
  }
  pub fn combined_validation_context(&self) -> super::common_tls_context::CombinedCertificateValidationContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CombinedCertificateValidationContextView::default())
  }
  pub fn combined_validation_context_mut(&mut self) -> super::common_tls_context::CombinedCertificateValidationContextMut<'_> {
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
  pub fn set_combined_validation_context(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CombinedCertificateValidationContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // validation_context_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider
  pub fn has_validation_context_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_validation_context_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn validation_context_certificate_provider_opt(&self) -> ::std::option::Option<super::common_tls_context::CertificateProviderView<'_>> {
    self.has_validation_context_certificate_provider().then(|| self.validation_context_certificate_provider())
  }
  pub fn validation_context_certificate_provider(&self) -> super::common_tls_context::CertificateProviderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderView::default())
  }
  pub fn validation_context_certificate_provider_mut(&mut self) -> super::common_tls_context::CertificateProviderMut<'_> {
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
  pub fn set_validation_context_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CertificateProvider>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // validation_context_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_validation_context_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_validation_context_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn validation_context_certificate_provider_instance_opt(&self) -> ::std::option::Option<super::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_validation_context_certificate_provider_instance().then(|| self.validation_context_certificate_provider_instance())
  }
  pub fn validation_context_certificate_provider_instance(&self) -> super::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn validation_context_certificate_provider_instance_mut(&mut self) -> super::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_validation_context_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<super::common_tls_context::CertificateProviderInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // alpn_protocols: repeated string
  pub fn alpn_protocols(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn alpn_protocols_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_alpn_protocols(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // custom_handshaker: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_handshaker(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_custom_handshaker(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn custom_handshaker_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_handshaker().then(|| self.custom_handshaker())
  }
  pub fn custom_handshaker(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_handshaker_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_handshaker(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // key_log: optional message envoy.extensions.transport_sockets.tls.v3.TlsKeyLog
  pub fn has_key_log(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_key_log(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn key_log_opt(&self) -> ::std::option::Option<super::TlsKeyLogView<'_>> {
    self.has_key_log().then(|| self.key_log())
  }
  pub fn key_log(&self) -> super::TlsKeyLogView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TlsKeyLogView::default())
  }
  pub fn key_log_mut(&mut self) -> super::TlsKeyLogMut<'_> {
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
  pub fn set_key_log(&mut self,
    val: impl ::protobuf::IntoProxied<super::TlsKeyLog>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

  pub fn validation_context_type(&self) -> super::common_tls_context::ValidationContextTypeOneof<'_> {
    match &self.validation_context_type_case() {
      super::common_tls_context::ValidationContextTypeCase::ValidationContext =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContext(self.validation_context()),
      super::common_tls_context::ValidationContextTypeCase::ValidationContextSdsSecretConfig =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContextSdsSecretConfig(self.validation_context_sds_secret_config()),
      super::common_tls_context::ValidationContextTypeCase::CombinedValidationContext =>
          super::common_tls_context::ValidationContextTypeOneof::CombinedValidationContext(self.combined_validation_context()),
      super::common_tls_context::ValidationContextTypeCase::ValidationContextCertificateProvider =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContextCertificateProvider(self.validation_context_certificate_provider()),
      super::common_tls_context::ValidationContextTypeCase::ValidationContextCertificateProviderInstance =>
          super::common_tls_context::ValidationContextTypeOneof::ValidationContextCertificateProviderInstance(self.validation_context_certificate_provider_instance()),
      _ => super::common_tls_context::ValidationContextTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn validation_context_type_case(&self) -> super::common_tls_context::ValidationContextTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(2);
      super::common_tls_context::ValidationContextTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CommonTlsContext

impl ::std::ops::Drop for CommonTlsContext {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CommonTlsContext {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CommonTlsContext {
  type Proxied = Self;
  fn as_view(&self) -> CommonTlsContextView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CommonTlsContext {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CommonTlsContextMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CommonTlsContext {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G3ETaG3333333333^$|)|*|,|.");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext_msg_init.0, &[<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsParameters as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::TlsCertificate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::common_tls_context::CombinedCertificateValidationContext as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::common_tls_context::CertificateProvider as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::common_tls_context::CertificateProvider as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::common_tls_context::CertificateProviderInstance as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::common_tls_context::CertificateProviderInstance as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateProviderPluginInstance as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TlsKeyLog as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommonTlsContext {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommonTlsContext {
  type Msg = CommonTlsContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonTlsContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommonTlsContext {
  type Msg = CommonTlsContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonTlsContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommonTlsContextMut<'_> {
  type Msg = CommonTlsContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonTlsContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommonTlsContextMut<'_> {
  type Msg = CommonTlsContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonTlsContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommonTlsContextView<'_> {
  type Msg = CommonTlsContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonTlsContext> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommonTlsContextMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod common_tls_context {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CertificateProvider_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CertificateProvider {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CertificateProvider>
}

impl ::protobuf::Message for CertificateProvider {
  type MessageView<'msg> = CertificateProviderView<'msg>;
  type MessageMut<'msg> = CertificateProviderMut<'msg>;
}

impl ::std::default::Default for CertificateProvider {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CertificateProvider {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CertificateProvider` is `Sync` because it does not implement interior mutability.
//    Neither does `CertificateProviderMut`.
unsafe impl ::std::marker::Sync for CertificateProvider {}

// SAFETY:
// - `CertificateProvider` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CertificateProvider {}

impl ::protobuf::Proxied for CertificateProvider {
  type View<'msg> = CertificateProviderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CertificateProvider {}

impl ::protobuf::MutProxied for CertificateProvider {
  type Mut<'msg> = CertificateProviderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CertificateProviderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProvider>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificateProviderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CertificateProviderView<'msg> {
  type Message = CertificateProvider;
}

impl ::std::fmt::Debug for CertificateProviderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CertificateProviderView<'_> {
  fn default() -> CertificateProviderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProvider>> for CertificateProviderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProvider>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificateProviderView<'msg> {

  pub fn to_owned(&self) -> CertificateProvider {
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

  // typed_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn typed_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  pub fn config(self) -> super::super::common_tls_context::certificate_provider::ConfigOneof<'msg> {
    match self.config_case() {
      super::super::common_tls_context::certificate_provider::ConfigCase::TypedConfig =>
          super::super::common_tls_context::certificate_provider::ConfigOneof::TypedConfig(self.typed_config()),
      _ => super::super::common_tls_context::certificate_provider::ConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_case(self) -> super::super::common_tls_context::certificate_provider::ConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::common_tls_context::certificate_provider::ConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CertificateProviderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CertificateProviderView<'_> {}

// SAFETY:
// - `CertificateProviderView` is `Send` because while its alive a `CertificateProviderMut` cannot.
// - `CertificateProviderView` does not use thread-local data.
unsafe impl ::std::marker::Send for CertificateProviderView<'_> {}

impl<'msg> ::protobuf::AsView for CertificateProviderView<'msg> {
  type Proxied = CertificateProvider;
  fn as_view(&self) -> ::protobuf::View<'msg, CertificateProvider> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificateProviderView<'msg> {
  fn into_view<'shorter>(self) -> CertificateProviderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateProvider> for CertificateProviderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateProvider {
    let mut dst = CertificateProvider::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateProvider> for CertificateProviderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateProvider {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CertificateProvider {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificateProviderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificateProviderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CertificateProviderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProvider>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificateProviderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CertificateProviderMut<'msg> {
  type Message = CertificateProvider;
}

impl ::std::fmt::Debug for CertificateProviderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProvider>> for CertificateProviderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProvider>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificateProviderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProvider> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CertificateProvider {
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

  // typed_config: optional message envoy.config.core.v3.TypedExtensionConfig
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
  pub fn typed_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn config(&self) -> super::super::common_tls_context::certificate_provider::ConfigOneof<'_> {
    match &self.config_case() {
      super::super::common_tls_context::certificate_provider::ConfigCase::TypedConfig =>
          super::super::common_tls_context::certificate_provider::ConfigOneof::TypedConfig(self.typed_config()),
      _ => super::super::common_tls_context::certificate_provider::ConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_case(&self) -> super::super::common_tls_context::certificate_provider::ConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::common_tls_context::certificate_provider::ConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CertificateProviderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CertificateProviderMut<'_> {}

// SAFETY:
// - `CertificateProviderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CertificateProviderMut<'_> {}

impl<'msg> ::protobuf::AsView for CertificateProviderMut<'msg> {
  type Proxied = CertificateProvider;
  fn as_view(&self) -> ::protobuf::View<'_, CertificateProvider> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificateProviderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CertificateProvider>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CertificateProviderMut<'msg> {
  type MutProxied = CertificateProvider;
  fn as_mut(&mut self) -> CertificateProviderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CertificateProviderMut<'msg> {
  fn into_mut<'shorter>(self) -> CertificateProviderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CertificateProvider {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CertificateProvider> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CertificateProviderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CertificateProviderMut<'_> {
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

  // typed_config: optional message envoy.config.core.v3.TypedExtensionConfig
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
  pub fn typed_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_config().then(|| self.typed_config())
  }
  pub fn typed_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  pub fn config(&self) -> super::super::common_tls_context::certificate_provider::ConfigOneof<'_> {
    match &self.config_case() {
      super::super::common_tls_context::certificate_provider::ConfigCase::TypedConfig =>
          super::super::common_tls_context::certificate_provider::ConfigOneof::TypedConfig(self.typed_config()),
      _ => super::super::common_tls_context::certificate_provider::ConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_case(&self) -> super::super::common_tls_context::certificate_provider::ConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::common_tls_context::certificate_provider::ConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CertificateProvider

impl ::std::ops::Drop for CertificateProvider {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CertificateProvider {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CertificateProvider {
  type Proxied = Self;
  fn as_view(&self) -> CertificateProviderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CertificateProvider {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CertificateProviderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CertificateProvider {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::common_tls_context::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CertificateProvider_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3^#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::common_tls_context::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CertificateProvider_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::common_tls_context::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CertificateProvider_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificateProvider {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificateProvider {
  type Msg = CertificateProvider;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProvider> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProvider {
  type Msg = CertificateProvider;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProvider> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificateProviderMut<'_> {
  type Msg = CertificateProvider;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProvider> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProviderMut<'_> {
  type Msg = CertificateProvider;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProvider> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProviderView<'_> {
  type Msg = CertificateProvider;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProvider> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificateProviderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod certificate_provider {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ConfigOneof<'msg> {
  TypedConfig(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ConfigCase {
  TypedConfig = 2,

  not_set = 0
}

impl ConfigCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ConfigCase> {
    match v {
      0 => Some(ConfigCase::not_set),
      2 => Some(ConfigCase::TypedConfig),
      _ => None
    }
  }
}
}  // pub mod certificate_provider

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CertificateProviderInstance_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CertificateProviderInstance {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CertificateProviderInstance>
}

impl ::protobuf::Message for CertificateProviderInstance {
  type MessageView<'msg> = CertificateProviderInstanceView<'msg>;
  type MessageMut<'msg> = CertificateProviderInstanceMut<'msg>;
}

impl ::std::default::Default for CertificateProviderInstance {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CertificateProviderInstance {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CertificateProviderInstance` is `Sync` because it does not implement interior mutability.
//    Neither does `CertificateProviderInstanceMut`.
unsafe impl ::std::marker::Sync for CertificateProviderInstance {}

// SAFETY:
// - `CertificateProviderInstance` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CertificateProviderInstance {}

impl ::protobuf::Proxied for CertificateProviderInstance {
  type View<'msg> = CertificateProviderInstanceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CertificateProviderInstance {}

impl ::protobuf::MutProxied for CertificateProviderInstance {
  type Mut<'msg> = CertificateProviderInstanceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CertificateProviderInstanceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProviderInstance>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificateProviderInstanceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CertificateProviderInstanceView<'msg> {
  type Message = CertificateProviderInstance;
}

impl ::std::fmt::Debug for CertificateProviderInstanceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CertificateProviderInstanceView<'_> {
  fn default() -> CertificateProviderInstanceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProviderInstance>> for CertificateProviderInstanceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProviderInstance>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificateProviderInstanceView<'msg> {

  pub fn to_owned(&self) -> CertificateProviderInstance {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // instance_name: optional string
  pub fn instance_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // certificate_name: optional string
  pub fn certificate_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `CertificateProviderInstanceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CertificateProviderInstanceView<'_> {}

// SAFETY:
// - `CertificateProviderInstanceView` is `Send` because while its alive a `CertificateProviderInstanceMut` cannot.
// - `CertificateProviderInstanceView` does not use thread-local data.
unsafe impl ::std::marker::Send for CertificateProviderInstanceView<'_> {}

impl<'msg> ::protobuf::AsView for CertificateProviderInstanceView<'msg> {
  type Proxied = CertificateProviderInstance;
  fn as_view(&self) -> ::protobuf::View<'msg, CertificateProviderInstance> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificateProviderInstanceView<'msg> {
  fn into_view<'shorter>(self) -> CertificateProviderInstanceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateProviderInstance> for CertificateProviderInstanceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateProviderInstance {
    let mut dst = CertificateProviderInstance::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateProviderInstance> for CertificateProviderInstanceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateProviderInstance {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CertificateProviderInstance {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificateProviderInstanceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificateProviderInstanceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CertificateProviderInstanceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProviderInstance>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificateProviderInstanceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CertificateProviderInstanceMut<'msg> {
  type Message = CertificateProviderInstance;
}

impl ::std::fmt::Debug for CertificateProviderInstanceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProviderInstance>> for CertificateProviderInstanceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProviderInstance>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificateProviderInstanceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProviderInstance> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CertificateProviderInstance {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // instance_name: optional string
  pub fn instance_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_instance_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // certificate_name: optional string
  pub fn certificate_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_certificate_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `CertificateProviderInstanceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CertificateProviderInstanceMut<'_> {}

// SAFETY:
// - `CertificateProviderInstanceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CertificateProviderInstanceMut<'_> {}

impl<'msg> ::protobuf::AsView for CertificateProviderInstanceMut<'msg> {
  type Proxied = CertificateProviderInstance;
  fn as_view(&self) -> ::protobuf::View<'_, CertificateProviderInstance> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificateProviderInstanceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CertificateProviderInstance>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CertificateProviderInstanceMut<'msg> {
  type MutProxied = CertificateProviderInstance;
  fn as_mut(&mut self) -> CertificateProviderInstanceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CertificateProviderInstanceMut<'msg> {
  fn into_mut<'shorter>(self) -> CertificateProviderInstanceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CertificateProviderInstance {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CertificateProviderInstance> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CertificateProviderInstanceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CertificateProviderInstanceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // instance_name: optional string
  pub fn instance_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_instance_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // certificate_name: optional string
  pub fn certificate_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_certificate_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl CertificateProviderInstance

impl ::std::ops::Drop for CertificateProviderInstance {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CertificateProviderInstance {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CertificateProviderInstance {
  type Proxied = Self;
  fn as_view(&self) -> CertificateProviderInstanceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CertificateProviderInstance {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CertificateProviderInstanceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CertificateProviderInstance {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::common_tls_context::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CertificateProviderInstance_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::common_tls_context::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CertificateProviderInstance_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::common_tls_context::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CertificateProviderInstance_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificateProviderInstance {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificateProviderInstance {
  type Msg = CertificateProviderInstance;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderInstance> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProviderInstance {
  type Msg = CertificateProviderInstance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderInstance> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificateProviderInstanceMut<'_> {
  type Msg = CertificateProviderInstance;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderInstance> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProviderInstanceMut<'_> {
  type Msg = CertificateProviderInstance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderInstance> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProviderInstanceView<'_> {
  type Msg = CertificateProviderInstance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderInstance> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificateProviderInstanceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CombinedCertificateValidationContext_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CombinedCertificateValidationContext {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CombinedCertificateValidationContext>
}

impl ::protobuf::Message for CombinedCertificateValidationContext {
  type MessageView<'msg> = CombinedCertificateValidationContextView<'msg>;
  type MessageMut<'msg> = CombinedCertificateValidationContextMut<'msg>;
}

impl ::std::default::Default for CombinedCertificateValidationContext {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CombinedCertificateValidationContext {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CombinedCertificateValidationContext` is `Sync` because it does not implement interior mutability.
//    Neither does `CombinedCertificateValidationContextMut`.
unsafe impl ::std::marker::Sync for CombinedCertificateValidationContext {}

// SAFETY:
// - `CombinedCertificateValidationContext` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CombinedCertificateValidationContext {}

impl ::protobuf::Proxied for CombinedCertificateValidationContext {
  type View<'msg> = CombinedCertificateValidationContextView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CombinedCertificateValidationContext {}

impl ::protobuf::MutProxied for CombinedCertificateValidationContext {
  type Mut<'msg> = CombinedCertificateValidationContextMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CombinedCertificateValidationContextView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CombinedCertificateValidationContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CombinedCertificateValidationContextView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CombinedCertificateValidationContextView<'msg> {
  type Message = CombinedCertificateValidationContext;
}

impl ::std::fmt::Debug for CombinedCertificateValidationContextView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CombinedCertificateValidationContextView<'_> {
  fn default() -> CombinedCertificateValidationContextView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CombinedCertificateValidationContext>> for CombinedCertificateValidationContextView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CombinedCertificateValidationContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CombinedCertificateValidationContextView<'msg> {

  pub fn to_owned(&self) -> CombinedCertificateValidationContext {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // default_validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
  pub fn has_default_validation_context(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn default_validation_context_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'msg>> {
    self.has_default_validation_context().then(|| self.default_validation_context())
  }
  pub fn default_validation_context(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView::default())
  }

  // validation_context_sds_secret_config: optional message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn has_validation_context_sds_secret_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn validation_context_sds_secret_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'msg>> {
    self.has_validation_context_sds_secret_config().then(|| self.validation_context_sds_secret_config())
  }
  pub fn validation_context_sds_secret_config(self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView::default())
  }

  // validation_context_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider
  pub fn has_validation_context_certificate_provider(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn validation_context_certificate_provider_opt(self) -> ::std::option::Option<super::super::common_tls_context::CertificateProviderView<'msg>> {
    self.has_validation_context_certificate_provider().then(|| self.validation_context_certificate_provider())
  }
  pub fn validation_context_certificate_provider(self) -> super::super::common_tls_context::CertificateProviderView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::common_tls_context::CertificateProviderView::default())
  }

  // validation_context_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_validation_context_certificate_provider_instance(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn validation_context_certificate_provider_instance_opt(self) -> ::std::option::Option<super::super::common_tls_context::CertificateProviderInstanceView<'msg>> {
    self.has_validation_context_certificate_provider_instance().then(|| self.validation_context_certificate_provider_instance())
  }
  pub fn validation_context_certificate_provider_instance(self) -> super::super::common_tls_context::CertificateProviderInstanceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::common_tls_context::CertificateProviderInstanceView::default())
  }

}

// SAFETY:
// - `CombinedCertificateValidationContextView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CombinedCertificateValidationContextView<'_> {}

// SAFETY:
// - `CombinedCertificateValidationContextView` is `Send` because while its alive a `CombinedCertificateValidationContextMut` cannot.
// - `CombinedCertificateValidationContextView` does not use thread-local data.
unsafe impl ::std::marker::Send for CombinedCertificateValidationContextView<'_> {}

impl<'msg> ::protobuf::AsView for CombinedCertificateValidationContextView<'msg> {
  type Proxied = CombinedCertificateValidationContext;
  fn as_view(&self) -> ::protobuf::View<'msg, CombinedCertificateValidationContext> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CombinedCertificateValidationContextView<'msg> {
  fn into_view<'shorter>(self) -> CombinedCertificateValidationContextView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CombinedCertificateValidationContext> for CombinedCertificateValidationContextView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CombinedCertificateValidationContext {
    let mut dst = CombinedCertificateValidationContext::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CombinedCertificateValidationContext> for CombinedCertificateValidationContextMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CombinedCertificateValidationContext {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CombinedCertificateValidationContext {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CombinedCertificateValidationContextView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CombinedCertificateValidationContextMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CombinedCertificateValidationContextMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CombinedCertificateValidationContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CombinedCertificateValidationContextMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CombinedCertificateValidationContextMut<'msg> {
  type Message = CombinedCertificateValidationContext;
}

impl ::std::fmt::Debug for CombinedCertificateValidationContextMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CombinedCertificateValidationContext>> for CombinedCertificateValidationContextMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CombinedCertificateValidationContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CombinedCertificateValidationContextMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CombinedCertificateValidationContext> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CombinedCertificateValidationContext {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // default_validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
  pub fn has_default_validation_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_default_validation_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn default_validation_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_>> {
    self.has_default_validation_context().then(|| self.default_validation_context())
  }
  pub fn default_validation_context(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView::default())
  }
  pub fn default_validation_context_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextMut<'_> {
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
  pub fn set_default_validation_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // validation_context_sds_secret_config: optional message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn has_validation_context_sds_secret_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_validation_context_sds_secret_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn validation_context_sds_secret_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_>> {
    self.has_validation_context_sds_secret_config().then(|| self.validation_context_sds_secret_config())
  }
  pub fn validation_context_sds_secret_config(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView::default())
  }
  pub fn validation_context_sds_secret_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigMut<'_> {
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
  pub fn set_validation_context_sds_secret_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // validation_context_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider
  pub fn has_validation_context_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_validation_context_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn validation_context_certificate_provider_opt(&self) -> ::std::option::Option<super::super::common_tls_context::CertificateProviderView<'_>> {
    self.has_validation_context_certificate_provider().then(|| self.validation_context_certificate_provider())
  }
  pub fn validation_context_certificate_provider(&self) -> super::super::common_tls_context::CertificateProviderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::common_tls_context::CertificateProviderView::default())
  }
  pub fn validation_context_certificate_provider_mut(&mut self) -> super::super::common_tls_context::CertificateProviderMut<'_> {
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
  pub fn set_validation_context_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::common_tls_context::CertificateProvider>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // validation_context_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_validation_context_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_validation_context_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn validation_context_certificate_provider_instance_opt(&self) -> ::std::option::Option<super::super::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_validation_context_certificate_provider_instance().then(|| self.validation_context_certificate_provider_instance())
  }
  pub fn validation_context_certificate_provider_instance(&self) -> super::super::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn validation_context_certificate_provider_instance_mut(&mut self) -> super::super::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_validation_context_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::common_tls_context::CertificateProviderInstance>) {

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
// - `CombinedCertificateValidationContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CombinedCertificateValidationContextMut<'_> {}

// SAFETY:
// - `CombinedCertificateValidationContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CombinedCertificateValidationContextMut<'_> {}

impl<'msg> ::protobuf::AsView for CombinedCertificateValidationContextMut<'msg> {
  type Proxied = CombinedCertificateValidationContext;
  fn as_view(&self) -> ::protobuf::View<'_, CombinedCertificateValidationContext> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CombinedCertificateValidationContextMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CombinedCertificateValidationContext>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CombinedCertificateValidationContextMut<'msg> {
  type MutProxied = CombinedCertificateValidationContext;
  fn as_mut(&mut self) -> CombinedCertificateValidationContextMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CombinedCertificateValidationContextMut<'msg> {
  fn into_mut<'shorter>(self) -> CombinedCertificateValidationContextMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CombinedCertificateValidationContext {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CombinedCertificateValidationContext> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CombinedCertificateValidationContextView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CombinedCertificateValidationContextMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // default_validation_context: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
  pub fn has_default_validation_context(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_default_validation_context(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn default_validation_context_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_>> {
    self.has_default_validation_context().then(|| self.default_validation_context())
  }
  pub fn default_validation_context(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextView::default())
  }
  pub fn default_validation_context_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContextMut<'_> {
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
  pub fn set_default_validation_context(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // validation_context_sds_secret_config: optional message envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig
  pub fn has_validation_context_sds_secret_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_validation_context_sds_secret_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn validation_context_sds_secret_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_>> {
    self.has_validation_context_sds_secret_config().then(|| self.validation_context_sds_secret_config())
  }
  pub fn validation_context_sds_secret_config(&self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigView::default())
  }
  pub fn validation_context_sds_secret_config_mut(&mut self) -> crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfigMut<'_> {
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
  pub fn set_validation_context_sds_secret_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // validation_context_certificate_provider: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider
  pub fn has_validation_context_certificate_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_validation_context_certificate_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn validation_context_certificate_provider_opt(&self) -> ::std::option::Option<super::super::common_tls_context::CertificateProviderView<'_>> {
    self.has_validation_context_certificate_provider().then(|| self.validation_context_certificate_provider())
  }
  pub fn validation_context_certificate_provider(&self) -> super::super::common_tls_context::CertificateProviderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::common_tls_context::CertificateProviderView::default())
  }
  pub fn validation_context_certificate_provider_mut(&mut self) -> super::super::common_tls_context::CertificateProviderMut<'_> {
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
  pub fn set_validation_context_certificate_provider(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::common_tls_context::CertificateProvider>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // validation_context_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance
  pub fn has_validation_context_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_validation_context_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn validation_context_certificate_provider_instance_opt(&self) -> ::std::option::Option<super::super::common_tls_context::CertificateProviderInstanceView<'_>> {
    self.has_validation_context_certificate_provider_instance().then(|| self.validation_context_certificate_provider_instance())
  }
  pub fn validation_context_certificate_provider_instance(&self) -> super::super::common_tls_context::CertificateProviderInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::common_tls_context::CertificateProviderInstanceView::default())
  }
  pub fn validation_context_certificate_provider_instance_mut(&mut self) -> super::super::common_tls_context::CertificateProviderInstanceMut<'_> {
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
  pub fn set_validation_context_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::common_tls_context::CertificateProviderInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl CombinedCertificateValidationContext

impl ::std::ops::Drop for CombinedCertificateValidationContext {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CombinedCertificateValidationContext {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CombinedCertificateValidationContext {
  type Proxied = Self;
  fn as_view(&self) -> CombinedCertificateValidationContextView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CombinedCertificateValidationContext {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CombinedCertificateValidationContextMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CombinedCertificateValidationContext {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::common_tls_context::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CombinedCertificateValidationContext_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::common_tls_context::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CombinedCertificateValidationContext_msg_init.0, &[<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::common_tls_context::CertificateProvider as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::common_tls_context::CertificateProviderInstance as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::common_tls_context::envoy__extensions__transport_0sockets__tls__v3__CommonTlsContext__CombinedCertificateValidationContext_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CombinedCertificateValidationContext {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CombinedCertificateValidationContext {
  type Msg = CombinedCertificateValidationContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CombinedCertificateValidationContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CombinedCertificateValidationContext {
  type Msg = CombinedCertificateValidationContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CombinedCertificateValidationContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CombinedCertificateValidationContextMut<'_> {
  type Msg = CombinedCertificateValidationContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CombinedCertificateValidationContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CombinedCertificateValidationContextMut<'_> {
  type Msg = CombinedCertificateValidationContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CombinedCertificateValidationContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CombinedCertificateValidationContextView<'_> {
  type Msg = CombinedCertificateValidationContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CombinedCertificateValidationContext> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CombinedCertificateValidationContextMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ValidationContextTypeOneof<'msg> {
  ValidationContext(::protobuf::View<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::common::CertificateValidationContext>) = 3,
  ValidationContextSdsSecretConfig(::protobuf::View<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::SdsSecretConfig>) = 7,
  CombinedValidationContext(::protobuf::View<'msg, super::super::common_tls_context::CombinedCertificateValidationContext>) = 8,
  ValidationContextCertificateProvider(::protobuf::View<'msg, super::super::common_tls_context::CertificateProvider>) = 10,
  ValidationContextCertificateProviderInstance(::protobuf::View<'msg, super::super::common_tls_context::CertificateProviderInstance>) = 12,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ValidationContextTypeCase {
  ValidationContext = 3,
  ValidationContextSdsSecretConfig = 7,
  CombinedValidationContext = 8,
  ValidationContextCertificateProvider = 10,
  ValidationContextCertificateProviderInstance = 12,

  not_set = 0
}

impl ValidationContextTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ValidationContextTypeCase> {
    match v {
      0 => Some(ValidationContextTypeCase::not_set),
      3 => Some(ValidationContextTypeCase::ValidationContext),
      7 => Some(ValidationContextTypeCase::ValidationContextSdsSecretConfig),
      8 => Some(ValidationContextTypeCase::CombinedValidationContext),
      10 => Some(ValidationContextTypeCase::ValidationContextCertificateProvider),
      12 => Some(ValidationContextTypeCase::ValidationContextCertificateProviderInstance),
      _ => None
    }
  }
}
}  // pub mod common_tls_context


