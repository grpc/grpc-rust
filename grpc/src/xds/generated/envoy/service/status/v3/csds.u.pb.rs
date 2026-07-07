const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__status__v3__ClientStatusRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClientStatusRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClientStatusRequest>
}

impl ::protobuf::Message for ClientStatusRequest {
  type MessageView<'msg> = ClientStatusRequestView<'msg>;
  type MessageMut<'msg> = ClientStatusRequestMut<'msg>;
}

impl ::std::default::Default for ClientStatusRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClientStatusRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClientStatusRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ClientStatusRequestMut`.
unsafe impl ::std::marker::Sync for ClientStatusRequest {}

// SAFETY:
// - `ClientStatusRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClientStatusRequest {}

impl ::protobuf::Proxied for ClientStatusRequest {
  type View<'msg> = ClientStatusRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClientStatusRequest {}

impl ::protobuf::MutProxied for ClientStatusRequest {
  type Mut<'msg> = ClientStatusRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClientStatusRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientStatusRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientStatusRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClientStatusRequestView<'msg> {
  type Message = ClientStatusRequest;
}

impl ::std::fmt::Debug for ClientStatusRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClientStatusRequestView<'_> {
  fn default() -> ClientStatusRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClientStatusRequest>> for ClientStatusRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientStatusRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientStatusRequestView<'msg> {

  pub fn to_owned(&self) -> ClientStatusRequest {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // node_matchers: repeated message envoy.type.matcher.v3.NodeMatcher
  pub fn node_matchers(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn node_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }

  // exclude_resource_contents: optional bool
  pub fn exclude_resource_contents(self) -> bool {
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

}

// SAFETY:
// - `ClientStatusRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClientStatusRequestView<'_> {}

// SAFETY:
// - `ClientStatusRequestView` is `Send` because while its alive a `ClientStatusRequestMut` cannot.
// - `ClientStatusRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClientStatusRequestView<'_> {}

impl<'msg> ::protobuf::AsView for ClientStatusRequestView<'msg> {
  type Proxied = ClientStatusRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, ClientStatusRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientStatusRequestView<'msg> {
  fn into_view<'shorter>(self) -> ClientStatusRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientStatusRequest> for ClientStatusRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientStatusRequest {
    let mut dst = ClientStatusRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientStatusRequest> for ClientStatusRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientStatusRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClientStatusRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClientStatusRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClientStatusRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClientStatusRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientStatusRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientStatusRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClientStatusRequestMut<'msg> {
  type Message = ClientStatusRequest;
}

impl ::std::fmt::Debug for ClientStatusRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClientStatusRequest>> for ClientStatusRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientStatusRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientStatusRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientStatusRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClientStatusRequest {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // node_matchers: repeated message envoy.type.matcher.v3.NodeMatcher
  pub fn node_matchers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn node_matchers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher> {
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
  pub fn set_node_matchers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
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
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // exclude_resource_contents: optional bool
  pub fn exclude_resource_contents(&self) -> bool {
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
  pub fn set_exclude_resource_contents(&mut self, val: bool) {
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

}

// SAFETY:
// - `ClientStatusRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClientStatusRequestMut<'_> {}

// SAFETY:
// - `ClientStatusRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClientStatusRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for ClientStatusRequestMut<'msg> {
  type Proxied = ClientStatusRequest;
  fn as_view(&self) -> ::protobuf::View<'_, ClientStatusRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientStatusRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClientStatusRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClientStatusRequestMut<'msg> {
  type MutProxied = ClientStatusRequest;
  fn as_mut(&mut self) -> ClientStatusRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClientStatusRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> ClientStatusRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClientStatusRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClientStatusRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClientStatusRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClientStatusRequestMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // node_matchers: repeated message envoy.type.matcher.v3.NodeMatcher
  pub fn node_matchers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn node_matchers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher> {
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
  pub fn set_node_matchers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
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
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // exclude_resource_contents: optional bool
  pub fn exclude_resource_contents(&self) -> bool {
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
  pub fn set_exclude_resource_contents(&mut self, val: bool) {
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

}  // impl ClientStatusRequest

impl ::std::ops::Drop for ClientStatusRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClientStatusRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClientStatusRequest {
  type Proxied = Self;
  fn as_view(&self) -> ClientStatusRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClientStatusRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClientStatusRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClientStatusRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__status__v3__ClientStatusRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G3/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__status__v3__ClientStatusRequest_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::node::NodeMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Node as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__status__v3__ClientStatusRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientStatusRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientStatusRequest {
  type Msg = ClientStatusRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientStatusRequest {
  type Msg = ClientStatusRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientStatusRequestMut<'_> {
  type Msg = ClientStatusRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientStatusRequestMut<'_> {
  type Msg = ClientStatusRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientStatusRequestView<'_> {
  type Msg = ClientStatusRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientStatusRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__status__v3__PerXdsConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PerXdsConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PerXdsConfig>
}

impl ::protobuf::Message for PerXdsConfig {
  type MessageView<'msg> = PerXdsConfigView<'msg>;
  type MessageMut<'msg> = PerXdsConfigMut<'msg>;
}

impl ::std::default::Default for PerXdsConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PerXdsConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PerXdsConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `PerXdsConfigMut`.
unsafe impl ::std::marker::Sync for PerXdsConfig {}

// SAFETY:
// - `PerXdsConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PerXdsConfig {}

impl ::protobuf::Proxied for PerXdsConfig {
  type View<'msg> = PerXdsConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PerXdsConfig {}

impl ::protobuf::MutProxied for PerXdsConfig {
  type Mut<'msg> = PerXdsConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PerXdsConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PerXdsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PerXdsConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PerXdsConfigView<'msg> {
  type Message = PerXdsConfig;
}

impl ::std::fmt::Debug for PerXdsConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PerXdsConfigView<'_> {
  fn default() -> PerXdsConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PerXdsConfig>> for PerXdsConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PerXdsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PerXdsConfigView<'msg> {

  pub fn to_owned(&self) -> PerXdsConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // status: optional enum envoy.service.status.v3.ConfigStatus
  pub fn status(self) -> super::ConfigStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ConfigStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

  // client_status: optional enum envoy.service.status.v3.ClientConfigStatus
  pub fn client_status(self) -> super::ClientConfigStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        6, (super::ClientConfigStatus::ClientUnknown).into()
      ).try_into().unwrap()
    }
  }

  // listener_config: optional message envoy.admin.v3.ListenersConfigDump
  pub fn has_listener_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn listener_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpView<'msg>> {
    self.has_listener_config().then(|| self.listener_config())
  }
  pub fn listener_config(self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpView::default())
  }

  // cluster_config: optional message envoy.admin.v3.ClustersConfigDump
  pub fn has_cluster_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn cluster_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpView<'msg>> {
    self.has_cluster_config().then(|| self.cluster_config())
  }
  pub fn cluster_config(self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpView::default())
  }

  // route_config: optional message envoy.admin.v3.RoutesConfigDump
  pub fn has_route_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn route_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpView<'msg>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpView::default())
  }

  // scoped_route_config: optional message envoy.admin.v3.ScopedRoutesConfigDump
  pub fn has_scoped_route_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn scoped_route_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpView<'msg>> {
    self.has_scoped_route_config().then(|| self.scoped_route_config())
  }
  pub fn scoped_route_config(self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpView::default())
  }

  // endpoint_config: optional message envoy.admin.v3.EndpointsConfigDump
  pub fn has_endpoint_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn endpoint_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpView<'msg>> {
    self.has_endpoint_config().then(|| self.endpoint_config())
  }
  pub fn endpoint_config(self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpView::default())
  }

  pub fn per_xds_config(self) -> super::per_xds_config::PerXdsConfigOneof<'msg> {
    match self.per_xds_config_case() {
      super::per_xds_config::PerXdsConfigCase::ListenerConfig =>
          super::per_xds_config::PerXdsConfigOneof::ListenerConfig(self.listener_config()),
      super::per_xds_config::PerXdsConfigCase::ClusterConfig =>
          super::per_xds_config::PerXdsConfigOneof::ClusterConfig(self.cluster_config()),
      super::per_xds_config::PerXdsConfigCase::RouteConfig =>
          super::per_xds_config::PerXdsConfigOneof::RouteConfig(self.route_config()),
      super::per_xds_config::PerXdsConfigCase::ScopedRouteConfig =>
          super::per_xds_config::PerXdsConfigOneof::ScopedRouteConfig(self.scoped_route_config()),
      super::per_xds_config::PerXdsConfigCase::EndpointConfig =>
          super::per_xds_config::PerXdsConfigOneof::EndpointConfig(self.endpoint_config()),
      _ => super::per_xds_config::PerXdsConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn per_xds_config_case(self) -> super::per_xds_config::PerXdsConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::per_xds_config::PerXdsConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PerXdsConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PerXdsConfigView<'_> {}

// SAFETY:
// - `PerXdsConfigView` is `Send` because while its alive a `PerXdsConfigMut` cannot.
// - `PerXdsConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for PerXdsConfigView<'_> {}

impl<'msg> ::protobuf::AsView for PerXdsConfigView<'msg> {
  type Proxied = PerXdsConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, PerXdsConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PerXdsConfigView<'msg> {
  fn into_view<'shorter>(self) -> PerXdsConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PerXdsConfig> for PerXdsConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PerXdsConfig {
    let mut dst = PerXdsConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PerXdsConfig> for PerXdsConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PerXdsConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PerXdsConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PerXdsConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PerXdsConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PerXdsConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PerXdsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PerXdsConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PerXdsConfigMut<'msg> {
  type Message = PerXdsConfig;
}

impl ::std::fmt::Debug for PerXdsConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PerXdsConfig>> for PerXdsConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PerXdsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PerXdsConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PerXdsConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PerXdsConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // status: optional enum envoy.service.status.v3.ConfigStatus
  pub fn status(&self) -> super::ConfigStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ConfigStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: super::ConfigStatus) {
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

  // client_status: optional enum envoy.service.status.v3.ClientConfigStatus
  pub fn client_status(&self) -> super::ClientConfigStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        6, (super::ClientConfigStatus::ClientUnknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::ClientConfigStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        6, val.into()
      )
    }
  }

  // listener_config: optional message envoy.admin.v3.ListenersConfigDump
  pub fn has_listener_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_listener_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn listener_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpView<'_>> {
    self.has_listener_config().then(|| self.listener_config())
  }
  pub fn listener_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpView::default())
  }
  pub fn listener_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpMut<'_> {
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
  pub fn set_listener_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cluster_config: optional message envoy.admin.v3.ClustersConfigDump
  pub fn has_cluster_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cluster_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cluster_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpView<'_>> {
    self.has_cluster_config().then(|| self.cluster_config())
  }
  pub fn cluster_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpView::default())
  }
  pub fn cluster_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpMut<'_> {
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
  pub fn set_cluster_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // route_config: optional message envoy.admin.v3.RoutesConfigDump
  pub fn has_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn route_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpView<'_>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpView::default())
  }
  pub fn route_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpMut<'_> {
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
  pub fn set_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // scoped_route_config: optional message envoy.admin.v3.ScopedRoutesConfigDump
  pub fn has_scoped_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_scoped_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn scoped_route_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpView<'_>> {
    self.has_scoped_route_config().then(|| self.scoped_route_config())
  }
  pub fn scoped_route_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpView::default())
  }
  pub fn scoped_route_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpMut<'_> {
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
  pub fn set_scoped_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // endpoint_config: optional message envoy.admin.v3.EndpointsConfigDump
  pub fn has_endpoint_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_endpoint_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn endpoint_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpView<'_>> {
    self.has_endpoint_config().then(|| self.endpoint_config())
  }
  pub fn endpoint_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpView::default())
  }
  pub fn endpoint_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpMut<'_> {
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
  pub fn set_endpoint_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  pub fn per_xds_config(&self) -> super::per_xds_config::PerXdsConfigOneof<'_> {
    match &self.per_xds_config_case() {
      super::per_xds_config::PerXdsConfigCase::ListenerConfig =>
          super::per_xds_config::PerXdsConfigOneof::ListenerConfig(self.listener_config()),
      super::per_xds_config::PerXdsConfigCase::ClusterConfig =>
          super::per_xds_config::PerXdsConfigOneof::ClusterConfig(self.cluster_config()),
      super::per_xds_config::PerXdsConfigCase::RouteConfig =>
          super::per_xds_config::PerXdsConfigOneof::RouteConfig(self.route_config()),
      super::per_xds_config::PerXdsConfigCase::ScopedRouteConfig =>
          super::per_xds_config::PerXdsConfigOneof::ScopedRouteConfig(self.scoped_route_config()),
      super::per_xds_config::PerXdsConfigCase::EndpointConfig =>
          super::per_xds_config::PerXdsConfigOneof::EndpointConfig(self.endpoint_config()),
      _ => super::per_xds_config::PerXdsConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn per_xds_config_case(&self) -> super::per_xds_config::PerXdsConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::per_xds_config::PerXdsConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PerXdsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PerXdsConfigMut<'_> {}

// SAFETY:
// - `PerXdsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PerXdsConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for PerXdsConfigMut<'msg> {
  type Proxied = PerXdsConfig;
  fn as_view(&self) -> ::protobuf::View<'_, PerXdsConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PerXdsConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PerXdsConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PerXdsConfigMut<'msg> {
  type MutProxied = PerXdsConfig;
  fn as_mut(&mut self) -> PerXdsConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PerXdsConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> PerXdsConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PerXdsConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PerXdsConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PerXdsConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PerXdsConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // status: optional enum envoy.service.status.v3.ConfigStatus
  pub fn status(&self) -> super::ConfigStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ConfigStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: super::ConfigStatus) {
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

  // client_status: optional enum envoy.service.status.v3.ClientConfigStatus
  pub fn client_status(&self) -> super::ClientConfigStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        6, (super::ClientConfigStatus::ClientUnknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: super::ClientConfigStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        6, val.into()
      )
    }
  }

  // listener_config: optional message envoy.admin.v3.ListenersConfigDump
  pub fn has_listener_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_listener_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn listener_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpView<'_>> {
    self.has_listener_config().then(|| self.listener_config())
  }
  pub fn listener_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpView::default())
  }
  pub fn listener_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDumpMut<'_> {
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
  pub fn set_listener_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cluster_config: optional message envoy.admin.v3.ClustersConfigDump
  pub fn has_cluster_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cluster_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cluster_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpView<'_>> {
    self.has_cluster_config().then(|| self.cluster_config())
  }
  pub fn cluster_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpView::default())
  }
  pub fn cluster_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDumpMut<'_> {
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
  pub fn set_cluster_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // route_config: optional message envoy.admin.v3.RoutesConfigDump
  pub fn has_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn route_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpView<'_>> {
    self.has_route_config().then(|| self.route_config())
  }
  pub fn route_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpView::default())
  }
  pub fn route_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDumpMut<'_> {
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
  pub fn set_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // scoped_route_config: optional message envoy.admin.v3.ScopedRoutesConfigDump
  pub fn has_scoped_route_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_scoped_route_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn scoped_route_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpView<'_>> {
    self.has_scoped_route_config().then(|| self.scoped_route_config())
  }
  pub fn scoped_route_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpView::default())
  }
  pub fn scoped_route_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDumpMut<'_> {
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
  pub fn set_scoped_route_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // endpoint_config: optional message envoy.admin.v3.EndpointsConfigDump
  pub fn has_endpoint_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_endpoint_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn endpoint_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpView<'_>> {
    self.has_endpoint_config().then(|| self.endpoint_config())
  }
  pub fn endpoint_config(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpView::default())
  }
  pub fn endpoint_config_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDumpMut<'_> {
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
  pub fn set_endpoint_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDump>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  pub fn per_xds_config(&self) -> super::per_xds_config::PerXdsConfigOneof<'_> {
    match &self.per_xds_config_case() {
      super::per_xds_config::PerXdsConfigCase::ListenerConfig =>
          super::per_xds_config::PerXdsConfigOneof::ListenerConfig(self.listener_config()),
      super::per_xds_config::PerXdsConfigCase::ClusterConfig =>
          super::per_xds_config::PerXdsConfigOneof::ClusterConfig(self.cluster_config()),
      super::per_xds_config::PerXdsConfigCase::RouteConfig =>
          super::per_xds_config::PerXdsConfigOneof::RouteConfig(self.route_config()),
      super::per_xds_config::PerXdsConfigCase::ScopedRouteConfig =>
          super::per_xds_config::PerXdsConfigOneof::ScopedRouteConfig(self.scoped_route_config()),
      super::per_xds_config::PerXdsConfigCase::EndpointConfig =>
          super::per_xds_config::PerXdsConfigOneof::EndpointConfig(self.endpoint_config()),
      _ => super::per_xds_config::PerXdsConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn per_xds_config_case(&self) -> super::per_xds_config::PerXdsConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::per_xds_config::PerXdsConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl PerXdsConfig

impl ::std::ops::Drop for PerXdsConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PerXdsConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PerXdsConfig {
  type Proxied = Self;
  fn as_view(&self) -> PerXdsConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PerXdsConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PerXdsConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PerXdsConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__status__v3__PerXdsConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P33333.P^#|$|%|&|(");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__status__v3__PerXdsConfig_msg_init.0, &[<crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDump as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDump as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDump as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDump as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDump as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__status__v3__PerXdsConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PerXdsConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PerXdsConfig {
  type Msg = PerXdsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerXdsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PerXdsConfig {
  type Msg = PerXdsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerXdsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PerXdsConfigMut<'_> {
  type Msg = PerXdsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerXdsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PerXdsConfigMut<'_> {
  type Msg = PerXdsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerXdsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PerXdsConfigView<'_> {
  type Msg = PerXdsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PerXdsConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PerXdsConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod per_xds_config {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum PerXdsConfigOneof<'msg> {
  ListenerConfig(::protobuf::View<'msg, crate::xds::generated::envoy::admin::v3::config_dump_shared::ListenersConfigDump>) = 2,
  ClusterConfig(::protobuf::View<'msg, crate::xds::generated::envoy::admin::v3::config_dump_shared::ClustersConfigDump>) = 3,
  RouteConfig(::protobuf::View<'msg, crate::xds::generated::envoy::admin::v3::config_dump_shared::RoutesConfigDump>) = 4,
  ScopedRouteConfig(::protobuf::View<'msg, crate::xds::generated::envoy::admin::v3::config_dump_shared::ScopedRoutesConfigDump>) = 5,
  EndpointConfig(::protobuf::View<'msg, crate::xds::generated::envoy::admin::v3::config_dump_shared::EndpointsConfigDump>) = 6,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum PerXdsConfigCase {
  ListenerConfig = 2,
  ClusterConfig = 3,
  RouteConfig = 4,
  ScopedRouteConfig = 5,
  EndpointConfig = 6,

  not_set = 0
}

impl PerXdsConfigCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<PerXdsConfigCase> {
    match v {
      0 => Some(PerXdsConfigCase::not_set),
      2 => Some(PerXdsConfigCase::ListenerConfig),
      3 => Some(PerXdsConfigCase::ClusterConfig),
      4 => Some(PerXdsConfigCase::RouteConfig),
      5 => Some(PerXdsConfigCase::ScopedRouteConfig),
      6 => Some(PerXdsConfigCase::EndpointConfig),
      _ => None
    }
  }
}
}  // pub mod per_xds_config


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__status__v3__ClientConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClientConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClientConfig>
}

impl ::protobuf::Message for ClientConfig {
  type MessageView<'msg> = ClientConfigView<'msg>;
  type MessageMut<'msg> = ClientConfigMut<'msg>;
}

impl ::std::default::Default for ClientConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClientConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClientConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ClientConfigMut`.
unsafe impl ::std::marker::Sync for ClientConfig {}

// SAFETY:
// - `ClientConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClientConfig {}

impl ::protobuf::Proxied for ClientConfig {
  type View<'msg> = ClientConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClientConfig {}

impl ::protobuf::MutProxied for ClientConfig {
  type Mut<'msg> = ClientConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClientConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClientConfigView<'msg> {
  type Message = ClientConfig;
}

impl ::std::fmt::Debug for ClientConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClientConfigView<'_> {
  fn default() -> ClientConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClientConfig>> for ClientConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientConfigView<'msg> {

  pub fn to_owned(&self) -> ClientConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn node_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }

  // xds_config: repeated message envoy.service.status.v3.PerXdsConfig
  pub fn xds_config(self) -> ::protobuf::RepeatedView<'msg, super::PerXdsConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PerXdsConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // generic_xds_configs: repeated message envoy.service.status.v3.ClientConfig.GenericXdsConfig
  pub fn generic_xds_configs(self) -> ::protobuf::RepeatedView<'msg, super::client_config::GenericXdsConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::client_config::GenericXdsConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // client_scope: optional string
  pub fn client_scope(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ClientConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClientConfigView<'_> {}

// SAFETY:
// - `ClientConfigView` is `Send` because while its alive a `ClientConfigMut` cannot.
// - `ClientConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClientConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ClientConfigView<'msg> {
  type Proxied = ClientConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ClientConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientConfigView<'msg> {
  fn into_view<'shorter>(self) -> ClientConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientConfig> for ClientConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientConfig {
    let mut dst = ClientConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientConfig> for ClientConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClientConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClientConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClientConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClientConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClientConfigMut<'msg> {
  type Message = ClientConfig;
}

impl ::std::fmt::Debug for ClientConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClientConfig>> for ClientConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClientConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
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
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // xds_config: repeated message envoy.service.status.v3.PerXdsConfig
  pub fn xds_config(&self) -> ::protobuf::RepeatedView<'_, super::PerXdsConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PerXdsConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn xds_config_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::PerXdsConfig> {
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
  pub fn set_xds_config(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::PerXdsConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // generic_xds_configs: repeated message envoy.service.status.v3.ClientConfig.GenericXdsConfig
  pub fn generic_xds_configs(&self) -> ::protobuf::RepeatedView<'_, super::client_config::GenericXdsConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::client_config::GenericXdsConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn generic_xds_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::client_config::GenericXdsConfig> {
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
  pub fn set_generic_xds_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::client_config::GenericXdsConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // client_scope: optional string
  pub fn client_scope(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_client_scope(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `ClientConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClientConfigMut<'_> {}

// SAFETY:
// - `ClientConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClientConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ClientConfigMut<'msg> {
  type Proxied = ClientConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ClientConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClientConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClientConfigMut<'msg> {
  type MutProxied = ClientConfig;
  fn as_mut(&mut self) -> ClientConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClientConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ClientConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClientConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClientConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClientConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClientConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
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
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // xds_config: repeated message envoy.service.status.v3.PerXdsConfig
  pub fn xds_config(&self) -> ::protobuf::RepeatedView<'_, super::PerXdsConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PerXdsConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn xds_config_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::PerXdsConfig> {
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
  pub fn set_xds_config(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::PerXdsConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // generic_xds_configs: repeated message envoy.service.status.v3.ClientConfig.GenericXdsConfig
  pub fn generic_xds_configs(&self) -> ::protobuf::RepeatedView<'_, super::client_config::GenericXdsConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::client_config::GenericXdsConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn generic_xds_configs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::client_config::GenericXdsConfig> {
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
  pub fn set_generic_xds_configs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::client_config::GenericXdsConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // client_scope: optional string
  pub fn client_scope(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_client_scope(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl ClientConfig

impl ::std::ops::Drop for ClientConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClientConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClientConfig {
  type Proxied = Self;
  fn as_view(&self) -> ClientConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClientConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClientConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClientConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__status__v3__ClientConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3GG1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__status__v3__ClientConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::Node as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::PerXdsConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::client_config::GenericXdsConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__status__v3__ClientConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientConfig {
  type Msg = ClientConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientConfig {
  type Msg = ClientConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientConfigMut<'_> {
  type Msg = ClientConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientConfigMut<'_> {
  type Msg = ClientConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientConfigView<'_> {
  type Msg = ClientConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod client_config {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__status__v3__ClientConfig__GenericXdsConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GenericXdsConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GenericXdsConfig>
}

impl ::protobuf::Message for GenericXdsConfig {
  type MessageView<'msg> = GenericXdsConfigView<'msg>;
  type MessageMut<'msg> = GenericXdsConfigMut<'msg>;
}

impl ::std::default::Default for GenericXdsConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GenericXdsConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GenericXdsConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `GenericXdsConfigMut`.
unsafe impl ::std::marker::Sync for GenericXdsConfig {}

// SAFETY:
// - `GenericXdsConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GenericXdsConfig {}

impl ::protobuf::Proxied for GenericXdsConfig {
  type View<'msg> = GenericXdsConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GenericXdsConfig {}

impl ::protobuf::MutProxied for GenericXdsConfig {
  type Mut<'msg> = GenericXdsConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GenericXdsConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GenericXdsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GenericXdsConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GenericXdsConfigView<'msg> {
  type Message = GenericXdsConfig;
}

impl ::std::fmt::Debug for GenericXdsConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GenericXdsConfigView<'_> {
  fn default() -> GenericXdsConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GenericXdsConfig>> for GenericXdsConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GenericXdsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GenericXdsConfigView<'msg> {

  pub fn to_owned(&self) -> GenericXdsConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // type_url: optional string
  pub fn type_url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // version_info: optional string
  pub fn version_info(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // xds_config: optional message google.protobuf.Any
  pub fn has_xds_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn xds_config_opt(self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'msg>> {
    self.has_xds_config().then(|| self.xds_config())
  }
  pub fn xds_config(self) -> ::protobuf_well_known_types::AnyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn last_updated_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // config_status: optional enum envoy.service.status.v3.ConfigStatus
  pub fn config_status(self) -> super::super::ConfigStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::ConfigStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        6, (crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn error_state_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'msg>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView::default())
  }

  // is_static_resource: optional bool
  pub fn is_static_resource(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `GenericXdsConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GenericXdsConfigView<'_> {}

// SAFETY:
// - `GenericXdsConfigView` is `Send` because while its alive a `GenericXdsConfigMut` cannot.
// - `GenericXdsConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for GenericXdsConfigView<'_> {}

impl<'msg> ::protobuf::AsView for GenericXdsConfigView<'msg> {
  type Proxied = GenericXdsConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, GenericXdsConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GenericXdsConfigView<'msg> {
  fn into_view<'shorter>(self) -> GenericXdsConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GenericXdsConfig> for GenericXdsConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GenericXdsConfig {
    let mut dst = GenericXdsConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GenericXdsConfig> for GenericXdsConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GenericXdsConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GenericXdsConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GenericXdsConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GenericXdsConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GenericXdsConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GenericXdsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GenericXdsConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GenericXdsConfigMut<'msg> {
  type Message = GenericXdsConfig;
}

impl ::std::fmt::Debug for GenericXdsConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GenericXdsConfig>> for GenericXdsConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GenericXdsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GenericXdsConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GenericXdsConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GenericXdsConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // xds_config: optional message google.protobuf.Any
  pub fn has_xds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_xds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn xds_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_xds_config().then(|| self.xds_config())
  }
  pub fn xds_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn xds_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_xds_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // config_status: optional enum envoy.service.status.v3.ConfigStatus
  pub fn config_status(&self) -> super::super::ConfigStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::ConfigStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_config_status(&mut self, val: super::super::ConfigStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        6, (crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        6, val.into()
      )
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // is_static_resource: optional bool
  pub fn is_static_resource(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_static_resource(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

}

// SAFETY:
// - `GenericXdsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GenericXdsConfigMut<'_> {}

// SAFETY:
// - `GenericXdsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GenericXdsConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for GenericXdsConfigMut<'msg> {
  type Proxied = GenericXdsConfig;
  fn as_view(&self) -> ::protobuf::View<'_, GenericXdsConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GenericXdsConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GenericXdsConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GenericXdsConfigMut<'msg> {
  type MutProxied = GenericXdsConfig;
  fn as_mut(&mut self) -> GenericXdsConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GenericXdsConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> GenericXdsConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GenericXdsConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GenericXdsConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GenericXdsConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GenericXdsConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // type_url: optional string
  pub fn type_url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_type_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // version_info: optional string
  pub fn version_info(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_version_info(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // xds_config: optional message google.protobuf.Any
  pub fn has_xds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_xds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn xds_config_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::AnyView<'_>> {
    self.has_xds_config().then(|| self.xds_config())
  }
  pub fn xds_config(&self) -> ::protobuf_well_known_types::AnyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::AnyView::default())
  }
  pub fn xds_config_mut(&mut self) -> ::protobuf_well_known_types::AnyMut<'_> {
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
  pub fn set_xds_config(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Any>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // last_updated: optional message google.protobuf.Timestamp
  pub fn has_last_updated(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_last_updated(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn last_updated_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_last_updated().then(|| self.last_updated())
  }
  pub fn last_updated(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn last_updated_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_last_updated(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // config_status: optional enum envoy.service.status.v3.ConfigStatus
  pub fn config_status(&self) -> super::super::ConfigStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::super::ConfigStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_config_status(&mut self, val: super::super::ConfigStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

  // client_status: optional enum envoy.admin.v3.ClientResourceStatus
  pub fn client_status(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        6, (crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_client_status(&mut self, val: crate::xds::generated::envoy::admin::v3::config_dump_shared::ClientResourceStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        6, val.into()
      )
    }
  }

  // error_state: optional message envoy.admin.v3.UpdateFailureState
  pub fn has_error_state(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_error_state(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn error_state_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'_>> {
    self.has_error_state().then(|| self.error_state())
  }
  pub fn error_state(&self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateView::default())
  }
  pub fn error_state_mut(&mut self) -> crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureStateMut<'_> {
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
  pub fn set_error_state(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureState>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // is_static_resource: optional bool
  pub fn is_static_resource(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_static_resource(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

}  // impl GenericXdsConfig

impl ::std::ops::Drop for GenericXdsConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GenericXdsConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GenericXdsConfig {
  type Proxied = Self;
  fn as_view(&self) -> GenericXdsConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GenericXdsConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GenericXdsConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GenericXdsConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::client_config::envoy__service__status__v3__ClientConfig__GenericXdsConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X1X33.P.P3/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::client_config::envoy__service__status__v3__ClientConfig__GenericXdsConfig_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::admin::v3::config_dump_shared::UpdateFailureState as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::client_config::envoy__service__status__v3__ClientConfig__GenericXdsConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GenericXdsConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GenericXdsConfig {
  type Msg = GenericXdsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericXdsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GenericXdsConfig {
  type Msg = GenericXdsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericXdsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GenericXdsConfigMut<'_> {
  type Msg = GenericXdsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericXdsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GenericXdsConfigMut<'_> {
  type Msg = GenericXdsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericXdsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GenericXdsConfigView<'_> {
  type Msg = GenericXdsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericXdsConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GenericXdsConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod client_config


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__status__v3__ClientStatusResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClientStatusResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClientStatusResponse>
}

impl ::protobuf::Message for ClientStatusResponse {
  type MessageView<'msg> = ClientStatusResponseView<'msg>;
  type MessageMut<'msg> = ClientStatusResponseMut<'msg>;
}

impl ::std::default::Default for ClientStatusResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClientStatusResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClientStatusResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ClientStatusResponseMut`.
unsafe impl ::std::marker::Sync for ClientStatusResponse {}

// SAFETY:
// - `ClientStatusResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClientStatusResponse {}

impl ::protobuf::Proxied for ClientStatusResponse {
  type View<'msg> = ClientStatusResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClientStatusResponse {}

impl ::protobuf::MutProxied for ClientStatusResponse {
  type Mut<'msg> = ClientStatusResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClientStatusResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientStatusResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientStatusResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClientStatusResponseView<'msg> {
  type Message = ClientStatusResponse;
}

impl ::std::fmt::Debug for ClientStatusResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClientStatusResponseView<'_> {
  fn default() -> ClientStatusResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClientStatusResponse>> for ClientStatusResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClientStatusResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientStatusResponseView<'msg> {

  pub fn to_owned(&self) -> ClientStatusResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config: repeated message envoy.service.status.v3.ClientConfig
  pub fn config(self) -> ::protobuf::RepeatedView<'msg, super::ClientConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ClientConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ClientStatusResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClientStatusResponseView<'_> {}

// SAFETY:
// - `ClientStatusResponseView` is `Send` because while its alive a `ClientStatusResponseMut` cannot.
// - `ClientStatusResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClientStatusResponseView<'_> {}

impl<'msg> ::protobuf::AsView for ClientStatusResponseView<'msg> {
  type Proxied = ClientStatusResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, ClientStatusResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientStatusResponseView<'msg> {
  fn into_view<'shorter>(self) -> ClientStatusResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientStatusResponse> for ClientStatusResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientStatusResponse {
    let mut dst = ClientStatusResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClientStatusResponse> for ClientStatusResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClientStatusResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClientStatusResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClientStatusResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClientStatusResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClientStatusResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientStatusResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientStatusResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClientStatusResponseMut<'msg> {
  type Message = ClientStatusResponse;
}

impl ::std::fmt::Debug for ClientStatusResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClientStatusResponse>> for ClientStatusResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientStatusResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientStatusResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClientStatusResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClientStatusResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // config: repeated message envoy.service.status.v3.ClientConfig
  pub fn config(&self) -> ::protobuf::RepeatedView<'_, super::ClientConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ClientConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn config_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ClientConfig> {
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
  pub fn set_config(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ClientConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `ClientStatusResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClientStatusResponseMut<'_> {}

// SAFETY:
// - `ClientStatusResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClientStatusResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for ClientStatusResponseMut<'msg> {
  type Proxied = ClientStatusResponse;
  fn as_view(&self) -> ::protobuf::View<'_, ClientStatusResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientStatusResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClientStatusResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClientStatusResponseMut<'msg> {
  type MutProxied = ClientStatusResponse;
  fn as_mut(&mut self) -> ClientStatusResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClientStatusResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> ClientStatusResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClientStatusResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClientStatusResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClientStatusResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClientStatusResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // config: repeated message envoy.service.status.v3.ClientConfig
  pub fn config(&self) -> ::protobuf::RepeatedView<'_, super::ClientConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ClientConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn config_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ClientConfig> {
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
  pub fn set_config(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ClientConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl ClientStatusResponse

impl ::std::ops::Drop for ClientStatusResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClientStatusResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClientStatusResponse {
  type Proxied = Self;
  fn as_view(&self) -> ClientStatusResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClientStatusResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClientStatusResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClientStatusResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__status__v3__ClientStatusResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__status__v3__ClientStatusResponse_msg_init.0, &[<super::ClientConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__status__v3__ClientStatusResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientStatusResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientStatusResponse {
  type Msg = ClientStatusResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientStatusResponse {
  type Msg = ClientStatusResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientStatusResponseMut<'_> {
  type Msg = ClientStatusResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientStatusResponseMut<'_> {
  type Msg = ClientStatusResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientStatusResponseView<'_> {
  type Msg = ClientStatusResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClientStatusResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientStatusResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConfigStatus(i32);

#[allow(non_upper_case_globals)]
impl ConfigStatus {
  pub const Unknown: ConfigStatus = ConfigStatus(0);
  pub const Synced: ConfigStatus = ConfigStatus(1);
  pub const NotSent: ConfigStatus = ConfigStatus(2);
  pub const Stale: ConfigStatus = ConfigStatus(3);
  pub const Error: ConfigStatus = ConfigStatus(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "Synced",
      2 => "NotSent",
      3 => "Stale",
      4 => "Error",
      _ => return None
    })
  }
}

impl ::std::convert::From<ConfigStatus> for i32 {
  fn from(val: ConfigStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ConfigStatus {
  fn from(val: i32) -> ConfigStatus {
    Self(val)
  }
}

impl ::std::default::Default for ConfigStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ConfigStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ConfigStatus::{}", constant_name)
    } else {
      write!(f, "ConfigStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ConfigStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ConfigStatus {}

impl ::protobuf::Proxied for ConfigStatus {
  type View<'a> = ConfigStatus;
}

impl ::protobuf::AsView for ConfigStatus {
  type Proxied = ConfigStatus;

  fn as_view(&self) -> ConfigStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConfigStatus {
  fn into_view<'shorter>(self) -> ConfigStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ConfigStatus {
  const NAME: &'static str = "ConfigStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for ConfigStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClientConfigStatus(i32);

#[allow(non_upper_case_globals)]
impl ClientConfigStatus {
  pub const ClientUnknown: ClientConfigStatus = ClientConfigStatus(0);
  pub const ClientRequested: ClientConfigStatus = ClientConfigStatus(1);
  pub const ClientAcked: ClientConfigStatus = ClientConfigStatus(2);
  pub const ClientNacked: ClientConfigStatus = ClientConfigStatus(3);
  pub const ClientReceivedError: ClientConfigStatus = ClientConfigStatus(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "ClientUnknown",
      1 => "ClientRequested",
      2 => "ClientAcked",
      3 => "ClientNacked",
      4 => "ClientReceivedError",
      _ => return None
    })
  }
}

impl ::std::convert::From<ClientConfigStatus> for i32 {
  fn from(val: ClientConfigStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ClientConfigStatus {
  fn from(val: i32) -> ClientConfigStatus {
    Self(val)
  }
}

impl ::std::default::Default for ClientConfigStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ClientConfigStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ClientConfigStatus::{}", constant_name)
    } else {
      write!(f, "ClientConfigStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ClientConfigStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ClientConfigStatus {}

impl ::protobuf::Proxied for ClientConfigStatus {
  type View<'a> = ClientConfigStatus;
}

impl ::protobuf::AsView for ClientConfigStatus {
  type Proxied = ClientConfigStatus;

  fn as_view(&self) -> ClientConfigStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientConfigStatus {
  fn into_view<'shorter>(self) -> ClientConfigStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ClientConfigStatus {
  const NAME: &'static str = "ClientConfigStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for ClientConfigStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


