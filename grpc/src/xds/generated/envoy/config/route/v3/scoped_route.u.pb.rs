const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__route__v3__ScopedRouteConfiguration_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScopedRouteConfiguration {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScopedRouteConfiguration>
}

impl ::protobuf::Message for ScopedRouteConfiguration {
  type MessageView<'msg> = ScopedRouteConfigurationView<'msg>;
  type MessageMut<'msg> = ScopedRouteConfigurationMut<'msg>;
}

impl ::std::default::Default for ScopedRouteConfiguration {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScopedRouteConfiguration {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScopedRouteConfiguration` is `Sync` because it does not implement interior mutability.
//    Neither does `ScopedRouteConfigurationMut`.
unsafe impl ::std::marker::Sync for ScopedRouteConfiguration {}

// SAFETY:
// - `ScopedRouteConfiguration` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRouteConfiguration {}

impl ::protobuf::Proxied for ScopedRouteConfiguration {
  type View<'msg> = ScopedRouteConfigurationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScopedRouteConfiguration {}

impl ::protobuf::MutProxied for ScopedRouteConfiguration {
  type Mut<'msg> = ScopedRouteConfigurationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScopedRouteConfigurationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRouteConfiguration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRouteConfigurationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScopedRouteConfigurationView<'msg> {
  type Message = ScopedRouteConfiguration;
}

impl ::std::fmt::Debug for ScopedRouteConfigurationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScopedRouteConfigurationView<'_> {
  fn default() -> ScopedRouteConfigurationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRouteConfiguration>> for ScopedRouteConfigurationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScopedRouteConfiguration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRouteConfigurationView<'msg> {

  pub fn to_owned(&self) -> ScopedRouteConfiguration {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // on_demand: optional bool
  pub fn on_demand(self) -> bool {
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

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // route_configuration_name: optional string
  pub fn route_configuration_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // route_configuration: optional message envoy.config.route.v3.RouteConfiguration
  pub fn has_route_configuration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn route_configuration_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'msg>> {
    self.has_route_configuration().then(|| self.route_configuration())
  }
  pub fn route_configuration(self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView::default())
  }

  // key: optional message envoy.config.route.v3.ScopedRouteConfiguration.Key
  pub fn has_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn key_opt(self) -> ::std::option::Option<super::scoped_route_configuration::KeyView<'msg>> {
    self.has_key().then(|| self.key())
  }
  pub fn key(self) -> super::scoped_route_configuration::KeyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::scoped_route_configuration::KeyView::default())
  }

}

// SAFETY:
// - `ScopedRouteConfigurationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScopedRouteConfigurationView<'_> {}

// SAFETY:
// - `ScopedRouteConfigurationView` is `Send` because while its alive a `ScopedRouteConfigurationMut` cannot.
// - `ScopedRouteConfigurationView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScopedRouteConfigurationView<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRouteConfigurationView<'msg> {
  type Proxied = ScopedRouteConfiguration;
  fn as_view(&self) -> ::protobuf::View<'msg, ScopedRouteConfiguration> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRouteConfigurationView<'msg> {
  fn into_view<'shorter>(self) -> ScopedRouteConfigurationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRouteConfiguration> for ScopedRouteConfigurationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRouteConfiguration {
    let mut dst = ScopedRouteConfiguration::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScopedRouteConfiguration> for ScopedRouteConfigurationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScopedRouteConfiguration {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScopedRouteConfiguration {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRouteConfigurationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScopedRouteConfigurationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScopedRouteConfigurationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRouteConfiguration>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScopedRouteConfigurationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScopedRouteConfigurationMut<'msg> {
  type Message = ScopedRouteConfiguration;
}

impl ::std::fmt::Debug for ScopedRouteConfigurationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRouteConfiguration>> for ScopedRouteConfigurationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRouteConfiguration>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScopedRouteConfigurationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScopedRouteConfiguration> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScopedRouteConfiguration {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // on_demand: optional bool
  pub fn on_demand(&self) -> bool {
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
  pub fn set_on_demand(&mut self, val: bool) {
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

  // route_configuration_name: optional string
  pub fn route_configuration_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_configuration_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // route_configuration: optional message envoy.config.route.v3.RouteConfiguration
  pub fn has_route_configuration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_route_configuration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn route_configuration_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'_>> {
    self.has_route_configuration().then(|| self.route_configuration())
  }
  pub fn route_configuration(&self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView::default())
  }
  pub fn route_configuration_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationMut<'_> {
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
  pub fn set_route_configuration(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route::RouteConfiguration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // key: optional message envoy.config.route.v3.ScopedRouteConfiguration.Key
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn key_opt(&self) -> ::std::option::Option<super::scoped_route_configuration::KeyView<'_>> {
    self.has_key().then(|| self.key())
  }
  pub fn key(&self) -> super::scoped_route_configuration::KeyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::scoped_route_configuration::KeyView::default())
  }
  pub fn key_mut(&mut self) -> super::scoped_route_configuration::KeyMut<'_> {
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
  pub fn set_key(&mut self,
    val: impl ::protobuf::IntoProxied<super::scoped_route_configuration::Key>) {

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
// - `ScopedRouteConfigurationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScopedRouteConfigurationMut<'_> {}

// SAFETY:
// - `ScopedRouteConfigurationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScopedRouteConfigurationMut<'_> {}

impl<'msg> ::protobuf::AsView for ScopedRouteConfigurationMut<'msg> {
  type Proxied = ScopedRouteConfiguration;
  fn as_view(&self) -> ::protobuf::View<'_, ScopedRouteConfiguration> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScopedRouteConfigurationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScopedRouteConfiguration>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScopedRouteConfigurationMut<'msg> {
  type MutProxied = ScopedRouteConfiguration;
  fn as_mut(&mut self) -> ScopedRouteConfigurationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScopedRouteConfigurationMut<'msg> {
  fn into_mut<'shorter>(self) -> ScopedRouteConfigurationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScopedRouteConfiguration {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScopedRouteConfiguration> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScopedRouteConfigurationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScopedRouteConfigurationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // on_demand: optional bool
  pub fn on_demand(&self) -> bool {
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
  pub fn set_on_demand(&mut self, val: bool) {
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

  // route_configuration_name: optional string
  pub fn route_configuration_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_configuration_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // route_configuration: optional message envoy.config.route.v3.RouteConfiguration
  pub fn has_route_configuration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_route_configuration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn route_configuration_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'_>> {
    self.has_route_configuration().then(|| self.route_configuration())
  }
  pub fn route_configuration(&self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationView::default())
  }
  pub fn route_configuration_mut(&mut self) -> crate::xds::generated::envoy::config::route::v3::route::RouteConfigurationMut<'_> {
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
  pub fn set_route_configuration(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::route::v3::route::RouteConfiguration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // key: optional message envoy.config.route.v3.ScopedRouteConfiguration.Key
  pub fn has_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn key_opt(&self) -> ::std::option::Option<super::scoped_route_configuration::KeyView<'_>> {
    self.has_key().then(|| self.key())
  }
  pub fn key(&self) -> super::scoped_route_configuration::KeyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::scoped_route_configuration::KeyView::default())
  }
  pub fn key_mut(&mut self) -> super::scoped_route_configuration::KeyMut<'_> {
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
  pub fn set_key(&mut self,
    val: impl ::protobuf::IntoProxied<super::scoped_route_configuration::Key>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl ScopedRouteConfiguration

impl ::std::ops::Drop for ScopedRouteConfiguration {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScopedRouteConfiguration {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScopedRouteConfiguration {
  type Proxied = Self;
  fn as_view(&self) -> ScopedRouteConfigurationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScopedRouteConfiguration {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScopedRouteConfigurationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScopedRouteConfiguration {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__route__v3__ScopedRouteConfiguration_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X3/P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__route__v3__ScopedRouteConfiguration_msg_init.0, &[<super::scoped_route_configuration::Key as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::route::v3::route::RouteConfiguration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__route__v3__ScopedRouteConfiguration_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRouteConfiguration {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRouteConfiguration {
  type Msg = ScopedRouteConfiguration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfiguration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRouteConfiguration {
  type Msg = ScopedRouteConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfiguration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScopedRouteConfigurationMut<'_> {
  type Msg = ScopedRouteConfiguration;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfiguration> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRouteConfigurationMut<'_> {
  type Msg = ScopedRouteConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfiguration> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScopedRouteConfigurationView<'_> {
  type Msg = ScopedRouteConfiguration;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScopedRouteConfiguration> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScopedRouteConfigurationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod scoped_route_configuration {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__route__v3__ScopedRouteConfiguration__Key_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Key {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Key>
}

impl ::protobuf::Message for Key {
  type MessageView<'msg> = KeyView<'msg>;
  type MessageMut<'msg> = KeyMut<'msg>;
}

impl ::std::default::Default for Key {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Key {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Key` is `Sync` because it does not implement interior mutability.
//    Neither does `KeyMut`.
unsafe impl ::std::marker::Sync for Key {}

// SAFETY:
// - `Key` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Key {}

impl ::protobuf::Proxied for Key {
  type View<'msg> = KeyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Key {}

impl ::protobuf::MutProxied for Key {
  type Mut<'msg> = KeyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct KeyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Key>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for KeyView<'msg> {
  type Message = Key;
}

impl ::std::fmt::Debug for KeyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for KeyView<'_> {
  fn default() -> KeyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Key>> for KeyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Key>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyView<'msg> {

  pub fn to_owned(&self) -> Key {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // fragments: repeated message envoy.config.route.v3.ScopedRouteConfiguration.Key.Fragment
  pub fn fragments(self) -> ::protobuf::RepeatedView<'msg, super::super::scoped_route_configuration::key::Fragment> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::scoped_route_configuration::key::Fragment>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `KeyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for KeyView<'_> {}

// SAFETY:
// - `KeyView` is `Send` because while its alive a `KeyMut` cannot.
// - `KeyView` does not use thread-local data.
unsafe impl ::std::marker::Send for KeyView<'_> {}

impl<'msg> ::protobuf::AsView for KeyView<'msg> {
  type Proxied = Key;
  fn as_view(&self) -> ::protobuf::View<'msg, Key> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyView<'msg> {
  fn into_view<'shorter>(self) -> KeyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Key> for KeyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Key {
    let mut dst = Key::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Key> for KeyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Key {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Key {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for KeyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct KeyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Key>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for KeyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for KeyMut<'msg> {
  type Message = Key;
}

impl ::std::fmt::Debug for KeyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Key>> for KeyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Key>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> KeyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Key> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Key {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // fragments: repeated message envoy.config.route.v3.ScopedRouteConfiguration.Key.Fragment
  pub fn fragments(&self) -> ::protobuf::RepeatedView<'_, super::super::scoped_route_configuration::key::Fragment> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::scoped_route_configuration::key::Fragment>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fragments_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::scoped_route_configuration::key::Fragment> {
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
  pub fn set_fragments(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::scoped_route_configuration::key::Fragment>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `KeyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for KeyMut<'_> {}

// SAFETY:
// - `KeyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for KeyMut<'_> {}

impl<'msg> ::protobuf::AsView for KeyMut<'msg> {
  type Proxied = Key;
  fn as_view(&self) -> ::protobuf::View<'_, Key> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for KeyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Key>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for KeyMut<'msg> {
  type MutProxied = Key;
  fn as_mut(&mut self) -> KeyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for KeyMut<'msg> {
  fn into_mut<'shorter>(self) -> KeyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Key {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Key> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> KeyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> KeyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // fragments: repeated message envoy.config.route.v3.ScopedRouteConfiguration.Key.Fragment
  pub fn fragments(&self) -> ::protobuf::RepeatedView<'_, super::super::scoped_route_configuration::key::Fragment> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::scoped_route_configuration::key::Fragment>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fragments_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::scoped_route_configuration::key::Fragment> {
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
  pub fn set_fragments(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::scoped_route_configuration::key::Fragment>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Key

impl ::std::ops::Drop for Key {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Key {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Key {
  type Proxied = Self;
  fn as_view(&self) -> KeyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Key {
  type MutProxied = Self;
  fn as_mut(&mut self) -> KeyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Key {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::scoped_route_configuration::envoy__config__route__v3__ScopedRouteConfiguration__Key_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::scoped_route_configuration::envoy__config__route__v3__ScopedRouteConfiguration__Key_msg_init.0, &[<super::super::scoped_route_configuration::key::Fragment as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::scoped_route_configuration::envoy__config__route__v3__ScopedRouteConfiguration__Key_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Key {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Key {
  type Msg = Key;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Key> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Key {
  type Msg = Key;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Key> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for KeyMut<'_> {
  type Msg = Key;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Key> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyMut<'_> {
  type Msg = Key;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Key> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for KeyView<'_> {
  type Msg = Key;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Key> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for KeyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod key {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__route__v3__ScopedRouteConfiguration__Key__Fragment_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Fragment {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Fragment>
}

impl ::protobuf::Message for Fragment {
  type MessageView<'msg> = FragmentView<'msg>;
  type MessageMut<'msg> = FragmentMut<'msg>;
}

impl ::std::default::Default for Fragment {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Fragment {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Fragment` is `Sync` because it does not implement interior mutability.
//    Neither does `FragmentMut`.
unsafe impl ::std::marker::Sync for Fragment {}

// SAFETY:
// - `Fragment` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Fragment {}

impl ::protobuf::Proxied for Fragment {
  type View<'msg> = FragmentView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Fragment {}

impl ::protobuf::MutProxied for Fragment {
  type Mut<'msg> = FragmentMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FragmentView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Fragment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FragmentView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FragmentView<'msg> {
  type Message = Fragment;
}

impl ::std::fmt::Debug for FragmentView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FragmentView<'_> {
  fn default() -> FragmentView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Fragment>> for FragmentView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Fragment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FragmentView<'msg> {

  pub fn to_owned(&self) -> Fragment {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // string_key: optional string
  pub fn has_string_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn string_key_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_string_key().then(|| self.string_key())
  }
  pub fn string_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn r#type(self) -> super::super::super::scoped_route_configuration::key::fragment::TypeOneof<'msg> {
    match self.r#type_case() {
      super::super::super::scoped_route_configuration::key::fragment::TypeCase::StringKey =>
          super::super::super::scoped_route_configuration::key::fragment::TypeOneof::StringKey(self.string_key()),
      _ => super::super::super::scoped_route_configuration::key::fragment::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(self) -> super::super::super::scoped_route_configuration::key::fragment::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::scoped_route_configuration::key::fragment::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FragmentView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FragmentView<'_> {}

// SAFETY:
// - `FragmentView` is `Send` because while its alive a `FragmentMut` cannot.
// - `FragmentView` does not use thread-local data.
unsafe impl ::std::marker::Send for FragmentView<'_> {}

impl<'msg> ::protobuf::AsView for FragmentView<'msg> {
  type Proxied = Fragment;
  fn as_view(&self) -> ::protobuf::View<'msg, Fragment> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FragmentView<'msg> {
  fn into_view<'shorter>(self) -> FragmentView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Fragment> for FragmentView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Fragment {
    let mut dst = Fragment::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Fragment> for FragmentMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Fragment {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Fragment {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FragmentView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FragmentMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FragmentMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Fragment>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FragmentMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FragmentMut<'msg> {
  type Message = Fragment;
}

impl ::std::fmt::Debug for FragmentMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Fragment>> for FragmentMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Fragment>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FragmentMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Fragment> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Fragment {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // string_key: optional string
  pub fn has_string_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_string_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn string_key_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_key().then(|| self.string_key())
  }
  pub fn string_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  pub fn r#type(&self) -> super::super::super::scoped_route_configuration::key::fragment::TypeOneof<'_> {
    match &self.r#type_case() {
      super::super::super::scoped_route_configuration::key::fragment::TypeCase::StringKey =>
          super::super::super::scoped_route_configuration::key::fragment::TypeOneof::StringKey(self.string_key()),
      _ => super::super::super::scoped_route_configuration::key::fragment::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::super::super::scoped_route_configuration::key::fragment::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::scoped_route_configuration::key::fragment::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `FragmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FragmentMut<'_> {}

// SAFETY:
// - `FragmentMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FragmentMut<'_> {}

impl<'msg> ::protobuf::AsView for FragmentMut<'msg> {
  type Proxied = Fragment;
  fn as_view(&self) -> ::protobuf::View<'_, Fragment> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FragmentMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Fragment>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FragmentMut<'msg> {
  type MutProxied = Fragment;
  fn as_mut(&mut self) -> FragmentMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FragmentMut<'msg> {
  fn into_mut<'shorter>(self) -> FragmentMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Fragment {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Fragment> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FragmentView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FragmentMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // string_key: optional string
  pub fn has_string_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_string_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn string_key_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_key().then(|| self.string_key())
  }
  pub fn string_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  pub fn r#type(&self) -> super::super::super::scoped_route_configuration::key::fragment::TypeOneof<'_> {
    match &self.r#type_case() {
      super::super::super::scoped_route_configuration::key::fragment::TypeCase::StringKey =>
          super::super::super::scoped_route_configuration::key::fragment::TypeOneof::StringKey(self.string_key()),
      _ => super::super::super::scoped_route_configuration::key::fragment::TypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn r#type_case(&self) -> super::super::super::scoped_route_configuration::key::fragment::TypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::scoped_route_configuration::key::fragment::TypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Fragment

impl ::std::ops::Drop for Fragment {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Fragment {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Fragment {
  type Proxied = Self;
  fn as_view(&self) -> FragmentView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Fragment {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FragmentMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Fragment {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::scoped_route_configuration::key::envoy__config__route__v3__ScopedRouteConfiguration__Key__Fragment_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$M1^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::scoped_route_configuration::key::envoy__config__route__v3__ScopedRouteConfiguration__Key__Fragment_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::scoped_route_configuration::key::envoy__config__route__v3__ScopedRouteConfiguration__Key__Fragment_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Fragment {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Fragment {
  type Msg = Fragment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fragment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Fragment {
  type Msg = Fragment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fragment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FragmentMut<'_> {
  type Msg = Fragment;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fragment> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FragmentMut<'_> {
  type Msg = Fragment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fragment> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FragmentView<'_> {
  type Msg = Fragment;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Fragment> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FragmentMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod fragment {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TypeOneof<'msg> {
  StringKey(&'msg ::protobuf::ProtoStr) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TypeCase {
  StringKey = 1,

  not_set = 0
}

impl TypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TypeCase> {
    match v {
      0 => Some(TypeCase::not_set),
      1 => Some(TypeCase::StringKey),
      _ => None
    }
  }
}
}  // pub mod fragment


}  // pub mod key


}  // pub mod scoped_route_configuration


