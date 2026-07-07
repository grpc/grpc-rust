const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__NodeMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NodeMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NodeMatcher>
}

impl ::protobuf::Message for NodeMatcher {
  type MessageView<'msg> = NodeMatcherView<'msg>;
  type MessageMut<'msg> = NodeMatcherMut<'msg>;
}

impl ::std::default::Default for NodeMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NodeMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NodeMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `NodeMatcherMut`.
unsafe impl ::std::marker::Sync for NodeMatcher {}

// SAFETY:
// - `NodeMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for NodeMatcher {}

impl ::protobuf::Proxied for NodeMatcher {
  type View<'msg> = NodeMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NodeMatcher {}

impl ::protobuf::MutProxied for NodeMatcher {
  type Mut<'msg> = NodeMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NodeMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NodeMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NodeMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NodeMatcherView<'msg> {
  type Message = NodeMatcher;
}

impl ::std::fmt::Debug for NodeMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NodeMatcherView<'_> {
  fn default() -> NodeMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NodeMatcher>> for NodeMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NodeMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NodeMatcherView<'msg> {

  pub fn to_owned(&self) -> NodeMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // node_id: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_node_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn node_id_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_node_id().then(|| self.node_id())
  }
  pub fn node_id(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

  // node_metadatas: repeated message envoy.type.matcher.v3.StructMatcher
  pub fn node_metadatas(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `NodeMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for NodeMatcherView<'_> {}

// SAFETY:
// - `NodeMatcherView` is `Send` because while its alive a `NodeMatcherMut` cannot.
// - `NodeMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for NodeMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for NodeMatcherView<'msg> {
  type Proxied = NodeMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, NodeMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NodeMatcherView<'msg> {
  fn into_view<'shorter>(self) -> NodeMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NodeMatcher> for NodeMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NodeMatcher {
    let mut dst = NodeMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NodeMatcher> for NodeMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NodeMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for NodeMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NodeMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NodeMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NodeMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NodeMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NodeMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NodeMatcherMut<'msg> {
  type Message = NodeMatcher;
}

impl ::std::fmt::Debug for NodeMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NodeMatcher>> for NodeMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NodeMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NodeMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NodeMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> NodeMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // node_id: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_node_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_node_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn node_id_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_node_id().then(|| self.node_id())
  }
  pub fn node_id(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn node_id_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_node_id(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // node_metadatas: repeated message envoy.type.matcher.v3.StructMatcher
  pub fn node_metadatas(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn node_metadatas_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher> {
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
  pub fn set_node_metadatas(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `NodeMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for NodeMatcherMut<'_> {}

// SAFETY:
// - `NodeMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for NodeMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for NodeMatcherMut<'msg> {
  type Proxied = NodeMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, NodeMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NodeMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NodeMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for NodeMatcherMut<'msg> {
  type MutProxied = NodeMatcher;
  fn as_mut(&mut self) -> NodeMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NodeMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> NodeMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NodeMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NodeMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NodeMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NodeMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // node_id: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_node_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_node_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn node_id_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_node_id().then(|| self.node_id())
  }
  pub fn node_id(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn node_id_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_node_id(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // node_metadatas: repeated message envoy.type.matcher.v3.StructMatcher
  pub fn node_metadatas(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn node_metadatas_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher> {
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
  pub fn set_node_metadatas(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl NodeMatcher

impl ::std::ops::Drop for NodeMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NodeMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NodeMatcher {
  type Proxied = Self;
  fn as_view(&self) -> NodeMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NodeMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NodeMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NodeMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__NodeMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__NodeMatcher_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::r#struct::StructMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__NodeMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NodeMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NodeMatcher {
  type Msg = NodeMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NodeMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NodeMatcher {
  type Msg = NodeMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NodeMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NodeMatcherMut<'_> {
  type Msg = NodeMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NodeMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NodeMatcherMut<'_> {
  type Msg = NodeMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NodeMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NodeMatcherView<'_> {
  type Msg = NodeMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NodeMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NodeMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



