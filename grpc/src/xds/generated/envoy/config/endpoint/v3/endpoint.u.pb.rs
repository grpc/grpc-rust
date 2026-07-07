const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__ClusterLoadAssignment_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClusterLoadAssignment {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClusterLoadAssignment>
}

impl ::protobuf::Message for ClusterLoadAssignment {
  type MessageView<'msg> = ClusterLoadAssignmentView<'msg>;
  type MessageMut<'msg> = ClusterLoadAssignmentMut<'msg>;
}

impl ::std::default::Default for ClusterLoadAssignment {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClusterLoadAssignment {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClusterLoadAssignment` is `Sync` because it does not implement interior mutability.
//    Neither does `ClusterLoadAssignmentMut`.
unsafe impl ::std::marker::Sync for ClusterLoadAssignment {}

// SAFETY:
// - `ClusterLoadAssignment` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClusterLoadAssignment {}

impl ::protobuf::Proxied for ClusterLoadAssignment {
  type View<'msg> = ClusterLoadAssignmentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClusterLoadAssignment {}

impl ::protobuf::MutProxied for ClusterLoadAssignment {
  type Mut<'msg> = ClusterLoadAssignmentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClusterLoadAssignmentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterLoadAssignment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterLoadAssignmentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClusterLoadAssignmentView<'msg> {
  type Message = ClusterLoadAssignment;
}

impl ::std::fmt::Debug for ClusterLoadAssignmentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClusterLoadAssignmentView<'_> {
  fn default() -> ClusterLoadAssignmentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterLoadAssignment>> for ClusterLoadAssignmentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterLoadAssignment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterLoadAssignmentView<'msg> {

  pub fn to_owned(&self) -> ClusterLoadAssignment {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // cluster_name: optional string
  pub fn cluster_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // endpoints: repeated message envoy.config.endpoint.v3.LocalityLbEndpoints
  pub fn endpoints(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // named_endpoints: repeated message envoy.config.endpoint.v3.ClusterLoadAssignment.NamedEndpointsEntry
  pub fn named_endpoints(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // policy: optional message envoy.config.endpoint.v3.ClusterLoadAssignment.Policy
  pub fn has_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn policy_opt(self) -> ::std::option::Option<super::cluster_load_assignment::PolicyView<'msg>> {
    self.has_policy().then(|| self.policy())
  }
  pub fn policy(self) -> super::cluster_load_assignment::PolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster_load_assignment::PolicyView::default())
  }

}

// SAFETY:
// - `ClusterLoadAssignmentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClusterLoadAssignmentView<'_> {}

// SAFETY:
// - `ClusterLoadAssignmentView` is `Send` because while its alive a `ClusterLoadAssignmentMut` cannot.
// - `ClusterLoadAssignmentView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClusterLoadAssignmentView<'_> {}

impl<'msg> ::protobuf::AsView for ClusterLoadAssignmentView<'msg> {
  type Proxied = ClusterLoadAssignment;
  fn as_view(&self) -> ::protobuf::View<'msg, ClusterLoadAssignment> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterLoadAssignmentView<'msg> {
  fn into_view<'shorter>(self) -> ClusterLoadAssignmentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterLoadAssignment> for ClusterLoadAssignmentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterLoadAssignment {
    let mut dst = ClusterLoadAssignment::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterLoadAssignment> for ClusterLoadAssignmentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterLoadAssignment {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClusterLoadAssignment {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterLoadAssignmentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterLoadAssignmentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClusterLoadAssignmentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterLoadAssignment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterLoadAssignmentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClusterLoadAssignmentMut<'msg> {
  type Message = ClusterLoadAssignment;
}

impl ::std::fmt::Debug for ClusterLoadAssignmentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterLoadAssignment>> for ClusterLoadAssignmentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterLoadAssignment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterLoadAssignmentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterLoadAssignment> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClusterLoadAssignment {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // cluster_name: optional string
  pub fn cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // endpoints: repeated message envoy.config.endpoint.v3.LocalityLbEndpoints
  pub fn endpoints(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn endpoints_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints> {
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
  pub fn set_endpoints(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // named_endpoints: repeated message envoy.config.endpoint.v3.ClusterLoadAssignment.NamedEndpointsEntry
  pub fn named_endpoints(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn named_endpoints_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_named_endpoints(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // policy: optional message envoy.config.endpoint.v3.ClusterLoadAssignment.Policy
  pub fn has_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn policy_opt(&self) -> ::std::option::Option<super::cluster_load_assignment::PolicyView<'_>> {
    self.has_policy().then(|| self.policy())
  }
  pub fn policy(&self) -> super::cluster_load_assignment::PolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster_load_assignment::PolicyView::default())
  }
  pub fn policy_mut(&mut self) -> super::cluster_load_assignment::PolicyMut<'_> {
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
  pub fn set_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster_load_assignment::Policy>) {

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
// - `ClusterLoadAssignmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClusterLoadAssignmentMut<'_> {}

// SAFETY:
// - `ClusterLoadAssignmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClusterLoadAssignmentMut<'_> {}

impl<'msg> ::protobuf::AsView for ClusterLoadAssignmentMut<'msg> {
  type Proxied = ClusterLoadAssignment;
  fn as_view(&self) -> ::protobuf::View<'_, ClusterLoadAssignment> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterLoadAssignmentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClusterLoadAssignment>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClusterLoadAssignmentMut<'msg> {
  type MutProxied = ClusterLoadAssignment;
  fn as_mut(&mut self) -> ClusterLoadAssignmentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClusterLoadAssignmentMut<'msg> {
  fn into_mut<'shorter>(self) -> ClusterLoadAssignmentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClusterLoadAssignment {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClusterLoadAssignment> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClusterLoadAssignmentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClusterLoadAssignmentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // cluster_name: optional string
  pub fn cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // endpoints: repeated message envoy.config.endpoint.v3.LocalityLbEndpoints
  pub fn endpoints(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn endpoints_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints> {
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
  pub fn set_endpoints(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // named_endpoints: repeated message envoy.config.endpoint.v3.ClusterLoadAssignment.NamedEndpointsEntry
  pub fn named_endpoints(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(3)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn named_endpoints_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          3, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_named_endpoints(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // policy: optional message envoy.config.endpoint.v3.ClusterLoadAssignment.Policy
  pub fn has_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn policy_opt(&self) -> ::std::option::Option<super::cluster_load_assignment::PolicyView<'_>> {
    self.has_policy().then(|| self.policy())
  }
  pub fn policy(&self) -> super::cluster_load_assignment::PolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster_load_assignment::PolicyView::default())
  }
  pub fn policy_mut(&mut self) -> super::cluster_load_assignment::PolicyMut<'_> {
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
  pub fn set_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster_load_assignment::Policy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl ClusterLoadAssignment

impl ::std::ops::Drop for ClusterLoadAssignment {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClusterLoadAssignment {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClusterLoadAssignment {
  type Proxied = Self;
  fn as_view(&self) -> ClusterLoadAssignmentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClusterLoadAssignment {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClusterLoadAssignmentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClusterLoadAssignment {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__endpoint__v3__ClusterLoadAssignment_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XGa3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__endpoint__v3__ClusterLoadAssignment_msg_init.0, &[<crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::LocalityLbEndpoints as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster_load_assignment::Policy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster_load_assignment::NamedEndpointsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__endpoint__v3__ClusterLoadAssignment_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterLoadAssignment {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterLoadAssignment {
  type Msg = ClusterLoadAssignment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterLoadAssignment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterLoadAssignment {
  type Msg = ClusterLoadAssignment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterLoadAssignment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterLoadAssignmentMut<'_> {
  type Msg = ClusterLoadAssignment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterLoadAssignment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterLoadAssignmentMut<'_> {
  type Msg = ClusterLoadAssignment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterLoadAssignment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterLoadAssignmentView<'_> {
  type Msg = ClusterLoadAssignment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterLoadAssignment> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterLoadAssignmentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod cluster_load_assignment {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__ClusterLoadAssignment__Policy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Policy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Policy>
}

impl ::protobuf::Message for Policy {
  type MessageView<'msg> = PolicyView<'msg>;
  type MessageMut<'msg> = PolicyMut<'msg>;
}

impl ::std::default::Default for Policy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Policy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Policy` is `Sync` because it does not implement interior mutability.
//    Neither does `PolicyMut`.
unsafe impl ::std::marker::Sync for Policy {}

// SAFETY:
// - `Policy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Policy {}

impl ::protobuf::Proxied for Policy {
  type View<'msg> = PolicyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Policy {}

impl ::protobuf::MutProxied for Policy {
  type Mut<'msg> = PolicyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PolicyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Policy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PolicyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PolicyView<'msg> {
  type Message = Policy;
}

impl ::std::fmt::Debug for PolicyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PolicyView<'_> {
  fn default() -> PolicyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Policy>> for PolicyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Policy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PolicyView<'msg> {

  pub fn to_owned(&self) -> Policy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // drop_overloads: repeated message envoy.config.endpoint.v3.ClusterLoadAssignment.Policy.DropOverload
  pub fn drop_overloads(self) -> ::protobuf::RepeatedView<'msg, super::super::cluster_load_assignment::policy::DropOverload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::cluster_load_assignment::policy::DropOverload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // overprovisioning_factor: optional message google.protobuf.UInt32Value
  pub fn has_overprovisioning_factor(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn overprovisioning_factor_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_overprovisioning_factor().then(|| self.overprovisioning_factor())
  }
  pub fn overprovisioning_factor(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // endpoint_stale_after: optional message google.protobuf.Duration
  pub fn has_endpoint_stale_after(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn endpoint_stale_after_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_endpoint_stale_after().then(|| self.endpoint_stale_after())
  }
  pub fn endpoint_stale_after(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // weighted_priority_health: optional bool
  pub fn weighted_priority_health(self) -> bool {
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
// - `PolicyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PolicyView<'_> {}

// SAFETY:
// - `PolicyView` is `Send` because while its alive a `PolicyMut` cannot.
// - `PolicyView` does not use thread-local data.
unsafe impl ::std::marker::Send for PolicyView<'_> {}

impl<'msg> ::protobuf::AsView for PolicyView<'msg> {
  type Proxied = Policy;
  fn as_view(&self) -> ::protobuf::View<'msg, Policy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PolicyView<'msg> {
  fn into_view<'shorter>(self) -> PolicyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Policy> for PolicyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Policy {
    let mut dst = Policy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Policy> for PolicyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Policy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Policy {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PolicyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PolicyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PolicyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Policy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PolicyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PolicyMut<'msg> {
  type Message = Policy;
}

impl ::std::fmt::Debug for PolicyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Policy>> for PolicyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Policy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PolicyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Policy> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Policy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // drop_overloads: repeated message envoy.config.endpoint.v3.ClusterLoadAssignment.Policy.DropOverload
  pub fn drop_overloads(&self) -> ::protobuf::RepeatedView<'_, super::super::cluster_load_assignment::policy::DropOverload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::cluster_load_assignment::policy::DropOverload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn drop_overloads_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::cluster_load_assignment::policy::DropOverload> {
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
  pub fn set_drop_overloads(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::cluster_load_assignment::policy::DropOverload>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // overprovisioning_factor: optional message google.protobuf.UInt32Value
  pub fn has_overprovisioning_factor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_overprovisioning_factor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn overprovisioning_factor_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_overprovisioning_factor().then(|| self.overprovisioning_factor())
  }
  pub fn overprovisioning_factor(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn overprovisioning_factor_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_overprovisioning_factor(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // endpoint_stale_after: optional message google.protobuf.Duration
  pub fn has_endpoint_stale_after(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_endpoint_stale_after(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn endpoint_stale_after_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_endpoint_stale_after().then(|| self.endpoint_stale_after())
  }
  pub fn endpoint_stale_after(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn endpoint_stale_after_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_endpoint_stale_after(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // weighted_priority_health: optional bool
  pub fn weighted_priority_health(&self) -> bool {
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
  pub fn set_weighted_priority_health(&mut self, val: bool) {
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
// - `PolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PolicyMut<'_> {}

// SAFETY:
// - `PolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PolicyMut<'_> {}

impl<'msg> ::protobuf::AsView for PolicyMut<'msg> {
  type Proxied = Policy;
  fn as_view(&self) -> ::protobuf::View<'_, Policy> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PolicyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Policy>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PolicyMut<'msg> {
  type MutProxied = Policy;
  fn as_mut(&mut self) -> PolicyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PolicyMut<'msg> {
  fn into_mut<'shorter>(self) -> PolicyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Policy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Policy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PolicyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PolicyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // drop_overloads: repeated message envoy.config.endpoint.v3.ClusterLoadAssignment.Policy.DropOverload
  pub fn drop_overloads(&self) -> ::protobuf::RepeatedView<'_, super::super::cluster_load_assignment::policy::DropOverload> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::cluster_load_assignment::policy::DropOverload>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn drop_overloads_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::cluster_load_assignment::policy::DropOverload> {
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
  pub fn set_drop_overloads(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::cluster_load_assignment::policy::DropOverload>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // overprovisioning_factor: optional message google.protobuf.UInt32Value
  pub fn has_overprovisioning_factor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_overprovisioning_factor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn overprovisioning_factor_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_overprovisioning_factor().then(|| self.overprovisioning_factor())
  }
  pub fn overprovisioning_factor(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn overprovisioning_factor_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_overprovisioning_factor(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // endpoint_stale_after: optional message google.protobuf.Duration
  pub fn has_endpoint_stale_after(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_endpoint_stale_after(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn endpoint_stale_after_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_endpoint_stale_after().then(|| self.endpoint_stale_after())
  }
  pub fn endpoint_stale_after(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn endpoint_stale_after_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_endpoint_stale_after(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // weighted_priority_health: optional bool
  pub fn weighted_priority_health(&self) -> bool {
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
  pub fn set_weighted_priority_health(&mut self, val: bool) {
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

}  // impl Policy

impl ::std::ops::Drop for Policy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Policy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Policy {
  type Proxied = Self;
  fn as_view(&self) -> PolicyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Policy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PolicyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Policy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster_load_assignment::envoy__config__endpoint__v3__ClusterLoadAssignment__Policy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$aG33a/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster_load_assignment::envoy__config__endpoint__v3__ClusterLoadAssignment__Policy_msg_init.0, &[<super::super::cluster_load_assignment::policy::DropOverload as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster_load_assignment::envoy__config__endpoint__v3__ClusterLoadAssignment__Policy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Policy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Policy {
  type Msg = Policy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Policy {
  type Msg = Policy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PolicyMut<'_> {
  type Msg = Policy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PolicyMut<'_> {
  type Msg = Policy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PolicyView<'_> {
  type Msg = Policy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Policy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PolicyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod policy {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__ClusterLoadAssignment__Policy__DropOverload_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DropOverload {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DropOverload>
}

impl ::protobuf::Message for DropOverload {
  type MessageView<'msg> = DropOverloadView<'msg>;
  type MessageMut<'msg> = DropOverloadMut<'msg>;
}

impl ::std::default::Default for DropOverload {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DropOverload {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DropOverload` is `Sync` because it does not implement interior mutability.
//    Neither does `DropOverloadMut`.
unsafe impl ::std::marker::Sync for DropOverload {}

// SAFETY:
// - `DropOverload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DropOverload {}

impl ::protobuf::Proxied for DropOverload {
  type View<'msg> = DropOverloadView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DropOverload {}

impl ::protobuf::MutProxied for DropOverload {
  type Mut<'msg> = DropOverloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DropOverloadView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DropOverload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DropOverloadView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DropOverloadView<'msg> {
  type Message = DropOverload;
}

impl ::std::fmt::Debug for DropOverloadView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DropOverloadView<'_> {
  fn default() -> DropOverloadView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DropOverload>> for DropOverloadView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DropOverload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DropOverloadView<'msg> {

  pub fn to_owned(&self) -> DropOverload {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // category: optional string
  pub fn category(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // drop_percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_drop_percentage(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn drop_percentage_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg>> {
    self.has_drop_percentage().then(|| self.drop_percentage())
  }
  pub fn drop_percentage(self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }

}

// SAFETY:
// - `DropOverloadView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DropOverloadView<'_> {}

// SAFETY:
// - `DropOverloadView` is `Send` because while its alive a `DropOverloadMut` cannot.
// - `DropOverloadView` does not use thread-local data.
unsafe impl ::std::marker::Send for DropOverloadView<'_> {}

impl<'msg> ::protobuf::AsView for DropOverloadView<'msg> {
  type Proxied = DropOverload;
  fn as_view(&self) -> ::protobuf::View<'msg, DropOverload> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DropOverloadView<'msg> {
  fn into_view<'shorter>(self) -> DropOverloadView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DropOverload> for DropOverloadView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DropOverload {
    let mut dst = DropOverload::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DropOverload> for DropOverloadMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DropOverload {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DropOverload {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DropOverloadView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DropOverloadMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DropOverloadMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DropOverload>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DropOverloadMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DropOverloadMut<'msg> {
  type Message = DropOverload;
}

impl ::std::fmt::Debug for DropOverloadMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DropOverload>> for DropOverloadMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DropOverload>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DropOverloadMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DropOverload> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DropOverload {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // category: optional string
  pub fn category(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_category(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // drop_percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_drop_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_drop_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn drop_percentage_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_drop_percentage().then(|| self.drop_percentage())
  }
  pub fn drop_percentage(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn drop_percentage_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_drop_percentage(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

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
// - `DropOverloadMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DropOverloadMut<'_> {}

// SAFETY:
// - `DropOverloadMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DropOverloadMut<'_> {}

impl<'msg> ::protobuf::AsView for DropOverloadMut<'msg> {
  type Proxied = DropOverload;
  fn as_view(&self) -> ::protobuf::View<'_, DropOverload> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DropOverloadMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DropOverload>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DropOverloadMut<'msg> {
  type MutProxied = DropOverload;
  fn as_mut(&mut self) -> DropOverloadMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DropOverloadMut<'msg> {
  fn into_mut<'shorter>(self) -> DropOverloadMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DropOverload {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DropOverload> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DropOverloadView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DropOverloadMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // category: optional string
  pub fn category(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_category(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // drop_percentage: optional message envoy.type.v3.FractionalPercent
  pub fn has_drop_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_drop_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn drop_percentage_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_>> {
    self.has_drop_percentage().then(|| self.drop_percentage())
  }
  pub fn drop_percentage(&self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentView::default())
  }
  pub fn drop_percentage_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::FractionalPercentMut<'_> {
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
  pub fn set_drop_percentage(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl DropOverload

impl ::std::ops::Drop for DropOverload {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DropOverload {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DropOverload {
  type Proxied = Self;
  fn as_view(&self) -> DropOverloadView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DropOverload {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DropOverloadMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DropOverload {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::cluster_load_assignment::policy::envoy__config__endpoint__v3__ClusterLoadAssignment__Policy__DropOverload_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::cluster_load_assignment::policy::envoy__config__endpoint__v3__ClusterLoadAssignment__Policy__DropOverload_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::FractionalPercent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::cluster_load_assignment::policy::envoy__config__endpoint__v3__ClusterLoadAssignment__Policy__DropOverload_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DropOverload {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DropOverload {
  type Msg = DropOverload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DropOverload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DropOverload {
  type Msg = DropOverload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DropOverload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DropOverloadMut<'_> {
  type Msg = DropOverload;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DropOverload> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DropOverloadMut<'_> {
  type Msg = DropOverload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DropOverload> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DropOverloadView<'_> {
  type Msg = DropOverload;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DropOverload> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DropOverloadMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod policy

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__endpoint__v3__ClusterLoadAssignment__NamedEndpointsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct NamedEndpointsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NamedEndpointsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster_load_assignment::envoy__config__endpoint__v3__ClusterLoadAssignment__NamedEndpointsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster_load_assignment::envoy__config__endpoint__v3__ClusterLoadAssignment__NamedEndpointsEntry_msg_init.0, &[<crate::xds::generated::envoy::config::endpoint::v3::endpoint_components::Endpoint as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster_load_assignment::envoy__config__endpoint__v3__ClusterLoadAssignment__NamedEndpointsEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod cluster_load_assignment


