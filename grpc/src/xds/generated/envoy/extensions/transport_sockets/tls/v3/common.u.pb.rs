const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__TlsParameters_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TlsParameters {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TlsParameters>
}

impl ::protobuf::Message for TlsParameters {
  type MessageView<'msg> = TlsParametersView<'msg>;
  type MessageMut<'msg> = TlsParametersMut<'msg>;
}

impl ::std::default::Default for TlsParameters {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TlsParameters {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TlsParameters` is `Sync` because it does not implement interior mutability.
//    Neither does `TlsParametersMut`.
unsafe impl ::std::marker::Sync for TlsParameters {}

// SAFETY:
// - `TlsParameters` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TlsParameters {}

impl ::protobuf::Proxied for TlsParameters {
  type View<'msg> = TlsParametersView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TlsParameters {}

impl ::protobuf::MutProxied for TlsParameters {
  type Mut<'msg> = TlsParametersMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TlsParametersView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsParameters>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsParametersView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TlsParametersView<'msg> {
  type Message = TlsParameters;
}

impl ::std::fmt::Debug for TlsParametersView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TlsParametersView<'_> {
  fn default() -> TlsParametersView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TlsParameters>> for TlsParametersView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsParameters>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsParametersView<'msg> {

  pub fn to_owned(&self) -> TlsParameters {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // tls_minimum_protocol_version: optional enum envoy.extensions.transport_sockets.tls.v3.TlsParameters.TlsProtocol
  pub fn tls_minimum_protocol_version(self) -> super::tls_parameters::TlsProtocol {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::tls_parameters::TlsProtocol::TlsAuto).into()
      ).try_into().unwrap()
    }
  }

  // tls_maximum_protocol_version: optional enum envoy.extensions.transport_sockets.tls.v3.TlsParameters.TlsProtocol
  pub fn tls_maximum_protocol_version(self) -> super::tls_parameters::TlsProtocol {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::tls_parameters::TlsProtocol::TlsAuto).into()
      ).try_into().unwrap()
    }
  }

  // cipher_suites: repeated string
  pub fn cipher_suites(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // ecdh_curves: repeated string
  pub fn ecdh_curves(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // signature_algorithms: repeated string
  pub fn signature_algorithms(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // compliance_policies: repeated enum envoy.extensions.transport_sockets.tls.v3.TlsParameters.CompliancePolicy
  pub fn compliance_policies(self) -> ::protobuf::RepeatedView<'msg, super::tls_parameters::CompliancePolicy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::tls_parameters::CompliancePolicy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `TlsParametersView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TlsParametersView<'_> {}

// SAFETY:
// - `TlsParametersView` is `Send` because while its alive a `TlsParametersMut` cannot.
// - `TlsParametersView` does not use thread-local data.
unsafe impl ::std::marker::Send for TlsParametersView<'_> {}

impl<'msg> ::protobuf::AsView for TlsParametersView<'msg> {
  type Proxied = TlsParameters;
  fn as_view(&self) -> ::protobuf::View<'msg, TlsParameters> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsParametersView<'msg> {
  fn into_view<'shorter>(self) -> TlsParametersView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsParameters> for TlsParametersView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsParameters {
    let mut dst = TlsParameters::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsParameters> for TlsParametersMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsParameters {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TlsParameters {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsParametersView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsParametersMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TlsParametersMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsParameters>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsParametersMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TlsParametersMut<'msg> {
  type Message = TlsParameters;
}

impl ::std::fmt::Debug for TlsParametersMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TlsParameters>> for TlsParametersMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsParameters>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsParametersMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsParameters> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TlsParameters {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // tls_minimum_protocol_version: optional enum envoy.extensions.transport_sockets.tls.v3.TlsParameters.TlsProtocol
  pub fn tls_minimum_protocol_version(&self) -> super::tls_parameters::TlsProtocol {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::tls_parameters::TlsProtocol::TlsAuto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_tls_minimum_protocol_version(&mut self, val: super::tls_parameters::TlsProtocol) {
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

  // tls_maximum_protocol_version: optional enum envoy.extensions.transport_sockets.tls.v3.TlsParameters.TlsProtocol
  pub fn tls_maximum_protocol_version(&self) -> super::tls_parameters::TlsProtocol {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::tls_parameters::TlsProtocol::TlsAuto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_tls_maximum_protocol_version(&mut self, val: super::tls_parameters::TlsProtocol) {
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

  // cipher_suites: repeated string
  pub fn cipher_suites(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn cipher_suites_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_cipher_suites(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // ecdh_curves: repeated string
  pub fn ecdh_curves(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn ecdh_curves_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_ecdh_curves(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // signature_algorithms: repeated string
  pub fn signature_algorithms(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn signature_algorithms_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_signature_algorithms(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // compliance_policies: repeated enum envoy.extensions.transport_sockets.tls.v3.TlsParameters.CompliancePolicy
  pub fn compliance_policies(&self) -> ::protobuf::RepeatedView<'_, super::tls_parameters::CompliancePolicy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::tls_parameters::CompliancePolicy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn compliance_policies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::tls_parameters::CompliancePolicy> {
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
  pub fn set_compliance_policies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::tls_parameters::CompliancePolicy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}

// SAFETY:
// - `TlsParametersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TlsParametersMut<'_> {}

// SAFETY:
// - `TlsParametersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TlsParametersMut<'_> {}

impl<'msg> ::protobuf::AsView for TlsParametersMut<'msg> {
  type Proxied = TlsParameters;
  fn as_view(&self) -> ::protobuf::View<'_, TlsParameters> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsParametersMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TlsParameters>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TlsParametersMut<'msg> {
  type MutProxied = TlsParameters;
  fn as_mut(&mut self) -> TlsParametersMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TlsParametersMut<'msg> {
  fn into_mut<'shorter>(self) -> TlsParametersMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TlsParameters {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TlsParameters> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TlsParametersView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TlsParametersMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // tls_minimum_protocol_version: optional enum envoy.extensions.transport_sockets.tls.v3.TlsParameters.TlsProtocol
  pub fn tls_minimum_protocol_version(&self) -> super::tls_parameters::TlsProtocol {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::tls_parameters::TlsProtocol::TlsAuto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_tls_minimum_protocol_version(&mut self, val: super::tls_parameters::TlsProtocol) {
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

  // tls_maximum_protocol_version: optional enum envoy.extensions.transport_sockets.tls.v3.TlsParameters.TlsProtocol
  pub fn tls_maximum_protocol_version(&self) -> super::tls_parameters::TlsProtocol {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::tls_parameters::TlsProtocol::TlsAuto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_tls_maximum_protocol_version(&mut self, val: super::tls_parameters::TlsProtocol) {
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

  // cipher_suites: repeated string
  pub fn cipher_suites(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn cipher_suites_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_cipher_suites(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // ecdh_curves: repeated string
  pub fn ecdh_curves(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn ecdh_curves_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_ecdh_curves(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // signature_algorithms: repeated string
  pub fn signature_algorithms(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn signature_algorithms_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_signature_algorithms(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // compliance_policies: repeated enum envoy.extensions.transport_sockets.tls.v3.TlsParameters.CompliancePolicy
  pub fn compliance_policies(&self) -> ::protobuf::RepeatedView<'_, super::tls_parameters::CompliancePolicy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::tls_parameters::CompliancePolicy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn compliance_policies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::tls_parameters::CompliancePolicy> {
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
  pub fn set_compliance_policies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::tls_parameters::CompliancePolicy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

}  // impl TlsParameters

impl ::std::ops::Drop for TlsParameters {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TlsParameters {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TlsParameters {
  type Proxied = Self;
  fn as_view(&self) -> TlsParametersView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TlsParameters {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TlsParametersMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TlsParameters {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__TlsParameters_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N.P.PETETETB");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__TlsParameters_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__TlsParameters_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsParameters {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsParameters {
  type Msg = TlsParameters;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsParameters> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsParameters {
  type Msg = TlsParameters;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsParameters> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsParametersMut<'_> {
  type Msg = TlsParameters;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsParameters> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsParametersMut<'_> {
  type Msg = TlsParameters;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsParameters> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsParametersView<'_> {
  type Msg = TlsParameters;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsParameters> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsParametersMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod tls_parameters {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TlsProtocol(i32);

#[allow(non_upper_case_globals)]
impl TlsProtocol {
  pub const TlsAuto: TlsProtocol = TlsProtocol(0);
  pub const Tlsv10: TlsProtocol = TlsProtocol(1);
  pub const Tlsv11: TlsProtocol = TlsProtocol(2);
  pub const Tlsv12: TlsProtocol = TlsProtocol(3);
  pub const Tlsv13: TlsProtocol = TlsProtocol(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "TlsAuto",
      1 => "Tlsv10",
      2 => "Tlsv11",
      3 => "Tlsv12",
      4 => "Tlsv13",
      _ => return None
    })
  }
}

impl ::std::convert::From<TlsProtocol> for i32 {
  fn from(val: TlsProtocol) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for TlsProtocol {
  fn from(val: i32) -> TlsProtocol {
    Self(val)
  }
}

impl ::std::default::Default for TlsProtocol {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for TlsProtocol {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "TlsProtocol::{}", constant_name)
    } else {
      write!(f, "TlsProtocol::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for TlsProtocol {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for TlsProtocol {}

impl ::protobuf::Proxied for TlsProtocol {
  type View<'a> = TlsProtocol;
}

impl ::protobuf::AsView for TlsProtocol {
  type Proxied = TlsProtocol;

  fn as_view(&self) -> TlsProtocol {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsProtocol {
  fn into_view<'shorter>(self) -> TlsProtocol where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for TlsProtocol {
  const NAME: &'static str = "TlsProtocol";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for TlsProtocol {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CompliancePolicy(i32);

#[allow(non_upper_case_globals)]
impl CompliancePolicy {
  pub const Fips202205: CompliancePolicy = CompliancePolicy(0);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Fips202205",
      _ => return None
    })
  }
}

impl ::std::convert::From<CompliancePolicy> for i32 {
  fn from(val: CompliancePolicy) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for CompliancePolicy {
  fn from(val: i32) -> CompliancePolicy {
    Self(val)
  }
}

impl ::std::default::Default for CompliancePolicy {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for CompliancePolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "CompliancePolicy::{}", constant_name)
    } else {
      write!(f, "CompliancePolicy::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for CompliancePolicy {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for CompliancePolicy {}

impl ::protobuf::Proxied for CompliancePolicy {
  type View<'a> = CompliancePolicy;
}

impl ::protobuf::AsView for CompliancePolicy {
  type Proxied = CompliancePolicy;

  fn as_view(&self) -> CompliancePolicy {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CompliancePolicy {
  fn into_view<'shorter>(self) -> CompliancePolicy where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for CompliancePolicy {
  const NAME: &'static str = "CompliancePolicy";

  fn is_known(value: i32) -> bool {
    matches!(value, 0)
  }
}

impl ::protobuf::__internal::EntityType for CompliancePolicy {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod tls_parameters


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__PrivateKeyProvider_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PrivateKeyProvider {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PrivateKeyProvider>
}

impl ::protobuf::Message for PrivateKeyProvider {
  type MessageView<'msg> = PrivateKeyProviderView<'msg>;
  type MessageMut<'msg> = PrivateKeyProviderMut<'msg>;
}

impl ::std::default::Default for PrivateKeyProvider {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PrivateKeyProvider {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PrivateKeyProvider` is `Sync` because it does not implement interior mutability.
//    Neither does `PrivateKeyProviderMut`.
unsafe impl ::std::marker::Sync for PrivateKeyProvider {}

// SAFETY:
// - `PrivateKeyProvider` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PrivateKeyProvider {}

impl ::protobuf::Proxied for PrivateKeyProvider {
  type View<'msg> = PrivateKeyProviderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PrivateKeyProvider {}

impl ::protobuf::MutProxied for PrivateKeyProvider {
  type Mut<'msg> = PrivateKeyProviderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PrivateKeyProviderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PrivateKeyProvider>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PrivateKeyProviderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PrivateKeyProviderView<'msg> {
  type Message = PrivateKeyProvider;
}

impl ::std::fmt::Debug for PrivateKeyProviderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PrivateKeyProviderView<'_> {
  fn default() -> PrivateKeyProviderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PrivateKeyProvider>> for PrivateKeyProviderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PrivateKeyProvider>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PrivateKeyProviderView<'msg> {

  pub fn to_owned(&self) -> PrivateKeyProvider {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // provider_name: optional string
  pub fn provider_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
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

  // fallback: optional bool
  pub fn fallback(self) -> bool {
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

  pub fn config_type(self) -> super::private_key_provider::ConfigTypeOneof<'msg> {
    match self.config_type_case() {
      super::private_key_provider::ConfigTypeCase::TypedConfig =>
          super::private_key_provider::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::private_key_provider::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(self) -> super::private_key_provider::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::private_key_provider::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PrivateKeyProviderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PrivateKeyProviderView<'_> {}

// SAFETY:
// - `PrivateKeyProviderView` is `Send` because while its alive a `PrivateKeyProviderMut` cannot.
// - `PrivateKeyProviderView` does not use thread-local data.
unsafe impl ::std::marker::Send for PrivateKeyProviderView<'_> {}

impl<'msg> ::protobuf::AsView for PrivateKeyProviderView<'msg> {
  type Proxied = PrivateKeyProvider;
  fn as_view(&self) -> ::protobuf::View<'msg, PrivateKeyProvider> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrivateKeyProviderView<'msg> {
  fn into_view<'shorter>(self) -> PrivateKeyProviderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PrivateKeyProvider> for PrivateKeyProviderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PrivateKeyProvider {
    let mut dst = PrivateKeyProvider::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PrivateKeyProvider> for PrivateKeyProviderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PrivateKeyProvider {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PrivateKeyProvider {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PrivateKeyProviderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PrivateKeyProviderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PrivateKeyProviderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PrivateKeyProvider>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PrivateKeyProviderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PrivateKeyProviderMut<'msg> {
  type Message = PrivateKeyProvider;
}

impl ::std::fmt::Debug for PrivateKeyProviderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PrivateKeyProvider>> for PrivateKeyProviderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PrivateKeyProvider>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PrivateKeyProviderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PrivateKeyProvider> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PrivateKeyProvider {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // provider_name: optional string
  pub fn provider_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_provider_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
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

  // fallback: optional bool
  pub fn fallback(&self) -> bool {
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
  pub fn set_fallback(&mut self, val: bool) {
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

  pub fn config_type(&self) -> super::private_key_provider::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::private_key_provider::ConfigTypeCase::TypedConfig =>
          super::private_key_provider::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::private_key_provider::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::private_key_provider::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::private_key_provider::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PrivateKeyProviderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PrivateKeyProviderMut<'_> {}

// SAFETY:
// - `PrivateKeyProviderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PrivateKeyProviderMut<'_> {}

impl<'msg> ::protobuf::AsView for PrivateKeyProviderMut<'msg> {
  type Proxied = PrivateKeyProvider;
  fn as_view(&self) -> ::protobuf::View<'_, PrivateKeyProvider> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrivateKeyProviderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PrivateKeyProvider>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PrivateKeyProviderMut<'msg> {
  type MutProxied = PrivateKeyProvider;
  fn as_mut(&mut self) -> PrivateKeyProviderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PrivateKeyProviderMut<'msg> {
  fn into_mut<'shorter>(self) -> PrivateKeyProviderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PrivateKeyProvider {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PrivateKeyProvider> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PrivateKeyProviderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PrivateKeyProviderMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // provider_name: optional string
  pub fn provider_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_provider_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
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

  // fallback: optional bool
  pub fn fallback(&self) -> bool {
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
  pub fn set_fallback(&mut self, val: bool) {
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

  pub fn config_type(&self) -> super::private_key_provider::ConfigTypeOneof<'_> {
    match &self.config_type_case() {
      super::private_key_provider::ConfigTypeCase::TypedConfig =>
          super::private_key_provider::ConfigTypeOneof::TypedConfig(self.typed_config()),
      _ => super::private_key_provider::ConfigTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn config_type_case(&self) -> super::private_key_provider::ConfigTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::private_key_provider::ConfigTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl PrivateKeyProvider

impl ::std::ops::Drop for PrivateKeyProvider {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PrivateKeyProvider {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PrivateKeyProvider {
  type Proxied = Self;
  fn as_view(&self) -> PrivateKeyProviderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PrivateKeyProvider {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PrivateKeyProviderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PrivateKeyProvider {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__PrivateKeyProvider_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1Xa3/P^$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__PrivateKeyProvider_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__PrivateKeyProvider_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PrivateKeyProvider {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PrivateKeyProvider {
  type Msg = PrivateKeyProvider;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrivateKeyProvider> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrivateKeyProvider {
  type Msg = PrivateKeyProvider;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrivateKeyProvider> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PrivateKeyProviderMut<'_> {
  type Msg = PrivateKeyProvider;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrivateKeyProvider> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrivateKeyProviderMut<'_> {
  type Msg = PrivateKeyProvider;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrivateKeyProvider> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PrivateKeyProviderView<'_> {
  type Msg = PrivateKeyProvider;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PrivateKeyProvider> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PrivateKeyProviderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod private_key_provider {

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
}  // pub mod private_key_provider


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__TlsCertificate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TlsCertificate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TlsCertificate>
}

impl ::protobuf::Message for TlsCertificate {
  type MessageView<'msg> = TlsCertificateView<'msg>;
  type MessageMut<'msg> = TlsCertificateMut<'msg>;
}

impl ::std::default::Default for TlsCertificate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TlsCertificate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TlsCertificate` is `Sync` because it does not implement interior mutability.
//    Neither does `TlsCertificateMut`.
unsafe impl ::std::marker::Sync for TlsCertificate {}

// SAFETY:
// - `TlsCertificate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TlsCertificate {}

impl ::protobuf::Proxied for TlsCertificate {
  type View<'msg> = TlsCertificateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TlsCertificate {}

impl ::protobuf::MutProxied for TlsCertificate {
  type Mut<'msg> = TlsCertificateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TlsCertificateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsCertificate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsCertificateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TlsCertificateView<'msg> {
  type Message = TlsCertificate;
}

impl ::std::fmt::Debug for TlsCertificateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TlsCertificateView<'_> {
  fn default() -> TlsCertificateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TlsCertificate>> for TlsCertificateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsCertificate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsCertificateView<'msg> {

  pub fn to_owned(&self) -> TlsCertificate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // certificate_chain: optional message envoy.config.core.v3.DataSource
  pub fn has_certificate_chain(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn certificate_chain_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_certificate_chain().then(|| self.certificate_chain())
  }
  pub fn certificate_chain(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // private_key: optional message envoy.config.core.v3.DataSource
  pub fn has_private_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn private_key_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_private_key().then(|| self.private_key())
  }
  pub fn private_key(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // pkcs12: optional message envoy.config.core.v3.DataSource
  pub fn has_pkcs12(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn pkcs12_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_pkcs12().then(|| self.pkcs12())
  }
  pub fn pkcs12(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn watched_directory_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'msg>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView::default())
  }

  // private_key_provider: optional message envoy.extensions.transport_sockets.tls.v3.PrivateKeyProvider
  pub fn has_private_key_provider(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn private_key_provider_opt(self) -> ::std::option::Option<super::PrivateKeyProviderView<'msg>> {
    self.has_private_key_provider().then(|| self.private_key_provider())
  }
  pub fn private_key_provider(self) -> super::PrivateKeyProviderView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PrivateKeyProviderView::default())
  }

  // password: optional message envoy.config.core.v3.DataSource
  pub fn has_password(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn password_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_password().then(|| self.password())
  }
  pub fn password(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // ocsp_staple: optional message envoy.config.core.v3.DataSource
  pub fn has_ocsp_staple(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn ocsp_staple_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_ocsp_staple().then(|| self.ocsp_staple())
  }
  pub fn ocsp_staple(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // signed_certificate_timestamp: repeated message envoy.config.core.v3.DataSource
  pub fn signed_certificate_timestamp(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::DataSource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `TlsCertificateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TlsCertificateView<'_> {}

// SAFETY:
// - `TlsCertificateView` is `Send` because while its alive a `TlsCertificateMut` cannot.
// - `TlsCertificateView` does not use thread-local data.
unsafe impl ::std::marker::Send for TlsCertificateView<'_> {}

impl<'msg> ::protobuf::AsView for TlsCertificateView<'msg> {
  type Proxied = TlsCertificate;
  fn as_view(&self) -> ::protobuf::View<'msg, TlsCertificate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsCertificateView<'msg> {
  fn into_view<'shorter>(self) -> TlsCertificateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsCertificate> for TlsCertificateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsCertificate {
    let mut dst = TlsCertificate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsCertificate> for TlsCertificateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsCertificate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TlsCertificate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsCertificateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsCertificateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TlsCertificateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsCertificate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsCertificateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TlsCertificateMut<'msg> {
  type Message = TlsCertificate;
}

impl ::std::fmt::Debug for TlsCertificateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TlsCertificate>> for TlsCertificateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsCertificate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsCertificateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsCertificate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TlsCertificate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // certificate_chain: optional message envoy.config.core.v3.DataSource
  pub fn has_certificate_chain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_certificate_chain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn certificate_chain_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_certificate_chain().then(|| self.certificate_chain())
  }
  pub fn certificate_chain(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn certificate_chain_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_certificate_chain(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // private_key: optional message envoy.config.core.v3.DataSource
  pub fn has_private_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_private_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn private_key_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_private_key().then(|| self.private_key())
  }
  pub fn private_key(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn private_key_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_private_key(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // pkcs12: optional message envoy.config.core.v3.DataSource
  pub fn has_pkcs12(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_pkcs12(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn pkcs12_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_pkcs12().then(|| self.pkcs12())
  }
  pub fn pkcs12(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn pkcs12_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_pkcs12(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_watched_directory(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn watched_directory_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(&self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView::default())
  }
  pub fn watched_directory_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryMut<'_> {
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
  pub fn set_watched_directory(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectory>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // private_key_provider: optional message envoy.extensions.transport_sockets.tls.v3.PrivateKeyProvider
  pub fn has_private_key_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_private_key_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn private_key_provider_opt(&self) -> ::std::option::Option<super::PrivateKeyProviderView<'_>> {
    self.has_private_key_provider().then(|| self.private_key_provider())
  }
  pub fn private_key_provider(&self) -> super::PrivateKeyProviderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PrivateKeyProviderView::default())
  }
  pub fn private_key_provider_mut(&mut self) -> super::PrivateKeyProviderMut<'_> {
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
  pub fn set_private_key_provider(&mut self,
    val: impl ::protobuf::IntoProxied<super::PrivateKeyProvider>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // password: optional message envoy.config.core.v3.DataSource
  pub fn has_password(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_password(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn password_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_password().then(|| self.password())
  }
  pub fn password(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn password_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_password(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // ocsp_staple: optional message envoy.config.core.v3.DataSource
  pub fn has_ocsp_staple(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_ocsp_staple(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn ocsp_staple_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_ocsp_staple().then(|| self.ocsp_staple())
  }
  pub fn ocsp_staple(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn ocsp_staple_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_ocsp_staple(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // signed_certificate_timestamp: repeated message envoy.config.core.v3.DataSource
  pub fn signed_certificate_timestamp(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::DataSource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn signed_certificate_timestamp_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
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
  pub fn set_signed_certificate_timestamp(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::DataSource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}

// SAFETY:
// - `TlsCertificateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TlsCertificateMut<'_> {}

// SAFETY:
// - `TlsCertificateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TlsCertificateMut<'_> {}

impl<'msg> ::protobuf::AsView for TlsCertificateMut<'msg> {
  type Proxied = TlsCertificate;
  fn as_view(&self) -> ::protobuf::View<'_, TlsCertificate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsCertificateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TlsCertificate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TlsCertificateMut<'msg> {
  type MutProxied = TlsCertificate;
  fn as_mut(&mut self) -> TlsCertificateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TlsCertificateMut<'msg> {
  fn into_mut<'shorter>(self) -> TlsCertificateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TlsCertificate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TlsCertificate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TlsCertificateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TlsCertificateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // certificate_chain: optional message envoy.config.core.v3.DataSource
  pub fn has_certificate_chain(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_certificate_chain(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn certificate_chain_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_certificate_chain().then(|| self.certificate_chain())
  }
  pub fn certificate_chain(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn certificate_chain_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_certificate_chain(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // private_key: optional message envoy.config.core.v3.DataSource
  pub fn has_private_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_private_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn private_key_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_private_key().then(|| self.private_key())
  }
  pub fn private_key(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn private_key_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_private_key(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // pkcs12: optional message envoy.config.core.v3.DataSource
  pub fn has_pkcs12(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_pkcs12(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn pkcs12_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_pkcs12().then(|| self.pkcs12())
  }
  pub fn pkcs12(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn pkcs12_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_pkcs12(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_watched_directory(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn watched_directory_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(&self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView::default())
  }
  pub fn watched_directory_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryMut<'_> {
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
  pub fn set_watched_directory(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectory>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // private_key_provider: optional message envoy.extensions.transport_sockets.tls.v3.PrivateKeyProvider
  pub fn has_private_key_provider(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_private_key_provider(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn private_key_provider_opt(&self) -> ::std::option::Option<super::PrivateKeyProviderView<'_>> {
    self.has_private_key_provider().then(|| self.private_key_provider())
  }
  pub fn private_key_provider(&self) -> super::PrivateKeyProviderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PrivateKeyProviderView::default())
  }
  pub fn private_key_provider_mut(&mut self) -> super::PrivateKeyProviderMut<'_> {
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
  pub fn set_private_key_provider(&mut self,
    val: impl ::protobuf::IntoProxied<super::PrivateKeyProvider>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // password: optional message envoy.config.core.v3.DataSource
  pub fn has_password(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_password(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn password_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_password().then(|| self.password())
  }
  pub fn password(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn password_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_password(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // ocsp_staple: optional message envoy.config.core.v3.DataSource
  pub fn has_ocsp_staple(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_ocsp_staple(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn ocsp_staple_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_ocsp_staple().then(|| self.ocsp_staple())
  }
  pub fn ocsp_staple(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn ocsp_staple_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_ocsp_staple(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // signed_certificate_timestamp: repeated message envoy.config.core.v3.DataSource
  pub fn signed_certificate_timestamp(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::DataSource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn signed_certificate_timestamp_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
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
  pub fn set_signed_certificate_timestamp(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::DataSource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

}  // impl TlsCertificate

impl ::std::ops::Drop for TlsCertificate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TlsCertificate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TlsCertificate {
  type Proxied = Self;
  fn as_view(&self) -> TlsCertificateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TlsCertificate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TlsCertificateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TlsCertificate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__TlsCertificate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333G333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__TlsCertificate_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::PrivateKeyProvider as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::WatchedDirectory as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__TlsCertificate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsCertificate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsCertificate {
  type Msg = TlsCertificate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCertificate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsCertificate {
  type Msg = TlsCertificate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCertificate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsCertificateMut<'_> {
  type Msg = TlsCertificate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCertificate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsCertificateMut<'_> {
  type Msg = TlsCertificate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCertificate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsCertificateView<'_> {
  type Msg = TlsCertificate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsCertificate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsCertificateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__TlsSessionTicketKeys_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TlsSessionTicketKeys {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TlsSessionTicketKeys>
}

impl ::protobuf::Message for TlsSessionTicketKeys {
  type MessageView<'msg> = TlsSessionTicketKeysView<'msg>;
  type MessageMut<'msg> = TlsSessionTicketKeysMut<'msg>;
}

impl ::std::default::Default for TlsSessionTicketKeys {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TlsSessionTicketKeys {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TlsSessionTicketKeys` is `Sync` because it does not implement interior mutability.
//    Neither does `TlsSessionTicketKeysMut`.
unsafe impl ::std::marker::Sync for TlsSessionTicketKeys {}

// SAFETY:
// - `TlsSessionTicketKeys` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TlsSessionTicketKeys {}

impl ::protobuf::Proxied for TlsSessionTicketKeys {
  type View<'msg> = TlsSessionTicketKeysView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TlsSessionTicketKeys {}

impl ::protobuf::MutProxied for TlsSessionTicketKeys {
  type Mut<'msg> = TlsSessionTicketKeysMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TlsSessionTicketKeysView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsSessionTicketKeys>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsSessionTicketKeysView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TlsSessionTicketKeysView<'msg> {
  type Message = TlsSessionTicketKeys;
}

impl ::std::fmt::Debug for TlsSessionTicketKeysView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TlsSessionTicketKeysView<'_> {
  fn default() -> TlsSessionTicketKeysView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TlsSessionTicketKeys>> for TlsSessionTicketKeysView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TlsSessionTicketKeys>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsSessionTicketKeysView<'msg> {

  pub fn to_owned(&self) -> TlsSessionTicketKeys {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // keys: repeated message envoy.config.core.v3.DataSource
  pub fn keys(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::DataSource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `TlsSessionTicketKeysView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TlsSessionTicketKeysView<'_> {}

// SAFETY:
// - `TlsSessionTicketKeysView` is `Send` because while its alive a `TlsSessionTicketKeysMut` cannot.
// - `TlsSessionTicketKeysView` does not use thread-local data.
unsafe impl ::std::marker::Send for TlsSessionTicketKeysView<'_> {}

impl<'msg> ::protobuf::AsView for TlsSessionTicketKeysView<'msg> {
  type Proxied = TlsSessionTicketKeys;
  fn as_view(&self) -> ::protobuf::View<'msg, TlsSessionTicketKeys> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsSessionTicketKeysView<'msg> {
  fn into_view<'shorter>(self) -> TlsSessionTicketKeysView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsSessionTicketKeys> for TlsSessionTicketKeysView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsSessionTicketKeys {
    let mut dst = TlsSessionTicketKeys::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TlsSessionTicketKeys> for TlsSessionTicketKeysMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TlsSessionTicketKeys {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TlsSessionTicketKeys {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsSessionTicketKeysView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TlsSessionTicketKeysMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TlsSessionTicketKeysMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsSessionTicketKeys>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TlsSessionTicketKeysMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TlsSessionTicketKeysMut<'msg> {
  type Message = TlsSessionTicketKeys;
}

impl ::std::fmt::Debug for TlsSessionTicketKeysMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TlsSessionTicketKeys>> for TlsSessionTicketKeysMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsSessionTicketKeys>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TlsSessionTicketKeysMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TlsSessionTicketKeys> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TlsSessionTicketKeys {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // keys: repeated message envoy.config.core.v3.DataSource
  pub fn keys(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::DataSource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn keys_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
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
  pub fn set_keys(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::DataSource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `TlsSessionTicketKeysMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TlsSessionTicketKeysMut<'_> {}

// SAFETY:
// - `TlsSessionTicketKeysMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TlsSessionTicketKeysMut<'_> {}

impl<'msg> ::protobuf::AsView for TlsSessionTicketKeysMut<'msg> {
  type Proxied = TlsSessionTicketKeys;
  fn as_view(&self) -> ::protobuf::View<'_, TlsSessionTicketKeys> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TlsSessionTicketKeysMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TlsSessionTicketKeys>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TlsSessionTicketKeysMut<'msg> {
  type MutProxied = TlsSessionTicketKeys;
  fn as_mut(&mut self) -> TlsSessionTicketKeysMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TlsSessionTicketKeysMut<'msg> {
  fn into_mut<'shorter>(self) -> TlsSessionTicketKeysMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TlsSessionTicketKeys {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TlsSessionTicketKeys> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TlsSessionTicketKeysView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TlsSessionTicketKeysMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // keys: repeated message envoy.config.core.v3.DataSource
  pub fn keys(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::base::DataSource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn keys_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::base::DataSource> {
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
  pub fn set_keys(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::base::DataSource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl TlsSessionTicketKeys

impl ::std::ops::Drop for TlsSessionTicketKeys {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TlsSessionTicketKeys {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TlsSessionTicketKeys {
  type Proxied = Self;
  fn as_view(&self) -> TlsSessionTicketKeysView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TlsSessionTicketKeys {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TlsSessionTicketKeysMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TlsSessionTicketKeys {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__TlsSessionTicketKeys_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__TlsSessionTicketKeys_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__TlsSessionTicketKeys_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsSessionTicketKeys {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsSessionTicketKeys {
  type Msg = TlsSessionTicketKeys;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsSessionTicketKeys> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsSessionTicketKeys {
  type Msg = TlsSessionTicketKeys;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsSessionTicketKeys> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TlsSessionTicketKeysMut<'_> {
  type Msg = TlsSessionTicketKeys;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsSessionTicketKeys> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsSessionTicketKeysMut<'_> {
  type Msg = TlsSessionTicketKeys;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsSessionTicketKeys> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TlsSessionTicketKeysView<'_> {
  type Msg = TlsSessionTicketKeys;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TlsSessionTicketKeys> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TlsSessionTicketKeysMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__CertificateProviderPluginInstance_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CertificateProviderPluginInstance {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CertificateProviderPluginInstance>
}

impl ::protobuf::Message for CertificateProviderPluginInstance {
  type MessageView<'msg> = CertificateProviderPluginInstanceView<'msg>;
  type MessageMut<'msg> = CertificateProviderPluginInstanceMut<'msg>;
}

impl ::std::default::Default for CertificateProviderPluginInstance {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CertificateProviderPluginInstance {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CertificateProviderPluginInstance` is `Sync` because it does not implement interior mutability.
//    Neither does `CertificateProviderPluginInstanceMut`.
unsafe impl ::std::marker::Sync for CertificateProviderPluginInstance {}

// SAFETY:
// - `CertificateProviderPluginInstance` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CertificateProviderPluginInstance {}

impl ::protobuf::Proxied for CertificateProviderPluginInstance {
  type View<'msg> = CertificateProviderPluginInstanceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CertificateProviderPluginInstance {}

impl ::protobuf::MutProxied for CertificateProviderPluginInstance {
  type Mut<'msg> = CertificateProviderPluginInstanceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CertificateProviderPluginInstanceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProviderPluginInstance>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificateProviderPluginInstanceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CertificateProviderPluginInstanceView<'msg> {
  type Message = CertificateProviderPluginInstance;
}

impl ::std::fmt::Debug for CertificateProviderPluginInstanceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CertificateProviderPluginInstanceView<'_> {
  fn default() -> CertificateProviderPluginInstanceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProviderPluginInstance>> for CertificateProviderPluginInstanceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateProviderPluginInstance>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificateProviderPluginInstanceView<'msg> {

  pub fn to_owned(&self) -> CertificateProviderPluginInstance {
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
// - `CertificateProviderPluginInstanceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CertificateProviderPluginInstanceView<'_> {}

// SAFETY:
// - `CertificateProviderPluginInstanceView` is `Send` because while its alive a `CertificateProviderPluginInstanceMut` cannot.
// - `CertificateProviderPluginInstanceView` does not use thread-local data.
unsafe impl ::std::marker::Send for CertificateProviderPluginInstanceView<'_> {}

impl<'msg> ::protobuf::AsView for CertificateProviderPluginInstanceView<'msg> {
  type Proxied = CertificateProviderPluginInstance;
  fn as_view(&self) -> ::protobuf::View<'msg, CertificateProviderPluginInstance> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificateProviderPluginInstanceView<'msg> {
  fn into_view<'shorter>(self) -> CertificateProviderPluginInstanceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateProviderPluginInstance> for CertificateProviderPluginInstanceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateProviderPluginInstance {
    let mut dst = CertificateProviderPluginInstance::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateProviderPluginInstance> for CertificateProviderPluginInstanceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateProviderPluginInstance {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CertificateProviderPluginInstance {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificateProviderPluginInstanceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificateProviderPluginInstanceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CertificateProviderPluginInstanceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProviderPluginInstance>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificateProviderPluginInstanceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CertificateProviderPluginInstanceMut<'msg> {
  type Message = CertificateProviderPluginInstance;
}

impl ::std::fmt::Debug for CertificateProviderPluginInstanceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProviderPluginInstance>> for CertificateProviderPluginInstanceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProviderPluginInstance>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificateProviderPluginInstanceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateProviderPluginInstance> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CertificateProviderPluginInstance {
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
// - `CertificateProviderPluginInstanceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CertificateProviderPluginInstanceMut<'_> {}

// SAFETY:
// - `CertificateProviderPluginInstanceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CertificateProviderPluginInstanceMut<'_> {}

impl<'msg> ::protobuf::AsView for CertificateProviderPluginInstanceMut<'msg> {
  type Proxied = CertificateProviderPluginInstance;
  fn as_view(&self) -> ::protobuf::View<'_, CertificateProviderPluginInstance> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificateProviderPluginInstanceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CertificateProviderPluginInstance>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CertificateProviderPluginInstanceMut<'msg> {
  type MutProxied = CertificateProviderPluginInstance;
  fn as_mut(&mut self) -> CertificateProviderPluginInstanceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CertificateProviderPluginInstanceMut<'msg> {
  fn into_mut<'shorter>(self) -> CertificateProviderPluginInstanceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CertificateProviderPluginInstance {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CertificateProviderPluginInstance> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CertificateProviderPluginInstanceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CertificateProviderPluginInstanceMut<'_> {
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

}  // impl CertificateProviderPluginInstance

impl ::std::ops::Drop for CertificateProviderPluginInstance {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CertificateProviderPluginInstance {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CertificateProviderPluginInstance {
  type Proxied = Self;
  fn as_view(&self) -> CertificateProviderPluginInstanceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CertificateProviderPluginInstance {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CertificateProviderPluginInstanceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CertificateProviderPluginInstance {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__CertificateProviderPluginInstance_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1P1P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__CertificateProviderPluginInstance_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__CertificateProviderPluginInstance_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificateProviderPluginInstance {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificateProviderPluginInstance {
  type Msg = CertificateProviderPluginInstance;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderPluginInstance> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProviderPluginInstance {
  type Msg = CertificateProviderPluginInstance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderPluginInstance> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificateProviderPluginInstanceMut<'_> {
  type Msg = CertificateProviderPluginInstance;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderPluginInstance> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProviderPluginInstanceMut<'_> {
  type Msg = CertificateProviderPluginInstance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderPluginInstance> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateProviderPluginInstanceView<'_> {
  type Msg = CertificateProviderPluginInstance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateProviderPluginInstance> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificateProviderPluginInstanceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__SubjectAltNameMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SubjectAltNameMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SubjectAltNameMatcher>
}

impl ::protobuf::Message for SubjectAltNameMatcher {
  type MessageView<'msg> = SubjectAltNameMatcherView<'msg>;
  type MessageMut<'msg> = SubjectAltNameMatcherMut<'msg>;
}

impl ::std::default::Default for SubjectAltNameMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SubjectAltNameMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SubjectAltNameMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `SubjectAltNameMatcherMut`.
unsafe impl ::std::marker::Sync for SubjectAltNameMatcher {}

// SAFETY:
// - `SubjectAltNameMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SubjectAltNameMatcher {}

impl ::protobuf::Proxied for SubjectAltNameMatcher {
  type View<'msg> = SubjectAltNameMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SubjectAltNameMatcher {}

impl ::protobuf::MutProxied for SubjectAltNameMatcher {
  type Mut<'msg> = SubjectAltNameMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SubjectAltNameMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SubjectAltNameMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubjectAltNameMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SubjectAltNameMatcherView<'msg> {
  type Message = SubjectAltNameMatcher;
}

impl ::std::fmt::Debug for SubjectAltNameMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SubjectAltNameMatcherView<'_> {
  fn default() -> SubjectAltNameMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SubjectAltNameMatcher>> for SubjectAltNameMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SubjectAltNameMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubjectAltNameMatcherView<'msg> {

  pub fn to_owned(&self) -> SubjectAltNameMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // san_type: optional enum envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher.SanType
  pub fn san_type(self) -> super::subject_alt_name_matcher::SanType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::subject_alt_name_matcher::SanType::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // matcher: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn matcher_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

  // oid: optional string
  pub fn oid(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `SubjectAltNameMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SubjectAltNameMatcherView<'_> {}

// SAFETY:
// - `SubjectAltNameMatcherView` is `Send` because while its alive a `SubjectAltNameMatcherMut` cannot.
// - `SubjectAltNameMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for SubjectAltNameMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for SubjectAltNameMatcherView<'msg> {
  type Proxied = SubjectAltNameMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, SubjectAltNameMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubjectAltNameMatcherView<'msg> {
  fn into_view<'shorter>(self) -> SubjectAltNameMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SubjectAltNameMatcher> for SubjectAltNameMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SubjectAltNameMatcher {
    let mut dst = SubjectAltNameMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SubjectAltNameMatcher> for SubjectAltNameMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SubjectAltNameMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SubjectAltNameMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SubjectAltNameMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SubjectAltNameMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SubjectAltNameMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectAltNameMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SubjectAltNameMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SubjectAltNameMatcherMut<'msg> {
  type Message = SubjectAltNameMatcher;
}

impl ::std::fmt::Debug for SubjectAltNameMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectAltNameMatcher>> for SubjectAltNameMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectAltNameMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SubjectAltNameMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SubjectAltNameMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SubjectAltNameMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // san_type: optional enum envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher.SanType
  pub fn san_type(&self) -> super::subject_alt_name_matcher::SanType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::subject_alt_name_matcher::SanType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_san_type(&mut self, val: super::subject_alt_name_matcher::SanType) {
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

  // matcher: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // oid: optional string
  pub fn oid(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_oid(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `SubjectAltNameMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SubjectAltNameMatcherMut<'_> {}

// SAFETY:
// - `SubjectAltNameMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SubjectAltNameMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for SubjectAltNameMatcherMut<'msg> {
  type Proxied = SubjectAltNameMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, SubjectAltNameMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SubjectAltNameMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SubjectAltNameMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SubjectAltNameMatcherMut<'msg> {
  type MutProxied = SubjectAltNameMatcher;
  fn as_mut(&mut self) -> SubjectAltNameMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SubjectAltNameMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> SubjectAltNameMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SubjectAltNameMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SubjectAltNameMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SubjectAltNameMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SubjectAltNameMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // san_type: optional enum envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher.SanType
  pub fn san_type(&self) -> super::subject_alt_name_matcher::SanType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::subject_alt_name_matcher::SanType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_san_type(&mut self, val: super::subject_alt_name_matcher::SanType) {
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

  // matcher: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // oid: optional string
  pub fn oid(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_oid(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl SubjectAltNameMatcher

impl ::std::ops::Drop for SubjectAltNameMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SubjectAltNameMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SubjectAltNameMatcher {
  type Proxied = Self;
  fn as_view(&self) -> SubjectAltNameMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SubjectAltNameMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SubjectAltNameMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SubjectAltNameMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__SubjectAltNameMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__SubjectAltNameMatcher_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__SubjectAltNameMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubjectAltNameMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubjectAltNameMatcher {
  type Msg = SubjectAltNameMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltNameMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectAltNameMatcher {
  type Msg = SubjectAltNameMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltNameMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SubjectAltNameMatcherMut<'_> {
  type Msg = SubjectAltNameMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltNameMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectAltNameMatcherMut<'_> {
  type Msg = SubjectAltNameMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltNameMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SubjectAltNameMatcherView<'_> {
  type Msg = SubjectAltNameMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SubjectAltNameMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SubjectAltNameMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod subject_alt_name_matcher {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SanType(i32);

#[allow(non_upper_case_globals)]
impl SanType {
  pub const Unspecified: SanType = SanType(0);
  pub const Email: SanType = SanType(1);
  pub const Dns: SanType = SanType(2);
  pub const Uri: SanType = SanType(3);
  pub const IpAddress: SanType = SanType(4);
  pub const OtherName: SanType = SanType(5);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Email",
      2 => "Dns",
      3 => "Uri",
      4 => "IpAddress",
      5 => "OtherName",
      _ => return None
    })
  }
}

impl ::std::convert::From<SanType> for i32 {
  fn from(val: SanType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for SanType {
  fn from(val: i32) -> SanType {
    Self(val)
  }
}

impl ::std::default::Default for SanType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for SanType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "SanType::{}", constant_name)
    } else {
      write!(f, "SanType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for SanType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for SanType {}

impl ::protobuf::Proxied for SanType {
  type View<'a> = SanType;
}

impl ::protobuf::AsView for SanType {
  type Proxied = SanType;

  fn as_view(&self) -> SanType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SanType {
  fn into_view<'shorter>(self) -> SanType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for SanType {
  const NAME: &'static str = "SanType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5)
  }
}

impl ::protobuf::__internal::EntityType for SanType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod subject_alt_name_matcher


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__CertificateValidationContext_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CertificateValidationContext {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CertificateValidationContext>
}

impl ::protobuf::Message for CertificateValidationContext {
  type MessageView<'msg> = CertificateValidationContextView<'msg>;
  type MessageMut<'msg> = CertificateValidationContextMut<'msg>;
}

impl ::std::default::Default for CertificateValidationContext {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CertificateValidationContext {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CertificateValidationContext` is `Sync` because it does not implement interior mutability.
//    Neither does `CertificateValidationContextMut`.
unsafe impl ::std::marker::Sync for CertificateValidationContext {}

// SAFETY:
// - `CertificateValidationContext` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CertificateValidationContext {}

impl ::protobuf::Proxied for CertificateValidationContext {
  type View<'msg> = CertificateValidationContextView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CertificateValidationContext {}

impl ::protobuf::MutProxied for CertificateValidationContext {
  type Mut<'msg> = CertificateValidationContextMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CertificateValidationContextView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateValidationContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificateValidationContextView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CertificateValidationContextView<'msg> {
  type Message = CertificateValidationContext;
}

impl ::std::fmt::Debug for CertificateValidationContextView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CertificateValidationContextView<'_> {
  fn default() -> CertificateValidationContextView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateValidationContext>> for CertificateValidationContextView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CertificateValidationContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificateValidationContextView<'msg> {

  pub fn to_owned(&self) -> CertificateValidationContext {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // trusted_ca: optional message envoy.config.core.v3.DataSource
  pub fn has_trusted_ca(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn trusted_ca_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_trusted_ca().then(|| self.trusted_ca())
  }
  pub fn trusted_ca(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // ca_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance
  pub fn has_ca_certificate_provider_instance(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn ca_certificate_provider_instance_opt(self) -> ::std::option::Option<super::CertificateProviderPluginInstanceView<'msg>> {
    self.has_ca_certificate_provider_instance().then(|| self.ca_certificate_provider_instance())
  }
  pub fn ca_certificate_provider_instance(self) -> super::CertificateProviderPluginInstanceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CertificateProviderPluginInstanceView::default())
  }

  // system_root_certs: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext.SystemRootCerts
  pub fn has_system_root_certs(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn system_root_certs_opt(self) -> ::std::option::Option<super::certificate_validation_context::SystemRootCertsView<'msg>> {
    self.has_system_root_certs().then(|| self.system_root_certs())
  }
  pub fn system_root_certs(self) -> super::certificate_validation_context::SystemRootCertsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::certificate_validation_context::SystemRootCertsView::default())
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn watched_directory_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'msg>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView::default())
  }

  // verify_certificate_spki: repeated string
  pub fn verify_certificate_spki(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // verify_certificate_hash: repeated string
  pub fn verify_certificate_hash(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // match_typed_subject_alt_names: repeated message envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher
  pub fn match_typed_subject_alt_names(self) -> ::protobuf::RepeatedView<'msg, super::SubjectAltNameMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SubjectAltNameMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // match_subject_alt_names: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn match_subject_alt_names(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // require_signed_certificate_timestamp: optional message google.protobuf.BoolValue
  pub fn has_require_signed_certificate_timestamp(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn require_signed_certificate_timestamp_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_require_signed_certificate_timestamp().then(|| self.require_signed_certificate_timestamp())
  }
  pub fn require_signed_certificate_timestamp(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // crl: optional message envoy.config.core.v3.DataSource
  pub fn has_crl(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn crl_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg>> {
    self.has_crl().then(|| self.crl())
  }
  pub fn crl(self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }

  // allow_expired_certificate: optional bool
  pub fn allow_expired_certificate(self) -> bool {
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

  // trust_chain_verification: optional enum envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext.TrustChainVerification
  pub fn trust_chain_verification(self) -> super::certificate_validation_context::TrustChainVerification {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::certificate_validation_context::TrustChainVerification::VerifyTrustChain).into()
      ).try_into().unwrap()
    }
  }

  // custom_validator_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_validator_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn custom_validator_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom_validator_config().then(|| self.custom_validator_config())
  }
  pub fn custom_validator_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // only_verify_leaf_cert_crl: optional bool
  pub fn only_verify_leaf_cert_crl(self) -> bool {
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

  // max_verify_depth: optional message google.protobuf.UInt32Value
  pub fn has_max_verify_depth(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn max_verify_depth_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_verify_depth().then(|| self.max_verify_depth())
  }
  pub fn max_verify_depth(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `CertificateValidationContextView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CertificateValidationContextView<'_> {}

// SAFETY:
// - `CertificateValidationContextView` is `Send` because while its alive a `CertificateValidationContextMut` cannot.
// - `CertificateValidationContextView` does not use thread-local data.
unsafe impl ::std::marker::Send for CertificateValidationContextView<'_> {}

impl<'msg> ::protobuf::AsView for CertificateValidationContextView<'msg> {
  type Proxied = CertificateValidationContext;
  fn as_view(&self) -> ::protobuf::View<'msg, CertificateValidationContext> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificateValidationContextView<'msg> {
  fn into_view<'shorter>(self) -> CertificateValidationContextView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateValidationContext> for CertificateValidationContextView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateValidationContext {
    let mut dst = CertificateValidationContext::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CertificateValidationContext> for CertificateValidationContextMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CertificateValidationContext {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CertificateValidationContext {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificateValidationContextView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CertificateValidationContextMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CertificateValidationContextMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateValidationContext>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CertificateValidationContextMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CertificateValidationContextMut<'msg> {
  type Message = CertificateValidationContext;
}

impl ::std::fmt::Debug for CertificateValidationContextMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateValidationContext>> for CertificateValidationContextMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateValidationContext>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CertificateValidationContextMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CertificateValidationContext> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CertificateValidationContext {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // trusted_ca: optional message envoy.config.core.v3.DataSource
  pub fn has_trusted_ca(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trusted_ca(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trusted_ca_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_trusted_ca().then(|| self.trusted_ca())
  }
  pub fn trusted_ca(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn trusted_ca_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_trusted_ca(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // ca_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance
  pub fn has_ca_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_ca_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn ca_certificate_provider_instance_opt(&self) -> ::std::option::Option<super::CertificateProviderPluginInstanceView<'_>> {
    self.has_ca_certificate_provider_instance().then(|| self.ca_certificate_provider_instance())
  }
  pub fn ca_certificate_provider_instance(&self) -> super::CertificateProviderPluginInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CertificateProviderPluginInstanceView::default())
  }
  pub fn ca_certificate_provider_instance_mut(&mut self) -> super::CertificateProviderPluginInstanceMut<'_> {
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
  pub fn set_ca_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<super::CertificateProviderPluginInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // system_root_certs: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext.SystemRootCerts
  pub fn has_system_root_certs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_system_root_certs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn system_root_certs_opt(&self) -> ::std::option::Option<super::certificate_validation_context::SystemRootCertsView<'_>> {
    self.has_system_root_certs().then(|| self.system_root_certs())
  }
  pub fn system_root_certs(&self) -> super::certificate_validation_context::SystemRootCertsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::certificate_validation_context::SystemRootCertsView::default())
  }
  pub fn system_root_certs_mut(&mut self) -> super::certificate_validation_context::SystemRootCertsMut<'_> {
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
  pub fn set_system_root_certs(&mut self,
    val: impl ::protobuf::IntoProxied<super::certificate_validation_context::SystemRootCerts>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_watched_directory(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn watched_directory_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(&self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView::default())
  }
  pub fn watched_directory_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryMut<'_> {
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
  pub fn set_watched_directory(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectory>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // verify_certificate_spki: repeated string
  pub fn verify_certificate_spki(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn verify_certificate_spki_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_verify_certificate_spki(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // verify_certificate_hash: repeated string
  pub fn verify_certificate_hash(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn verify_certificate_hash_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_verify_certificate_hash(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // match_typed_subject_alt_names: repeated message envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher
  pub fn match_typed_subject_alt_names(&self) -> ::protobuf::RepeatedView<'_, super::SubjectAltNameMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SubjectAltNameMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn match_typed_subject_alt_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::SubjectAltNameMatcher> {
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
  pub fn set_match_typed_subject_alt_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::SubjectAltNameMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // match_subject_alt_names: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn match_subject_alt_names(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn match_subject_alt_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_match_subject_alt_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // require_signed_certificate_timestamp: optional message google.protobuf.BoolValue
  pub fn has_require_signed_certificate_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_require_signed_certificate_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn require_signed_certificate_timestamp_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_require_signed_certificate_timestamp().then(|| self.require_signed_certificate_timestamp())
  }
  pub fn require_signed_certificate_timestamp(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn require_signed_certificate_timestamp_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_require_signed_certificate_timestamp(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // crl: optional message envoy.config.core.v3.DataSource
  pub fn has_crl(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_crl(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn crl_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_crl().then(|| self.crl())
  }
  pub fn crl(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn crl_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_crl(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // allow_expired_certificate: optional bool
  pub fn allow_expired_certificate(&self) -> bool {
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
  pub fn set_allow_expired_certificate(&mut self, val: bool) {
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

  // trust_chain_verification: optional enum envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext.TrustChainVerification
  pub fn trust_chain_verification(&self) -> super::certificate_validation_context::TrustChainVerification {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::certificate_validation_context::TrustChainVerification::VerifyTrustChain).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_trust_chain_verification(&mut self, val: super::certificate_validation_context::TrustChainVerification) {
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

  // custom_validator_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_validator_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_custom_validator_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn custom_validator_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_validator_config().then(|| self.custom_validator_config())
  }
  pub fn custom_validator_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_validator_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_validator_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // only_verify_leaf_cert_crl: optional bool
  pub fn only_verify_leaf_cert_crl(&self) -> bool {
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
  pub fn set_only_verify_leaf_cert_crl(&mut self, val: bool) {
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

  // max_verify_depth: optional message google.protobuf.UInt32Value
  pub fn has_max_verify_depth(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_max_verify_depth(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn max_verify_depth_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_verify_depth().then(|| self.max_verify_depth())
  }
  pub fn max_verify_depth(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_verify_depth_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_verify_depth(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

}

// SAFETY:
// - `CertificateValidationContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CertificateValidationContextMut<'_> {}

// SAFETY:
// - `CertificateValidationContextMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CertificateValidationContextMut<'_> {}

impl<'msg> ::protobuf::AsView for CertificateValidationContextMut<'msg> {
  type Proxied = CertificateValidationContext;
  fn as_view(&self) -> ::protobuf::View<'_, CertificateValidationContext> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CertificateValidationContextMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CertificateValidationContext>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CertificateValidationContextMut<'msg> {
  type MutProxied = CertificateValidationContext;
  fn as_mut(&mut self) -> CertificateValidationContextMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CertificateValidationContextMut<'msg> {
  fn into_mut<'shorter>(self) -> CertificateValidationContextMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CertificateValidationContext {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CertificateValidationContext> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CertificateValidationContextView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CertificateValidationContextMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // trusted_ca: optional message envoy.config.core.v3.DataSource
  pub fn has_trusted_ca(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trusted_ca(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trusted_ca_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_trusted_ca().then(|| self.trusted_ca())
  }
  pub fn trusted_ca(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn trusted_ca_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_trusted_ca(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // ca_certificate_provider_instance: optional message envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance
  pub fn has_ca_certificate_provider_instance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_ca_certificate_provider_instance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn ca_certificate_provider_instance_opt(&self) -> ::std::option::Option<super::CertificateProviderPluginInstanceView<'_>> {
    self.has_ca_certificate_provider_instance().then(|| self.ca_certificate_provider_instance())
  }
  pub fn ca_certificate_provider_instance(&self) -> super::CertificateProviderPluginInstanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::CertificateProviderPluginInstanceView::default())
  }
  pub fn ca_certificate_provider_instance_mut(&mut self) -> super::CertificateProviderPluginInstanceMut<'_> {
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
  pub fn set_ca_certificate_provider_instance(&mut self,
    val: impl ::protobuf::IntoProxied<super::CertificateProviderPluginInstance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // system_root_certs: optional message envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext.SystemRootCerts
  pub fn has_system_root_certs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_system_root_certs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn system_root_certs_opt(&self) -> ::std::option::Option<super::certificate_validation_context::SystemRootCertsView<'_>> {
    self.has_system_root_certs().then(|| self.system_root_certs())
  }
  pub fn system_root_certs(&self) -> super::certificate_validation_context::SystemRootCertsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::certificate_validation_context::SystemRootCertsView::default())
  }
  pub fn system_root_certs_mut(&mut self) -> super::certificate_validation_context::SystemRootCertsMut<'_> {
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
  pub fn set_system_root_certs(&mut self,
    val: impl ::protobuf::IntoProxied<super::certificate_validation_context::SystemRootCerts>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // watched_directory: optional message envoy.config.core.v3.WatchedDirectory
  pub fn has_watched_directory(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_watched_directory(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn watched_directory_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_>> {
    self.has_watched_directory().then(|| self.watched_directory())
  }
  pub fn watched_directory(&self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryView::default())
  }
  pub fn watched_directory_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::WatchedDirectoryMut<'_> {
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
  pub fn set_watched_directory(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::WatchedDirectory>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // verify_certificate_spki: repeated string
  pub fn verify_certificate_spki(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn verify_certificate_spki_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_verify_certificate_spki(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // verify_certificate_hash: repeated string
  pub fn verify_certificate_hash(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn verify_certificate_hash_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_verify_certificate_hash(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // match_typed_subject_alt_names: repeated message envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher
  pub fn match_typed_subject_alt_names(&self) -> ::protobuf::RepeatedView<'_, super::SubjectAltNameMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        12
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SubjectAltNameMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn match_typed_subject_alt_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::SubjectAltNameMatcher> {
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
  pub fn set_match_typed_subject_alt_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::SubjectAltNameMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        src);
    }
  }

  // match_subject_alt_names: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn match_subject_alt_names(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn match_subject_alt_names_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
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
  pub fn set_match_subject_alt_names(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // require_signed_certificate_timestamp: optional message google.protobuf.BoolValue
  pub fn has_require_signed_certificate_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_require_signed_certificate_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn require_signed_certificate_timestamp_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_require_signed_certificate_timestamp().then(|| self.require_signed_certificate_timestamp())
  }
  pub fn require_signed_certificate_timestamp(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn require_signed_certificate_timestamp_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_require_signed_certificate_timestamp(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // crl: optional message envoy.config.core.v3.DataSource
  pub fn has_crl(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_crl(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn crl_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_>> {
    self.has_crl().then(|| self.crl())
  }
  pub fn crl(&self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::DataSourceView::default())
  }
  pub fn crl_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::DataSourceMut<'_> {
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
  pub fn set_crl(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::DataSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // allow_expired_certificate: optional bool
  pub fn allow_expired_certificate(&self) -> bool {
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
  pub fn set_allow_expired_certificate(&mut self, val: bool) {
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

  // trust_chain_verification: optional enum envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext.TrustChainVerification
  pub fn trust_chain_verification(&self) -> super::certificate_validation_context::TrustChainVerification {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::certificate_validation_context::TrustChainVerification::VerifyTrustChain).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_trust_chain_verification(&mut self, val: super::certificate_validation_context::TrustChainVerification) {
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

  // custom_validator_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_validator_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_custom_validator_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn custom_validator_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_validator_config().then(|| self.custom_validator_config())
  }
  pub fn custom_validator_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_validator_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_validator_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // only_verify_leaf_cert_crl: optional bool
  pub fn only_verify_leaf_cert_crl(&self) -> bool {
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
  pub fn set_only_verify_leaf_cert_crl(&mut self, val: bool) {
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

  // max_verify_depth: optional message google.protobuf.UInt32Value
  pub fn has_max_verify_depth(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_max_verify_depth(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn max_verify_depth_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_verify_depth().then(|| self.max_verify_depth())
  }
  pub fn max_verify_depth(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_verify_depth_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_verify_depth(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

}  // impl CertificateValidationContext

impl ::std::ops::Drop for CertificateValidationContext {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CertificateValidationContext {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CertificateValidationContext {
  type Proxied = Self;
  fn as_view(&self) -> CertificateValidationContextView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CertificateValidationContext {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CertificateValidationContextMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CertificateValidationContext {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__transport_0sockets__tls__v3__CertificateValidationContext_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3ETETb33/PG.P333/PG33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__transport_0sockets__tls__v3__CertificateValidationContext_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::DataSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::WatchedDirectory as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::CertificateProviderPluginInstance as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SubjectAltNameMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::certificate_validation_context::SystemRootCerts as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__transport_0sockets__tls__v3__CertificateValidationContext_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificateValidationContext {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificateValidationContext {
  type Msg = CertificateValidationContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateValidationContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateValidationContext {
  type Msg = CertificateValidationContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateValidationContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CertificateValidationContextMut<'_> {
  type Msg = CertificateValidationContext;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateValidationContext> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateValidationContextMut<'_> {
  type Msg = CertificateValidationContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateValidationContext> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CertificateValidationContextView<'_> {
  type Msg = CertificateValidationContext;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CertificateValidationContext> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CertificateValidationContextMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod certificate_validation_context {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__transport_0sockets__tls__v3__CertificateValidationContext__SystemRootCerts_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SystemRootCerts {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SystemRootCerts>
}

impl ::protobuf::Message for SystemRootCerts {
  type MessageView<'msg> = SystemRootCertsView<'msg>;
  type MessageMut<'msg> = SystemRootCertsMut<'msg>;
}

impl ::std::default::Default for SystemRootCerts {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SystemRootCerts {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SystemRootCerts` is `Sync` because it does not implement interior mutability.
//    Neither does `SystemRootCertsMut`.
unsafe impl ::std::marker::Sync for SystemRootCerts {}

// SAFETY:
// - `SystemRootCerts` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SystemRootCerts {}

impl ::protobuf::Proxied for SystemRootCerts {
  type View<'msg> = SystemRootCertsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SystemRootCerts {}

impl ::protobuf::MutProxied for SystemRootCerts {
  type Mut<'msg> = SystemRootCertsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SystemRootCertsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SystemRootCerts>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SystemRootCertsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SystemRootCertsView<'msg> {
  type Message = SystemRootCerts;
}

impl ::std::fmt::Debug for SystemRootCertsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SystemRootCertsView<'_> {
  fn default() -> SystemRootCertsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SystemRootCerts>> for SystemRootCertsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SystemRootCerts>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SystemRootCertsView<'msg> {

  pub fn to_owned(&self) -> SystemRootCerts {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `SystemRootCertsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SystemRootCertsView<'_> {}

// SAFETY:
// - `SystemRootCertsView` is `Send` because while its alive a `SystemRootCertsMut` cannot.
// - `SystemRootCertsView` does not use thread-local data.
unsafe impl ::std::marker::Send for SystemRootCertsView<'_> {}

impl<'msg> ::protobuf::AsView for SystemRootCertsView<'msg> {
  type Proxied = SystemRootCerts;
  fn as_view(&self) -> ::protobuf::View<'msg, SystemRootCerts> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SystemRootCertsView<'msg> {
  fn into_view<'shorter>(self) -> SystemRootCertsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SystemRootCerts> for SystemRootCertsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SystemRootCerts {
    let mut dst = SystemRootCerts::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SystemRootCerts> for SystemRootCertsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SystemRootCerts {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SystemRootCerts {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SystemRootCertsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SystemRootCertsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SystemRootCertsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SystemRootCerts>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SystemRootCertsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SystemRootCertsMut<'msg> {
  type Message = SystemRootCerts;
}

impl ::std::fmt::Debug for SystemRootCertsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SystemRootCerts>> for SystemRootCertsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SystemRootCerts>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SystemRootCertsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SystemRootCerts> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SystemRootCerts {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `SystemRootCertsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SystemRootCertsMut<'_> {}

// SAFETY:
// - `SystemRootCertsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SystemRootCertsMut<'_> {}

impl<'msg> ::protobuf::AsView for SystemRootCertsMut<'msg> {
  type Proxied = SystemRootCerts;
  fn as_view(&self) -> ::protobuf::View<'_, SystemRootCerts> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SystemRootCertsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SystemRootCerts>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SystemRootCertsMut<'msg> {
  type MutProxied = SystemRootCerts;
  fn as_mut(&mut self) -> SystemRootCertsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SystemRootCertsMut<'msg> {
  fn into_mut<'shorter>(self) -> SystemRootCertsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SystemRootCerts {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SystemRootCerts> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SystemRootCertsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SystemRootCertsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl SystemRootCerts

impl ::std::ops::Drop for SystemRootCerts {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SystemRootCerts {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SystemRootCerts {
  type Proxied = Self;
  fn as_view(&self) -> SystemRootCertsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SystemRootCerts {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SystemRootCertsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SystemRootCerts {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::certificate_validation_context::envoy__extensions__transport_0sockets__tls__v3__CertificateValidationContext__SystemRootCerts_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::certificate_validation_context::envoy__extensions__transport_0sockets__tls__v3__CertificateValidationContext__SystemRootCerts_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::certificate_validation_context::envoy__extensions__transport_0sockets__tls__v3__CertificateValidationContext__SystemRootCerts_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SystemRootCerts {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SystemRootCerts {
  type Msg = SystemRootCerts;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemRootCerts> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SystemRootCerts {
  type Msg = SystemRootCerts;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemRootCerts> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SystemRootCertsMut<'_> {
  type Msg = SystemRootCerts;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemRootCerts> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SystemRootCertsMut<'_> {
  type Msg = SystemRootCerts;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemRootCerts> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SystemRootCertsView<'_> {
  type Msg = SystemRootCerts;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SystemRootCerts> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SystemRootCertsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TrustChainVerification(i32);

#[allow(non_upper_case_globals)]
impl TrustChainVerification {
  pub const VerifyTrustChain: TrustChainVerification = TrustChainVerification(0);
  pub const AcceptUntrusted: TrustChainVerification = TrustChainVerification(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "VerifyTrustChain",
      1 => "AcceptUntrusted",
      _ => return None
    })
  }
}

impl ::std::convert::From<TrustChainVerification> for i32 {
  fn from(val: TrustChainVerification) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for TrustChainVerification {
  fn from(val: i32) -> TrustChainVerification {
    Self(val)
  }
}

impl ::std::default::Default for TrustChainVerification {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for TrustChainVerification {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "TrustChainVerification::{}", constant_name)
    } else {
      write!(f, "TrustChainVerification::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for TrustChainVerification {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for TrustChainVerification {}

impl ::protobuf::Proxied for TrustChainVerification {
  type View<'a> = TrustChainVerification;
}

impl ::protobuf::AsView for TrustChainVerification {
  type Proxied = TrustChainVerification;

  fn as_view(&self) -> TrustChainVerification {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TrustChainVerification {
  fn into_view<'shorter>(self) -> TrustChainVerification where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for TrustChainVerification {
  const NAME: &'static str = "TrustChainVerification";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for TrustChainVerification {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod certificate_validation_context


