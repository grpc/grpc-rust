const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__ClusterCollection_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClusterCollection {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClusterCollection>
}

impl ::protobuf::Message for ClusterCollection {
  type MessageView<'msg> = ClusterCollectionView<'msg>;
  type MessageMut<'msg> = ClusterCollectionMut<'msg>;
}

impl ::std::default::Default for ClusterCollection {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClusterCollection {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClusterCollection` is `Sync` because it does not implement interior mutability.
//    Neither does `ClusterCollectionMut`.
unsafe impl ::std::marker::Sync for ClusterCollection {}

// SAFETY:
// - `ClusterCollection` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClusterCollection {}

impl ::protobuf::Proxied for ClusterCollection {
  type View<'msg> = ClusterCollectionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClusterCollection {}

impl ::protobuf::MutProxied for ClusterCollection {
  type Mut<'msg> = ClusterCollectionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClusterCollectionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterCollection>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterCollectionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClusterCollectionView<'msg> {
  type Message = ClusterCollection;
}

impl ::std::fmt::Debug for ClusterCollectionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClusterCollectionView<'_> {
  fn default() -> ClusterCollectionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterCollection>> for ClusterCollectionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterCollection>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterCollectionView<'msg> {

  pub fn to_owned(&self) -> ClusterCollection {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // entries: optional message xds.core.v3.CollectionEntry
  pub fn has_entries(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn entries_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'msg>> {
    self.has_entries().then(|| self.entries())
  }
  pub fn entries(self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView::default())
  }

}

// SAFETY:
// - `ClusterCollectionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClusterCollectionView<'_> {}

// SAFETY:
// - `ClusterCollectionView` is `Send` because while its alive a `ClusterCollectionMut` cannot.
// - `ClusterCollectionView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClusterCollectionView<'_> {}

impl<'msg> ::protobuf::AsView for ClusterCollectionView<'msg> {
  type Proxied = ClusterCollection;
  fn as_view(&self) -> ::protobuf::View<'msg, ClusterCollection> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterCollectionView<'msg> {
  fn into_view<'shorter>(self) -> ClusterCollectionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterCollection> for ClusterCollectionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterCollection {
    let mut dst = ClusterCollection::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterCollection> for ClusterCollectionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterCollection {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClusterCollection {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterCollectionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterCollectionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClusterCollectionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterCollection>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterCollectionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClusterCollectionMut<'msg> {
  type Message = ClusterCollection;
}

impl ::std::fmt::Debug for ClusterCollectionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterCollection>> for ClusterCollectionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterCollection>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterCollectionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterCollection> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClusterCollection {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // entries: optional message xds.core.v3.CollectionEntry
  pub fn has_entries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entries_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'_>> {
    self.has_entries().then(|| self.entries())
  }
  pub fn entries(&self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView::default())
  }
  pub fn entries_mut(&mut self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryMut<'_> {
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
  pub fn set_entries(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `ClusterCollectionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClusterCollectionMut<'_> {}

// SAFETY:
// - `ClusterCollectionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClusterCollectionMut<'_> {}

impl<'msg> ::protobuf::AsView for ClusterCollectionMut<'msg> {
  type Proxied = ClusterCollection;
  fn as_view(&self) -> ::protobuf::View<'_, ClusterCollection> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterCollectionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClusterCollection>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClusterCollectionMut<'msg> {
  type MutProxied = ClusterCollection;
  fn as_mut(&mut self) -> ClusterCollectionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClusterCollectionMut<'msg> {
  fn into_mut<'shorter>(self) -> ClusterCollectionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClusterCollection {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClusterCollection> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClusterCollectionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClusterCollectionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // entries: optional message xds.core.v3.CollectionEntry
  pub fn has_entries(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entries(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entries_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'_>> {
    self.has_entries().then(|| self.entries())
  }
  pub fn entries(&self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryView::default())
  }
  pub fn entries_mut(&mut self) -> crate::xds::generated::xds::core::v3::collection_entry::CollectionEntryMut<'_> {
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
  pub fn set_entries(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl ClusterCollection

impl ::std::ops::Drop for ClusterCollection {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClusterCollection {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClusterCollection {
  type Proxied = Self;
  fn as_view(&self) -> ClusterCollectionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClusterCollection {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClusterCollectionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClusterCollection {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__cluster__v3__ClusterCollection_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__cluster__v3__ClusterCollection_msg_init.0, &[<crate::xds::generated::xds::core::v3::collection_entry::CollectionEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__cluster__v3__ClusterCollection_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterCollection {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterCollection {
  type Msg = ClusterCollection;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterCollection> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterCollection {
  type Msg = ClusterCollection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterCollection> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterCollectionMut<'_> {
  type Msg = ClusterCollection;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterCollection> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterCollectionMut<'_> {
  type Msg = ClusterCollection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterCollection> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterCollectionView<'_> {
  type Msg = ClusterCollection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterCollection> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterCollectionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Cluster {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Cluster>
}

impl ::protobuf::Message for Cluster {
  type MessageView<'msg> = ClusterView<'msg>;
  type MessageMut<'msg> = ClusterMut<'msg>;
}

impl ::std::default::Default for Cluster {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Cluster {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Cluster` is `Sync` because it does not implement interior mutability.
//    Neither does `ClusterMut`.
unsafe impl ::std::marker::Sync for Cluster {}

// SAFETY:
// - `Cluster` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Cluster {}

impl ::protobuf::Proxied for Cluster {
  type View<'msg> = ClusterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Cluster {}

impl ::protobuf::MutProxied for Cluster {
  type Mut<'msg> = ClusterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClusterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Cluster>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClusterView<'msg> {
  type Message = Cluster;
}

impl ::std::fmt::Debug for ClusterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClusterView<'_> {
  fn default() -> ClusterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Cluster>> for ClusterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Cluster>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterView<'msg> {

  pub fn to_owned(&self) -> Cluster {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // transport_socket_matches: repeated message envoy.config.cluster.v3.Cluster.TransportSocketMatch
  pub fn transport_socket_matches(self) -> ::protobuf::RepeatedView<'msg, super::cluster::TransportSocketMatch> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        37
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::cluster::TransportSocketMatch>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // transport_socket_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_transport_socket_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(53)
    }
  }
  pub fn transport_socket_matcher_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_transport_socket_matcher().then(|| self.transport_socket_matcher())
  }
  pub fn transport_socket_matcher(self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(53)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
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

  // alt_stat_name: optional string
  pub fn alt_stat_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        23, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // type: optional enum envoy.config.cluster.v3.Cluster.DiscoveryType
  pub fn has_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn type_opt(self) -> ::std::option::Option<super::cluster::DiscoveryType> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(self) -> super::cluster::DiscoveryType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::cluster::DiscoveryType::Static).into()
      ).try_into().unwrap()
    }
  }

  // cluster_type: optional message envoy.config.cluster.v3.Cluster.CustomClusterType
  pub fn has_cluster_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(32)
    }
  }
  pub fn cluster_type_opt(self) -> ::std::option::Option<super::cluster::CustomClusterTypeView<'msg>> {
    self.has_cluster_type().then(|| self.cluster_type())
  }
  pub fn cluster_type(self) -> super::cluster::CustomClusterTypeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(32)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::CustomClusterTypeView::default())
  }

  // eds_cluster_config: optional message envoy.config.cluster.v3.Cluster.EdsClusterConfig
  pub fn has_eds_cluster_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn eds_cluster_config_opt(self) -> ::std::option::Option<super::cluster::EdsClusterConfigView<'msg>> {
    self.has_eds_cluster_config().then(|| self.eds_cluster_config())
  }
  pub fn eds_cluster_config(self) -> super::cluster::EdsClusterConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::EdsClusterConfigView::default())
  }

  // connect_timeout: optional message google.protobuf.Duration
  pub fn has_connect_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn connect_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_connect_timeout().then(|| self.connect_timeout())
  }
  pub fn connect_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // per_connection_buffer_limit_bytes: optional message google.protobuf.UInt32Value
  pub fn has_per_connection_buffer_limit_bytes(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn per_connection_buffer_limit_bytes_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_per_connection_buffer_limit_bytes().then(|| self.per_connection_buffer_limit_bytes())
  }
  pub fn per_connection_buffer_limit_bytes(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // lb_policy: optional enum envoy.config.cluster.v3.Cluster.LbPolicy
  pub fn lb_policy(self) -> super::cluster::LbPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::cluster::LbPolicy::RoundRobin).into()
      ).try_into().unwrap()
    }
  }

  // load_assignment: optional message envoy.config.endpoint.v3.ClusterLoadAssignment
  pub fn has_load_assignment(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(28)
    }
  }
  pub fn load_assignment_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentView<'msg>> {
    self.has_load_assignment().then(|| self.load_assignment())
  }
  pub fn load_assignment(self) -> crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(28)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentView::default())
  }

  // health_checks: repeated message envoy.config.core.v3.HealthCheck
  pub fn health_checks(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // max_requests_per_connection: optional message google.protobuf.UInt32Value
  pub fn has_max_requests_per_connection(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn max_requests_per_connection_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_max_requests_per_connection().then(|| self.max_requests_per_connection())
  }
  pub fn max_requests_per_connection(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // circuit_breakers: optional message envoy.config.cluster.v3.CircuitBreakers
  pub fn has_circuit_breakers(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn circuit_breakers_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersView<'msg>> {
    self.has_circuit_breakers().then(|| self.circuit_breakers())
  }
  pub fn circuit_breakers(self) -> crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersView::default())
  }

  // upstream_http_protocol_options: optional message envoy.config.core.v3.UpstreamHttpProtocolOptions
  pub fn has_upstream_http_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(40)
    }
  }
  pub fn upstream_http_protocol_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsView<'msg>> {
    self.has_upstream_http_protocol_options().then(|| self.upstream_http_protocol_options())
  }
  pub fn upstream_http_protocol_options(self) -> crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(40)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsView::default())
  }

  // common_http_protocol_options: optional message envoy.config.core.v3.HttpProtocolOptions
  pub fn has_common_http_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn common_http_protocol_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'msg>> {
    self.has_common_http_protocol_options().then(|| self.common_http_protocol_options())
  }
  pub fn common_http_protocol_options(self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView::default())
  }

  // http_protocol_options: optional message envoy.config.core.v3.Http1ProtocolOptions
  pub fn has_http_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn http_protocol_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'msg>> {
    self.has_http_protocol_options().then(|| self.http_protocol_options())
  }
  pub fn http_protocol_options(self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView::default())
  }

  // http2_protocol_options: optional message envoy.config.core.v3.Http2ProtocolOptions
  pub fn has_http2_protocol_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn http2_protocol_options_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'msg>> {
    self.has_http2_protocol_options().then(|| self.http2_protocol_options())
  }
  pub fn http2_protocol_options(self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView::default())
  }

  // typed_extension_protocol_options: repeated message envoy.config.cluster.v3.Cluster.TypedExtensionProtocolOptionsEntry
  pub fn typed_extension_protocol_options(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(30)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // dns_refresh_rate: optional message google.protobuf.Duration
  pub fn has_dns_refresh_rate(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn dns_refresh_rate_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_dns_refresh_rate().then(|| self.dns_refresh_rate())
  }
  pub fn dns_refresh_rate(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // dns_jitter: optional message google.protobuf.Duration
  pub fn has_dns_jitter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(52)
    }
  }
  pub fn dns_jitter_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_dns_jitter().then(|| self.dns_jitter())
  }
  pub fn dns_jitter(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(52)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // dns_failure_refresh_rate: optional message envoy.config.cluster.v3.Cluster.RefreshRate
  pub fn has_dns_failure_refresh_rate(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(38)
    }
  }
  pub fn dns_failure_refresh_rate_opt(self) -> ::std::option::Option<super::cluster::RefreshRateView<'msg>> {
    self.has_dns_failure_refresh_rate().then(|| self.dns_failure_refresh_rate())
  }
  pub fn dns_failure_refresh_rate(self) -> super::cluster::RefreshRateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(38)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::RefreshRateView::default())
  }

  // respect_dns_ttl: optional bool
  pub fn respect_dns_ttl(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        33, (false).into()
      ).try_into().unwrap()
    }
  }

  // dns_lookup_family: optional enum envoy.config.cluster.v3.Cluster.DnsLookupFamily
  pub fn dns_lookup_family(self) -> super::cluster::DnsLookupFamily {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        12, (super::cluster::DnsLookupFamily::Auto).into()
      ).try_into().unwrap()
    }
  }

  // dns_resolvers: repeated message envoy.config.core.v3.Address
  pub fn dns_resolvers(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        13
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::Address>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
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
        39, (false).into()
      ).try_into().unwrap()
    }
  }

  // dns_resolution_config: optional message envoy.config.core.v3.DnsResolutionConfig
  pub fn has_dns_resolution_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(47)
    }
  }
  pub fn dns_resolution_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'msg>> {
    self.has_dns_resolution_config().then(|| self.dns_resolution_config())
  }
  pub fn dns_resolution_config(self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(47)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView::default())
  }

  // typed_dns_resolver_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_dns_resolver_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(49)
    }
  }
  pub fn typed_dns_resolver_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_typed_dns_resolver_config().then(|| self.typed_dns_resolver_config())
  }
  pub fn typed_dns_resolver_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(49)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // wait_for_warm_on_init: optional message google.protobuf.BoolValue
  pub fn has_wait_for_warm_on_init(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(48)
    }
  }
  pub fn wait_for_warm_on_init_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_wait_for_warm_on_init().then(|| self.wait_for_warm_on_init())
  }
  pub fn wait_for_warm_on_init(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(48)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // outlier_detection: optional message envoy.config.cluster.v3.OutlierDetection
  pub fn has_outlier_detection(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn outlier_detection_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionView<'msg>> {
    self.has_outlier_detection().then(|| self.outlier_detection())
  }
  pub fn outlier_detection(self) -> crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionView::default())
  }

  // cleanup_interval: optional message google.protobuf.Duration
  pub fn has_cleanup_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn cleanup_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_cleanup_interval().then(|| self.cleanup_interval())
  }
  pub fn cleanup_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // upstream_bind_config: optional message envoy.config.core.v3.BindConfig
  pub fn has_upstream_bind_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn upstream_bind_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'msg>> {
    self.has_upstream_bind_config().then(|| self.upstream_bind_config())
  }
  pub fn upstream_bind_config(self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::BindConfigView::default())
  }

  // lb_subset_config: optional message envoy.config.cluster.v3.Cluster.LbSubsetConfig
  pub fn has_lb_subset_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn lb_subset_config_opt(self) -> ::std::option::Option<super::cluster::LbSubsetConfigView<'msg>> {
    self.has_lb_subset_config().then(|| self.lb_subset_config())
  }
  pub fn lb_subset_config(self) -> super::cluster::LbSubsetConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::LbSubsetConfigView::default())
  }

  // ring_hash_lb_config: optional message envoy.config.cluster.v3.Cluster.RingHashLbConfig
  pub fn has_ring_hash_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn ring_hash_lb_config_opt(self) -> ::std::option::Option<super::cluster::RingHashLbConfigView<'msg>> {
    self.has_ring_hash_lb_config().then(|| self.ring_hash_lb_config())
  }
  pub fn ring_hash_lb_config(self) -> super::cluster::RingHashLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::RingHashLbConfigView::default())
  }

  // maglev_lb_config: optional message envoy.config.cluster.v3.Cluster.MaglevLbConfig
  pub fn has_maglev_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(46)
    }
  }
  pub fn maglev_lb_config_opt(self) -> ::std::option::Option<super::cluster::MaglevLbConfigView<'msg>> {
    self.has_maglev_lb_config().then(|| self.maglev_lb_config())
  }
  pub fn maglev_lb_config(self) -> super::cluster::MaglevLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(46)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::MaglevLbConfigView::default())
  }

  // original_dst_lb_config: optional message envoy.config.cluster.v3.Cluster.OriginalDstLbConfig
  pub fn has_original_dst_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(29)
    }
  }
  pub fn original_dst_lb_config_opt(self) -> ::std::option::Option<super::cluster::OriginalDstLbConfigView<'msg>> {
    self.has_original_dst_lb_config().then(|| self.original_dst_lb_config())
  }
  pub fn original_dst_lb_config(self) -> super::cluster::OriginalDstLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(29)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::OriginalDstLbConfigView::default())
  }

  // least_request_lb_config: optional message envoy.config.cluster.v3.Cluster.LeastRequestLbConfig
  pub fn has_least_request_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(31)
    }
  }
  pub fn least_request_lb_config_opt(self) -> ::std::option::Option<super::cluster::LeastRequestLbConfigView<'msg>> {
    self.has_least_request_lb_config().then(|| self.least_request_lb_config())
  }
  pub fn least_request_lb_config(self) -> super::cluster::LeastRequestLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(31)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::LeastRequestLbConfigView::default())
  }

  // round_robin_lb_config: optional message envoy.config.cluster.v3.Cluster.RoundRobinLbConfig
  pub fn has_round_robin_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(50)
    }
  }
  pub fn round_robin_lb_config_opt(self) -> ::std::option::Option<super::cluster::RoundRobinLbConfigView<'msg>> {
    self.has_round_robin_lb_config().then(|| self.round_robin_lb_config())
  }
  pub fn round_robin_lb_config(self) -> super::cluster::RoundRobinLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(50)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::RoundRobinLbConfigView::default())
  }

  // common_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig
  pub fn has_common_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn common_lb_config_opt(self) -> ::std::option::Option<super::cluster::CommonLbConfigView<'msg>> {
    self.has_common_lb_config().then(|| self.common_lb_config())
  }
  pub fn common_lb_config(self) -> super::cluster::CommonLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::CommonLbConfigView::default())
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn transport_socket_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'msg>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn metadata_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }

  // protocol_selection: optional enum envoy.config.cluster.v3.Cluster.ClusterProtocolSelection
  pub fn protocol_selection(self) -> super::cluster::ClusterProtocolSelection {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        21, (super::cluster::ClusterProtocolSelection::UseConfiguredProtocol).into()
      ).try_into().unwrap()
    }
  }

  // upstream_connection_options: optional message envoy.config.cluster.v3.UpstreamConnectionOptions
  pub fn has_upstream_connection_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn upstream_connection_options_opt(self) -> ::std::option::Option<super::UpstreamConnectionOptionsView<'msg>> {
    self.has_upstream_connection_options().then(|| self.upstream_connection_options())
  }
  pub fn upstream_connection_options(self) -> super::UpstreamConnectionOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UpstreamConnectionOptionsView::default())
  }

  // close_connections_on_host_health_failure: optional bool
  pub fn close_connections_on_host_health_failure(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }

  // ignore_health_on_host_removal: optional bool
  pub fn ignore_health_on_host_removal(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        27, (false).into()
      ).try_into().unwrap()
    }
  }

  // filters: repeated message envoy.config.cluster.v3.Filter
  pub fn filters(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::cluster::v3::filter::Filter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        34
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::cluster::v3::filter::Filter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // load_balancing_policy: optional message envoy.config.cluster.v3.LoadBalancingPolicy
  pub fn has_load_balancing_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(35)
    }
  }
  pub fn load_balancing_policy_opt(self) -> ::std::option::Option<super::LoadBalancingPolicyView<'msg>> {
    self.has_load_balancing_policy().then(|| self.load_balancing_policy())
  }
  pub fn load_balancing_policy(self) -> super::LoadBalancingPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(35)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LoadBalancingPolicyView::default())
  }

  // lrs_server: optional message envoy.config.core.v3.ConfigSource
  pub fn has_lrs_server(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(36)
    }
  }
  pub fn lrs_server_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_lrs_server().then(|| self.lrs_server())
  }
  pub fn lrs_server(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(36)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // lrs_report_endpoint_metrics: repeated string
  pub fn lrs_report_endpoint_metrics(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        51
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // track_timeout_budgets: optional bool
  pub fn track_timeout_budgets(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        41, (false).into()
      ).try_into().unwrap()
    }
  }

  // upstream_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_upstream_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(42)
    }
  }
  pub fn upstream_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_upstream_config().then(|| self.upstream_config())
  }
  pub fn upstream_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(42)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // track_cluster_stats: optional message envoy.config.cluster.v3.TrackClusterStats
  pub fn has_track_cluster_stats(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(43)
    }
  }
  pub fn track_cluster_stats_opt(self) -> ::std::option::Option<super::TrackClusterStatsView<'msg>> {
    self.has_track_cluster_stats().then(|| self.track_cluster_stats())
  }
  pub fn track_cluster_stats(self) -> super::TrackClusterStatsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(43)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TrackClusterStatsView::default())
  }

  // preconnect_policy: optional message envoy.config.cluster.v3.Cluster.PreconnectPolicy
  pub fn has_preconnect_policy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(44)
    }
  }
  pub fn preconnect_policy_opt(self) -> ::std::option::Option<super::cluster::PreconnectPolicyView<'msg>> {
    self.has_preconnect_policy().then(|| self.preconnect_policy())
  }
  pub fn preconnect_policy(self) -> super::cluster::PreconnectPolicyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(44)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::PreconnectPolicyView::default())
  }

  // connection_pool_per_downstream_connection: optional bool
  pub fn connection_pool_per_downstream_connection(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        45, (false).into()
      ).try_into().unwrap()
    }
  }

  pub fn cluster_discovery_type(self) -> super::cluster::ClusterDiscoveryTypeOneof<'msg> {
    match self.cluster_discovery_type_case() {
      super::cluster::ClusterDiscoveryTypeCase::Type =>
          super::cluster::ClusterDiscoveryTypeOneof::Type(self.r#type()),
      super::cluster::ClusterDiscoveryTypeCase::ClusterType =>
          super::cluster::ClusterDiscoveryTypeOneof::ClusterType(self.cluster_type()),
      _ => super::cluster::ClusterDiscoveryTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn cluster_discovery_type_case(self) -> super::cluster::ClusterDiscoveryTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::cluster::ClusterDiscoveryTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
  pub fn lb_config(self) -> super::cluster::LbConfigOneof<'msg> {
    match self.lb_config_case() {
      super::cluster::LbConfigCase::RingHashLbConfig =>
          super::cluster::LbConfigOneof::RingHashLbConfig(self.ring_hash_lb_config()),
      super::cluster::LbConfigCase::MaglevLbConfig =>
          super::cluster::LbConfigOneof::MaglevLbConfig(self.maglev_lb_config()),
      super::cluster::LbConfigCase::OriginalDstLbConfig =>
          super::cluster::LbConfigOneof::OriginalDstLbConfig(self.original_dst_lb_config()),
      super::cluster::LbConfigCase::LeastRequestLbConfig =>
          super::cluster::LbConfigOneof::LeastRequestLbConfig(self.least_request_lb_config()),
      super::cluster::LbConfigCase::RoundRobinLbConfig =>
          super::cluster::LbConfigOneof::RoundRobinLbConfig(self.round_robin_lb_config()),
      _ => super::cluster::LbConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn lb_config_case(self) -> super::cluster::LbConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(18);
      super::cluster::LbConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ClusterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClusterView<'_> {}

// SAFETY:
// - `ClusterView` is `Send` because while its alive a `ClusterMut` cannot.
// - `ClusterView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClusterView<'_> {}

impl<'msg> ::protobuf::AsView for ClusterView<'msg> {
  type Proxied = Cluster;
  fn as_view(&self) -> ::protobuf::View<'msg, Cluster> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterView<'msg> {
  fn into_view<'shorter>(self) -> ClusterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Cluster> for ClusterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Cluster {
    let mut dst = Cluster::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Cluster> for ClusterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Cluster {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Cluster {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClusterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Cluster>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClusterMut<'msg> {
  type Message = Cluster;
}

impl ::std::fmt::Debug for ClusterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Cluster>> for ClusterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Cluster>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Cluster> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Cluster {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // transport_socket_matches: repeated message envoy.config.cluster.v3.Cluster.TransportSocketMatch
  pub fn transport_socket_matches(&self) -> ::protobuf::RepeatedView<'_, super::cluster::TransportSocketMatch> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        37
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::cluster::TransportSocketMatch>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn transport_socket_matches_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::cluster::TransportSocketMatch> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        37,
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
  pub fn set_transport_socket_matches(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::cluster::TransportSocketMatch>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        37,
        src);
    }
  }

  // transport_socket_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_transport_socket_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(53)
    }
  }
  pub fn clear_transport_socket_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        53
      );
    }
  }
  pub fn transport_socket_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_transport_socket_matcher().then(|| self.transport_socket_matcher())
  }
  pub fn transport_socket_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(53)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn transport_socket_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         53, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_transport_socket_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        53,
        val
      );
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

  // alt_stat_name: optional string
  pub fn alt_stat_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        23, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_alt_stat_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val);
    }
  }

  // type: optional enum envoy.config.cluster.v3.Cluster.DiscoveryType
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<super::cluster::DiscoveryType> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> super::cluster::DiscoveryType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::cluster::DiscoveryType::Static).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_type(&mut self, val: super::cluster::DiscoveryType) {
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

  // cluster_type: optional message envoy.config.cluster.v3.Cluster.CustomClusterType
  pub fn has_cluster_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(32)
    }
  }
  pub fn clear_cluster_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        32
      );
    }
  }
  pub fn cluster_type_opt(&self) -> ::std::option::Option<super::cluster::CustomClusterTypeView<'_>> {
    self.has_cluster_type().then(|| self.cluster_type())
  }
  pub fn cluster_type(&self) -> super::cluster::CustomClusterTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(32)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::CustomClusterTypeView::default())
  }
  pub fn cluster_type_mut(&mut self) -> super::cluster::CustomClusterTypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         32, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cluster_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::CustomClusterType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        32,
        val
      );
    }
  }

  // eds_cluster_config: optional message envoy.config.cluster.v3.Cluster.EdsClusterConfig
  pub fn has_eds_cluster_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_eds_cluster_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn eds_cluster_config_opt(&self) -> ::std::option::Option<super::cluster::EdsClusterConfigView<'_>> {
    self.has_eds_cluster_config().then(|| self.eds_cluster_config())
  }
  pub fn eds_cluster_config(&self) -> super::cluster::EdsClusterConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::EdsClusterConfigView::default())
  }
  pub fn eds_cluster_config_mut(&mut self) -> super::cluster::EdsClusterConfigMut<'_> {
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
  pub fn set_eds_cluster_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::EdsClusterConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // connect_timeout: optional message google.protobuf.Duration
  pub fn has_connect_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_connect_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn connect_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_connect_timeout().then(|| self.connect_timeout())
  }
  pub fn connect_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn connect_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_connect_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // per_connection_buffer_limit_bytes: optional message google.protobuf.UInt32Value
  pub fn has_per_connection_buffer_limit_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_per_connection_buffer_limit_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn per_connection_buffer_limit_bytes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_per_connection_buffer_limit_bytes().then(|| self.per_connection_buffer_limit_bytes())
  }
  pub fn per_connection_buffer_limit_bytes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn per_connection_buffer_limit_bytes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_per_connection_buffer_limit_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // lb_policy: optional enum envoy.config.cluster.v3.Cluster.LbPolicy
  pub fn lb_policy(&self) -> super::cluster::LbPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::cluster::LbPolicy::RoundRobin).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lb_policy(&mut self, val: super::cluster::LbPolicy) {
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

  // load_assignment: optional message envoy.config.endpoint.v3.ClusterLoadAssignment
  pub fn has_load_assignment(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(28)
    }
  }
  pub fn clear_load_assignment(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        28
      );
    }
  }
  pub fn load_assignment_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentView<'_>> {
    self.has_load_assignment().then(|| self.load_assignment())
  }
  pub fn load_assignment(&self) -> crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(28)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentView::default())
  }
  pub fn load_assignment_mut(&mut self) -> crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         28, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_load_assignment(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignment>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        28,
        val
      );
    }
  }

  // health_checks: repeated message envoy.config.core.v3.HealthCheck
  pub fn health_checks(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn health_checks_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck> {
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
  pub fn set_health_checks(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // max_requests_per_connection: optional message google.protobuf.UInt32Value
  pub fn has_max_requests_per_connection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_max_requests_per_connection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn max_requests_per_connection_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_requests_per_connection().then(|| self.max_requests_per_connection())
  }
  pub fn max_requests_per_connection(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_requests_per_connection_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_requests_per_connection(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // circuit_breakers: optional message envoy.config.cluster.v3.CircuitBreakers
  pub fn has_circuit_breakers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_circuit_breakers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn circuit_breakers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersView<'_>> {
    self.has_circuit_breakers().then(|| self.circuit_breakers())
  }
  pub fn circuit_breakers(&self) -> crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersView::default())
  }
  pub fn circuit_breakers_mut(&mut self) -> crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersMut<'_> {
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
  pub fn set_circuit_breakers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakers>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // upstream_http_protocol_options: optional message envoy.config.core.v3.UpstreamHttpProtocolOptions
  pub fn has_upstream_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(40)
    }
  }
  pub fn clear_upstream_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        40
      );
    }
  }
  pub fn upstream_http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsView<'_>> {
    self.has_upstream_http_protocol_options().then(|| self.upstream_http_protocol_options())
  }
  pub fn upstream_http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(40)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsView::default())
  }
  pub fn upstream_http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         40, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        40,
        val
      );
    }
  }

  // common_http_protocol_options: optional message envoy.config.core.v3.HttpProtocolOptions
  pub fn has_common_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_common_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn common_http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'_>> {
    self.has_common_http_protocol_options().then(|| self.common_http_protocol_options())
  }
  pub fn common_http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView::default())
  }
  pub fn common_http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         24, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_common_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // http_protocol_options: optional message envoy.config.core.v3.Http1ProtocolOptions
  pub fn has_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'_>> {
    self.has_http_protocol_options().then(|| self.http_protocol_options())
  }
  pub fn http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView::default())
  }
  pub fn http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsMut<'_> {
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
  pub fn set_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // http2_protocol_options: optional message envoy.config.core.v3.Http2ProtocolOptions
  pub fn has_http2_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_http2_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn http2_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'_>> {
    self.has_http2_protocol_options().then(|| self.http2_protocol_options())
  }
  pub fn http2_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView::default())
  }
  pub fn http2_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsMut<'_> {
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
  pub fn set_http2_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // typed_extension_protocol_options: repeated message envoy.config.cluster.v3.Cluster.TypedExtensionProtocolOptionsEntry
  pub fn typed_extension_protocol_options(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(30)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn typed_extension_protocol_options_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          30, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_typed_extension_protocol_options(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        30,
        src);
    }
  }

  // dns_refresh_rate: optional message google.protobuf.Duration
  pub fn has_dns_refresh_rate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_dns_refresh_rate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn dns_refresh_rate_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_dns_refresh_rate().then(|| self.dns_refresh_rate())
  }
  pub fn dns_refresh_rate(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn dns_refresh_rate_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_dns_refresh_rate(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // dns_jitter: optional message google.protobuf.Duration
  pub fn has_dns_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(52)
    }
  }
  pub fn clear_dns_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        52
      );
    }
  }
  pub fn dns_jitter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_dns_jitter().then(|| self.dns_jitter())
  }
  pub fn dns_jitter(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(52)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn dns_jitter_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         52, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dns_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        52,
        val
      );
    }
  }

  // dns_failure_refresh_rate: optional message envoy.config.cluster.v3.Cluster.RefreshRate
  pub fn has_dns_failure_refresh_rate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(38)
    }
  }
  pub fn clear_dns_failure_refresh_rate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        38
      );
    }
  }
  pub fn dns_failure_refresh_rate_opt(&self) -> ::std::option::Option<super::cluster::RefreshRateView<'_>> {
    self.has_dns_failure_refresh_rate().then(|| self.dns_failure_refresh_rate())
  }
  pub fn dns_failure_refresh_rate(&self) -> super::cluster::RefreshRateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(38)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::RefreshRateView::default())
  }
  pub fn dns_failure_refresh_rate_mut(&mut self) -> super::cluster::RefreshRateMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         38, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dns_failure_refresh_rate(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::RefreshRate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        38,
        val
      );
    }
  }

  // respect_dns_ttl: optional bool
  pub fn respect_dns_ttl(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        33, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_respect_dns_ttl(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        33, val.into()
      )
    }
  }

  // dns_lookup_family: optional enum envoy.config.cluster.v3.Cluster.DnsLookupFamily
  pub fn dns_lookup_family(&self) -> super::cluster::DnsLookupFamily {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        12, (super::cluster::DnsLookupFamily::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_dns_lookup_family(&mut self, val: super::cluster::DnsLookupFamily) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        12, val.into()
      )
    }
  }

  // dns_resolvers: repeated message envoy.config.core.v3.Address
  pub fn dns_resolvers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        13
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::Address>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dns_resolvers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        13,
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
  pub fn set_dns_resolvers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::Address>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        src);
    }
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
        39, (false).into()
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
        39, val.into()
      )
    }
  }

  // dns_resolution_config: optional message envoy.config.core.v3.DnsResolutionConfig
  pub fn has_dns_resolution_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(47)
    }
  }
  pub fn clear_dns_resolution_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        47
      );
    }
  }
  pub fn dns_resolution_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'_>> {
    self.has_dns_resolution_config().then(|| self.dns_resolution_config())
  }
  pub fn dns_resolution_config(&self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(47)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView::default())
  }
  pub fn dns_resolution_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         47, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dns_resolution_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        47,
        val
      );
    }
  }

  // typed_dns_resolver_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_dns_resolver_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(49)
    }
  }
  pub fn clear_typed_dns_resolver_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        49
      );
    }
  }
  pub fn typed_dns_resolver_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_dns_resolver_config().then(|| self.typed_dns_resolver_config())
  }
  pub fn typed_dns_resolver_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(49)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_dns_resolver_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         49, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_typed_dns_resolver_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        49,
        val
      );
    }
  }

  // wait_for_warm_on_init: optional message google.protobuf.BoolValue
  pub fn has_wait_for_warm_on_init(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(48)
    }
  }
  pub fn clear_wait_for_warm_on_init(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        48
      );
    }
  }
  pub fn wait_for_warm_on_init_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_wait_for_warm_on_init().then(|| self.wait_for_warm_on_init())
  }
  pub fn wait_for_warm_on_init(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(48)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn wait_for_warm_on_init_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         48, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_wait_for_warm_on_init(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        48,
        val
      );
    }
  }

  // outlier_detection: optional message envoy.config.cluster.v3.OutlierDetection
  pub fn has_outlier_detection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_outlier_detection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn outlier_detection_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionView<'_>> {
    self.has_outlier_detection().then(|| self.outlier_detection())
  }
  pub fn outlier_detection(&self) -> crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionView::default())
  }
  pub fn outlier_detection_mut(&mut self) -> crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionMut<'_> {
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
  pub fn set_outlier_detection(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetection>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // cleanup_interval: optional message google.protobuf.Duration
  pub fn has_cleanup_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_cleanup_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn cleanup_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_cleanup_interval().then(|| self.cleanup_interval())
  }
  pub fn cleanup_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn cleanup_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cleanup_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // upstream_bind_config: optional message envoy.config.core.v3.BindConfig
  pub fn has_upstream_bind_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_upstream_bind_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn upstream_bind_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'_>> {
    self.has_upstream_bind_config().then(|| self.upstream_bind_config())
  }
  pub fn upstream_bind_config(&self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::BindConfigView::default())
  }
  pub fn upstream_bind_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_bind_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::BindConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // lb_subset_config: optional message envoy.config.cluster.v3.Cluster.LbSubsetConfig
  pub fn has_lb_subset_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_lb_subset_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn lb_subset_config_opt(&self) -> ::std::option::Option<super::cluster::LbSubsetConfigView<'_>> {
    self.has_lb_subset_config().then(|| self.lb_subset_config())
  }
  pub fn lb_subset_config(&self) -> super::cluster::LbSubsetConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::LbSubsetConfigView::default())
  }
  pub fn lb_subset_config_mut(&mut self) -> super::cluster::LbSubsetConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_lb_subset_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::LbSubsetConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

  // ring_hash_lb_config: optional message envoy.config.cluster.v3.Cluster.RingHashLbConfig
  pub fn has_ring_hash_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_ring_hash_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn ring_hash_lb_config_opt(&self) -> ::std::option::Option<super::cluster::RingHashLbConfigView<'_>> {
    self.has_ring_hash_lb_config().then(|| self.ring_hash_lb_config())
  }
  pub fn ring_hash_lb_config(&self) -> super::cluster::RingHashLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::RingHashLbConfigView::default())
  }
  pub fn ring_hash_lb_config_mut(&mut self) -> super::cluster::RingHashLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         18, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_ring_hash_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::RingHashLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // maglev_lb_config: optional message envoy.config.cluster.v3.Cluster.MaglevLbConfig
  pub fn has_maglev_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(46)
    }
  }
  pub fn clear_maglev_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        46
      );
    }
  }
  pub fn maglev_lb_config_opt(&self) -> ::std::option::Option<super::cluster::MaglevLbConfigView<'_>> {
    self.has_maglev_lb_config().then(|| self.maglev_lb_config())
  }
  pub fn maglev_lb_config(&self) -> super::cluster::MaglevLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(46)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::MaglevLbConfigView::default())
  }
  pub fn maglev_lb_config_mut(&mut self) -> super::cluster::MaglevLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         46, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_maglev_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::MaglevLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        46,
        val
      );
    }
  }

  // original_dst_lb_config: optional message envoy.config.cluster.v3.Cluster.OriginalDstLbConfig
  pub fn has_original_dst_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(29)
    }
  }
  pub fn clear_original_dst_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        29
      );
    }
  }
  pub fn original_dst_lb_config_opt(&self) -> ::std::option::Option<super::cluster::OriginalDstLbConfigView<'_>> {
    self.has_original_dst_lb_config().then(|| self.original_dst_lb_config())
  }
  pub fn original_dst_lb_config(&self) -> super::cluster::OriginalDstLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(29)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::OriginalDstLbConfigView::default())
  }
  pub fn original_dst_lb_config_mut(&mut self) -> super::cluster::OriginalDstLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         29, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_original_dst_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::OriginalDstLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        29,
        val
      );
    }
  }

  // least_request_lb_config: optional message envoy.config.cluster.v3.Cluster.LeastRequestLbConfig
  pub fn has_least_request_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(31)
    }
  }
  pub fn clear_least_request_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        31
      );
    }
  }
  pub fn least_request_lb_config_opt(&self) -> ::std::option::Option<super::cluster::LeastRequestLbConfigView<'_>> {
    self.has_least_request_lb_config().then(|| self.least_request_lb_config())
  }
  pub fn least_request_lb_config(&self) -> super::cluster::LeastRequestLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(31)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::LeastRequestLbConfigView::default())
  }
  pub fn least_request_lb_config_mut(&mut self) -> super::cluster::LeastRequestLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         31, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_least_request_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::LeastRequestLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        31,
        val
      );
    }
  }

  // round_robin_lb_config: optional message envoy.config.cluster.v3.Cluster.RoundRobinLbConfig
  pub fn has_round_robin_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(50)
    }
  }
  pub fn clear_round_robin_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        50
      );
    }
  }
  pub fn round_robin_lb_config_opt(&self) -> ::std::option::Option<super::cluster::RoundRobinLbConfigView<'_>> {
    self.has_round_robin_lb_config().then(|| self.round_robin_lb_config())
  }
  pub fn round_robin_lb_config(&self) -> super::cluster::RoundRobinLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(50)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::RoundRobinLbConfigView::default())
  }
  pub fn round_robin_lb_config_mut(&mut self) -> super::cluster::RoundRobinLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         50, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_round_robin_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::RoundRobinLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        50,
        val
      );
    }
  }

  // common_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig
  pub fn has_common_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_common_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn common_lb_config_opt(&self) -> ::std::option::Option<super::cluster::CommonLbConfigView<'_>> {
    self.has_common_lb_config().then(|| self.common_lb_config())
  }
  pub fn common_lb_config(&self) -> super::cluster::CommonLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::CommonLbConfigView::default())
  }
  pub fn common_lb_config_mut(&mut self) -> super::cluster::CommonLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         22, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_common_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::CommonLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_transport_socket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn transport_socket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(&self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }
  pub fn transport_socket_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         19, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_transport_socket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::TransportSocket>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         20, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val
      );
    }
  }

  // protocol_selection: optional enum envoy.config.cluster.v3.Cluster.ClusterProtocolSelection
  pub fn protocol_selection(&self) -> super::cluster::ClusterProtocolSelection {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        21, (super::cluster::ClusterProtocolSelection::UseConfiguredProtocol).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_protocol_selection(&mut self, val: super::cluster::ClusterProtocolSelection) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        21, val.into()
      )
    }
  }

  // upstream_connection_options: optional message envoy.config.cluster.v3.UpstreamConnectionOptions
  pub fn has_upstream_connection_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_upstream_connection_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn upstream_connection_options_opt(&self) -> ::std::option::Option<super::UpstreamConnectionOptionsView<'_>> {
    self.has_upstream_connection_options().then(|| self.upstream_connection_options())
  }
  pub fn upstream_connection_options(&self) -> super::UpstreamConnectionOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UpstreamConnectionOptionsView::default())
  }
  pub fn upstream_connection_options_mut(&mut self) -> super::UpstreamConnectionOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         25, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_connection_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::UpstreamConnectionOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val
      );
    }
  }

  // close_connections_on_host_health_failure: optional bool
  pub fn close_connections_on_host_health_failure(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_close_connections_on_host_health_failure(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // ignore_health_on_host_removal: optional bool
  pub fn ignore_health_on_host_removal(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        27, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_health_on_host_removal(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        27, val.into()
      )
    }
  }

  // filters: repeated message envoy.config.cluster.v3.Filter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::cluster::v3::filter::Filter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        34
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::cluster::v3::filter::Filter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::cluster::v3::filter::Filter> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        34,
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::cluster::v3::filter::Filter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        34,
        src);
    }
  }

  // load_balancing_policy: optional message envoy.config.cluster.v3.LoadBalancingPolicy
  pub fn has_load_balancing_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(35)
    }
  }
  pub fn clear_load_balancing_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        35
      );
    }
  }
  pub fn load_balancing_policy_opt(&self) -> ::std::option::Option<super::LoadBalancingPolicyView<'_>> {
    self.has_load_balancing_policy().then(|| self.load_balancing_policy())
  }
  pub fn load_balancing_policy(&self) -> super::LoadBalancingPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(35)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LoadBalancingPolicyView::default())
  }
  pub fn load_balancing_policy_mut(&mut self) -> super::LoadBalancingPolicyMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         35, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_load_balancing_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::LoadBalancingPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        35,
        val
      );
    }
  }

  // lrs_server: optional message envoy.config.core.v3.ConfigSource
  pub fn has_lrs_server(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(36)
    }
  }
  pub fn clear_lrs_server(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        36
      );
    }
  }
  pub fn lrs_server_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_lrs_server().then(|| self.lrs_server())
  }
  pub fn lrs_server(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(36)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn lrs_server_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         36, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_lrs_server(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        36,
        val
      );
    }
  }

  // lrs_report_endpoint_metrics: repeated string
  pub fn lrs_report_endpoint_metrics(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        51
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn lrs_report_endpoint_metrics_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        51,
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
  pub fn set_lrs_report_endpoint_metrics(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        51,
        src);
    }
  }

  // track_timeout_budgets: optional bool
  pub fn track_timeout_budgets(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        41, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_track_timeout_budgets(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        41, val.into()
      )
    }
  }

  // upstream_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_upstream_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(42)
    }
  }
  pub fn clear_upstream_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        42
      );
    }
  }
  pub fn upstream_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_upstream_config().then(|| self.upstream_config())
  }
  pub fn upstream_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(42)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn upstream_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         42, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        42,
        val
      );
    }
  }

  // track_cluster_stats: optional message envoy.config.cluster.v3.TrackClusterStats
  pub fn has_track_cluster_stats(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(43)
    }
  }
  pub fn clear_track_cluster_stats(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        43
      );
    }
  }
  pub fn track_cluster_stats_opt(&self) -> ::std::option::Option<super::TrackClusterStatsView<'_>> {
    self.has_track_cluster_stats().then(|| self.track_cluster_stats())
  }
  pub fn track_cluster_stats(&self) -> super::TrackClusterStatsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(43)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TrackClusterStatsView::default())
  }
  pub fn track_cluster_stats_mut(&mut self) -> super::TrackClusterStatsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         43, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_track_cluster_stats(&mut self,
    val: impl ::protobuf::IntoProxied<super::TrackClusterStats>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        43,
        val
      );
    }
  }

  // preconnect_policy: optional message envoy.config.cluster.v3.Cluster.PreconnectPolicy
  pub fn has_preconnect_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(44)
    }
  }
  pub fn clear_preconnect_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        44
      );
    }
  }
  pub fn preconnect_policy_opt(&self) -> ::std::option::Option<super::cluster::PreconnectPolicyView<'_>> {
    self.has_preconnect_policy().then(|| self.preconnect_policy())
  }
  pub fn preconnect_policy(&self) -> super::cluster::PreconnectPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(44)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::PreconnectPolicyView::default())
  }
  pub fn preconnect_policy_mut(&mut self) -> super::cluster::PreconnectPolicyMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         44, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_preconnect_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::PreconnectPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        44,
        val
      );
    }
  }

  // connection_pool_per_downstream_connection: optional bool
  pub fn connection_pool_per_downstream_connection(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        45, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_connection_pool_per_downstream_connection(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        45, val.into()
      )
    }
  }

  pub fn cluster_discovery_type(&self) -> super::cluster::ClusterDiscoveryTypeOneof<'_> {
    match &self.cluster_discovery_type_case() {
      super::cluster::ClusterDiscoveryTypeCase::Type =>
          super::cluster::ClusterDiscoveryTypeOneof::Type(self.r#type()),
      super::cluster::ClusterDiscoveryTypeCase::ClusterType =>
          super::cluster::ClusterDiscoveryTypeOneof::ClusterType(self.cluster_type()),
      _ => super::cluster::ClusterDiscoveryTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn cluster_discovery_type_case(&self) -> super::cluster::ClusterDiscoveryTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::cluster::ClusterDiscoveryTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
  pub fn lb_config(&self) -> super::cluster::LbConfigOneof<'_> {
    match &self.lb_config_case() {
      super::cluster::LbConfigCase::RingHashLbConfig =>
          super::cluster::LbConfigOneof::RingHashLbConfig(self.ring_hash_lb_config()),
      super::cluster::LbConfigCase::MaglevLbConfig =>
          super::cluster::LbConfigOneof::MaglevLbConfig(self.maglev_lb_config()),
      super::cluster::LbConfigCase::OriginalDstLbConfig =>
          super::cluster::LbConfigOneof::OriginalDstLbConfig(self.original_dst_lb_config()),
      super::cluster::LbConfigCase::LeastRequestLbConfig =>
          super::cluster::LbConfigOneof::LeastRequestLbConfig(self.least_request_lb_config()),
      super::cluster::LbConfigCase::RoundRobinLbConfig =>
          super::cluster::LbConfigOneof::RoundRobinLbConfig(self.round_robin_lb_config()),
      _ => super::cluster::LbConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn lb_config_case(&self) -> super::cluster::LbConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(18);
      super::cluster::LbConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `ClusterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClusterMut<'_> {}

// SAFETY:
// - `ClusterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClusterMut<'_> {}

impl<'msg> ::protobuf::AsView for ClusterMut<'msg> {
  type Proxied = Cluster;
  fn as_view(&self) -> ::protobuf::View<'_, Cluster> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Cluster>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClusterMut<'msg> {
  type MutProxied = Cluster;
  fn as_mut(&mut self) -> ClusterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClusterMut<'msg> {
  fn into_mut<'shorter>(self) -> ClusterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Cluster {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Cluster> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClusterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClusterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // transport_socket_matches: repeated message envoy.config.cluster.v3.Cluster.TransportSocketMatch
  pub fn transport_socket_matches(&self) -> ::protobuf::RepeatedView<'_, super::cluster::TransportSocketMatch> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        37
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::cluster::TransportSocketMatch>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn transport_socket_matches_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::cluster::TransportSocketMatch> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        37,
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
  pub fn set_transport_socket_matches(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::cluster::TransportSocketMatch>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        37,
        src);
    }
  }

  // transport_socket_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_transport_socket_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(53)
    }
  }
  pub fn clear_transport_socket_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        53
      );
    }
  }
  pub fn transport_socket_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_transport_socket_matcher().then(|| self.transport_socket_matcher())
  }
  pub fn transport_socket_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(53)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn transport_socket_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         53, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_transport_socket_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        53,
        val
      );
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

  // alt_stat_name: optional string
  pub fn alt_stat_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        23, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_alt_stat_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val);
    }
  }

  // type: optional enum envoy.config.cluster.v3.Cluster.DiscoveryType
  pub fn has_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn type_opt(&self) -> ::std::option::Option<super::cluster::DiscoveryType> {
    self.has_type().then(|| self.r#type())
  }
  pub fn r#type(&self) -> super::cluster::DiscoveryType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::cluster::DiscoveryType::Static).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_type(&mut self, val: super::cluster::DiscoveryType) {
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

  // cluster_type: optional message envoy.config.cluster.v3.Cluster.CustomClusterType
  pub fn has_cluster_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(32)
    }
  }
  pub fn clear_cluster_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        32
      );
    }
  }
  pub fn cluster_type_opt(&self) -> ::std::option::Option<super::cluster::CustomClusterTypeView<'_>> {
    self.has_cluster_type().then(|| self.cluster_type())
  }
  pub fn cluster_type(&self) -> super::cluster::CustomClusterTypeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(32)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::CustomClusterTypeView::default())
  }
  pub fn cluster_type_mut(&mut self) -> super::cluster::CustomClusterTypeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         32, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cluster_type(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::CustomClusterType>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        32,
        val
      );
    }
  }

  // eds_cluster_config: optional message envoy.config.cluster.v3.Cluster.EdsClusterConfig
  pub fn has_eds_cluster_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_eds_cluster_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn eds_cluster_config_opt(&self) -> ::std::option::Option<super::cluster::EdsClusterConfigView<'_>> {
    self.has_eds_cluster_config().then(|| self.eds_cluster_config())
  }
  pub fn eds_cluster_config(&self) -> super::cluster::EdsClusterConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::EdsClusterConfigView::default())
  }
  pub fn eds_cluster_config_mut(&mut self) -> super::cluster::EdsClusterConfigMut<'_> {
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
  pub fn set_eds_cluster_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::EdsClusterConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // connect_timeout: optional message google.protobuf.Duration
  pub fn has_connect_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_connect_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn connect_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_connect_timeout().then(|| self.connect_timeout())
  }
  pub fn connect_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn connect_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_connect_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // per_connection_buffer_limit_bytes: optional message google.protobuf.UInt32Value
  pub fn has_per_connection_buffer_limit_bytes(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_per_connection_buffer_limit_bytes(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn per_connection_buffer_limit_bytes_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_per_connection_buffer_limit_bytes().then(|| self.per_connection_buffer_limit_bytes())
  }
  pub fn per_connection_buffer_limit_bytes(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn per_connection_buffer_limit_bytes_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_per_connection_buffer_limit_bytes(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // lb_policy: optional enum envoy.config.cluster.v3.Cluster.LbPolicy
  pub fn lb_policy(&self) -> super::cluster::LbPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::cluster::LbPolicy::RoundRobin).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_lb_policy(&mut self, val: super::cluster::LbPolicy) {
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

  // load_assignment: optional message envoy.config.endpoint.v3.ClusterLoadAssignment
  pub fn has_load_assignment(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(28)
    }
  }
  pub fn clear_load_assignment(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        28
      );
    }
  }
  pub fn load_assignment_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentView<'_>> {
    self.has_load_assignment().then(|| self.load_assignment())
  }
  pub fn load_assignment(&self) -> crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(28)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentView::default())
  }
  pub fn load_assignment_mut(&mut self) -> crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignmentMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         28, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_load_assignment(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignment>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        28,
        val
      );
    }
  }

  // health_checks: repeated message envoy.config.core.v3.HealthCheck
  pub fn health_checks(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn health_checks_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck> {
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
  pub fn set_health_checks(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // max_requests_per_connection: optional message google.protobuf.UInt32Value
  pub fn has_max_requests_per_connection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_max_requests_per_connection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn max_requests_per_connection_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_max_requests_per_connection().then(|| self.max_requests_per_connection())
  }
  pub fn max_requests_per_connection(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn max_requests_per_connection_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_max_requests_per_connection(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // circuit_breakers: optional message envoy.config.cluster.v3.CircuitBreakers
  pub fn has_circuit_breakers(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_circuit_breakers(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn circuit_breakers_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersView<'_>> {
    self.has_circuit_breakers().then(|| self.circuit_breakers())
  }
  pub fn circuit_breakers(&self) -> crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersView::default())
  }
  pub fn circuit_breakers_mut(&mut self) -> crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakersMut<'_> {
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
  pub fn set_circuit_breakers(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakers>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // upstream_http_protocol_options: optional message envoy.config.core.v3.UpstreamHttpProtocolOptions
  pub fn has_upstream_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(40)
    }
  }
  pub fn clear_upstream_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        40
      );
    }
  }
  pub fn upstream_http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsView<'_>> {
    self.has_upstream_http_protocol_options().then(|| self.upstream_http_protocol_options())
  }
  pub fn upstream_http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(40)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsView::default())
  }
  pub fn upstream_http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         40, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        40,
        val
      );
    }
  }

  // common_http_protocol_options: optional message envoy.config.core.v3.HttpProtocolOptions
  pub fn has_common_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_common_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn common_http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'_>> {
    self.has_common_http_protocol_options().then(|| self.common_http_protocol_options())
  }
  pub fn common_http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsView::default())
  }
  pub fn common_http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         24, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_common_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // http_protocol_options: optional message envoy.config.core.v3.Http1ProtocolOptions
  pub fn has_http_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_http_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn http_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'_>> {
    self.has_http_protocol_options().then(|| self.http_protocol_options())
  }
  pub fn http_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsView::default())
  }
  pub fn http_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptionsMut<'_> {
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
  pub fn set_http_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // http2_protocol_options: optional message envoy.config.core.v3.Http2ProtocolOptions
  pub fn has_http2_protocol_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_http2_protocol_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn http2_protocol_options_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'_>> {
    self.has_http2_protocol_options().then(|| self.http2_protocol_options())
  }
  pub fn http2_protocol_options(&self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsView::default())
  }
  pub fn http2_protocol_options_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptionsMut<'_> {
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
  pub fn set_http2_protocol_options(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // typed_extension_protocol_options: repeated message envoy.config.cluster.v3.Cluster.TypedExtensionProtocolOptionsEntry
  pub fn typed_extension_protocol_options(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(30)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, ::protobuf_well_known_types::Any>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn typed_extension_protocol_options_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, ::protobuf_well_known_types::Any> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          30, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_typed_extension_protocol_options(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, ::protobuf_well_known_types::Any>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        30,
        src);
    }
  }

  // dns_refresh_rate: optional message google.protobuf.Duration
  pub fn has_dns_refresh_rate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_dns_refresh_rate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn dns_refresh_rate_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_dns_refresh_rate().then(|| self.dns_refresh_rate())
  }
  pub fn dns_refresh_rate(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn dns_refresh_rate_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_dns_refresh_rate(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // dns_jitter: optional message google.protobuf.Duration
  pub fn has_dns_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(52)
    }
  }
  pub fn clear_dns_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        52
      );
    }
  }
  pub fn dns_jitter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_dns_jitter().then(|| self.dns_jitter())
  }
  pub fn dns_jitter(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(52)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn dns_jitter_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         52, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dns_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        52,
        val
      );
    }
  }

  // dns_failure_refresh_rate: optional message envoy.config.cluster.v3.Cluster.RefreshRate
  pub fn has_dns_failure_refresh_rate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(38)
    }
  }
  pub fn clear_dns_failure_refresh_rate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        38
      );
    }
  }
  pub fn dns_failure_refresh_rate_opt(&self) -> ::std::option::Option<super::cluster::RefreshRateView<'_>> {
    self.has_dns_failure_refresh_rate().then(|| self.dns_failure_refresh_rate())
  }
  pub fn dns_failure_refresh_rate(&self) -> super::cluster::RefreshRateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(38)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::RefreshRateView::default())
  }
  pub fn dns_failure_refresh_rate_mut(&mut self) -> super::cluster::RefreshRateMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         38, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dns_failure_refresh_rate(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::RefreshRate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        38,
        val
      );
    }
  }

  // respect_dns_ttl: optional bool
  pub fn respect_dns_ttl(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        33, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_respect_dns_ttl(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        33, val.into()
      )
    }
  }

  // dns_lookup_family: optional enum envoy.config.cluster.v3.Cluster.DnsLookupFamily
  pub fn dns_lookup_family(&self) -> super::cluster::DnsLookupFamily {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        12, (super::cluster::DnsLookupFamily::Auto).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_dns_lookup_family(&mut self, val: super::cluster::DnsLookupFamily) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        12, val.into()
      )
    }
  }

  // dns_resolvers: repeated message envoy.config.core.v3.Address
  pub fn dns_resolvers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        13
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::address::Address>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn dns_resolvers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::address::Address> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        13,
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
  pub fn set_dns_resolvers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::address::Address>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        src);
    }
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
        39, (false).into()
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
        39, val.into()
      )
    }
  }

  // dns_resolution_config: optional message envoy.config.core.v3.DnsResolutionConfig
  pub fn has_dns_resolution_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(47)
    }
  }
  pub fn clear_dns_resolution_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        47
      );
    }
  }
  pub fn dns_resolution_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'_>> {
    self.has_dns_resolution_config().then(|| self.dns_resolution_config())
  }
  pub fn dns_resolution_config(&self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(47)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView::default())
  }
  pub fn dns_resolution_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         47, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dns_resolution_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        47,
        val
      );
    }
  }

  // typed_dns_resolver_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_dns_resolver_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(49)
    }
  }
  pub fn clear_typed_dns_resolver_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        49
      );
    }
  }
  pub fn typed_dns_resolver_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_dns_resolver_config().then(|| self.typed_dns_resolver_config())
  }
  pub fn typed_dns_resolver_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(49)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_dns_resolver_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         49, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_typed_dns_resolver_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        49,
        val
      );
    }
  }

  // wait_for_warm_on_init: optional message google.protobuf.BoolValue
  pub fn has_wait_for_warm_on_init(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(48)
    }
  }
  pub fn clear_wait_for_warm_on_init(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        48
      );
    }
  }
  pub fn wait_for_warm_on_init_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_wait_for_warm_on_init().then(|| self.wait_for_warm_on_init())
  }
  pub fn wait_for_warm_on_init(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(48)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn wait_for_warm_on_init_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         48, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_wait_for_warm_on_init(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        48,
        val
      );
    }
  }

  // outlier_detection: optional message envoy.config.cluster.v3.OutlierDetection
  pub fn has_outlier_detection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_outlier_detection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn outlier_detection_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionView<'_>> {
    self.has_outlier_detection().then(|| self.outlier_detection())
  }
  pub fn outlier_detection(&self) -> crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionView::default())
  }
  pub fn outlier_detection_mut(&mut self) -> crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetectionMut<'_> {
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
  pub fn set_outlier_detection(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetection>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // cleanup_interval: optional message google.protobuf.Duration
  pub fn has_cleanup_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(15)
    }
  }
  pub fn clear_cleanup_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        15
      );
    }
  }
  pub fn cleanup_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_cleanup_interval().then(|| self.cleanup_interval())
  }
  pub fn cleanup_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(15)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn cleanup_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         15, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cleanup_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val
      );
    }
  }

  // upstream_bind_config: optional message envoy.config.core.v3.BindConfig
  pub fn has_upstream_bind_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_upstream_bind_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn upstream_bind_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'_>> {
    self.has_upstream_bind_config().then(|| self.upstream_bind_config())
  }
  pub fn upstream_bind_config(&self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::BindConfigView::default())
  }
  pub fn upstream_bind_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_bind_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::BindConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // lb_subset_config: optional message envoy.config.cluster.v3.Cluster.LbSubsetConfig
  pub fn has_lb_subset_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_lb_subset_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn lb_subset_config_opt(&self) -> ::std::option::Option<super::cluster::LbSubsetConfigView<'_>> {
    self.has_lb_subset_config().then(|| self.lb_subset_config())
  }
  pub fn lb_subset_config(&self) -> super::cluster::LbSubsetConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::LbSubsetConfigView::default())
  }
  pub fn lb_subset_config_mut(&mut self) -> super::cluster::LbSubsetConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_lb_subset_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::LbSubsetConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

  // ring_hash_lb_config: optional message envoy.config.cluster.v3.Cluster.RingHashLbConfig
  pub fn has_ring_hash_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(18)
    }
  }
  pub fn clear_ring_hash_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        18
      );
    }
  }
  pub fn ring_hash_lb_config_opt(&self) -> ::std::option::Option<super::cluster::RingHashLbConfigView<'_>> {
    self.has_ring_hash_lb_config().then(|| self.ring_hash_lb_config())
  }
  pub fn ring_hash_lb_config(&self) -> super::cluster::RingHashLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(18)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::RingHashLbConfigView::default())
  }
  pub fn ring_hash_lb_config_mut(&mut self) -> super::cluster::RingHashLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         18, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_ring_hash_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::RingHashLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        val
      );
    }
  }

  // maglev_lb_config: optional message envoy.config.cluster.v3.Cluster.MaglevLbConfig
  pub fn has_maglev_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(46)
    }
  }
  pub fn clear_maglev_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        46
      );
    }
  }
  pub fn maglev_lb_config_opt(&self) -> ::std::option::Option<super::cluster::MaglevLbConfigView<'_>> {
    self.has_maglev_lb_config().then(|| self.maglev_lb_config())
  }
  pub fn maglev_lb_config(&self) -> super::cluster::MaglevLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(46)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::MaglevLbConfigView::default())
  }
  pub fn maglev_lb_config_mut(&mut self) -> super::cluster::MaglevLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         46, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_maglev_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::MaglevLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        46,
        val
      );
    }
  }

  // original_dst_lb_config: optional message envoy.config.cluster.v3.Cluster.OriginalDstLbConfig
  pub fn has_original_dst_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(29)
    }
  }
  pub fn clear_original_dst_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        29
      );
    }
  }
  pub fn original_dst_lb_config_opt(&self) -> ::std::option::Option<super::cluster::OriginalDstLbConfigView<'_>> {
    self.has_original_dst_lb_config().then(|| self.original_dst_lb_config())
  }
  pub fn original_dst_lb_config(&self) -> super::cluster::OriginalDstLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(29)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::OriginalDstLbConfigView::default())
  }
  pub fn original_dst_lb_config_mut(&mut self) -> super::cluster::OriginalDstLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         29, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_original_dst_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::OriginalDstLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        29,
        val
      );
    }
  }

  // least_request_lb_config: optional message envoy.config.cluster.v3.Cluster.LeastRequestLbConfig
  pub fn has_least_request_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(31)
    }
  }
  pub fn clear_least_request_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        31
      );
    }
  }
  pub fn least_request_lb_config_opt(&self) -> ::std::option::Option<super::cluster::LeastRequestLbConfigView<'_>> {
    self.has_least_request_lb_config().then(|| self.least_request_lb_config())
  }
  pub fn least_request_lb_config(&self) -> super::cluster::LeastRequestLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(31)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::LeastRequestLbConfigView::default())
  }
  pub fn least_request_lb_config_mut(&mut self) -> super::cluster::LeastRequestLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         31, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_least_request_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::LeastRequestLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        31,
        val
      );
    }
  }

  // round_robin_lb_config: optional message envoy.config.cluster.v3.Cluster.RoundRobinLbConfig
  pub fn has_round_robin_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(50)
    }
  }
  pub fn clear_round_robin_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        50
      );
    }
  }
  pub fn round_robin_lb_config_opt(&self) -> ::std::option::Option<super::cluster::RoundRobinLbConfigView<'_>> {
    self.has_round_robin_lb_config().then(|| self.round_robin_lb_config())
  }
  pub fn round_robin_lb_config(&self) -> super::cluster::RoundRobinLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(50)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::RoundRobinLbConfigView::default())
  }
  pub fn round_robin_lb_config_mut(&mut self) -> super::cluster::RoundRobinLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         50, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_round_robin_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::RoundRobinLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        50,
        val
      );
    }
  }

  // common_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig
  pub fn has_common_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_common_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn common_lb_config_opt(&self) -> ::std::option::Option<super::cluster::CommonLbConfigView<'_>> {
    self.has_common_lb_config().then(|| self.common_lb_config())
  }
  pub fn common_lb_config(&self) -> super::cluster::CommonLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::CommonLbConfigView::default())
  }
  pub fn common_lb_config_mut(&mut self) -> super::cluster::CommonLbConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         22, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_common_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::CommonLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(19)
    }
  }
  pub fn clear_transport_socket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        19
      );
    }
  }
  pub fn transport_socket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(&self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(19)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }
  pub fn transport_socket_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         19, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_transport_socket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::TransportSocket>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        val
      );
    }
  }

  // metadata: optional message envoy.config.core.v3.Metadata
  pub fn has_metadata(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_metadata(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn metadata_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_>> {
    self.has_metadata().then(|| self.metadata())
  }
  pub fn metadata(&self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::MetadataView::default())
  }
  pub fn metadata_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::MetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         20, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_metadata(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Metadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val
      );
    }
  }

  // protocol_selection: optional enum envoy.config.cluster.v3.Cluster.ClusterProtocolSelection
  pub fn protocol_selection(&self) -> super::cluster::ClusterProtocolSelection {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        21, (super::cluster::ClusterProtocolSelection::UseConfiguredProtocol).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_protocol_selection(&mut self, val: super::cluster::ClusterProtocolSelection) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        21, val.into()
      )
    }
  }

  // upstream_connection_options: optional message envoy.config.cluster.v3.UpstreamConnectionOptions
  pub fn has_upstream_connection_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_upstream_connection_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn upstream_connection_options_opt(&self) -> ::std::option::Option<super::UpstreamConnectionOptionsView<'_>> {
    self.has_upstream_connection_options().then(|| self.upstream_connection_options())
  }
  pub fn upstream_connection_options(&self) -> super::UpstreamConnectionOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UpstreamConnectionOptionsView::default())
  }
  pub fn upstream_connection_options_mut(&mut self) -> super::UpstreamConnectionOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         25, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_connection_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::UpstreamConnectionOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val
      );
    }
  }

  // close_connections_on_host_health_failure: optional bool
  pub fn close_connections_on_host_health_failure(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_close_connections_on_host_health_failure(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // ignore_health_on_host_removal: optional bool
  pub fn ignore_health_on_host_removal(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        27, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_health_on_host_removal(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        27, val.into()
      )
    }
  }

  // filters: repeated message envoy.config.cluster.v3.Filter
  pub fn filters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::cluster::v3::filter::Filter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        34
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::cluster::v3::filter::Filter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::cluster::v3::filter::Filter> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        34,
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
  pub fn set_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::cluster::v3::filter::Filter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        34,
        src);
    }
  }

  // load_balancing_policy: optional message envoy.config.cluster.v3.LoadBalancingPolicy
  pub fn has_load_balancing_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(35)
    }
  }
  pub fn clear_load_balancing_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        35
      );
    }
  }
  pub fn load_balancing_policy_opt(&self) -> ::std::option::Option<super::LoadBalancingPolicyView<'_>> {
    self.has_load_balancing_policy().then(|| self.load_balancing_policy())
  }
  pub fn load_balancing_policy(&self) -> super::LoadBalancingPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(35)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LoadBalancingPolicyView::default())
  }
  pub fn load_balancing_policy_mut(&mut self) -> super::LoadBalancingPolicyMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         35, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_load_balancing_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::LoadBalancingPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        35,
        val
      );
    }
  }

  // lrs_server: optional message envoy.config.core.v3.ConfigSource
  pub fn has_lrs_server(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(36)
    }
  }
  pub fn clear_lrs_server(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        36
      );
    }
  }
  pub fn lrs_server_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_lrs_server().then(|| self.lrs_server())
  }
  pub fn lrs_server(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(36)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn lrs_server_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         36, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_lrs_server(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        36,
        val
      );
    }
  }

  // lrs_report_endpoint_metrics: repeated string
  pub fn lrs_report_endpoint_metrics(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        51
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn lrs_report_endpoint_metrics_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        51,
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
  pub fn set_lrs_report_endpoint_metrics(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        51,
        src);
    }
  }

  // track_timeout_budgets: optional bool
  pub fn track_timeout_budgets(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        41, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_track_timeout_budgets(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        41, val.into()
      )
    }
  }

  // upstream_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_upstream_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(42)
    }
  }
  pub fn clear_upstream_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        42
      );
    }
  }
  pub fn upstream_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_upstream_config().then(|| self.upstream_config())
  }
  pub fn upstream_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(42)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn upstream_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         42, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        42,
        val
      );
    }
  }

  // track_cluster_stats: optional message envoy.config.cluster.v3.TrackClusterStats
  pub fn has_track_cluster_stats(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(43)
    }
  }
  pub fn clear_track_cluster_stats(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        43
      );
    }
  }
  pub fn track_cluster_stats_opt(&self) -> ::std::option::Option<super::TrackClusterStatsView<'_>> {
    self.has_track_cluster_stats().then(|| self.track_cluster_stats())
  }
  pub fn track_cluster_stats(&self) -> super::TrackClusterStatsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(43)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TrackClusterStatsView::default())
  }
  pub fn track_cluster_stats_mut(&mut self) -> super::TrackClusterStatsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         43, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_track_cluster_stats(&mut self,
    val: impl ::protobuf::IntoProxied<super::TrackClusterStats>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        43,
        val
      );
    }
  }

  // preconnect_policy: optional message envoy.config.cluster.v3.Cluster.PreconnectPolicy
  pub fn has_preconnect_policy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(44)
    }
  }
  pub fn clear_preconnect_policy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        44
      );
    }
  }
  pub fn preconnect_policy_opt(&self) -> ::std::option::Option<super::cluster::PreconnectPolicyView<'_>> {
    self.has_preconnect_policy().then(|| self.preconnect_policy())
  }
  pub fn preconnect_policy(&self) -> super::cluster::PreconnectPolicyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(44)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster::PreconnectPolicyView::default())
  }
  pub fn preconnect_policy_mut(&mut self) -> super::cluster::PreconnectPolicyMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         44, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_preconnect_policy(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster::PreconnectPolicy>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        44,
        val
      );
    }
  }

  // connection_pool_per_downstream_connection: optional bool
  pub fn connection_pool_per_downstream_connection(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        45, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_connection_pool_per_downstream_connection(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        45, val.into()
      )
    }
  }

  pub fn cluster_discovery_type(&self) -> super::cluster::ClusterDiscoveryTypeOneof<'_> {
    match &self.cluster_discovery_type_case() {
      super::cluster::ClusterDiscoveryTypeCase::Type =>
          super::cluster::ClusterDiscoveryTypeOneof::Type(self.r#type()),
      super::cluster::ClusterDiscoveryTypeCase::ClusterType =>
          super::cluster::ClusterDiscoveryTypeOneof::ClusterType(self.cluster_type()),
      _ => super::cluster::ClusterDiscoveryTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn cluster_discovery_type_case(&self) -> super::cluster::ClusterDiscoveryTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::cluster::ClusterDiscoveryTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
  pub fn lb_config(&self) -> super::cluster::LbConfigOneof<'_> {
    match &self.lb_config_case() {
      super::cluster::LbConfigCase::RingHashLbConfig =>
          super::cluster::LbConfigOneof::RingHashLbConfig(self.ring_hash_lb_config()),
      super::cluster::LbConfigCase::MaglevLbConfig =>
          super::cluster::LbConfigOneof::MaglevLbConfig(self.maglev_lb_config()),
      super::cluster::LbConfigCase::OriginalDstLbConfig =>
          super::cluster::LbConfigOneof::OriginalDstLbConfig(self.original_dst_lb_config()),
      super::cluster::LbConfigCase::LeastRequestLbConfig =>
          super::cluster::LbConfigOneof::LeastRequestLbConfig(self.least_request_lb_config()),
      super::cluster::LbConfigCase::RoundRobinLbConfig =>
          super::cluster::LbConfigOneof::RoundRobinLbConfig(self.round_robin_lb_config()),
      _ => super::cluster::LbConfigOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn lb_config_case(&self) -> super::cluster::LbConfigCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(18);
      super::cluster::LbConfigCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Cluster

impl ::std::ops::Drop for Cluster {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Cluster {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Cluster {
  type Proxied = Self;
  fn as_view(&self) -> ClusterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Cluster {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClusterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Cluster {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__cluster__v3__Cluster_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.333.PaG33b33a3.PG3333333.P31X33/P/P33aG33/PG33G3/P3/P333/P33333ET33^#|H~9|V|D|G|Z");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__cluster__v3__Cluster_msg_init.0, &[<super::cluster::EdsClusterConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::health_check::HealthCheck as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::cluster::v3::circuit_breaker::CircuitBreakers as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::protocol::Http1ProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::protocol::Http2ProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::cluster::v3::outlier_detection::OutlierDetection as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::BindConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::LbSubsetConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::RingHashLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::TransportSocket as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::Metadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::CommonLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::protocol::HttpProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::UpstreamConnectionOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::endpoint::v3::endpoint::ClusterLoadAssignment as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::OriginalDstLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::TypedExtensionProtocolOptionsEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::LeastRequestLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::CustomClusterType as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::cluster::v3::filter::Filter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::LoadBalancingPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::TransportSocketMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::RefreshRate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::protocol::UpstreamHttpProtocolOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TrackClusterStats as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::PreconnectPolicy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::MaglevLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::cluster::RoundRobinLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__cluster__v3__Cluster_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Cluster {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Cluster {
  type Msg = Cluster;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Cluster {
  type Msg = Cluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterMut<'_> {
  type Msg = Cluster;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterMut<'_> {
  type Msg = Cluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterView<'_> {
  type Msg = Cluster;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Cluster> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod cluster {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__TransportSocketMatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TransportSocketMatch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TransportSocketMatch>
}

impl ::protobuf::Message for TransportSocketMatch {
  type MessageView<'msg> = TransportSocketMatchView<'msg>;
  type MessageMut<'msg> = TransportSocketMatchMut<'msg>;
}

impl ::std::default::Default for TransportSocketMatch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TransportSocketMatch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TransportSocketMatch` is `Sync` because it does not implement interior mutability.
//    Neither does `TransportSocketMatchMut`.
unsafe impl ::std::marker::Sync for TransportSocketMatch {}

// SAFETY:
// - `TransportSocketMatch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TransportSocketMatch {}

impl ::protobuf::Proxied for TransportSocketMatch {
  type View<'msg> = TransportSocketMatchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TransportSocketMatch {}

impl ::protobuf::MutProxied for TransportSocketMatch {
  type Mut<'msg> = TransportSocketMatchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TransportSocketMatchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TransportSocketMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TransportSocketMatchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TransportSocketMatchView<'msg> {
  type Message = TransportSocketMatch;
}

impl ::std::fmt::Debug for TransportSocketMatchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TransportSocketMatchView<'_> {
  fn default() -> TransportSocketMatchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TransportSocketMatch>> for TransportSocketMatchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TransportSocketMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TransportSocketMatchView<'msg> {

  pub fn to_owned(&self) -> TransportSocketMatch {
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

  // match: optional message google.protobuf.Struct
  pub fn has_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn match_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_match().then(|| self.r#match())
  }
  pub fn r#match(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn transport_socket_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'msg>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }

}

// SAFETY:
// - `TransportSocketMatchView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TransportSocketMatchView<'_> {}

// SAFETY:
// - `TransportSocketMatchView` is `Send` because while its alive a `TransportSocketMatchMut` cannot.
// - `TransportSocketMatchView` does not use thread-local data.
unsafe impl ::std::marker::Send for TransportSocketMatchView<'_> {}

impl<'msg> ::protobuf::AsView for TransportSocketMatchView<'msg> {
  type Proxied = TransportSocketMatch;
  fn as_view(&self) -> ::protobuf::View<'msg, TransportSocketMatch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TransportSocketMatchView<'msg> {
  fn into_view<'shorter>(self) -> TransportSocketMatchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TransportSocketMatch> for TransportSocketMatchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TransportSocketMatch {
    let mut dst = TransportSocketMatch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TransportSocketMatch> for TransportSocketMatchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TransportSocketMatch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TransportSocketMatch {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TransportSocketMatchView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TransportSocketMatchMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TransportSocketMatchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TransportSocketMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TransportSocketMatchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TransportSocketMatchMut<'msg> {
  type Message = TransportSocketMatch;
}

impl ::std::fmt::Debug for TransportSocketMatchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TransportSocketMatch>> for TransportSocketMatchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TransportSocketMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TransportSocketMatchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TransportSocketMatch> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TransportSocketMatch {
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

  // match: optional message google.protobuf.Struct
  pub fn has_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn match_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_match().then(|| self.r#match())
  }
  pub fn r#match(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn match_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_match(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_transport_socket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn transport_socket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(&self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }
  pub fn transport_socket_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketMut<'_> {
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
  pub fn set_transport_socket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::TransportSocket>) {

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
// - `TransportSocketMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TransportSocketMatchMut<'_> {}

// SAFETY:
// - `TransportSocketMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TransportSocketMatchMut<'_> {}

impl<'msg> ::protobuf::AsView for TransportSocketMatchMut<'msg> {
  type Proxied = TransportSocketMatch;
  fn as_view(&self) -> ::protobuf::View<'_, TransportSocketMatch> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TransportSocketMatchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TransportSocketMatch>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TransportSocketMatchMut<'msg> {
  type MutProxied = TransportSocketMatch;
  fn as_mut(&mut self) -> TransportSocketMatchMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TransportSocketMatchMut<'msg> {
  fn into_mut<'shorter>(self) -> TransportSocketMatchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TransportSocketMatch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TransportSocketMatch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TransportSocketMatchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TransportSocketMatchMut<'_> {
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

  // match: optional message google.protobuf.Struct
  pub fn has_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn match_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_match().then(|| self.r#match())
  }
  pub fn r#match(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn match_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_match(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // transport_socket: optional message envoy.config.core.v3.TransportSocket
  pub fn has_transport_socket(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_transport_socket(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn transport_socket_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_>> {
    self.has_transport_socket().then(|| self.transport_socket())
  }
  pub fn transport_socket(&self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::TransportSocketView::default())
  }
  pub fn transport_socket_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::TransportSocketMut<'_> {
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
  pub fn set_transport_socket(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::TransportSocket>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl TransportSocketMatch

impl ::std::ops::Drop for TransportSocketMatch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TransportSocketMatch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TransportSocketMatch {
  type Proxied = Self;
  fn as_view(&self) -> TransportSocketMatchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TransportSocketMatch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TransportSocketMatchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TransportSocketMatch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__TransportSocketMatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__TransportSocketMatch_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::TransportSocket as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__TransportSocketMatch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TransportSocketMatch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TransportSocketMatch {
  type Msg = TransportSocketMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocketMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TransportSocketMatch {
  type Msg = TransportSocketMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocketMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TransportSocketMatchMut<'_> {
  type Msg = TransportSocketMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocketMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TransportSocketMatchMut<'_> {
  type Msg = TransportSocketMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocketMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TransportSocketMatchView<'_> {
  type Msg = TransportSocketMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TransportSocketMatch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TransportSocketMatchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__CustomClusterType_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CustomClusterType {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CustomClusterType>
}

impl ::protobuf::Message for CustomClusterType {
  type MessageView<'msg> = CustomClusterTypeView<'msg>;
  type MessageMut<'msg> = CustomClusterTypeMut<'msg>;
}

impl ::std::default::Default for CustomClusterType {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CustomClusterType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CustomClusterType` is `Sync` because it does not implement interior mutability.
//    Neither does `CustomClusterTypeMut`.
unsafe impl ::std::marker::Sync for CustomClusterType {}

// SAFETY:
// - `CustomClusterType` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CustomClusterType {}

impl ::protobuf::Proxied for CustomClusterType {
  type View<'msg> = CustomClusterTypeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CustomClusterType {}

impl ::protobuf::MutProxied for CustomClusterType {
  type Mut<'msg> = CustomClusterTypeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CustomClusterTypeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomClusterType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomClusterTypeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CustomClusterTypeView<'msg> {
  type Message = CustomClusterType;
}

impl ::std::fmt::Debug for CustomClusterTypeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CustomClusterTypeView<'_> {
  fn default() -> CustomClusterTypeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CustomClusterType>> for CustomClusterTypeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomClusterType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomClusterTypeView<'msg> {

  pub fn to_owned(&self) -> CustomClusterType {
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

}

// SAFETY:
// - `CustomClusterTypeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CustomClusterTypeView<'_> {}

// SAFETY:
// - `CustomClusterTypeView` is `Send` because while its alive a `CustomClusterTypeMut` cannot.
// - `CustomClusterTypeView` does not use thread-local data.
unsafe impl ::std::marker::Send for CustomClusterTypeView<'_> {}

impl<'msg> ::protobuf::AsView for CustomClusterTypeView<'msg> {
  type Proxied = CustomClusterType;
  fn as_view(&self) -> ::protobuf::View<'msg, CustomClusterType> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomClusterTypeView<'msg> {
  fn into_view<'shorter>(self) -> CustomClusterTypeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomClusterType> for CustomClusterTypeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomClusterType {
    let mut dst = CustomClusterType::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomClusterType> for CustomClusterTypeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomClusterType {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CustomClusterType {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomClusterTypeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomClusterTypeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CustomClusterTypeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomClusterType>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomClusterTypeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CustomClusterTypeMut<'msg> {
  type Message = CustomClusterType;
}

impl ::std::fmt::Debug for CustomClusterTypeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CustomClusterType>> for CustomClusterTypeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomClusterType>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomClusterTypeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomClusterType> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CustomClusterType {
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

}

// SAFETY:
// - `CustomClusterTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CustomClusterTypeMut<'_> {}

// SAFETY:
// - `CustomClusterTypeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CustomClusterTypeMut<'_> {}

impl<'msg> ::protobuf::AsView for CustomClusterTypeMut<'msg> {
  type Proxied = CustomClusterType;
  fn as_view(&self) -> ::protobuf::View<'_, CustomClusterType> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomClusterTypeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CustomClusterType>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CustomClusterTypeMut<'msg> {
  type MutProxied = CustomClusterType;
  fn as_mut(&mut self) -> CustomClusterTypeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CustomClusterTypeMut<'msg> {
  fn into_mut<'shorter>(self) -> CustomClusterTypeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CustomClusterType {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CustomClusterType> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CustomClusterTypeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CustomClusterTypeMut<'_> {
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

}  // impl CustomClusterType

impl ::std::ops::Drop for CustomClusterType {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CustomClusterType {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CustomClusterType {
  type Proxied = Self;
  fn as_view(&self) -> CustomClusterTypeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CustomClusterType {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CustomClusterTypeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CustomClusterType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__CustomClusterType_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__CustomClusterType_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__CustomClusterType_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomClusterType {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomClusterType {
  type Msg = CustomClusterType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomClusterType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomClusterType {
  type Msg = CustomClusterType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomClusterType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomClusterTypeMut<'_> {
  type Msg = CustomClusterType;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomClusterType> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomClusterTypeMut<'_> {
  type Msg = CustomClusterType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomClusterType> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomClusterTypeView<'_> {
  type Msg = CustomClusterType;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomClusterType> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomClusterTypeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__EdsClusterConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EdsClusterConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EdsClusterConfig>
}

impl ::protobuf::Message for EdsClusterConfig {
  type MessageView<'msg> = EdsClusterConfigView<'msg>;
  type MessageMut<'msg> = EdsClusterConfigMut<'msg>;
}

impl ::std::default::Default for EdsClusterConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EdsClusterConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EdsClusterConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `EdsClusterConfigMut`.
unsafe impl ::std::marker::Sync for EdsClusterConfig {}

// SAFETY:
// - `EdsClusterConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EdsClusterConfig {}

impl ::protobuf::Proxied for EdsClusterConfig {
  type View<'msg> = EdsClusterConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EdsClusterConfig {}

impl ::protobuf::MutProxied for EdsClusterConfig {
  type Mut<'msg> = EdsClusterConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EdsClusterConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EdsClusterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EdsClusterConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EdsClusterConfigView<'msg> {
  type Message = EdsClusterConfig;
}

impl ::std::fmt::Debug for EdsClusterConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EdsClusterConfigView<'_> {
  fn default() -> EdsClusterConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EdsClusterConfig>> for EdsClusterConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EdsClusterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EdsClusterConfigView<'msg> {

  pub fn to_owned(&self) -> EdsClusterConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // eds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_eds_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn eds_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_eds_config().then(|| self.eds_config())
  }
  pub fn eds_config(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // service_name: optional string
  pub fn service_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `EdsClusterConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EdsClusterConfigView<'_> {}

// SAFETY:
// - `EdsClusterConfigView` is `Send` because while its alive a `EdsClusterConfigMut` cannot.
// - `EdsClusterConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for EdsClusterConfigView<'_> {}

impl<'msg> ::protobuf::AsView for EdsClusterConfigView<'msg> {
  type Proxied = EdsClusterConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, EdsClusterConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EdsClusterConfigView<'msg> {
  fn into_view<'shorter>(self) -> EdsClusterConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EdsClusterConfig> for EdsClusterConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EdsClusterConfig {
    let mut dst = EdsClusterConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EdsClusterConfig> for EdsClusterConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EdsClusterConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EdsClusterConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EdsClusterConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EdsClusterConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EdsClusterConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EdsClusterConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EdsClusterConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EdsClusterConfigMut<'msg> {
  type Message = EdsClusterConfig;
}

impl ::std::fmt::Debug for EdsClusterConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EdsClusterConfig>> for EdsClusterConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EdsClusterConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EdsClusterConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EdsClusterConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EdsClusterConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // eds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_eds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_eds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn eds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_eds_config().then(|| self.eds_config())
  }
  pub fn eds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn eds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_eds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // service_name: optional string
  pub fn service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `EdsClusterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EdsClusterConfigMut<'_> {}

// SAFETY:
// - `EdsClusterConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EdsClusterConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for EdsClusterConfigMut<'msg> {
  type Proxied = EdsClusterConfig;
  fn as_view(&self) -> ::protobuf::View<'_, EdsClusterConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EdsClusterConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EdsClusterConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EdsClusterConfigMut<'msg> {
  type MutProxied = EdsClusterConfig;
  fn as_mut(&mut self) -> EdsClusterConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EdsClusterConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> EdsClusterConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EdsClusterConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EdsClusterConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EdsClusterConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EdsClusterConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // eds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_eds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_eds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn eds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_eds_config().then(|| self.eds_config())
  }
  pub fn eds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn eds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
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
  pub fn set_eds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // service_name: optional string
  pub fn service_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl EdsClusterConfig

impl ::std::ops::Drop for EdsClusterConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EdsClusterConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EdsClusterConfig {
  type Proxied = Self;
  fn as_view(&self) -> EdsClusterConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EdsClusterConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EdsClusterConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EdsClusterConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__EdsClusterConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__EdsClusterConfig_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__EdsClusterConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EdsClusterConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EdsClusterConfig {
  type Msg = EdsClusterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EdsClusterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EdsClusterConfig {
  type Msg = EdsClusterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EdsClusterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EdsClusterConfigMut<'_> {
  type Msg = EdsClusterConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EdsClusterConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EdsClusterConfigMut<'_> {
  type Msg = EdsClusterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EdsClusterConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EdsClusterConfigView<'_> {
  type Msg = EdsClusterConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EdsClusterConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EdsClusterConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__LbSubsetConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LbSubsetConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LbSubsetConfig>
}

impl ::protobuf::Message for LbSubsetConfig {
  type MessageView<'msg> = LbSubsetConfigView<'msg>;
  type MessageMut<'msg> = LbSubsetConfigMut<'msg>;
}

impl ::std::default::Default for LbSubsetConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LbSubsetConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LbSubsetConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `LbSubsetConfigMut`.
unsafe impl ::std::marker::Sync for LbSubsetConfig {}

// SAFETY:
// - `LbSubsetConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LbSubsetConfig {}

impl ::protobuf::Proxied for LbSubsetConfig {
  type View<'msg> = LbSubsetConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LbSubsetConfig {}

impl ::protobuf::MutProxied for LbSubsetConfig {
  type Mut<'msg> = LbSubsetConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LbSubsetConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbSubsetConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbSubsetConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LbSubsetConfigView<'msg> {
  type Message = LbSubsetConfig;
}

impl ::std::fmt::Debug for LbSubsetConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LbSubsetConfigView<'_> {
  fn default() -> LbSubsetConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LbSubsetConfig>> for LbSubsetConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbSubsetConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbSubsetConfigView<'msg> {

  pub fn to_owned(&self) -> LbSubsetConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // fallback_policy: optional enum envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetFallbackPolicy
  pub fn fallback_policy(self) -> super::super::cluster::lb_subset_config::LbSubsetFallbackPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::cluster::lb_subset_config::LbSubsetFallbackPolicy::NoFallback).into()
      ).try_into().unwrap()
    }
  }

  // default_subset: optional message google.protobuf.Struct
  pub fn has_default_subset(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn default_subset_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_default_subset().then(|| self.default_subset())
  }
  pub fn default_subset(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // subset_selectors: repeated message envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector
  pub fn subset_selectors(self) -> ::protobuf::RepeatedView<'msg, super::super::cluster::lb_subset_config::LbSubsetSelector> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::cluster::lb_subset_config::LbSubsetSelector>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // locality_weight_aware: optional bool
  pub fn locality_weight_aware(self) -> bool {
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

  // scale_locality_weight: optional bool
  pub fn scale_locality_weight(self) -> bool {
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

  // panic_mode_any: optional bool
  pub fn panic_mode_any(self) -> bool {
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

  // list_as_any: optional bool
  pub fn list_as_any(self) -> bool {
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

  // metadata_fallback_policy: optional enum envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetMetadataFallbackPolicy
  pub fn metadata_fallback_policy(self) -> super::super::cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::super::cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy::MetadataNoFallback).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `LbSubsetConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LbSubsetConfigView<'_> {}

// SAFETY:
// - `LbSubsetConfigView` is `Send` because while its alive a `LbSubsetConfigMut` cannot.
// - `LbSubsetConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for LbSubsetConfigView<'_> {}

impl<'msg> ::protobuf::AsView for LbSubsetConfigView<'msg> {
  type Proxied = LbSubsetConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, LbSubsetConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbSubsetConfigView<'msg> {
  fn into_view<'shorter>(self) -> LbSubsetConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LbSubsetConfig> for LbSubsetConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbSubsetConfig {
    let mut dst = LbSubsetConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LbSubsetConfig> for LbSubsetConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbSubsetConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LbSubsetConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbSubsetConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbSubsetConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LbSubsetConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbSubsetConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbSubsetConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LbSubsetConfigMut<'msg> {
  type Message = LbSubsetConfig;
}

impl ::std::fmt::Debug for LbSubsetConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LbSubsetConfig>> for LbSubsetConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbSubsetConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbSubsetConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LbSubsetConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LbSubsetConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // fallback_policy: optional enum envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetFallbackPolicy
  pub fn fallback_policy(&self) -> super::super::cluster::lb_subset_config::LbSubsetFallbackPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::cluster::lb_subset_config::LbSubsetFallbackPolicy::NoFallback).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fallback_policy(&mut self, val: super::super::cluster::lb_subset_config::LbSubsetFallbackPolicy) {
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

  // default_subset: optional message google.protobuf.Struct
  pub fn has_default_subset(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_default_subset(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn default_subset_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_default_subset().then(|| self.default_subset())
  }
  pub fn default_subset(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn default_subset_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_default_subset(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // subset_selectors: repeated message envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector
  pub fn subset_selectors(&self) -> ::protobuf::RepeatedView<'_, super::super::cluster::lb_subset_config::LbSubsetSelector> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::cluster::lb_subset_config::LbSubsetSelector>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn subset_selectors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::cluster::lb_subset_config::LbSubsetSelector> {
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
  pub fn set_subset_selectors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::cluster::lb_subset_config::LbSubsetSelector>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // locality_weight_aware: optional bool
  pub fn locality_weight_aware(&self) -> bool {
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
  pub fn set_locality_weight_aware(&mut self, val: bool) {
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

  // scale_locality_weight: optional bool
  pub fn scale_locality_weight(&self) -> bool {
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
  pub fn set_scale_locality_weight(&mut self, val: bool) {
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

  // panic_mode_any: optional bool
  pub fn panic_mode_any(&self) -> bool {
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
  pub fn set_panic_mode_any(&mut self, val: bool) {
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

  // list_as_any: optional bool
  pub fn list_as_any(&self) -> bool {
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
  pub fn set_list_as_any(&mut self, val: bool) {
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

  // metadata_fallback_policy: optional enum envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetMetadataFallbackPolicy
  pub fn metadata_fallback_policy(&self) -> super::super::cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::super::cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy::MetadataNoFallback).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_metadata_fallback_policy(&mut self, val: super::super::cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy) {
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

}

// SAFETY:
// - `LbSubsetConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LbSubsetConfigMut<'_> {}

// SAFETY:
// - `LbSubsetConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LbSubsetConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for LbSubsetConfigMut<'msg> {
  type Proxied = LbSubsetConfig;
  fn as_view(&self) -> ::protobuf::View<'_, LbSubsetConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbSubsetConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LbSubsetConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LbSubsetConfigMut<'msg> {
  type MutProxied = LbSubsetConfig;
  fn as_mut(&mut self) -> LbSubsetConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LbSubsetConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> LbSubsetConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LbSubsetConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LbSubsetConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LbSubsetConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LbSubsetConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // fallback_policy: optional enum envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetFallbackPolicy
  pub fn fallback_policy(&self) -> super::super::cluster::lb_subset_config::LbSubsetFallbackPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::cluster::lb_subset_config::LbSubsetFallbackPolicy::NoFallback).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fallback_policy(&mut self, val: super::super::cluster::lb_subset_config::LbSubsetFallbackPolicy) {
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

  // default_subset: optional message google.protobuf.Struct
  pub fn has_default_subset(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_default_subset(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn default_subset_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_default_subset().then(|| self.default_subset())
  }
  pub fn default_subset(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn default_subset_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
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
  pub fn set_default_subset(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // subset_selectors: repeated message envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector
  pub fn subset_selectors(&self) -> ::protobuf::RepeatedView<'_, super::super::cluster::lb_subset_config::LbSubsetSelector> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::cluster::lb_subset_config::LbSubsetSelector>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn subset_selectors_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::cluster::lb_subset_config::LbSubsetSelector> {
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
  pub fn set_subset_selectors(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::cluster::lb_subset_config::LbSubsetSelector>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // locality_weight_aware: optional bool
  pub fn locality_weight_aware(&self) -> bool {
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
  pub fn set_locality_weight_aware(&mut self, val: bool) {
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

  // scale_locality_weight: optional bool
  pub fn scale_locality_weight(&self) -> bool {
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
  pub fn set_scale_locality_weight(&mut self, val: bool) {
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

  // panic_mode_any: optional bool
  pub fn panic_mode_any(&self) -> bool {
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
  pub fn set_panic_mode_any(&mut self, val: bool) {
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

  // list_as_any: optional bool
  pub fn list_as_any(&self) -> bool {
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
  pub fn set_list_as_any(&mut self, val: bool) {
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

  // metadata_fallback_policy: optional enum envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetMetadataFallbackPolicy
  pub fn metadata_fallback_policy(&self) -> super::super::cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        7, (super::super::cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy::MetadataNoFallback).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_metadata_fallback_policy(&mut self, val: super::super::cluster::lb_subset_config::LbSubsetMetadataFallbackPolicy) {
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

}  // impl LbSubsetConfig

impl ::std::ops::Drop for LbSubsetConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LbSubsetConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LbSubsetConfig {
  type Proxied = Self;
  fn as_view(&self) -> LbSubsetConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LbSubsetConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LbSubsetConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LbSubsetConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__LbSubsetConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P3G/P/P/P/P.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__LbSubsetConfig_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::cluster::lb_subset_config::LbSubsetSelector as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__LbSubsetConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbSubsetConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbSubsetConfig {
  type Msg = LbSubsetConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbSubsetConfig {
  type Msg = LbSubsetConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbSubsetConfigMut<'_> {
  type Msg = LbSubsetConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbSubsetConfigMut<'_> {
  type Msg = LbSubsetConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbSubsetConfigView<'_> {
  type Msg = LbSubsetConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbSubsetConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod lb_subset_config {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__LbSubsetConfig__LbSubsetSelector_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LbSubsetSelector {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LbSubsetSelector>
}

impl ::protobuf::Message for LbSubsetSelector {
  type MessageView<'msg> = LbSubsetSelectorView<'msg>;
  type MessageMut<'msg> = LbSubsetSelectorMut<'msg>;
}

impl ::std::default::Default for LbSubsetSelector {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LbSubsetSelector {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LbSubsetSelector` is `Sync` because it does not implement interior mutability.
//    Neither does `LbSubsetSelectorMut`.
unsafe impl ::std::marker::Sync for LbSubsetSelector {}

// SAFETY:
// - `LbSubsetSelector` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LbSubsetSelector {}

impl ::protobuf::Proxied for LbSubsetSelector {
  type View<'msg> = LbSubsetSelectorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LbSubsetSelector {}

impl ::protobuf::MutProxied for LbSubsetSelector {
  type Mut<'msg> = LbSubsetSelectorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LbSubsetSelectorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbSubsetSelector>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbSubsetSelectorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LbSubsetSelectorView<'msg> {
  type Message = LbSubsetSelector;
}

impl ::std::fmt::Debug for LbSubsetSelectorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LbSubsetSelectorView<'_> {
  fn default() -> LbSubsetSelectorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LbSubsetSelector>> for LbSubsetSelectorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LbSubsetSelector>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbSubsetSelectorView<'msg> {

  pub fn to_owned(&self) -> LbSubsetSelector {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // keys: repeated string
  pub fn keys(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // single_host_per_subset: optional bool
  pub fn single_host_per_subset(self) -> bool {
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

  // fallback_policy: optional enum envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector.LbSubsetSelectorFallbackPolicy
  pub fn fallback_policy(self) -> super::super::super::cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::super::super::cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::NotDefined).into()
      ).try_into().unwrap()
    }
  }

  // fallback_keys_subset: repeated string
  pub fn fallback_keys_subset(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

}

// SAFETY:
// - `LbSubsetSelectorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LbSubsetSelectorView<'_> {}

// SAFETY:
// - `LbSubsetSelectorView` is `Send` because while its alive a `LbSubsetSelectorMut` cannot.
// - `LbSubsetSelectorView` does not use thread-local data.
unsafe impl ::std::marker::Send for LbSubsetSelectorView<'_> {}

impl<'msg> ::protobuf::AsView for LbSubsetSelectorView<'msg> {
  type Proxied = LbSubsetSelector;
  fn as_view(&self) -> ::protobuf::View<'msg, LbSubsetSelector> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbSubsetSelectorView<'msg> {
  fn into_view<'shorter>(self) -> LbSubsetSelectorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LbSubsetSelector> for LbSubsetSelectorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbSubsetSelector {
    let mut dst = LbSubsetSelector::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LbSubsetSelector> for LbSubsetSelectorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LbSubsetSelector {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LbSubsetSelector {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbSubsetSelectorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LbSubsetSelectorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LbSubsetSelectorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbSubsetSelector>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LbSubsetSelectorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LbSubsetSelectorMut<'msg> {
  type Message = LbSubsetSelector;
}

impl ::std::fmt::Debug for LbSubsetSelectorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LbSubsetSelector>> for LbSubsetSelectorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LbSubsetSelector>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LbSubsetSelectorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LbSubsetSelector> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LbSubsetSelector {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // keys: repeated string
  pub fn keys(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn keys_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_keys(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // single_host_per_subset: optional bool
  pub fn single_host_per_subset(&self) -> bool {
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
  pub fn set_single_host_per_subset(&mut self, val: bool) {
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

  // fallback_policy: optional enum envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector.LbSubsetSelectorFallbackPolicy
  pub fn fallback_policy(&self) -> super::super::super::cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::super::super::cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::NotDefined).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fallback_policy(&mut self, val: super::super::super::cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy) {
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

  // fallback_keys_subset: repeated string
  pub fn fallback_keys_subset(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn fallback_keys_subset_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_fallback_keys_subset(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `LbSubsetSelectorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LbSubsetSelectorMut<'_> {}

// SAFETY:
// - `LbSubsetSelectorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LbSubsetSelectorMut<'_> {}

impl<'msg> ::protobuf::AsView for LbSubsetSelectorMut<'msg> {
  type Proxied = LbSubsetSelector;
  fn as_view(&self) -> ::protobuf::View<'_, LbSubsetSelector> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbSubsetSelectorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LbSubsetSelector>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LbSubsetSelectorMut<'msg> {
  type MutProxied = LbSubsetSelector;
  fn as_mut(&mut self) -> LbSubsetSelectorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LbSubsetSelectorMut<'msg> {
  fn into_mut<'shorter>(self) -> LbSubsetSelectorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LbSubsetSelector {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LbSubsetSelector> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LbSubsetSelectorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LbSubsetSelectorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // keys: repeated string
  pub fn keys(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn keys_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_keys(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // single_host_per_subset: optional bool
  pub fn single_host_per_subset(&self) -> bool {
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
  pub fn set_single_host_per_subset(&mut self, val: bool) {
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

  // fallback_policy: optional enum envoy.config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector.LbSubsetSelectorFallbackPolicy
  pub fn fallback_policy(&self) -> super::super::super::cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::super::super::cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::NotDefined).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fallback_policy(&mut self, val: super::super::super::cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy) {
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

  // fallback_keys_subset: repeated string
  pub fn fallback_keys_subset(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn fallback_keys_subset_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_fallback_keys_subset(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl LbSubsetSelector

impl ::std::ops::Drop for LbSubsetSelector {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LbSubsetSelector {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LbSubsetSelector {
  type Proxied = Self;
  fn as_view(&self) -> LbSubsetSelectorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LbSubsetSelector {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LbSubsetSelectorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LbSubsetSelector {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::cluster::lb_subset_config::envoy__config__cluster__v3__Cluster__LbSubsetConfig__LbSubsetSelector_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ET.PET/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::cluster::lb_subset_config::envoy__config__cluster__v3__Cluster__LbSubsetConfig__LbSubsetSelector_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::cluster::lb_subset_config::envoy__config__cluster__v3__Cluster__LbSubsetConfig__LbSubsetSelector_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbSubsetSelector {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbSubsetSelector {
  type Msg = LbSubsetSelector;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetSelector> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbSubsetSelector {
  type Msg = LbSubsetSelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetSelector> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LbSubsetSelectorMut<'_> {
  type Msg = LbSubsetSelector;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetSelector> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbSubsetSelectorMut<'_> {
  type Msg = LbSubsetSelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetSelector> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LbSubsetSelectorView<'_> {
  type Msg = LbSubsetSelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LbSubsetSelector> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LbSubsetSelectorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod lb_subset_selector {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LbSubsetSelectorFallbackPolicy(i32);

#[allow(non_upper_case_globals)]
impl LbSubsetSelectorFallbackPolicy {
  pub const NotDefined: LbSubsetSelectorFallbackPolicy = LbSubsetSelectorFallbackPolicy(0);
  pub const NoFallback: LbSubsetSelectorFallbackPolicy = LbSubsetSelectorFallbackPolicy(1);
  pub const AnyEndpoint: LbSubsetSelectorFallbackPolicy = LbSubsetSelectorFallbackPolicy(2);
  pub const DefaultSubset: LbSubsetSelectorFallbackPolicy = LbSubsetSelectorFallbackPolicy(3);
  pub const KeysSubset: LbSubsetSelectorFallbackPolicy = LbSubsetSelectorFallbackPolicy(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "NotDefined",
      1 => "NoFallback",
      2 => "AnyEndpoint",
      3 => "DefaultSubset",
      4 => "KeysSubset",
      _ => return None
    })
  }
}

impl ::std::convert::From<LbSubsetSelectorFallbackPolicy> for i32 {
  fn from(val: LbSubsetSelectorFallbackPolicy) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for LbSubsetSelectorFallbackPolicy {
  fn from(val: i32) -> LbSubsetSelectorFallbackPolicy {
    Self(val)
  }
}

impl ::std::default::Default for LbSubsetSelectorFallbackPolicy {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for LbSubsetSelectorFallbackPolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "LbSubsetSelectorFallbackPolicy::{}", constant_name)
    } else {
      write!(f, "LbSubsetSelectorFallbackPolicy::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for LbSubsetSelectorFallbackPolicy {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for LbSubsetSelectorFallbackPolicy {}

impl ::protobuf::Proxied for LbSubsetSelectorFallbackPolicy {
  type View<'a> = LbSubsetSelectorFallbackPolicy;
}

impl ::protobuf::AsView for LbSubsetSelectorFallbackPolicy {
  type Proxied = LbSubsetSelectorFallbackPolicy;

  fn as_view(&self) -> LbSubsetSelectorFallbackPolicy {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbSubsetSelectorFallbackPolicy {
  fn into_view<'shorter>(self) -> LbSubsetSelectorFallbackPolicy where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for LbSubsetSelectorFallbackPolicy {
  const NAME: &'static str = "LbSubsetSelectorFallbackPolicy";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for LbSubsetSelectorFallbackPolicy {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod lb_subset_selector

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LbSubsetFallbackPolicy(i32);

#[allow(non_upper_case_globals)]
impl LbSubsetFallbackPolicy {
  pub const NoFallback: LbSubsetFallbackPolicy = LbSubsetFallbackPolicy(0);
  pub const AnyEndpoint: LbSubsetFallbackPolicy = LbSubsetFallbackPolicy(1);
  pub const DefaultSubset: LbSubsetFallbackPolicy = LbSubsetFallbackPolicy(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "NoFallback",
      1 => "AnyEndpoint",
      2 => "DefaultSubset",
      _ => return None
    })
  }
}

impl ::std::convert::From<LbSubsetFallbackPolicy> for i32 {
  fn from(val: LbSubsetFallbackPolicy) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for LbSubsetFallbackPolicy {
  fn from(val: i32) -> LbSubsetFallbackPolicy {
    Self(val)
  }
}

impl ::std::default::Default for LbSubsetFallbackPolicy {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for LbSubsetFallbackPolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "LbSubsetFallbackPolicy::{}", constant_name)
    } else {
      write!(f, "LbSubsetFallbackPolicy::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for LbSubsetFallbackPolicy {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for LbSubsetFallbackPolicy {}

impl ::protobuf::Proxied for LbSubsetFallbackPolicy {
  type View<'a> = LbSubsetFallbackPolicy;
}

impl ::protobuf::AsView for LbSubsetFallbackPolicy {
  type Proxied = LbSubsetFallbackPolicy;

  fn as_view(&self) -> LbSubsetFallbackPolicy {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbSubsetFallbackPolicy {
  fn into_view<'shorter>(self) -> LbSubsetFallbackPolicy where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for LbSubsetFallbackPolicy {
  const NAME: &'static str = "LbSubsetFallbackPolicy";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for LbSubsetFallbackPolicy {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LbSubsetMetadataFallbackPolicy(i32);

#[allow(non_upper_case_globals)]
impl LbSubsetMetadataFallbackPolicy {
  pub const MetadataNoFallback: LbSubsetMetadataFallbackPolicy = LbSubsetMetadataFallbackPolicy(0);
  pub const FallbackList: LbSubsetMetadataFallbackPolicy = LbSubsetMetadataFallbackPolicy(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "MetadataNoFallback",
      1 => "FallbackList",
      _ => return None
    })
  }
}

impl ::std::convert::From<LbSubsetMetadataFallbackPolicy> for i32 {
  fn from(val: LbSubsetMetadataFallbackPolicy) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for LbSubsetMetadataFallbackPolicy {
  fn from(val: i32) -> LbSubsetMetadataFallbackPolicy {
    Self(val)
  }
}

impl ::std::default::Default for LbSubsetMetadataFallbackPolicy {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for LbSubsetMetadataFallbackPolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "LbSubsetMetadataFallbackPolicy::{}", constant_name)
    } else {
      write!(f, "LbSubsetMetadataFallbackPolicy::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for LbSubsetMetadataFallbackPolicy {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for LbSubsetMetadataFallbackPolicy {}

impl ::protobuf::Proxied for LbSubsetMetadataFallbackPolicy {
  type View<'a> = LbSubsetMetadataFallbackPolicy;
}

impl ::protobuf::AsView for LbSubsetMetadataFallbackPolicy {
  type Proxied = LbSubsetMetadataFallbackPolicy;

  fn as_view(&self) -> LbSubsetMetadataFallbackPolicy {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbSubsetMetadataFallbackPolicy {
  fn into_view<'shorter>(self) -> LbSubsetMetadataFallbackPolicy where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for LbSubsetMetadataFallbackPolicy {
  const NAME: &'static str = "LbSubsetMetadataFallbackPolicy";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for LbSubsetMetadataFallbackPolicy {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod lb_subset_config

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__SlowStartConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SlowStartConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SlowStartConfig>
}

impl ::protobuf::Message for SlowStartConfig {
  type MessageView<'msg> = SlowStartConfigView<'msg>;
  type MessageMut<'msg> = SlowStartConfigMut<'msg>;
}

impl ::std::default::Default for SlowStartConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SlowStartConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SlowStartConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `SlowStartConfigMut`.
unsafe impl ::std::marker::Sync for SlowStartConfig {}

// SAFETY:
// - `SlowStartConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SlowStartConfig {}

impl ::protobuf::Proxied for SlowStartConfig {
  type View<'msg> = SlowStartConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SlowStartConfig {}

impl ::protobuf::MutProxied for SlowStartConfig {
  type Mut<'msg> = SlowStartConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SlowStartConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SlowStartConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SlowStartConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SlowStartConfigView<'msg> {
  type Message = SlowStartConfig;
}

impl ::std::fmt::Debug for SlowStartConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SlowStartConfigView<'_> {
  fn default() -> SlowStartConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SlowStartConfig>> for SlowStartConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SlowStartConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SlowStartConfigView<'msg> {

  pub fn to_owned(&self) -> SlowStartConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // slow_start_window: optional message google.protobuf.Duration
  pub fn has_slow_start_window(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn slow_start_window_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_slow_start_window().then(|| self.slow_start_window())
  }
  pub fn slow_start_window(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // aggression: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_aggression(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn aggression_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'msg>> {
    self.has_aggression().then(|| self.aggression())
  }
  pub fn aggression(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }

  // min_weight_percent: optional message envoy.type.v3.Percent
  pub fn has_min_weight_percent(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn min_weight_percent_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_min_weight_percent().then(|| self.min_weight_percent())
  }
  pub fn min_weight_percent(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

}

// SAFETY:
// - `SlowStartConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SlowStartConfigView<'_> {}

// SAFETY:
// - `SlowStartConfigView` is `Send` because while its alive a `SlowStartConfigMut` cannot.
// - `SlowStartConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for SlowStartConfigView<'_> {}

impl<'msg> ::protobuf::AsView for SlowStartConfigView<'msg> {
  type Proxied = SlowStartConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, SlowStartConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SlowStartConfigView<'msg> {
  fn into_view<'shorter>(self) -> SlowStartConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SlowStartConfig> for SlowStartConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SlowStartConfig {
    let mut dst = SlowStartConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SlowStartConfig> for SlowStartConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SlowStartConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SlowStartConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SlowStartConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SlowStartConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SlowStartConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SlowStartConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SlowStartConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SlowStartConfigMut<'msg> {
  type Message = SlowStartConfig;
}

impl ::std::fmt::Debug for SlowStartConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SlowStartConfig>> for SlowStartConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SlowStartConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SlowStartConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SlowStartConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SlowStartConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // slow_start_window: optional message google.protobuf.Duration
  pub fn has_slow_start_window(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_slow_start_window(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn slow_start_window_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_slow_start_window().then(|| self.slow_start_window())
  }
  pub fn slow_start_window(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn slow_start_window_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_slow_start_window(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // aggression: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_aggression(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_aggression(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn aggression_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_>> {
    self.has_aggression().then(|| self.aggression())
  }
  pub fn aggression(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }
  pub fn aggression_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleMut<'_> {
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
  pub fn set_aggression(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // min_weight_percent: optional message envoy.type.v3.Percent
  pub fn has_min_weight_percent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_min_weight_percent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn min_weight_percent_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_min_weight_percent().then(|| self.min_weight_percent())
  }
  pub fn min_weight_percent(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn min_weight_percent_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_min_weight_percent(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

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
// - `SlowStartConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SlowStartConfigMut<'_> {}

// SAFETY:
// - `SlowStartConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SlowStartConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for SlowStartConfigMut<'msg> {
  type Proxied = SlowStartConfig;
  fn as_view(&self) -> ::protobuf::View<'_, SlowStartConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SlowStartConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SlowStartConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SlowStartConfigMut<'msg> {
  type MutProxied = SlowStartConfig;
  fn as_mut(&mut self) -> SlowStartConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SlowStartConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> SlowStartConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SlowStartConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SlowStartConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SlowStartConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SlowStartConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // slow_start_window: optional message google.protobuf.Duration
  pub fn has_slow_start_window(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_slow_start_window(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn slow_start_window_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_slow_start_window().then(|| self.slow_start_window())
  }
  pub fn slow_start_window(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn slow_start_window_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_slow_start_window(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // aggression: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_aggression(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_aggression(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn aggression_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_>> {
    self.has_aggression().then(|| self.aggression())
  }
  pub fn aggression(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }
  pub fn aggression_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleMut<'_> {
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
  pub fn set_aggression(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // min_weight_percent: optional message envoy.type.v3.Percent
  pub fn has_min_weight_percent(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_min_weight_percent(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn min_weight_percent_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_min_weight_percent().then(|| self.min_weight_percent())
  }
  pub fn min_weight_percent(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn min_weight_percent_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_min_weight_percent(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl SlowStartConfig

impl ::std::ops::Drop for SlowStartConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SlowStartConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SlowStartConfig {
  type Proxied = Self;
  fn as_view(&self) -> SlowStartConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SlowStartConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SlowStartConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SlowStartConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__SlowStartConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__SlowStartConfig_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__SlowStartConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SlowStartConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SlowStartConfig {
  type Msg = SlowStartConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SlowStartConfig {
  type Msg = SlowStartConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SlowStartConfigMut<'_> {
  type Msg = SlowStartConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SlowStartConfigMut<'_> {
  type Msg = SlowStartConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SlowStartConfigView<'_> {
  type Msg = SlowStartConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SlowStartConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SlowStartConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__RoundRobinLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RoundRobinLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RoundRobinLbConfig>
}

impl ::protobuf::Message for RoundRobinLbConfig {
  type MessageView<'msg> = RoundRobinLbConfigView<'msg>;
  type MessageMut<'msg> = RoundRobinLbConfigMut<'msg>;
}

impl ::std::default::Default for RoundRobinLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RoundRobinLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RoundRobinLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `RoundRobinLbConfigMut`.
unsafe impl ::std::marker::Sync for RoundRobinLbConfig {}

// SAFETY:
// - `RoundRobinLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RoundRobinLbConfig {}

impl ::protobuf::Proxied for RoundRobinLbConfig {
  type View<'msg> = RoundRobinLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RoundRobinLbConfig {}

impl ::protobuf::MutProxied for RoundRobinLbConfig {
  type Mut<'msg> = RoundRobinLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RoundRobinLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RoundRobinLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoundRobinLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RoundRobinLbConfigView<'msg> {
  type Message = RoundRobinLbConfig;
}

impl ::std::fmt::Debug for RoundRobinLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RoundRobinLbConfigView<'_> {
  fn default() -> RoundRobinLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RoundRobinLbConfig>> for RoundRobinLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RoundRobinLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RoundRobinLbConfigView<'msg> {

  pub fn to_owned(&self) -> RoundRobinLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // slow_start_config: optional message envoy.config.cluster.v3.Cluster.SlowStartConfig
  pub fn has_slow_start_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn slow_start_config_opt(self) -> ::std::option::Option<super::super::cluster::SlowStartConfigView<'msg>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(self) -> super::super::cluster::SlowStartConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::SlowStartConfigView::default())
  }

}

// SAFETY:
// - `RoundRobinLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RoundRobinLbConfigView<'_> {}

// SAFETY:
// - `RoundRobinLbConfigView` is `Send` because while its alive a `RoundRobinLbConfigMut` cannot.
// - `RoundRobinLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for RoundRobinLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for RoundRobinLbConfigView<'msg> {
  type Proxied = RoundRobinLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, RoundRobinLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoundRobinLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> RoundRobinLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RoundRobinLbConfig> for RoundRobinLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RoundRobinLbConfig {
    let mut dst = RoundRobinLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RoundRobinLbConfig> for RoundRobinLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RoundRobinLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RoundRobinLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RoundRobinLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RoundRobinLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RoundRobinLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RoundRobinLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RoundRobinLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RoundRobinLbConfigMut<'msg> {
  type Message = RoundRobinLbConfig;
}

impl ::std::fmt::Debug for RoundRobinLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RoundRobinLbConfig>> for RoundRobinLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RoundRobinLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RoundRobinLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RoundRobinLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RoundRobinLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // slow_start_config: optional message envoy.config.cluster.v3.Cluster.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<super::super::cluster::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> super::super::cluster::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> super::super::cluster::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::SlowStartConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `RoundRobinLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RoundRobinLbConfigMut<'_> {}

// SAFETY:
// - `RoundRobinLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RoundRobinLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for RoundRobinLbConfigMut<'msg> {
  type Proxied = RoundRobinLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, RoundRobinLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RoundRobinLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RoundRobinLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RoundRobinLbConfigMut<'msg> {
  type MutProxied = RoundRobinLbConfig;
  fn as_mut(&mut self) -> RoundRobinLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RoundRobinLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> RoundRobinLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RoundRobinLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RoundRobinLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RoundRobinLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RoundRobinLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // slow_start_config: optional message envoy.config.cluster.v3.Cluster.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<super::super::cluster::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> super::super::cluster::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> super::super::cluster::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::SlowStartConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl RoundRobinLbConfig

impl ::std::ops::Drop for RoundRobinLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RoundRobinLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RoundRobinLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> RoundRobinLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RoundRobinLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RoundRobinLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RoundRobinLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__RoundRobinLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__RoundRobinLbConfig_msg_init.0, &[<super::super::cluster::SlowStartConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__RoundRobinLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RoundRobinLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RoundRobinLbConfig {
  type Msg = RoundRobinLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobinLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoundRobinLbConfig {
  type Msg = RoundRobinLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobinLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RoundRobinLbConfigMut<'_> {
  type Msg = RoundRobinLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobinLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoundRobinLbConfigMut<'_> {
  type Msg = RoundRobinLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobinLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RoundRobinLbConfigView<'_> {
  type Msg = RoundRobinLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RoundRobinLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RoundRobinLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__LeastRequestLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LeastRequestLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LeastRequestLbConfig>
}

impl ::protobuf::Message for LeastRequestLbConfig {
  type MessageView<'msg> = LeastRequestLbConfigView<'msg>;
  type MessageMut<'msg> = LeastRequestLbConfigMut<'msg>;
}

impl ::std::default::Default for LeastRequestLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LeastRequestLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LeastRequestLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `LeastRequestLbConfigMut`.
unsafe impl ::std::marker::Sync for LeastRequestLbConfig {}

// SAFETY:
// - `LeastRequestLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LeastRequestLbConfig {}

impl ::protobuf::Proxied for LeastRequestLbConfig {
  type View<'msg> = LeastRequestLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LeastRequestLbConfig {}

impl ::protobuf::MutProxied for LeastRequestLbConfig {
  type Mut<'msg> = LeastRequestLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LeastRequestLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LeastRequestLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LeastRequestLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LeastRequestLbConfigView<'msg> {
  type Message = LeastRequestLbConfig;
}

impl ::std::fmt::Debug for LeastRequestLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LeastRequestLbConfigView<'_> {
  fn default() -> LeastRequestLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LeastRequestLbConfig>> for LeastRequestLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LeastRequestLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LeastRequestLbConfigView<'msg> {

  pub fn to_owned(&self) -> LeastRequestLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // choice_count: optional message google.protobuf.UInt32Value
  pub fn has_choice_count(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn choice_count_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_choice_count().then(|| self.choice_count())
  }
  pub fn choice_count(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // active_request_bias: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_active_request_bias(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn active_request_bias_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'msg>> {
    self.has_active_request_bias().then(|| self.active_request_bias())
  }
  pub fn active_request_bias(self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }

  // slow_start_config: optional message envoy.config.cluster.v3.Cluster.SlowStartConfig
  pub fn has_slow_start_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn slow_start_config_opt(self) -> ::std::option::Option<super::super::cluster::SlowStartConfigView<'msg>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(self) -> super::super::cluster::SlowStartConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::SlowStartConfigView::default())
  }

}

// SAFETY:
// - `LeastRequestLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LeastRequestLbConfigView<'_> {}

// SAFETY:
// - `LeastRequestLbConfigView` is `Send` because while its alive a `LeastRequestLbConfigMut` cannot.
// - `LeastRequestLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for LeastRequestLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for LeastRequestLbConfigView<'msg> {
  type Proxied = LeastRequestLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, LeastRequestLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LeastRequestLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> LeastRequestLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LeastRequestLbConfig> for LeastRequestLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LeastRequestLbConfig {
    let mut dst = LeastRequestLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LeastRequestLbConfig> for LeastRequestLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LeastRequestLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LeastRequestLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LeastRequestLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LeastRequestLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LeastRequestLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LeastRequestLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LeastRequestLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LeastRequestLbConfigMut<'msg> {
  type Message = LeastRequestLbConfig;
}

impl ::std::fmt::Debug for LeastRequestLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LeastRequestLbConfig>> for LeastRequestLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LeastRequestLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LeastRequestLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LeastRequestLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LeastRequestLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // choice_count: optional message google.protobuf.UInt32Value
  pub fn has_choice_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_choice_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn choice_count_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_choice_count().then(|| self.choice_count())
  }
  pub fn choice_count(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn choice_count_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_choice_count(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // active_request_bias: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_active_request_bias(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_active_request_bias(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn active_request_bias_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_>> {
    self.has_active_request_bias().then(|| self.active_request_bias())
  }
  pub fn active_request_bias(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }
  pub fn active_request_bias_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleMut<'_> {
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
  pub fn set_active_request_bias(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // slow_start_config: optional message envoy.config.cluster.v3.Cluster.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<super::super::cluster::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> super::super::cluster::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> super::super::cluster::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::SlowStartConfig>) {

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
// - `LeastRequestLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LeastRequestLbConfigMut<'_> {}

// SAFETY:
// - `LeastRequestLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LeastRequestLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for LeastRequestLbConfigMut<'msg> {
  type Proxied = LeastRequestLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, LeastRequestLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LeastRequestLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LeastRequestLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LeastRequestLbConfigMut<'msg> {
  type MutProxied = LeastRequestLbConfig;
  fn as_mut(&mut self) -> LeastRequestLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LeastRequestLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> LeastRequestLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LeastRequestLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LeastRequestLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LeastRequestLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LeastRequestLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // choice_count: optional message google.protobuf.UInt32Value
  pub fn has_choice_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_choice_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn choice_count_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_choice_count().then(|| self.choice_count())
  }
  pub fn choice_count(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn choice_count_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_choice_count(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // active_request_bias: optional message envoy.config.core.v3.RuntimeDouble
  pub fn has_active_request_bias(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_active_request_bias(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn active_request_bias_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_>> {
    self.has_active_request_bias().then(|| self.active_request_bias())
  }
  pub fn active_request_bias(&self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleView::default())
  }
  pub fn active_request_bias_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::RuntimeDoubleMut<'_> {
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
  pub fn set_active_request_bias(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // slow_start_config: optional message envoy.config.cluster.v3.Cluster.SlowStartConfig
  pub fn has_slow_start_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_slow_start_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn slow_start_config_opt(&self) -> ::std::option::Option<super::super::cluster::SlowStartConfigView<'_>> {
    self.has_slow_start_config().then(|| self.slow_start_config())
  }
  pub fn slow_start_config(&self) -> super::super::cluster::SlowStartConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::SlowStartConfigView::default())
  }
  pub fn slow_start_config_mut(&mut self) -> super::super::cluster::SlowStartConfigMut<'_> {
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
  pub fn set_slow_start_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::SlowStartConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl LeastRequestLbConfig

impl ::std::ops::Drop for LeastRequestLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LeastRequestLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LeastRequestLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> LeastRequestLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LeastRequestLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LeastRequestLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LeastRequestLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__LeastRequestLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__LeastRequestLbConfig_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::base::RuntimeDouble as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::cluster::SlowStartConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__LeastRequestLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LeastRequestLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LeastRequestLbConfig {
  type Msg = LeastRequestLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequestLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LeastRequestLbConfig {
  type Msg = LeastRequestLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequestLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LeastRequestLbConfigMut<'_> {
  type Msg = LeastRequestLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequestLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LeastRequestLbConfigMut<'_> {
  type Msg = LeastRequestLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequestLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LeastRequestLbConfigView<'_> {
  type Msg = LeastRequestLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LeastRequestLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LeastRequestLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__RingHashLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RingHashLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RingHashLbConfig>
}

impl ::protobuf::Message for RingHashLbConfig {
  type MessageView<'msg> = RingHashLbConfigView<'msg>;
  type MessageMut<'msg> = RingHashLbConfigMut<'msg>;
}

impl ::std::default::Default for RingHashLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RingHashLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RingHashLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `RingHashLbConfigMut`.
unsafe impl ::std::marker::Sync for RingHashLbConfig {}

// SAFETY:
// - `RingHashLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RingHashLbConfig {}

impl ::protobuf::Proxied for RingHashLbConfig {
  type View<'msg> = RingHashLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RingHashLbConfig {}

impl ::protobuf::MutProxied for RingHashLbConfig {
  type Mut<'msg> = RingHashLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RingHashLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RingHashLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RingHashLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RingHashLbConfigView<'msg> {
  type Message = RingHashLbConfig;
}

impl ::std::fmt::Debug for RingHashLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RingHashLbConfigView<'_> {
  fn default() -> RingHashLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RingHashLbConfig>> for RingHashLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RingHashLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RingHashLbConfigView<'msg> {

  pub fn to_owned(&self) -> RingHashLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // minimum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_minimum_ring_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn minimum_ring_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_minimum_ring_size().then(|| self.minimum_ring_size())
  }
  pub fn minimum_ring_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // hash_function: optional enum envoy.config.cluster.v3.Cluster.RingHashLbConfig.HashFunction
  pub fn hash_function(self) -> super::super::cluster::ring_hash_lb_config::HashFunction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::super::cluster::ring_hash_lb_config::HashFunction::XxHash).into()
      ).try_into().unwrap()
    }
  }

  // maximum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_maximum_ring_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn maximum_ring_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_maximum_ring_size().then(|| self.maximum_ring_size())
  }
  pub fn maximum_ring_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

}

// SAFETY:
// - `RingHashLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RingHashLbConfigView<'_> {}

// SAFETY:
// - `RingHashLbConfigView` is `Send` because while its alive a `RingHashLbConfigMut` cannot.
// - `RingHashLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for RingHashLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for RingHashLbConfigView<'msg> {
  type Proxied = RingHashLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, RingHashLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RingHashLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> RingHashLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RingHashLbConfig> for RingHashLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RingHashLbConfig {
    let mut dst = RingHashLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RingHashLbConfig> for RingHashLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RingHashLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RingHashLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RingHashLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RingHashLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RingHashLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RingHashLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RingHashLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RingHashLbConfigMut<'msg> {
  type Message = RingHashLbConfig;
}

impl ::std::fmt::Debug for RingHashLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RingHashLbConfig>> for RingHashLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RingHashLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RingHashLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RingHashLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RingHashLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // minimum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_minimum_ring_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_minimum_ring_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn minimum_ring_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_minimum_ring_size().then(|| self.minimum_ring_size())
  }
  pub fn minimum_ring_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn minimum_ring_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_minimum_ring_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // hash_function: optional enum envoy.config.cluster.v3.Cluster.RingHashLbConfig.HashFunction
  pub fn hash_function(&self) -> super::super::cluster::ring_hash_lb_config::HashFunction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::super::cluster::ring_hash_lb_config::HashFunction::XxHash).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_hash_function(&mut self, val: super::super::cluster::ring_hash_lb_config::HashFunction) {
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

  // maximum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_maximum_ring_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_maximum_ring_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn maximum_ring_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_maximum_ring_size().then(|| self.maximum_ring_size())
  }
  pub fn maximum_ring_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn maximum_ring_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_maximum_ring_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

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
// - `RingHashLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RingHashLbConfigMut<'_> {}

// SAFETY:
// - `RingHashLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RingHashLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for RingHashLbConfigMut<'msg> {
  type Proxied = RingHashLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, RingHashLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RingHashLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RingHashLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RingHashLbConfigMut<'msg> {
  type MutProxied = RingHashLbConfig;
  fn as_mut(&mut self) -> RingHashLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RingHashLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> RingHashLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RingHashLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RingHashLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RingHashLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RingHashLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // minimum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_minimum_ring_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_minimum_ring_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn minimum_ring_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_minimum_ring_size().then(|| self.minimum_ring_size())
  }
  pub fn minimum_ring_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn minimum_ring_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_minimum_ring_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // hash_function: optional enum envoy.config.cluster.v3.Cluster.RingHashLbConfig.HashFunction
  pub fn hash_function(&self) -> super::super::cluster::ring_hash_lb_config::HashFunction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::super::cluster::ring_hash_lb_config::HashFunction::XxHash).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_hash_function(&mut self, val: super::super::cluster::ring_hash_lb_config::HashFunction) {
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

  // maximum_ring_size: optional message google.protobuf.UInt64Value
  pub fn has_maximum_ring_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_maximum_ring_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn maximum_ring_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_maximum_ring_size().then(|| self.maximum_ring_size())
  }
  pub fn maximum_ring_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn maximum_ring_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_maximum_ring_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl RingHashLbConfig

impl ::std::ops::Drop for RingHashLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RingHashLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RingHashLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> RingHashLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RingHashLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RingHashLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RingHashLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__RingHashLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3a.P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__RingHashLbConfig_msg_init.0, &[<::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__RingHashLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RingHashLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RingHashLbConfig {
  type Msg = RingHashLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHashLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RingHashLbConfig {
  type Msg = RingHashLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHashLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RingHashLbConfigMut<'_> {
  type Msg = RingHashLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHashLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RingHashLbConfigMut<'_> {
  type Msg = RingHashLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHashLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RingHashLbConfigView<'_> {
  type Msg = RingHashLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RingHashLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RingHashLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod ring_hash_lb_config {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HashFunction(i32);

#[allow(non_upper_case_globals)]
impl HashFunction {
  pub const XxHash: HashFunction = HashFunction(0);
  pub const MurmurHash2: HashFunction = HashFunction(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "XxHash",
      1 => "MurmurHash2",
      _ => return None
    })
  }
}

impl ::std::convert::From<HashFunction> for i32 {
  fn from(val: HashFunction) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for HashFunction {
  fn from(val: i32) -> HashFunction {
    Self(val)
  }
}

impl ::std::default::Default for HashFunction {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for HashFunction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "HashFunction::{}", constant_name)
    } else {
      write!(f, "HashFunction::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for HashFunction {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for HashFunction {}

impl ::protobuf::Proxied for HashFunction {
  type View<'a> = HashFunction;
}

impl ::protobuf::AsView for HashFunction {
  type Proxied = HashFunction;

  fn as_view(&self) -> HashFunction {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HashFunction {
  fn into_view<'shorter>(self) -> HashFunction where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for HashFunction {
  const NAME: &'static str = "HashFunction";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for HashFunction {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod ring_hash_lb_config

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__MaglevLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MaglevLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MaglevLbConfig>
}

impl ::protobuf::Message for MaglevLbConfig {
  type MessageView<'msg> = MaglevLbConfigView<'msg>;
  type MessageMut<'msg> = MaglevLbConfigMut<'msg>;
}

impl ::std::default::Default for MaglevLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MaglevLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MaglevLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `MaglevLbConfigMut`.
unsafe impl ::std::marker::Sync for MaglevLbConfig {}

// SAFETY:
// - `MaglevLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MaglevLbConfig {}

impl ::protobuf::Proxied for MaglevLbConfig {
  type View<'msg> = MaglevLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MaglevLbConfig {}

impl ::protobuf::MutProxied for MaglevLbConfig {
  type Mut<'msg> = MaglevLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MaglevLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MaglevLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MaglevLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MaglevLbConfigView<'msg> {
  type Message = MaglevLbConfig;
}

impl ::std::fmt::Debug for MaglevLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MaglevLbConfigView<'_> {
  fn default() -> MaglevLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MaglevLbConfig>> for MaglevLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MaglevLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MaglevLbConfigView<'msg> {

  pub fn to_owned(&self) -> MaglevLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // table_size: optional message google.protobuf.UInt64Value
  pub fn has_table_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn table_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_table_size().then(|| self.table_size())
  }
  pub fn table_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

}

// SAFETY:
// - `MaglevLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MaglevLbConfigView<'_> {}

// SAFETY:
// - `MaglevLbConfigView` is `Send` because while its alive a `MaglevLbConfigMut` cannot.
// - `MaglevLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for MaglevLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for MaglevLbConfigView<'msg> {
  type Proxied = MaglevLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, MaglevLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MaglevLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> MaglevLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MaglevLbConfig> for MaglevLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MaglevLbConfig {
    let mut dst = MaglevLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MaglevLbConfig> for MaglevLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MaglevLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MaglevLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MaglevLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MaglevLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MaglevLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MaglevLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MaglevLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MaglevLbConfigMut<'msg> {
  type Message = MaglevLbConfig;
}

impl ::std::fmt::Debug for MaglevLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MaglevLbConfig>> for MaglevLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MaglevLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MaglevLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MaglevLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MaglevLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // table_size: optional message google.protobuf.UInt64Value
  pub fn has_table_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_table_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn table_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_table_size().then(|| self.table_size())
  }
  pub fn table_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn table_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_table_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `MaglevLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MaglevLbConfigMut<'_> {}

// SAFETY:
// - `MaglevLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MaglevLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for MaglevLbConfigMut<'msg> {
  type Proxied = MaglevLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, MaglevLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MaglevLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MaglevLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MaglevLbConfigMut<'msg> {
  type MutProxied = MaglevLbConfig;
  fn as_mut(&mut self) -> MaglevLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MaglevLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> MaglevLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MaglevLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MaglevLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MaglevLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MaglevLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // table_size: optional message google.protobuf.UInt64Value
  pub fn has_table_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_table_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn table_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_table_size().then(|| self.table_size())
  }
  pub fn table_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn table_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_table_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl MaglevLbConfig

impl ::std::ops::Drop for MaglevLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MaglevLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MaglevLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> MaglevLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MaglevLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MaglevLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MaglevLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__MaglevLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__MaglevLbConfig_msg_init.0, &[<::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__MaglevLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MaglevLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MaglevLbConfig {
  type Msg = MaglevLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MaglevLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MaglevLbConfig {
  type Msg = MaglevLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MaglevLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MaglevLbConfigMut<'_> {
  type Msg = MaglevLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MaglevLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MaglevLbConfigMut<'_> {
  type Msg = MaglevLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MaglevLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MaglevLbConfigView<'_> {
  type Msg = MaglevLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MaglevLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MaglevLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__OriginalDstLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OriginalDstLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OriginalDstLbConfig>
}

impl ::protobuf::Message for OriginalDstLbConfig {
  type MessageView<'msg> = OriginalDstLbConfigView<'msg>;
  type MessageMut<'msg> = OriginalDstLbConfigMut<'msg>;
}

impl ::std::default::Default for OriginalDstLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OriginalDstLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OriginalDstLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `OriginalDstLbConfigMut`.
unsafe impl ::std::marker::Sync for OriginalDstLbConfig {}

// SAFETY:
// - `OriginalDstLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OriginalDstLbConfig {}

impl ::protobuf::Proxied for OriginalDstLbConfig {
  type View<'msg> = OriginalDstLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OriginalDstLbConfig {}

impl ::protobuf::MutProxied for OriginalDstLbConfig {
  type Mut<'msg> = OriginalDstLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OriginalDstLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OriginalDstLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OriginalDstLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OriginalDstLbConfigView<'msg> {
  type Message = OriginalDstLbConfig;
}

impl ::std::fmt::Debug for OriginalDstLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OriginalDstLbConfigView<'_> {
  fn default() -> OriginalDstLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OriginalDstLbConfig>> for OriginalDstLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OriginalDstLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OriginalDstLbConfigView<'msg> {

  pub fn to_owned(&self) -> OriginalDstLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // use_http_header: optional bool
  pub fn use_http_header(self) -> bool {
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

  // http_header_name: optional string
  pub fn http_header_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // upstream_port_override: optional message google.protobuf.UInt32Value
  pub fn has_upstream_port_override(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn upstream_port_override_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_upstream_port_override().then(|| self.upstream_port_override())
  }
  pub fn upstream_port_override(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

  // metadata_key: optional message envoy.type.metadata.v3.MetadataKey
  pub fn has_metadata_key(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn metadata_key_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'msg>> {
    self.has_metadata_key().then(|| self.metadata_key())
  }
  pub fn metadata_key(self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView::default())
  }

}

// SAFETY:
// - `OriginalDstLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OriginalDstLbConfigView<'_> {}

// SAFETY:
// - `OriginalDstLbConfigView` is `Send` because while its alive a `OriginalDstLbConfigMut` cannot.
// - `OriginalDstLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for OriginalDstLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for OriginalDstLbConfigView<'msg> {
  type Proxied = OriginalDstLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, OriginalDstLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OriginalDstLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> OriginalDstLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OriginalDstLbConfig> for OriginalDstLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OriginalDstLbConfig {
    let mut dst = OriginalDstLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OriginalDstLbConfig> for OriginalDstLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OriginalDstLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OriginalDstLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OriginalDstLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OriginalDstLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OriginalDstLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OriginalDstLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OriginalDstLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OriginalDstLbConfigMut<'msg> {
  type Message = OriginalDstLbConfig;
}

impl ::std::fmt::Debug for OriginalDstLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OriginalDstLbConfig>> for OriginalDstLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OriginalDstLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OriginalDstLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OriginalDstLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OriginalDstLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // use_http_header: optional bool
  pub fn use_http_header(&self) -> bool {
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
  pub fn set_use_http_header(&mut self, val: bool) {
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

  // http_header_name: optional string
  pub fn http_header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_http_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // upstream_port_override: optional message google.protobuf.UInt32Value
  pub fn has_upstream_port_override(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_upstream_port_override(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn upstream_port_override_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_upstream_port_override().then(|| self.upstream_port_override())
  }
  pub fn upstream_port_override(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn upstream_port_override_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_upstream_port_override(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // metadata_key: optional message envoy.type.metadata.v3.MetadataKey
  pub fn has_metadata_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_metadata_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn metadata_key_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'_>> {
    self.has_metadata_key().then(|| self.metadata_key())
  }
  pub fn metadata_key(&self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView::default())
  }
  pub fn metadata_key_mut(&mut self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyMut<'_> {
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
  pub fn set_metadata_key(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKey>) {

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
// - `OriginalDstLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OriginalDstLbConfigMut<'_> {}

// SAFETY:
// - `OriginalDstLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OriginalDstLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for OriginalDstLbConfigMut<'msg> {
  type Proxied = OriginalDstLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, OriginalDstLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OriginalDstLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OriginalDstLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OriginalDstLbConfigMut<'msg> {
  type MutProxied = OriginalDstLbConfig;
  fn as_mut(&mut self) -> OriginalDstLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OriginalDstLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> OriginalDstLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OriginalDstLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OriginalDstLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OriginalDstLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OriginalDstLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // use_http_header: optional bool
  pub fn use_http_header(&self) -> bool {
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
  pub fn set_use_http_header(&mut self, val: bool) {
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

  // http_header_name: optional string
  pub fn http_header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_http_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // upstream_port_override: optional message google.protobuf.UInt32Value
  pub fn has_upstream_port_override(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_upstream_port_override(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn upstream_port_override_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_upstream_port_override().then(|| self.upstream_port_override())
  }
  pub fn upstream_port_override(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn upstream_port_override_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_upstream_port_override(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // metadata_key: optional message envoy.type.metadata.v3.MetadataKey
  pub fn has_metadata_key(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_metadata_key(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn metadata_key_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'_>> {
    self.has_metadata_key().then(|| self.metadata_key())
  }
  pub fn metadata_key(&self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyView::default())
  }
  pub fn metadata_key_mut(&mut self) -> crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKeyMut<'_> {
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
  pub fn set_metadata_key(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKey>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl OriginalDstLbConfig

impl ::std::ops::Drop for OriginalDstLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OriginalDstLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OriginalDstLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> OriginalDstLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OriginalDstLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OriginalDstLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OriginalDstLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__OriginalDstLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P1X33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__OriginalDstLbConfig_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::metadata::v3::metadata::MetadataKey as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__OriginalDstLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OriginalDstLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OriginalDstLbConfig {
  type Msg = OriginalDstLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OriginalDstLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OriginalDstLbConfig {
  type Msg = OriginalDstLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OriginalDstLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OriginalDstLbConfigMut<'_> {
  type Msg = OriginalDstLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OriginalDstLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OriginalDstLbConfigMut<'_> {
  type Msg = OriginalDstLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OriginalDstLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OriginalDstLbConfigView<'_> {
  type Msg = OriginalDstLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OriginalDstLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OriginalDstLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__CommonLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CommonLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CommonLbConfig>
}

impl ::protobuf::Message for CommonLbConfig {
  type MessageView<'msg> = CommonLbConfigView<'msg>;
  type MessageMut<'msg> = CommonLbConfigMut<'msg>;
}

impl ::std::default::Default for CommonLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CommonLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CommonLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `CommonLbConfigMut`.
unsafe impl ::std::marker::Sync for CommonLbConfig {}

// SAFETY:
// - `CommonLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CommonLbConfig {}

impl ::protobuf::Proxied for CommonLbConfig {
  type View<'msg> = CommonLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CommonLbConfig {}

impl ::protobuf::MutProxied for CommonLbConfig {
  type Mut<'msg> = CommonLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CommonLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommonLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommonLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CommonLbConfigView<'msg> {
  type Message = CommonLbConfig;
}

impl ::std::fmt::Debug for CommonLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CommonLbConfigView<'_> {
  fn default() -> CommonLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CommonLbConfig>> for CommonLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CommonLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommonLbConfigView<'msg> {

  pub fn to_owned(&self) -> CommonLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // healthy_panic_threshold: optional message envoy.type.v3.Percent
  pub fn has_healthy_panic_threshold(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn healthy_panic_threshold_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_healthy_panic_threshold().then(|| self.healthy_panic_threshold())
  }
  pub fn healthy_panic_threshold(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  // zone_aware_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig.ZoneAwareLbConfig
  pub fn has_zone_aware_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn zone_aware_lb_config_opt(self) -> ::std::option::Option<super::super::cluster::common_lb_config::ZoneAwareLbConfigView<'msg>> {
    self.has_zone_aware_lb_config().then(|| self.zone_aware_lb_config())
  }
  pub fn zone_aware_lb_config(self) -> super::super::cluster::common_lb_config::ZoneAwareLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::common_lb_config::ZoneAwareLbConfigView::default())
  }

  // locality_weighted_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig.LocalityWeightedLbConfig
  pub fn has_locality_weighted_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn locality_weighted_lb_config_opt(self) -> ::std::option::Option<super::super::cluster::common_lb_config::LocalityWeightedLbConfigView<'msg>> {
    self.has_locality_weighted_lb_config().then(|| self.locality_weighted_lb_config())
  }
  pub fn locality_weighted_lb_config(self) -> super::super::cluster::common_lb_config::LocalityWeightedLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::common_lb_config::LocalityWeightedLbConfigView::default())
  }

  // update_merge_window: optional message google.protobuf.Duration
  pub fn has_update_merge_window(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn update_merge_window_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_update_merge_window().then(|| self.update_merge_window())
  }
  pub fn update_merge_window(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // ignore_new_hosts_until_first_hc: optional bool
  pub fn ignore_new_hosts_until_first_hc(self) -> bool {
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

  // close_connections_on_host_set_change: optional bool
  pub fn close_connections_on_host_set_change(self) -> bool {
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

  // consistent_hashing_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig.ConsistentHashingLbConfig
  pub fn has_consistent_hashing_lb_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn consistent_hashing_lb_config_opt(self) -> ::std::option::Option<super::super::cluster::common_lb_config::ConsistentHashingLbConfigView<'msg>> {
    self.has_consistent_hashing_lb_config().then(|| self.consistent_hashing_lb_config())
  }
  pub fn consistent_hashing_lb_config(self) -> super::super::cluster::common_lb_config::ConsistentHashingLbConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::common_lb_config::ConsistentHashingLbConfigView::default())
  }

  // override_host_status: optional message envoy.config.core.v3.HealthStatusSet
  pub fn has_override_host_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn override_host_status_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetView<'msg>> {
    self.has_override_host_status().then(|| self.override_host_status())
  }
  pub fn override_host_status(self) -> crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetView::default())
  }

  pub fn locality_config_specifier(self) -> super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof<'msg> {
    match self.locality_config_specifier_case() {
      super::super::cluster::common_lb_config::LocalityConfigSpecifierCase::ZoneAwareLbConfig =>
          super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof::ZoneAwareLbConfig(self.zone_aware_lb_config()),
      super::super::cluster::common_lb_config::LocalityConfigSpecifierCase::LocalityWeightedLbConfig =>
          super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof::LocalityWeightedLbConfig(self.locality_weighted_lb_config()),
      _ => super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn locality_config_specifier_case(self) -> super::super::cluster::common_lb_config::LocalityConfigSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::cluster::common_lb_config::LocalityConfigSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CommonLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CommonLbConfigView<'_> {}

// SAFETY:
// - `CommonLbConfigView` is `Send` because while its alive a `CommonLbConfigMut` cannot.
// - `CommonLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for CommonLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for CommonLbConfigView<'msg> {
  type Proxied = CommonLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, CommonLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommonLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> CommonLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CommonLbConfig> for CommonLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommonLbConfig {
    let mut dst = CommonLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CommonLbConfig> for CommonLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CommonLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CommonLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CommonLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CommonLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CommonLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommonLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CommonLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CommonLbConfigMut<'msg> {
  type Message = CommonLbConfig;
}

impl ::std::fmt::Debug for CommonLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CommonLbConfig>> for CommonLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CommonLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CommonLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CommonLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CommonLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // healthy_panic_threshold: optional message envoy.type.v3.Percent
  pub fn has_healthy_panic_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_healthy_panic_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn healthy_panic_threshold_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_healthy_panic_threshold().then(|| self.healthy_panic_threshold())
  }
  pub fn healthy_panic_threshold(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn healthy_panic_threshold_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_healthy_panic_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // zone_aware_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig.ZoneAwareLbConfig
  pub fn has_zone_aware_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_zone_aware_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn zone_aware_lb_config_opt(&self) -> ::std::option::Option<super::super::cluster::common_lb_config::ZoneAwareLbConfigView<'_>> {
    self.has_zone_aware_lb_config().then(|| self.zone_aware_lb_config())
  }
  pub fn zone_aware_lb_config(&self) -> super::super::cluster::common_lb_config::ZoneAwareLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::common_lb_config::ZoneAwareLbConfigView::default())
  }
  pub fn zone_aware_lb_config_mut(&mut self) -> super::super::cluster::common_lb_config::ZoneAwareLbConfigMut<'_> {
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
  pub fn set_zone_aware_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::common_lb_config::ZoneAwareLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // locality_weighted_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig.LocalityWeightedLbConfig
  pub fn has_locality_weighted_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_locality_weighted_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn locality_weighted_lb_config_opt(&self) -> ::std::option::Option<super::super::cluster::common_lb_config::LocalityWeightedLbConfigView<'_>> {
    self.has_locality_weighted_lb_config().then(|| self.locality_weighted_lb_config())
  }
  pub fn locality_weighted_lb_config(&self) -> super::super::cluster::common_lb_config::LocalityWeightedLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::common_lb_config::LocalityWeightedLbConfigView::default())
  }
  pub fn locality_weighted_lb_config_mut(&mut self) -> super::super::cluster::common_lb_config::LocalityWeightedLbConfigMut<'_> {
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
  pub fn set_locality_weighted_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::common_lb_config::LocalityWeightedLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // update_merge_window: optional message google.protobuf.Duration
  pub fn has_update_merge_window(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_update_merge_window(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn update_merge_window_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_update_merge_window().then(|| self.update_merge_window())
  }
  pub fn update_merge_window(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn update_merge_window_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_update_merge_window(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // ignore_new_hosts_until_first_hc: optional bool
  pub fn ignore_new_hosts_until_first_hc(&self) -> bool {
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
  pub fn set_ignore_new_hosts_until_first_hc(&mut self, val: bool) {
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

  // close_connections_on_host_set_change: optional bool
  pub fn close_connections_on_host_set_change(&self) -> bool {
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
  pub fn set_close_connections_on_host_set_change(&mut self, val: bool) {
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

  // consistent_hashing_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig.ConsistentHashingLbConfig
  pub fn has_consistent_hashing_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_consistent_hashing_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn consistent_hashing_lb_config_opt(&self) -> ::std::option::Option<super::super::cluster::common_lb_config::ConsistentHashingLbConfigView<'_>> {
    self.has_consistent_hashing_lb_config().then(|| self.consistent_hashing_lb_config())
  }
  pub fn consistent_hashing_lb_config(&self) -> super::super::cluster::common_lb_config::ConsistentHashingLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::common_lb_config::ConsistentHashingLbConfigView::default())
  }
  pub fn consistent_hashing_lb_config_mut(&mut self) -> super::super::cluster::common_lb_config::ConsistentHashingLbConfigMut<'_> {
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
  pub fn set_consistent_hashing_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::common_lb_config::ConsistentHashingLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // override_host_status: optional message envoy.config.core.v3.HealthStatusSet
  pub fn has_override_host_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_override_host_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn override_host_status_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetView<'_>> {
    self.has_override_host_status().then(|| self.override_host_status())
  }
  pub fn override_host_status(&self) -> crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetView::default())
  }
  pub fn override_host_status_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetMut<'_> {
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
  pub fn set_override_host_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  pub fn locality_config_specifier(&self) -> super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof<'_> {
    match &self.locality_config_specifier_case() {
      super::super::cluster::common_lb_config::LocalityConfigSpecifierCase::ZoneAwareLbConfig =>
          super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof::ZoneAwareLbConfig(self.zone_aware_lb_config()),
      super::super::cluster::common_lb_config::LocalityConfigSpecifierCase::LocalityWeightedLbConfig =>
          super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof::LocalityWeightedLbConfig(self.locality_weighted_lb_config()),
      _ => super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn locality_config_specifier_case(&self) -> super::super::cluster::common_lb_config::LocalityConfigSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::cluster::common_lb_config::LocalityConfigSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `CommonLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CommonLbConfigMut<'_> {}

// SAFETY:
// - `CommonLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CommonLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for CommonLbConfigMut<'msg> {
  type Proxied = CommonLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, CommonLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CommonLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CommonLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CommonLbConfigMut<'msg> {
  type MutProxied = CommonLbConfig;
  fn as_mut(&mut self) -> CommonLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CommonLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> CommonLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CommonLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CommonLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CommonLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CommonLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // healthy_panic_threshold: optional message envoy.type.v3.Percent
  pub fn has_healthy_panic_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_healthy_panic_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn healthy_panic_threshold_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_healthy_panic_threshold().then(|| self.healthy_panic_threshold())
  }
  pub fn healthy_panic_threshold(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn healthy_panic_threshold_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_healthy_panic_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // zone_aware_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig.ZoneAwareLbConfig
  pub fn has_zone_aware_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_zone_aware_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn zone_aware_lb_config_opt(&self) -> ::std::option::Option<super::super::cluster::common_lb_config::ZoneAwareLbConfigView<'_>> {
    self.has_zone_aware_lb_config().then(|| self.zone_aware_lb_config())
  }
  pub fn zone_aware_lb_config(&self) -> super::super::cluster::common_lb_config::ZoneAwareLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::common_lb_config::ZoneAwareLbConfigView::default())
  }
  pub fn zone_aware_lb_config_mut(&mut self) -> super::super::cluster::common_lb_config::ZoneAwareLbConfigMut<'_> {
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
  pub fn set_zone_aware_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::common_lb_config::ZoneAwareLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // locality_weighted_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig.LocalityWeightedLbConfig
  pub fn has_locality_weighted_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_locality_weighted_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn locality_weighted_lb_config_opt(&self) -> ::std::option::Option<super::super::cluster::common_lb_config::LocalityWeightedLbConfigView<'_>> {
    self.has_locality_weighted_lb_config().then(|| self.locality_weighted_lb_config())
  }
  pub fn locality_weighted_lb_config(&self) -> super::super::cluster::common_lb_config::LocalityWeightedLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::common_lb_config::LocalityWeightedLbConfigView::default())
  }
  pub fn locality_weighted_lb_config_mut(&mut self) -> super::super::cluster::common_lb_config::LocalityWeightedLbConfigMut<'_> {
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
  pub fn set_locality_weighted_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::common_lb_config::LocalityWeightedLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // update_merge_window: optional message google.protobuf.Duration
  pub fn has_update_merge_window(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_update_merge_window(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn update_merge_window_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_update_merge_window().then(|| self.update_merge_window())
  }
  pub fn update_merge_window(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn update_merge_window_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_update_merge_window(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // ignore_new_hosts_until_first_hc: optional bool
  pub fn ignore_new_hosts_until_first_hc(&self) -> bool {
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
  pub fn set_ignore_new_hosts_until_first_hc(&mut self, val: bool) {
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

  // close_connections_on_host_set_change: optional bool
  pub fn close_connections_on_host_set_change(&self) -> bool {
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
  pub fn set_close_connections_on_host_set_change(&mut self, val: bool) {
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

  // consistent_hashing_lb_config: optional message envoy.config.cluster.v3.Cluster.CommonLbConfig.ConsistentHashingLbConfig
  pub fn has_consistent_hashing_lb_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_consistent_hashing_lb_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn consistent_hashing_lb_config_opt(&self) -> ::std::option::Option<super::super::cluster::common_lb_config::ConsistentHashingLbConfigView<'_>> {
    self.has_consistent_hashing_lb_config().then(|| self.consistent_hashing_lb_config())
  }
  pub fn consistent_hashing_lb_config(&self) -> super::super::cluster::common_lb_config::ConsistentHashingLbConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::cluster::common_lb_config::ConsistentHashingLbConfigView::default())
  }
  pub fn consistent_hashing_lb_config_mut(&mut self) -> super::super::cluster::common_lb_config::ConsistentHashingLbConfigMut<'_> {
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
  pub fn set_consistent_hashing_lb_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::cluster::common_lb_config::ConsistentHashingLbConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // override_host_status: optional message envoy.config.core.v3.HealthStatusSet
  pub fn has_override_host_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_override_host_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn override_host_status_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetView<'_>> {
    self.has_override_host_status().then(|| self.override_host_status())
  }
  pub fn override_host_status(&self) -> crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetView::default())
  }
  pub fn override_host_status_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSetMut<'_> {
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
  pub fn set_override_host_status(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  pub fn locality_config_specifier(&self) -> super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof<'_> {
    match &self.locality_config_specifier_case() {
      super::super::cluster::common_lb_config::LocalityConfigSpecifierCase::ZoneAwareLbConfig =>
          super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof::ZoneAwareLbConfig(self.zone_aware_lb_config()),
      super::super::cluster::common_lb_config::LocalityConfigSpecifierCase::LocalityWeightedLbConfig =>
          super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof::LocalityWeightedLbConfig(self.locality_weighted_lb_config()),
      _ => super::super::cluster::common_lb_config::LocalityConfigSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn locality_config_specifier_case(&self) -> super::super::cluster::common_lb_config::LocalityConfigSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::cluster::common_lb_config::LocalityConfigSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl CommonLbConfig

impl ::std::ops::Drop for CommonLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CommonLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CommonLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> CommonLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CommonLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CommonLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CommonLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__CommonLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333/P/P33^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__CommonLbConfig_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::cluster::common_lb_config::ZoneAwareLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::cluster::common_lb_config::LocalityWeightedLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::cluster::common_lb_config::ConsistentHashingLbConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::health_check::HealthStatusSet as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__CommonLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommonLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommonLbConfig {
  type Msg = CommonLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommonLbConfig {
  type Msg = CommonLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CommonLbConfigMut<'_> {
  type Msg = CommonLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommonLbConfigMut<'_> {
  type Msg = CommonLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CommonLbConfigView<'_> {
  type Msg = CommonLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CommonLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CommonLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod common_lb_config {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__CommonLbConfig__ZoneAwareLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ZoneAwareLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ZoneAwareLbConfig>
}

impl ::protobuf::Message for ZoneAwareLbConfig {
  type MessageView<'msg> = ZoneAwareLbConfigView<'msg>;
  type MessageMut<'msg> = ZoneAwareLbConfigMut<'msg>;
}

impl ::std::default::Default for ZoneAwareLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ZoneAwareLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ZoneAwareLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ZoneAwareLbConfigMut`.
unsafe impl ::std::marker::Sync for ZoneAwareLbConfig {}

// SAFETY:
// - `ZoneAwareLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ZoneAwareLbConfig {}

impl ::protobuf::Proxied for ZoneAwareLbConfig {
  type View<'msg> = ZoneAwareLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ZoneAwareLbConfig {}

impl ::protobuf::MutProxied for ZoneAwareLbConfig {
  type Mut<'msg> = ZoneAwareLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ZoneAwareLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ZoneAwareLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ZoneAwareLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ZoneAwareLbConfigView<'msg> {
  type Message = ZoneAwareLbConfig;
}

impl ::std::fmt::Debug for ZoneAwareLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ZoneAwareLbConfigView<'_> {
  fn default() -> ZoneAwareLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ZoneAwareLbConfig>> for ZoneAwareLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ZoneAwareLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ZoneAwareLbConfigView<'msg> {

  pub fn to_owned(&self) -> ZoneAwareLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // routing_enabled: optional message envoy.type.v3.Percent
  pub fn has_routing_enabled(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn routing_enabled_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_routing_enabled().then(|| self.routing_enabled())
  }
  pub fn routing_enabled(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

  // min_cluster_size: optional message google.protobuf.UInt64Value
  pub fn has_min_cluster_size(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn min_cluster_size_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_min_cluster_size().then(|| self.min_cluster_size())
  }
  pub fn min_cluster_size(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // fail_traffic_on_panic: optional bool
  pub fn fail_traffic_on_panic(self) -> bool {
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
// - `ZoneAwareLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ZoneAwareLbConfigView<'_> {}

// SAFETY:
// - `ZoneAwareLbConfigView` is `Send` because while its alive a `ZoneAwareLbConfigMut` cannot.
// - `ZoneAwareLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ZoneAwareLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ZoneAwareLbConfigView<'msg> {
  type Proxied = ZoneAwareLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ZoneAwareLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ZoneAwareLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> ZoneAwareLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ZoneAwareLbConfig> for ZoneAwareLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ZoneAwareLbConfig {
    let mut dst = ZoneAwareLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ZoneAwareLbConfig> for ZoneAwareLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ZoneAwareLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ZoneAwareLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ZoneAwareLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ZoneAwareLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ZoneAwareLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ZoneAwareLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ZoneAwareLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ZoneAwareLbConfigMut<'msg> {
  type Message = ZoneAwareLbConfig;
}

impl ::std::fmt::Debug for ZoneAwareLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ZoneAwareLbConfig>> for ZoneAwareLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ZoneAwareLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ZoneAwareLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ZoneAwareLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ZoneAwareLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // routing_enabled: optional message envoy.type.v3.Percent
  pub fn has_routing_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_routing_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn routing_enabled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_routing_enabled().then(|| self.routing_enabled())
  }
  pub fn routing_enabled(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn routing_enabled_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_routing_enabled(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // min_cluster_size: optional message google.protobuf.UInt64Value
  pub fn has_min_cluster_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_cluster_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_cluster_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_min_cluster_size().then(|| self.min_cluster_size())
  }
  pub fn min_cluster_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn min_cluster_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_min_cluster_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // fail_traffic_on_panic: optional bool
  pub fn fail_traffic_on_panic(&self) -> bool {
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
  pub fn set_fail_traffic_on_panic(&mut self, val: bool) {
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
// - `ZoneAwareLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ZoneAwareLbConfigMut<'_> {}

// SAFETY:
// - `ZoneAwareLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ZoneAwareLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ZoneAwareLbConfigMut<'msg> {
  type Proxied = ZoneAwareLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ZoneAwareLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ZoneAwareLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ZoneAwareLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ZoneAwareLbConfigMut<'msg> {
  type MutProxied = ZoneAwareLbConfig;
  fn as_mut(&mut self) -> ZoneAwareLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ZoneAwareLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ZoneAwareLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ZoneAwareLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ZoneAwareLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ZoneAwareLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ZoneAwareLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // routing_enabled: optional message envoy.type.v3.Percent
  pub fn has_routing_enabled(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_routing_enabled(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn routing_enabled_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_routing_enabled().then(|| self.routing_enabled())
  }
  pub fn routing_enabled(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn routing_enabled_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
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
  pub fn set_routing_enabled(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // min_cluster_size: optional message google.protobuf.UInt64Value
  pub fn has_min_cluster_size(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_min_cluster_size(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn min_cluster_size_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_min_cluster_size().then(|| self.min_cluster_size())
  }
  pub fn min_cluster_size(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn min_cluster_size_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
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
  pub fn set_min_cluster_size(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // fail_traffic_on_panic: optional bool
  pub fn fail_traffic_on_panic(&self) -> bool {
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
  pub fn set_fail_traffic_on_panic(&mut self, val: bool) {
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

}  // impl ZoneAwareLbConfig

impl ::std::ops::Drop for ZoneAwareLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ZoneAwareLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ZoneAwareLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> ZoneAwareLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ZoneAwareLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ZoneAwareLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ZoneAwareLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::cluster::common_lb_config::envoy__config__cluster__v3__Cluster__CommonLbConfig__ZoneAwareLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::cluster::common_lb_config::envoy__config__cluster__v3__Cluster__CommonLbConfig__ZoneAwareLbConfig_msg_init.0, &[<crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::cluster::common_lb_config::envoy__config__cluster__v3__Cluster__CommonLbConfig__ZoneAwareLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ZoneAwareLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ZoneAwareLbConfig {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ZoneAwareLbConfig {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ZoneAwareLbConfigMut<'_> {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ZoneAwareLbConfigMut<'_> {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ZoneAwareLbConfigView<'_> {
  type Msg = ZoneAwareLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ZoneAwareLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ZoneAwareLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__CommonLbConfig__LocalityWeightedLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LocalityWeightedLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LocalityWeightedLbConfig>
}

impl ::protobuf::Message for LocalityWeightedLbConfig {
  type MessageView<'msg> = LocalityWeightedLbConfigView<'msg>;
  type MessageMut<'msg> = LocalityWeightedLbConfigMut<'msg>;
}

impl ::std::default::Default for LocalityWeightedLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LocalityWeightedLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LocalityWeightedLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `LocalityWeightedLbConfigMut`.
unsafe impl ::std::marker::Sync for LocalityWeightedLbConfig {}

// SAFETY:
// - `LocalityWeightedLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LocalityWeightedLbConfig {}

impl ::protobuf::Proxied for LocalityWeightedLbConfig {
  type View<'msg> = LocalityWeightedLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LocalityWeightedLbConfig {}

impl ::protobuf::MutProxied for LocalityWeightedLbConfig {
  type Mut<'msg> = LocalityWeightedLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocalityWeightedLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityWeightedLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityWeightedLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocalityWeightedLbConfigView<'msg> {
  type Message = LocalityWeightedLbConfig;
}

impl ::std::fmt::Debug for LocalityWeightedLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocalityWeightedLbConfigView<'_> {
  fn default() -> LocalityWeightedLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityWeightedLbConfig>> for LocalityWeightedLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalityWeightedLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityWeightedLbConfigView<'msg> {

  pub fn to_owned(&self) -> LocalityWeightedLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `LocalityWeightedLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LocalityWeightedLbConfigView<'_> {}

// SAFETY:
// - `LocalityWeightedLbConfigView` is `Send` because while its alive a `LocalityWeightedLbConfigMut` cannot.
// - `LocalityWeightedLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for LocalityWeightedLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for LocalityWeightedLbConfigView<'msg> {
  type Proxied = LocalityWeightedLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, LocalityWeightedLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityWeightedLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> LocalityWeightedLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalityWeightedLbConfig> for LocalityWeightedLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalityWeightedLbConfig {
    let mut dst = LocalityWeightedLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalityWeightedLbConfig> for LocalityWeightedLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalityWeightedLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LocalityWeightedLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityWeightedLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalityWeightedLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocalityWeightedLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityWeightedLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalityWeightedLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocalityWeightedLbConfigMut<'msg> {
  type Message = LocalityWeightedLbConfig;
}

impl ::std::fmt::Debug for LocalityWeightedLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityWeightedLbConfig>> for LocalityWeightedLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityWeightedLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalityWeightedLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalityWeightedLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LocalityWeightedLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `LocalityWeightedLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LocalityWeightedLbConfigMut<'_> {}

// SAFETY:
// - `LocalityWeightedLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LocalityWeightedLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for LocalityWeightedLbConfigMut<'msg> {
  type Proxied = LocalityWeightedLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, LocalityWeightedLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalityWeightedLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LocalityWeightedLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LocalityWeightedLbConfigMut<'msg> {
  type MutProxied = LocalityWeightedLbConfig;
  fn as_mut(&mut self) -> LocalityWeightedLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocalityWeightedLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> LocalityWeightedLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LocalityWeightedLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LocalityWeightedLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocalityWeightedLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocalityWeightedLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl LocalityWeightedLbConfig

impl ::std::ops::Drop for LocalityWeightedLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LocalityWeightedLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LocalityWeightedLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> LocalityWeightedLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LocalityWeightedLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocalityWeightedLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LocalityWeightedLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::cluster::common_lb_config::envoy__config__cluster__v3__Cluster__CommonLbConfig__LocalityWeightedLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::cluster::common_lb_config::envoy__config__cluster__v3__Cluster__CommonLbConfig__LocalityWeightedLbConfig_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::cluster::common_lb_config::envoy__config__cluster__v3__Cluster__CommonLbConfig__LocalityWeightedLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalityWeightedLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalityWeightedLbConfig {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityWeightedLbConfig {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalityWeightedLbConfigMut<'_> {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityWeightedLbConfigMut<'_> {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalityWeightedLbConfigView<'_> {
  type Msg = LocalityWeightedLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalityWeightedLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalityWeightedLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__CommonLbConfig__ConsistentHashingLbConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ConsistentHashingLbConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ConsistentHashingLbConfig>
}

impl ::protobuf::Message for ConsistentHashingLbConfig {
  type MessageView<'msg> = ConsistentHashingLbConfigView<'msg>;
  type MessageMut<'msg> = ConsistentHashingLbConfigMut<'msg>;
}

impl ::std::default::Default for ConsistentHashingLbConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ConsistentHashingLbConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ConsistentHashingLbConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ConsistentHashingLbConfigMut`.
unsafe impl ::std::marker::Sync for ConsistentHashingLbConfig {}

// SAFETY:
// - `ConsistentHashingLbConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ConsistentHashingLbConfig {}

impl ::protobuf::Proxied for ConsistentHashingLbConfig {
  type View<'msg> = ConsistentHashingLbConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ConsistentHashingLbConfig {}

impl ::protobuf::MutProxied for ConsistentHashingLbConfig {
  type Mut<'msg> = ConsistentHashingLbConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ConsistentHashingLbConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConsistentHashingLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConsistentHashingLbConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ConsistentHashingLbConfigView<'msg> {
  type Message = ConsistentHashingLbConfig;
}

impl ::std::fmt::Debug for ConsistentHashingLbConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ConsistentHashingLbConfigView<'_> {
  fn default() -> ConsistentHashingLbConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ConsistentHashingLbConfig>> for ConsistentHashingLbConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ConsistentHashingLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConsistentHashingLbConfigView<'msg> {

  pub fn to_owned(&self) -> ConsistentHashingLbConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // use_hostname_for_hashing: optional bool
  pub fn use_hostname_for_hashing(self) -> bool {
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

  // hash_balance_factor: optional message google.protobuf.UInt32Value
  pub fn has_hash_balance_factor(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn hash_balance_factor_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_hash_balance_factor().then(|| self.hash_balance_factor())
  }
  pub fn hash_balance_factor(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `ConsistentHashingLbConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ConsistentHashingLbConfigView<'_> {}

// SAFETY:
// - `ConsistentHashingLbConfigView` is `Send` because while its alive a `ConsistentHashingLbConfigMut` cannot.
// - `ConsistentHashingLbConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ConsistentHashingLbConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ConsistentHashingLbConfigView<'msg> {
  type Proxied = ConsistentHashingLbConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ConsistentHashingLbConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConsistentHashingLbConfigView<'msg> {
  fn into_view<'shorter>(self) -> ConsistentHashingLbConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ConsistentHashingLbConfig> for ConsistentHashingLbConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConsistentHashingLbConfig {
    let mut dst = ConsistentHashingLbConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ConsistentHashingLbConfig> for ConsistentHashingLbConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ConsistentHashingLbConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ConsistentHashingLbConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConsistentHashingLbConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ConsistentHashingLbConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ConsistentHashingLbConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConsistentHashingLbConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ConsistentHashingLbConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ConsistentHashingLbConfigMut<'msg> {
  type Message = ConsistentHashingLbConfig;
}

impl ::std::fmt::Debug for ConsistentHashingLbConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ConsistentHashingLbConfig>> for ConsistentHashingLbConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ConsistentHashingLbConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ConsistentHashingLbConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ConsistentHashingLbConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ConsistentHashingLbConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // use_hostname_for_hashing: optional bool
  pub fn use_hostname_for_hashing(&self) -> bool {
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
  pub fn set_use_hostname_for_hashing(&mut self, val: bool) {
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

  // hash_balance_factor: optional message google.protobuf.UInt32Value
  pub fn has_hash_balance_factor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_hash_balance_factor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn hash_balance_factor_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_hash_balance_factor().then(|| self.hash_balance_factor())
  }
  pub fn hash_balance_factor(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn hash_balance_factor_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_hash_balance_factor(&mut self,
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
// - `ConsistentHashingLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ConsistentHashingLbConfigMut<'_> {}

// SAFETY:
// - `ConsistentHashingLbConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ConsistentHashingLbConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ConsistentHashingLbConfigMut<'msg> {
  type Proxied = ConsistentHashingLbConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ConsistentHashingLbConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ConsistentHashingLbConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ConsistentHashingLbConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ConsistentHashingLbConfigMut<'msg> {
  type MutProxied = ConsistentHashingLbConfig;
  fn as_mut(&mut self) -> ConsistentHashingLbConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ConsistentHashingLbConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ConsistentHashingLbConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ConsistentHashingLbConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ConsistentHashingLbConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ConsistentHashingLbConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ConsistentHashingLbConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // use_hostname_for_hashing: optional bool
  pub fn use_hostname_for_hashing(&self) -> bool {
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
  pub fn set_use_hostname_for_hashing(&mut self, val: bool) {
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

  // hash_balance_factor: optional message google.protobuf.UInt32Value
  pub fn has_hash_balance_factor(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_hash_balance_factor(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn hash_balance_factor_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_hash_balance_factor().then(|| self.hash_balance_factor())
  }
  pub fn hash_balance_factor(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn hash_balance_factor_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_hash_balance_factor(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ConsistentHashingLbConfig

impl ::std::ops::Drop for ConsistentHashingLbConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ConsistentHashingLbConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ConsistentHashingLbConfig {
  type Proxied = Self;
  fn as_view(&self) -> ConsistentHashingLbConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ConsistentHashingLbConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ConsistentHashingLbConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ConsistentHashingLbConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::cluster::common_lb_config::envoy__config__cluster__v3__Cluster__CommonLbConfig__ConsistentHashingLbConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::cluster::common_lb_config::envoy__config__cluster__v3__Cluster__CommonLbConfig__ConsistentHashingLbConfig_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::cluster::common_lb_config::envoy__config__cluster__v3__Cluster__CommonLbConfig__ConsistentHashingLbConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConsistentHashingLbConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConsistentHashingLbConfig {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConsistentHashingLbConfig {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ConsistentHashingLbConfigMut<'_> {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConsistentHashingLbConfigMut<'_> {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ConsistentHashingLbConfigView<'_> {
  type Msg = ConsistentHashingLbConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ConsistentHashingLbConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ConsistentHashingLbConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum LocalityConfigSpecifierOneof<'msg> {
  ZoneAwareLbConfig(::protobuf::View<'msg, super::super::super::cluster::common_lb_config::ZoneAwareLbConfig>) = 2,
  LocalityWeightedLbConfig(::protobuf::View<'msg, super::super::super::cluster::common_lb_config::LocalityWeightedLbConfig>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum LocalityConfigSpecifierCase {
  ZoneAwareLbConfig = 2,
  LocalityWeightedLbConfig = 3,

  not_set = 0
}

impl LocalityConfigSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<LocalityConfigSpecifierCase> {
    match v {
      0 => Some(LocalityConfigSpecifierCase::not_set),
      2 => Some(LocalityConfigSpecifierCase::ZoneAwareLbConfig),
      3 => Some(LocalityConfigSpecifierCase::LocalityWeightedLbConfig),
      _ => None
    }
  }
}
}  // pub mod common_lb_config

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__RefreshRate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RefreshRate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RefreshRate>
}

impl ::protobuf::Message for RefreshRate {
  type MessageView<'msg> = RefreshRateView<'msg>;
  type MessageMut<'msg> = RefreshRateMut<'msg>;
}

impl ::std::default::Default for RefreshRate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RefreshRate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RefreshRate` is `Sync` because it does not implement interior mutability.
//    Neither does `RefreshRateMut`.
unsafe impl ::std::marker::Sync for RefreshRate {}

// SAFETY:
// - `RefreshRate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RefreshRate {}

impl ::protobuf::Proxied for RefreshRate {
  type View<'msg> = RefreshRateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RefreshRate {}

impl ::protobuf::MutProxied for RefreshRate {
  type Mut<'msg> = RefreshRateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RefreshRateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RefreshRate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RefreshRateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RefreshRateView<'msg> {
  type Message = RefreshRate;
}

impl ::std::fmt::Debug for RefreshRateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RefreshRateView<'_> {
  fn default() -> RefreshRateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RefreshRate>> for RefreshRateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RefreshRate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RefreshRateView<'msg> {

  pub fn to_owned(&self) -> RefreshRate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // base_interval: optional message google.protobuf.Duration
  pub fn has_base_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn base_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_base_interval().then(|| self.base_interval())
  }
  pub fn base_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // max_interval: optional message google.protobuf.Duration
  pub fn has_max_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn max_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_interval().then(|| self.max_interval())
  }
  pub fn max_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `RefreshRateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RefreshRateView<'_> {}

// SAFETY:
// - `RefreshRateView` is `Send` because while its alive a `RefreshRateMut` cannot.
// - `RefreshRateView` does not use thread-local data.
unsafe impl ::std::marker::Send for RefreshRateView<'_> {}

impl<'msg> ::protobuf::AsView for RefreshRateView<'msg> {
  type Proxied = RefreshRate;
  fn as_view(&self) -> ::protobuf::View<'msg, RefreshRate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RefreshRateView<'msg> {
  fn into_view<'shorter>(self) -> RefreshRateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RefreshRate> for RefreshRateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RefreshRate {
    let mut dst = RefreshRate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RefreshRate> for RefreshRateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RefreshRate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RefreshRate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RefreshRateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RefreshRateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RefreshRateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RefreshRate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RefreshRateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RefreshRateMut<'msg> {
  type Message = RefreshRate;
}

impl ::std::fmt::Debug for RefreshRateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RefreshRate>> for RefreshRateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RefreshRate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RefreshRateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RefreshRate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RefreshRate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // base_interval: optional message google.protobuf.Duration
  pub fn has_base_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_base_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn base_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_base_interval().then(|| self.base_interval())
  }
  pub fn base_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn base_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_base_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_interval: optional message google.protobuf.Duration
  pub fn has_max_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_interval().then(|| self.max_interval())
  }
  pub fn max_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

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
// - `RefreshRateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RefreshRateMut<'_> {}

// SAFETY:
// - `RefreshRateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RefreshRateMut<'_> {}

impl<'msg> ::protobuf::AsView for RefreshRateMut<'msg> {
  type Proxied = RefreshRate;
  fn as_view(&self) -> ::protobuf::View<'_, RefreshRate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RefreshRateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RefreshRate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RefreshRateMut<'msg> {
  type MutProxied = RefreshRate;
  fn as_mut(&mut self) -> RefreshRateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RefreshRateMut<'msg> {
  fn into_mut<'shorter>(self) -> RefreshRateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RefreshRate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RefreshRate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RefreshRateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RefreshRateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // base_interval: optional message google.protobuf.Duration
  pub fn has_base_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_base_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn base_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_base_interval().then(|| self.base_interval())
  }
  pub fn base_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn base_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_base_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // max_interval: optional message google.protobuf.Duration
  pub fn has_max_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_max_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn max_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_interval().then(|| self.max_interval())
  }
  pub fn max_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_max_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl RefreshRate

impl ::std::ops::Drop for RefreshRate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RefreshRate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RefreshRate {
  type Proxied = Self;
  fn as_view(&self) -> RefreshRateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RefreshRate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RefreshRateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RefreshRate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__RefreshRate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__RefreshRate_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__RefreshRate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RefreshRate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RefreshRate {
  type Msg = RefreshRate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RefreshRate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RefreshRate {
  type Msg = RefreshRate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RefreshRate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RefreshRateMut<'_> {
  type Msg = RefreshRate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RefreshRate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RefreshRateMut<'_> {
  type Msg = RefreshRate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RefreshRate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RefreshRateView<'_> {
  type Msg = RefreshRate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RefreshRate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RefreshRateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__PreconnectPolicy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PreconnectPolicy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PreconnectPolicy>
}

impl ::protobuf::Message for PreconnectPolicy {
  type MessageView<'msg> = PreconnectPolicyView<'msg>;
  type MessageMut<'msg> = PreconnectPolicyMut<'msg>;
}

impl ::std::default::Default for PreconnectPolicy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PreconnectPolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PreconnectPolicy` is `Sync` because it does not implement interior mutability.
//    Neither does `PreconnectPolicyMut`.
unsafe impl ::std::marker::Sync for PreconnectPolicy {}

// SAFETY:
// - `PreconnectPolicy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PreconnectPolicy {}

impl ::protobuf::Proxied for PreconnectPolicy {
  type View<'msg> = PreconnectPolicyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PreconnectPolicy {}

impl ::protobuf::MutProxied for PreconnectPolicy {
  type Mut<'msg> = PreconnectPolicyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PreconnectPolicyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PreconnectPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PreconnectPolicyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PreconnectPolicyView<'msg> {
  type Message = PreconnectPolicy;
}

impl ::std::fmt::Debug for PreconnectPolicyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PreconnectPolicyView<'_> {
  fn default() -> PreconnectPolicyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PreconnectPolicy>> for PreconnectPolicyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PreconnectPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PreconnectPolicyView<'msg> {

  pub fn to_owned(&self) -> PreconnectPolicy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // per_upstream_preconnect_ratio: optional message google.protobuf.DoubleValue
  pub fn has_per_upstream_preconnect_ratio(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn per_upstream_preconnect_ratio_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DoubleValueView<'msg>> {
    self.has_per_upstream_preconnect_ratio().then(|| self.per_upstream_preconnect_ratio())
  }
  pub fn per_upstream_preconnect_ratio(self) -> ::protobuf_well_known_types::DoubleValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DoubleValueView::default())
  }

  // predictive_preconnect_ratio: optional message google.protobuf.DoubleValue
  pub fn has_predictive_preconnect_ratio(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn predictive_preconnect_ratio_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DoubleValueView<'msg>> {
    self.has_predictive_preconnect_ratio().then(|| self.predictive_preconnect_ratio())
  }
  pub fn predictive_preconnect_ratio(self) -> ::protobuf_well_known_types::DoubleValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DoubleValueView::default())
  }

}

// SAFETY:
// - `PreconnectPolicyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PreconnectPolicyView<'_> {}

// SAFETY:
// - `PreconnectPolicyView` is `Send` because while its alive a `PreconnectPolicyMut` cannot.
// - `PreconnectPolicyView` does not use thread-local data.
unsafe impl ::std::marker::Send for PreconnectPolicyView<'_> {}

impl<'msg> ::protobuf::AsView for PreconnectPolicyView<'msg> {
  type Proxied = PreconnectPolicy;
  fn as_view(&self) -> ::protobuf::View<'msg, PreconnectPolicy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PreconnectPolicyView<'msg> {
  fn into_view<'shorter>(self) -> PreconnectPolicyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PreconnectPolicy> for PreconnectPolicyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PreconnectPolicy {
    let mut dst = PreconnectPolicy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PreconnectPolicy> for PreconnectPolicyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PreconnectPolicy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PreconnectPolicy {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PreconnectPolicyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PreconnectPolicyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PreconnectPolicyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PreconnectPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PreconnectPolicyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PreconnectPolicyMut<'msg> {
  type Message = PreconnectPolicy;
}

impl ::std::fmt::Debug for PreconnectPolicyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PreconnectPolicy>> for PreconnectPolicyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PreconnectPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PreconnectPolicyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PreconnectPolicy> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PreconnectPolicy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // per_upstream_preconnect_ratio: optional message google.protobuf.DoubleValue
  pub fn has_per_upstream_preconnect_ratio(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_per_upstream_preconnect_ratio(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn per_upstream_preconnect_ratio_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DoubleValueView<'_>> {
    self.has_per_upstream_preconnect_ratio().then(|| self.per_upstream_preconnect_ratio())
  }
  pub fn per_upstream_preconnect_ratio(&self) -> ::protobuf_well_known_types::DoubleValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DoubleValueView::default())
  }
  pub fn per_upstream_preconnect_ratio_mut(&mut self) -> ::protobuf_well_known_types::DoubleValueMut<'_> {
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
  pub fn set_per_upstream_preconnect_ratio(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::DoubleValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // predictive_preconnect_ratio: optional message google.protobuf.DoubleValue
  pub fn has_predictive_preconnect_ratio(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_predictive_preconnect_ratio(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn predictive_preconnect_ratio_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DoubleValueView<'_>> {
    self.has_predictive_preconnect_ratio().then(|| self.predictive_preconnect_ratio())
  }
  pub fn predictive_preconnect_ratio(&self) -> ::protobuf_well_known_types::DoubleValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DoubleValueView::default())
  }
  pub fn predictive_preconnect_ratio_mut(&mut self) -> ::protobuf_well_known_types::DoubleValueMut<'_> {
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
  pub fn set_predictive_preconnect_ratio(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::DoubleValue>) {

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
// - `PreconnectPolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PreconnectPolicyMut<'_> {}

// SAFETY:
// - `PreconnectPolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PreconnectPolicyMut<'_> {}

impl<'msg> ::protobuf::AsView for PreconnectPolicyMut<'msg> {
  type Proxied = PreconnectPolicy;
  fn as_view(&self) -> ::protobuf::View<'_, PreconnectPolicy> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PreconnectPolicyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PreconnectPolicy>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PreconnectPolicyMut<'msg> {
  type MutProxied = PreconnectPolicy;
  fn as_mut(&mut self) -> PreconnectPolicyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PreconnectPolicyMut<'msg> {
  fn into_mut<'shorter>(self) -> PreconnectPolicyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PreconnectPolicy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PreconnectPolicy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PreconnectPolicyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PreconnectPolicyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // per_upstream_preconnect_ratio: optional message google.protobuf.DoubleValue
  pub fn has_per_upstream_preconnect_ratio(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_per_upstream_preconnect_ratio(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn per_upstream_preconnect_ratio_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DoubleValueView<'_>> {
    self.has_per_upstream_preconnect_ratio().then(|| self.per_upstream_preconnect_ratio())
  }
  pub fn per_upstream_preconnect_ratio(&self) -> ::protobuf_well_known_types::DoubleValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DoubleValueView::default())
  }
  pub fn per_upstream_preconnect_ratio_mut(&mut self) -> ::protobuf_well_known_types::DoubleValueMut<'_> {
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
  pub fn set_per_upstream_preconnect_ratio(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::DoubleValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // predictive_preconnect_ratio: optional message google.protobuf.DoubleValue
  pub fn has_predictive_preconnect_ratio(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_predictive_preconnect_ratio(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn predictive_preconnect_ratio_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DoubleValueView<'_>> {
    self.has_predictive_preconnect_ratio().then(|| self.predictive_preconnect_ratio())
  }
  pub fn predictive_preconnect_ratio(&self) -> ::protobuf_well_known_types::DoubleValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DoubleValueView::default())
  }
  pub fn predictive_preconnect_ratio_mut(&mut self) -> ::protobuf_well_known_types::DoubleValueMut<'_> {
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
  pub fn set_predictive_preconnect_ratio(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::DoubleValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl PreconnectPolicy

impl ::std::ops::Drop for PreconnectPolicy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PreconnectPolicy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PreconnectPolicy {
  type Proxied = Self;
  fn as_view(&self) -> PreconnectPolicyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PreconnectPolicy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PreconnectPolicyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PreconnectPolicy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__PreconnectPolicy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__PreconnectPolicy_msg_init.0, &[<::protobuf_well_known_types::DoubleValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::DoubleValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__PreconnectPolicy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PreconnectPolicy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PreconnectPolicy {
  type Msg = PreconnectPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PreconnectPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PreconnectPolicy {
  type Msg = PreconnectPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PreconnectPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PreconnectPolicyMut<'_> {
  type Msg = PreconnectPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PreconnectPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PreconnectPolicyMut<'_> {
  type Msg = PreconnectPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PreconnectPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PreconnectPolicyView<'_> {
  type Msg = PreconnectPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PreconnectPolicy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PreconnectPolicyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__Cluster__TypedExtensionProtocolOptionsEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct TypedExtensionProtocolOptionsEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TypedExtensionProtocolOptionsEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster::envoy__config__cluster__v3__Cluster__TypedExtensionProtocolOptionsEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster::envoy__config__cluster__v3__Cluster__TypedExtensionProtocolOptionsEntry_msg_init.0, &[<::protobuf_well_known_types::Any as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster::envoy__config__cluster__v3__Cluster__TypedExtensionProtocolOptionsEntry_msg_init.0)
      }).0
    }
  }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DiscoveryType(i32);

#[allow(non_upper_case_globals)]
impl DiscoveryType {
  pub const Static: DiscoveryType = DiscoveryType(0);
  pub const StrictDns: DiscoveryType = DiscoveryType(1);
  pub const LogicalDns: DiscoveryType = DiscoveryType(2);
  pub const Eds: DiscoveryType = DiscoveryType(3);
  pub const OriginalDst: DiscoveryType = DiscoveryType(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Static",
      1 => "StrictDns",
      2 => "LogicalDns",
      3 => "Eds",
      4 => "OriginalDst",
      _ => return None
    })
  }
}

impl ::std::convert::From<DiscoveryType> for i32 {
  fn from(val: DiscoveryType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for DiscoveryType {
  fn from(val: i32) -> DiscoveryType {
    Self(val)
  }
}

impl ::std::default::Default for DiscoveryType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for DiscoveryType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "DiscoveryType::{}", constant_name)
    } else {
      write!(f, "DiscoveryType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for DiscoveryType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for DiscoveryType {}

impl ::protobuf::Proxied for DiscoveryType {
  type View<'a> = DiscoveryType;
}

impl ::protobuf::AsView for DiscoveryType {
  type Proxied = DiscoveryType;

  fn as_view(&self) -> DiscoveryType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiscoveryType {
  fn into_view<'shorter>(self) -> DiscoveryType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for DiscoveryType {
  const NAME: &'static str = "DiscoveryType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for DiscoveryType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LbPolicy(i32);

#[allow(non_upper_case_globals)]
impl LbPolicy {
  pub const RoundRobin: LbPolicy = LbPolicy(0);
  pub const LeastRequest: LbPolicy = LbPolicy(1);
  pub const RingHash: LbPolicy = LbPolicy(2);
  pub const Random: LbPolicy = LbPolicy(3);
  pub const Maglev: LbPolicy = LbPolicy(5);
  pub const ClusterProvided: LbPolicy = LbPolicy(6);
  pub const LoadBalancingPolicyConfig: LbPolicy = LbPolicy(7);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "RoundRobin",
      1 => "LeastRequest",
      2 => "RingHash",
      3 => "Random",
      5 => "Maglev",
      6 => "ClusterProvided",
      7 => "LoadBalancingPolicyConfig",
      _ => return None
    })
  }
}

impl ::std::convert::From<LbPolicy> for i32 {
  fn from(val: LbPolicy) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for LbPolicy {
  fn from(val: i32) -> LbPolicy {
    Self(val)
  }
}

impl ::std::default::Default for LbPolicy {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for LbPolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "LbPolicy::{}", constant_name)
    } else {
      write!(f, "LbPolicy::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for LbPolicy {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for LbPolicy {}

impl ::protobuf::Proxied for LbPolicy {
  type View<'a> = LbPolicy;
}

impl ::protobuf::AsView for LbPolicy {
  type Proxied = LbPolicy;

  fn as_view(&self) -> LbPolicy {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LbPolicy {
  fn into_view<'shorter>(self) -> LbPolicy where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for LbPolicy {
  const NAME: &'static str = "LbPolicy";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|5|6|7)
  }
}

impl ::protobuf::__internal::EntityType for LbPolicy {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DnsLookupFamily(i32);

#[allow(non_upper_case_globals)]
impl DnsLookupFamily {
  pub const Auto: DnsLookupFamily = DnsLookupFamily(0);
  pub const V4Only: DnsLookupFamily = DnsLookupFamily(1);
  pub const V6Only: DnsLookupFamily = DnsLookupFamily(2);
  pub const V4Preferred: DnsLookupFamily = DnsLookupFamily(3);
  pub const All: DnsLookupFamily = DnsLookupFamily(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Auto",
      1 => "V4Only",
      2 => "V6Only",
      3 => "V4Preferred",
      4 => "All",
      _ => return None
    })
  }
}

impl ::std::convert::From<DnsLookupFamily> for i32 {
  fn from(val: DnsLookupFamily) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for DnsLookupFamily {
  fn from(val: i32) -> DnsLookupFamily {
    Self(val)
  }
}

impl ::std::default::Default for DnsLookupFamily {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for DnsLookupFamily {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "DnsLookupFamily::{}", constant_name)
    } else {
      write!(f, "DnsLookupFamily::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for DnsLookupFamily {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for DnsLookupFamily {}

impl ::protobuf::Proxied for DnsLookupFamily {
  type View<'a> = DnsLookupFamily;
}

impl ::protobuf::AsView for DnsLookupFamily {
  type Proxied = DnsLookupFamily;

  fn as_view(&self) -> DnsLookupFamily {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DnsLookupFamily {
  fn into_view<'shorter>(self) -> DnsLookupFamily where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for DnsLookupFamily {
  const NAME: &'static str = "DnsLookupFamily";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for DnsLookupFamily {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClusterProtocolSelection(i32);

#[allow(non_upper_case_globals)]
impl ClusterProtocolSelection {
  pub const UseConfiguredProtocol: ClusterProtocolSelection = ClusterProtocolSelection(0);
  pub const UseDownstreamProtocol: ClusterProtocolSelection = ClusterProtocolSelection(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "UseConfiguredProtocol",
      1 => "UseDownstreamProtocol",
      _ => return None
    })
  }
}

impl ::std::convert::From<ClusterProtocolSelection> for i32 {
  fn from(val: ClusterProtocolSelection) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ClusterProtocolSelection {
  fn from(val: i32) -> ClusterProtocolSelection {
    Self(val)
  }
}

impl ::std::default::Default for ClusterProtocolSelection {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ClusterProtocolSelection {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ClusterProtocolSelection::{}", constant_name)
    } else {
      write!(f, "ClusterProtocolSelection::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ClusterProtocolSelection {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ClusterProtocolSelection {}

impl ::protobuf::Proxied for ClusterProtocolSelection {
  type View<'a> = ClusterProtocolSelection;
}

impl ::protobuf::AsView for ClusterProtocolSelection {
  type Proxied = ClusterProtocolSelection;

  fn as_view(&self) -> ClusterProtocolSelection {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterProtocolSelection {
  fn into_view<'shorter>(self) -> ClusterProtocolSelection where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ClusterProtocolSelection {
  const NAME: &'static str = "ClusterProtocolSelection";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for ClusterProtocolSelection {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ClusterDiscoveryTypeOneof<'msg> {
  Type(::protobuf::View<'msg, super::super::cluster::DiscoveryType>) = 2,
  ClusterType(::protobuf::View<'msg, super::super::cluster::CustomClusterType>) = 38,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ClusterDiscoveryTypeCase {
  Type = 2,
  ClusterType = 38,

  not_set = 0
}

impl ClusterDiscoveryTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ClusterDiscoveryTypeCase> {
    match v {
      0 => Some(ClusterDiscoveryTypeCase::not_set),
      2 => Some(ClusterDiscoveryTypeCase::Type),
      38 => Some(ClusterDiscoveryTypeCase::ClusterType),
      _ => None
    }
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum LbConfigOneof<'msg> {
  RingHashLbConfig(::protobuf::View<'msg, super::super::cluster::RingHashLbConfig>) = 23,
  MaglevLbConfig(::protobuf::View<'msg, super::super::cluster::MaglevLbConfig>) = 52,
  OriginalDstLbConfig(::protobuf::View<'msg, super::super::cluster::OriginalDstLbConfig>) = 34,
  LeastRequestLbConfig(::protobuf::View<'msg, super::super::cluster::LeastRequestLbConfig>) = 37,
  RoundRobinLbConfig(::protobuf::View<'msg, super::super::cluster::RoundRobinLbConfig>) = 56,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum LbConfigCase {
  RingHashLbConfig = 23,
  MaglevLbConfig = 52,
  OriginalDstLbConfig = 34,
  LeastRequestLbConfig = 37,
  RoundRobinLbConfig = 56,

  not_set = 0
}

impl LbConfigCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<LbConfigCase> {
    match v {
      0 => Some(LbConfigCase::not_set),
      23 => Some(LbConfigCase::RingHashLbConfig),
      52 => Some(LbConfigCase::MaglevLbConfig),
      34 => Some(LbConfigCase::OriginalDstLbConfig),
      37 => Some(LbConfigCase::LeastRequestLbConfig),
      56 => Some(LbConfigCase::RoundRobinLbConfig),
      _ => None
    }
  }
}
}  // pub mod cluster


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__LoadBalancingPolicy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LoadBalancingPolicy {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LoadBalancingPolicy>
}

impl ::protobuf::Message for LoadBalancingPolicy {
  type MessageView<'msg> = LoadBalancingPolicyView<'msg>;
  type MessageMut<'msg> = LoadBalancingPolicyMut<'msg>;
}

impl ::std::default::Default for LoadBalancingPolicy {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LoadBalancingPolicy {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LoadBalancingPolicy` is `Sync` because it does not implement interior mutability.
//    Neither does `LoadBalancingPolicyMut`.
unsafe impl ::std::marker::Sync for LoadBalancingPolicy {}

// SAFETY:
// - `LoadBalancingPolicy` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LoadBalancingPolicy {}

impl ::protobuf::Proxied for LoadBalancingPolicy {
  type View<'msg> = LoadBalancingPolicyView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LoadBalancingPolicy {}

impl ::protobuf::MutProxied for LoadBalancingPolicy {
  type Mut<'msg> = LoadBalancingPolicyMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LoadBalancingPolicyView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LoadBalancingPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LoadBalancingPolicyView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LoadBalancingPolicyView<'msg> {
  type Message = LoadBalancingPolicy;
}

impl ::std::fmt::Debug for LoadBalancingPolicyView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LoadBalancingPolicyView<'_> {
  fn default() -> LoadBalancingPolicyView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LoadBalancingPolicy>> for LoadBalancingPolicyView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LoadBalancingPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LoadBalancingPolicyView<'msg> {

  pub fn to_owned(&self) -> LoadBalancingPolicy {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // policies: repeated message envoy.config.cluster.v3.LoadBalancingPolicy.Policy
  pub fn policies(self) -> ::protobuf::RepeatedView<'msg, super::load_balancing_policy::Policy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::load_balancing_policy::Policy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `LoadBalancingPolicyView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LoadBalancingPolicyView<'_> {}

// SAFETY:
// - `LoadBalancingPolicyView` is `Send` because while its alive a `LoadBalancingPolicyMut` cannot.
// - `LoadBalancingPolicyView` does not use thread-local data.
unsafe impl ::std::marker::Send for LoadBalancingPolicyView<'_> {}

impl<'msg> ::protobuf::AsView for LoadBalancingPolicyView<'msg> {
  type Proxied = LoadBalancingPolicy;
  fn as_view(&self) -> ::protobuf::View<'msg, LoadBalancingPolicy> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LoadBalancingPolicyView<'msg> {
  fn into_view<'shorter>(self) -> LoadBalancingPolicyView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LoadBalancingPolicy> for LoadBalancingPolicyView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LoadBalancingPolicy {
    let mut dst = LoadBalancingPolicy::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LoadBalancingPolicy> for LoadBalancingPolicyMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LoadBalancingPolicy {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LoadBalancingPolicy {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LoadBalancingPolicyView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LoadBalancingPolicyMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LoadBalancingPolicyMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadBalancingPolicy>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LoadBalancingPolicyMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LoadBalancingPolicyMut<'msg> {
  type Message = LoadBalancingPolicy;
}

impl ::std::fmt::Debug for LoadBalancingPolicyMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LoadBalancingPolicy>> for LoadBalancingPolicyMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadBalancingPolicy>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LoadBalancingPolicyMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadBalancingPolicy> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LoadBalancingPolicy {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // policies: repeated message envoy.config.cluster.v3.LoadBalancingPolicy.Policy
  pub fn policies(&self) -> ::protobuf::RepeatedView<'_, super::load_balancing_policy::Policy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::load_balancing_policy::Policy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn policies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::load_balancing_policy::Policy> {
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
  pub fn set_policies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::load_balancing_policy::Policy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `LoadBalancingPolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LoadBalancingPolicyMut<'_> {}

// SAFETY:
// - `LoadBalancingPolicyMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LoadBalancingPolicyMut<'_> {}

impl<'msg> ::protobuf::AsView for LoadBalancingPolicyMut<'msg> {
  type Proxied = LoadBalancingPolicy;
  fn as_view(&self) -> ::protobuf::View<'_, LoadBalancingPolicy> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LoadBalancingPolicyMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LoadBalancingPolicy>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LoadBalancingPolicyMut<'msg> {
  type MutProxied = LoadBalancingPolicy;
  fn as_mut(&mut self) -> LoadBalancingPolicyMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LoadBalancingPolicyMut<'msg> {
  fn into_mut<'shorter>(self) -> LoadBalancingPolicyMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LoadBalancingPolicy {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LoadBalancingPolicy> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LoadBalancingPolicyView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LoadBalancingPolicyMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // policies: repeated message envoy.config.cluster.v3.LoadBalancingPolicy.Policy
  pub fn policies(&self) -> ::protobuf::RepeatedView<'_, super::load_balancing_policy::Policy> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::load_balancing_policy::Policy>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn policies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::load_balancing_policy::Policy> {
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
  pub fn set_policies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::load_balancing_policy::Policy>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl LoadBalancingPolicy

impl ::std::ops::Drop for LoadBalancingPolicy {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LoadBalancingPolicy {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LoadBalancingPolicy {
  type Proxied = Self;
  fn as_view(&self) -> LoadBalancingPolicyView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LoadBalancingPolicy {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LoadBalancingPolicyMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LoadBalancingPolicy {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__cluster__v3__LoadBalancingPolicy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__cluster__v3__LoadBalancingPolicy_msg_init.0, &[<super::load_balancing_policy::Policy as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__cluster__v3__LoadBalancingPolicy_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LoadBalancingPolicy {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LoadBalancingPolicy {
  type Msg = LoadBalancingPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadBalancingPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadBalancingPolicy {
  type Msg = LoadBalancingPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadBalancingPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LoadBalancingPolicyMut<'_> {
  type Msg = LoadBalancingPolicy;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadBalancingPolicy> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadBalancingPolicyMut<'_> {
  type Msg = LoadBalancingPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadBalancingPolicy> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadBalancingPolicyView<'_> {
  type Msg = LoadBalancingPolicy;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadBalancingPolicy> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LoadBalancingPolicyMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod load_balancing_policy {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__LoadBalancingPolicy__Policy_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // typed_extension_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_extension_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn typed_extension_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_typed_extension_config().then(|| self.typed_extension_config())
  }
  pub fn typed_extension_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
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

  // typed_extension_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_extension_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_typed_extension_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn typed_extension_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_extension_config().then(|| self.typed_extension_config())
  }
  pub fn typed_extension_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_extension_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_typed_extension_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
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

  // typed_extension_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_extension_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_typed_extension_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn typed_extension_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_extension_config().then(|| self.typed_extension_config())
  }
  pub fn typed_extension_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_extension_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_typed_extension_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
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
        super::super::load_balancing_policy::envoy__config__cluster__v3__LoadBalancingPolicy__Policy_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$c3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::load_balancing_policy::envoy__config__cluster__v3__LoadBalancingPolicy__Policy_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::load_balancing_policy::envoy__config__cluster__v3__LoadBalancingPolicy__Policy_msg_init.0)
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



}  // pub mod load_balancing_policy


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__UpstreamConnectionOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpstreamConnectionOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpstreamConnectionOptions>
}

impl ::protobuf::Message for UpstreamConnectionOptions {
  type MessageView<'msg> = UpstreamConnectionOptionsView<'msg>;
  type MessageMut<'msg> = UpstreamConnectionOptionsMut<'msg>;
}

impl ::std::default::Default for UpstreamConnectionOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpstreamConnectionOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpstreamConnectionOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `UpstreamConnectionOptionsMut`.
unsafe impl ::std::marker::Sync for UpstreamConnectionOptions {}

// SAFETY:
// - `UpstreamConnectionOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamConnectionOptions {}

impl ::protobuf::Proxied for UpstreamConnectionOptions {
  type View<'msg> = UpstreamConnectionOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpstreamConnectionOptions {}

impl ::protobuf::MutProxied for UpstreamConnectionOptions {
  type Mut<'msg> = UpstreamConnectionOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpstreamConnectionOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamConnectionOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamConnectionOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpstreamConnectionOptionsView<'msg> {
  type Message = UpstreamConnectionOptions;
}

impl ::std::fmt::Debug for UpstreamConnectionOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpstreamConnectionOptionsView<'_> {
  fn default() -> UpstreamConnectionOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamConnectionOptions>> for UpstreamConnectionOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamConnectionOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamConnectionOptionsView<'msg> {

  pub fn to_owned(&self) -> UpstreamConnectionOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // tcp_keepalive: optional message envoy.config.core.v3.TcpKeepalive
  pub fn has_tcp_keepalive(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn tcp_keepalive_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'msg>> {
    self.has_tcp_keepalive().then(|| self.tcp_keepalive())
  }
  pub fn tcp_keepalive(self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView::default())
  }

  // set_local_interface_name_on_upstream_connections: optional bool
  pub fn set_local_interface_name_on_upstream_connections(self) -> bool {
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

  // happy_eyeballs_config: optional message envoy.config.cluster.v3.UpstreamConnectionOptions.HappyEyeballsConfig
  pub fn has_happy_eyeballs_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn happy_eyeballs_config_opt(self) -> ::std::option::Option<super::upstream_connection_options::HappyEyeballsConfigView<'msg>> {
    self.has_happy_eyeballs_config().then(|| self.happy_eyeballs_config())
  }
  pub fn happy_eyeballs_config(self) -> super::upstream_connection_options::HappyEyeballsConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::upstream_connection_options::HappyEyeballsConfigView::default())
  }

}

// SAFETY:
// - `UpstreamConnectionOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpstreamConnectionOptionsView<'_> {}

// SAFETY:
// - `UpstreamConnectionOptionsView` is `Send` because while its alive a `UpstreamConnectionOptionsMut` cannot.
// - `UpstreamConnectionOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamConnectionOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamConnectionOptionsView<'msg> {
  type Proxied = UpstreamConnectionOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, UpstreamConnectionOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamConnectionOptionsView<'msg> {
  fn into_view<'shorter>(self) -> UpstreamConnectionOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamConnectionOptions> for UpstreamConnectionOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamConnectionOptions {
    let mut dst = UpstreamConnectionOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamConnectionOptions> for UpstreamConnectionOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamConnectionOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UpstreamConnectionOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamConnectionOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamConnectionOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpstreamConnectionOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamConnectionOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamConnectionOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpstreamConnectionOptionsMut<'msg> {
  type Message = UpstreamConnectionOptions;
}

impl ::std::fmt::Debug for UpstreamConnectionOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamConnectionOptions>> for UpstreamConnectionOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamConnectionOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamConnectionOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamConnectionOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UpstreamConnectionOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // tcp_keepalive: optional message envoy.config.core.v3.TcpKeepalive
  pub fn has_tcp_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_tcp_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn tcp_keepalive_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_>> {
    self.has_tcp_keepalive().then(|| self.tcp_keepalive())
  }
  pub fn tcp_keepalive(&self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView::default())
  }
  pub fn tcp_keepalive_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveMut<'_> {
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
  pub fn set_tcp_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::TcpKeepalive>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // set_local_interface_name_on_upstream_connections: optional bool
  pub fn set_local_interface_name_on_upstream_connections(&self) -> bool {
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
  pub fn set_set_local_interface_name_on_upstream_connections(&mut self, val: bool) {
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

  // happy_eyeballs_config: optional message envoy.config.cluster.v3.UpstreamConnectionOptions.HappyEyeballsConfig
  pub fn has_happy_eyeballs_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_happy_eyeballs_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn happy_eyeballs_config_opt(&self) -> ::std::option::Option<super::upstream_connection_options::HappyEyeballsConfigView<'_>> {
    self.has_happy_eyeballs_config().then(|| self.happy_eyeballs_config())
  }
  pub fn happy_eyeballs_config(&self) -> super::upstream_connection_options::HappyEyeballsConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::upstream_connection_options::HappyEyeballsConfigView::default())
  }
  pub fn happy_eyeballs_config_mut(&mut self) -> super::upstream_connection_options::HappyEyeballsConfigMut<'_> {
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
  pub fn set_happy_eyeballs_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::upstream_connection_options::HappyEyeballsConfig>) {

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
// - `UpstreamConnectionOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpstreamConnectionOptionsMut<'_> {}

// SAFETY:
// - `UpstreamConnectionOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpstreamConnectionOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamConnectionOptionsMut<'msg> {
  type Proxied = UpstreamConnectionOptions;
  fn as_view(&self) -> ::protobuf::View<'_, UpstreamConnectionOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamConnectionOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpstreamConnectionOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpstreamConnectionOptionsMut<'msg> {
  type MutProxied = UpstreamConnectionOptions;
  fn as_mut(&mut self) -> UpstreamConnectionOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpstreamConnectionOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> UpstreamConnectionOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpstreamConnectionOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpstreamConnectionOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpstreamConnectionOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpstreamConnectionOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // tcp_keepalive: optional message envoy.config.core.v3.TcpKeepalive
  pub fn has_tcp_keepalive(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_tcp_keepalive(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn tcp_keepalive_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_>> {
    self.has_tcp_keepalive().then(|| self.tcp_keepalive())
  }
  pub fn tcp_keepalive(&self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveView::default())
  }
  pub fn tcp_keepalive_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::TcpKeepaliveMut<'_> {
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
  pub fn set_tcp_keepalive(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::TcpKeepalive>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // set_local_interface_name_on_upstream_connections: optional bool
  pub fn set_local_interface_name_on_upstream_connections(&self) -> bool {
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
  pub fn set_set_local_interface_name_on_upstream_connections(&mut self, val: bool) {
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

  // happy_eyeballs_config: optional message envoy.config.cluster.v3.UpstreamConnectionOptions.HappyEyeballsConfig
  pub fn has_happy_eyeballs_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_happy_eyeballs_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn happy_eyeballs_config_opt(&self) -> ::std::option::Option<super::upstream_connection_options::HappyEyeballsConfigView<'_>> {
    self.has_happy_eyeballs_config().then(|| self.happy_eyeballs_config())
  }
  pub fn happy_eyeballs_config(&self) -> super::upstream_connection_options::HappyEyeballsConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::upstream_connection_options::HappyEyeballsConfigView::default())
  }
  pub fn happy_eyeballs_config_mut(&mut self) -> super::upstream_connection_options::HappyEyeballsConfigMut<'_> {
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
  pub fn set_happy_eyeballs_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::upstream_connection_options::HappyEyeballsConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl UpstreamConnectionOptions

impl ::std::ops::Drop for UpstreamConnectionOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpstreamConnectionOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpstreamConnectionOptions {
  type Proxied = Self;
  fn as_view(&self) -> UpstreamConnectionOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpstreamConnectionOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpstreamConnectionOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpstreamConnectionOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__cluster__v3__UpstreamConnectionOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3/P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__cluster__v3__UpstreamConnectionOptions_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::TcpKeepalive as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::upstream_connection_options::HappyEyeballsConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__cluster__v3__UpstreamConnectionOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamConnectionOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamConnectionOptions {
  type Msg = UpstreamConnectionOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamConnectionOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamConnectionOptions {
  type Msg = UpstreamConnectionOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamConnectionOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamConnectionOptionsMut<'_> {
  type Msg = UpstreamConnectionOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamConnectionOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamConnectionOptionsMut<'_> {
  type Msg = UpstreamConnectionOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamConnectionOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamConnectionOptionsView<'_> {
  type Msg = UpstreamConnectionOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamConnectionOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamConnectionOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod upstream_connection_options {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__UpstreamConnectionOptions__HappyEyeballsConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HappyEyeballsConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HappyEyeballsConfig>
}

impl ::protobuf::Message for HappyEyeballsConfig {
  type MessageView<'msg> = HappyEyeballsConfigView<'msg>;
  type MessageMut<'msg> = HappyEyeballsConfigMut<'msg>;
}

impl ::std::default::Default for HappyEyeballsConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HappyEyeballsConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HappyEyeballsConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `HappyEyeballsConfigMut`.
unsafe impl ::std::marker::Sync for HappyEyeballsConfig {}

// SAFETY:
// - `HappyEyeballsConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HappyEyeballsConfig {}

impl ::protobuf::Proxied for HappyEyeballsConfig {
  type View<'msg> = HappyEyeballsConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HappyEyeballsConfig {}

impl ::protobuf::MutProxied for HappyEyeballsConfig {
  type Mut<'msg> = HappyEyeballsConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HappyEyeballsConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HappyEyeballsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HappyEyeballsConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HappyEyeballsConfigView<'msg> {
  type Message = HappyEyeballsConfig;
}

impl ::std::fmt::Debug for HappyEyeballsConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HappyEyeballsConfigView<'_> {
  fn default() -> HappyEyeballsConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HappyEyeballsConfig>> for HappyEyeballsConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HappyEyeballsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HappyEyeballsConfigView<'msg> {

  pub fn to_owned(&self) -> HappyEyeballsConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // first_address_family_version: optional enum envoy.config.cluster.v3.UpstreamConnectionOptions.FirstAddressFamilyVersion
  pub fn first_address_family_version(self) -> super::super::upstream_connection_options::FirstAddressFamilyVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::upstream_connection_options::FirstAddressFamilyVersion::Default).into()
      ).try_into().unwrap()
    }
  }

  // first_address_family_count: optional message google.protobuf.UInt32Value
  pub fn has_first_address_family_count(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn first_address_family_count_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'msg>> {
    self.has_first_address_family_count().then(|| self.first_address_family_count())
  }
  pub fn first_address_family_count(self) -> ::protobuf_well_known_types::UInt32ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }

}

// SAFETY:
// - `HappyEyeballsConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HappyEyeballsConfigView<'_> {}

// SAFETY:
// - `HappyEyeballsConfigView` is `Send` because while its alive a `HappyEyeballsConfigMut` cannot.
// - `HappyEyeballsConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for HappyEyeballsConfigView<'_> {}

impl<'msg> ::protobuf::AsView for HappyEyeballsConfigView<'msg> {
  type Proxied = HappyEyeballsConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, HappyEyeballsConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HappyEyeballsConfigView<'msg> {
  fn into_view<'shorter>(self) -> HappyEyeballsConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HappyEyeballsConfig> for HappyEyeballsConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HappyEyeballsConfig {
    let mut dst = HappyEyeballsConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HappyEyeballsConfig> for HappyEyeballsConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HappyEyeballsConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HappyEyeballsConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HappyEyeballsConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HappyEyeballsConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HappyEyeballsConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HappyEyeballsConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HappyEyeballsConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HappyEyeballsConfigMut<'msg> {
  type Message = HappyEyeballsConfig;
}

impl ::std::fmt::Debug for HappyEyeballsConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HappyEyeballsConfig>> for HappyEyeballsConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HappyEyeballsConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HappyEyeballsConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HappyEyeballsConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HappyEyeballsConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // first_address_family_version: optional enum envoy.config.cluster.v3.UpstreamConnectionOptions.FirstAddressFamilyVersion
  pub fn first_address_family_version(&self) -> super::super::upstream_connection_options::FirstAddressFamilyVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::upstream_connection_options::FirstAddressFamilyVersion::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_first_address_family_version(&mut self, val: super::super::upstream_connection_options::FirstAddressFamilyVersion) {
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

  // first_address_family_count: optional message google.protobuf.UInt32Value
  pub fn has_first_address_family_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_first_address_family_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn first_address_family_count_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_first_address_family_count().then(|| self.first_address_family_count())
  }
  pub fn first_address_family_count(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn first_address_family_count_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_first_address_family_count(&mut self,
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
// - `HappyEyeballsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HappyEyeballsConfigMut<'_> {}

// SAFETY:
// - `HappyEyeballsConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HappyEyeballsConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for HappyEyeballsConfigMut<'msg> {
  type Proxied = HappyEyeballsConfig;
  fn as_view(&self) -> ::protobuf::View<'_, HappyEyeballsConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HappyEyeballsConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HappyEyeballsConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HappyEyeballsConfigMut<'msg> {
  type MutProxied = HappyEyeballsConfig;
  fn as_mut(&mut self) -> HappyEyeballsConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HappyEyeballsConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> HappyEyeballsConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HappyEyeballsConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HappyEyeballsConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HappyEyeballsConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HappyEyeballsConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // first_address_family_version: optional enum envoy.config.cluster.v3.UpstreamConnectionOptions.FirstAddressFamilyVersion
  pub fn first_address_family_version(&self) -> super::super::upstream_connection_options::FirstAddressFamilyVersion {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::super::upstream_connection_options::FirstAddressFamilyVersion::Default).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_first_address_family_version(&mut self, val: super::super::upstream_connection_options::FirstAddressFamilyVersion) {
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

  // first_address_family_count: optional message google.protobuf.UInt32Value
  pub fn has_first_address_family_count(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_first_address_family_count(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn first_address_family_count_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt32ValueView<'_>> {
    self.has_first_address_family_count().then(|| self.first_address_family_count())
  }
  pub fn first_address_family_count(&self) -> ::protobuf_well_known_types::UInt32ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt32ValueView::default())
  }
  pub fn first_address_family_count_mut(&mut self) -> ::protobuf_well_known_types::UInt32ValueMut<'_> {
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
  pub fn set_first_address_family_count(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt32Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl HappyEyeballsConfig

impl ::std::ops::Drop for HappyEyeballsConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HappyEyeballsConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HappyEyeballsConfig {
  type Proxied = Self;
  fn as_view(&self) -> HappyEyeballsConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HappyEyeballsConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HappyEyeballsConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HappyEyeballsConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::upstream_connection_options::envoy__config__cluster__v3__UpstreamConnectionOptions__HappyEyeballsConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::upstream_connection_options::envoy__config__cluster__v3__UpstreamConnectionOptions__HappyEyeballsConfig_msg_init.0, &[<::protobuf_well_known_types::UInt32Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::upstream_connection_options::envoy__config__cluster__v3__UpstreamConnectionOptions__HappyEyeballsConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HappyEyeballsConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HappyEyeballsConfig {
  type Msg = HappyEyeballsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HappyEyeballsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HappyEyeballsConfig {
  type Msg = HappyEyeballsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HappyEyeballsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HappyEyeballsConfigMut<'_> {
  type Msg = HappyEyeballsConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HappyEyeballsConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HappyEyeballsConfigMut<'_> {
  type Msg = HappyEyeballsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HappyEyeballsConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HappyEyeballsConfigView<'_> {
  type Msg = HappyEyeballsConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HappyEyeballsConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HappyEyeballsConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FirstAddressFamilyVersion(i32);

#[allow(non_upper_case_globals)]
impl FirstAddressFamilyVersion {
  pub const Default: FirstAddressFamilyVersion = FirstAddressFamilyVersion(0);
  pub const V4: FirstAddressFamilyVersion = FirstAddressFamilyVersion(1);
  pub const V6: FirstAddressFamilyVersion = FirstAddressFamilyVersion(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Default",
      1 => "V4",
      2 => "V6",
      _ => return None
    })
  }
}

impl ::std::convert::From<FirstAddressFamilyVersion> for i32 {
  fn from(val: FirstAddressFamilyVersion) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for FirstAddressFamilyVersion {
  fn from(val: i32) -> FirstAddressFamilyVersion {
    Self(val)
  }
}

impl ::std::default::Default for FirstAddressFamilyVersion {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for FirstAddressFamilyVersion {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "FirstAddressFamilyVersion::{}", constant_name)
    } else {
      write!(f, "FirstAddressFamilyVersion::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for FirstAddressFamilyVersion {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for FirstAddressFamilyVersion {}

impl ::protobuf::Proxied for FirstAddressFamilyVersion {
  type View<'a> = FirstAddressFamilyVersion;
}

impl ::protobuf::AsView for FirstAddressFamilyVersion {
  type Proxied = FirstAddressFamilyVersion;

  fn as_view(&self) -> FirstAddressFamilyVersion {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FirstAddressFamilyVersion {
  fn into_view<'shorter>(self) -> FirstAddressFamilyVersion where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for FirstAddressFamilyVersion {
  const NAME: &'static str = "FirstAddressFamilyVersion";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for FirstAddressFamilyVersion {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod upstream_connection_options


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__cluster__v3__TrackClusterStats_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TrackClusterStats {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TrackClusterStats>
}

impl ::protobuf::Message for TrackClusterStats {
  type MessageView<'msg> = TrackClusterStatsView<'msg>;
  type MessageMut<'msg> = TrackClusterStatsMut<'msg>;
}

impl ::std::default::Default for TrackClusterStats {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TrackClusterStats {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TrackClusterStats` is `Sync` because it does not implement interior mutability.
//    Neither does `TrackClusterStatsMut`.
unsafe impl ::std::marker::Sync for TrackClusterStats {}

// SAFETY:
// - `TrackClusterStats` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TrackClusterStats {}

impl ::protobuf::Proxied for TrackClusterStats {
  type View<'msg> = TrackClusterStatsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TrackClusterStats {}

impl ::protobuf::MutProxied for TrackClusterStats {
  type Mut<'msg> = TrackClusterStatsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TrackClusterStatsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TrackClusterStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TrackClusterStatsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TrackClusterStatsView<'msg> {
  type Message = TrackClusterStats;
}

impl ::std::fmt::Debug for TrackClusterStatsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TrackClusterStatsView<'_> {
  fn default() -> TrackClusterStatsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TrackClusterStats>> for TrackClusterStatsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TrackClusterStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TrackClusterStatsView<'msg> {

  pub fn to_owned(&self) -> TrackClusterStats {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // timeout_budgets: optional bool
  pub fn timeout_budgets(self) -> bool {
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

  // request_response_sizes: optional bool
  pub fn request_response_sizes(self) -> bool {
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

  // per_endpoint_stats: optional bool
  pub fn per_endpoint_stats(self) -> bool {
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
// - `TrackClusterStatsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TrackClusterStatsView<'_> {}

// SAFETY:
// - `TrackClusterStatsView` is `Send` because while its alive a `TrackClusterStatsMut` cannot.
// - `TrackClusterStatsView` does not use thread-local data.
unsafe impl ::std::marker::Send for TrackClusterStatsView<'_> {}

impl<'msg> ::protobuf::AsView for TrackClusterStatsView<'msg> {
  type Proxied = TrackClusterStats;
  fn as_view(&self) -> ::protobuf::View<'msg, TrackClusterStats> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TrackClusterStatsView<'msg> {
  fn into_view<'shorter>(self) -> TrackClusterStatsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TrackClusterStats> for TrackClusterStatsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TrackClusterStats {
    let mut dst = TrackClusterStats::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TrackClusterStats> for TrackClusterStatsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TrackClusterStats {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TrackClusterStats {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TrackClusterStatsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TrackClusterStatsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TrackClusterStatsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TrackClusterStats>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TrackClusterStatsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TrackClusterStatsMut<'msg> {
  type Message = TrackClusterStats;
}

impl ::std::fmt::Debug for TrackClusterStatsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TrackClusterStats>> for TrackClusterStatsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TrackClusterStats>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TrackClusterStatsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TrackClusterStats> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TrackClusterStats {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // timeout_budgets: optional bool
  pub fn timeout_budgets(&self) -> bool {
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
  pub fn set_timeout_budgets(&mut self, val: bool) {
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

  // request_response_sizes: optional bool
  pub fn request_response_sizes(&self) -> bool {
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
  pub fn set_request_response_sizes(&mut self, val: bool) {
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

  // per_endpoint_stats: optional bool
  pub fn per_endpoint_stats(&self) -> bool {
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
  pub fn set_per_endpoint_stats(&mut self, val: bool) {
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
// - `TrackClusterStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TrackClusterStatsMut<'_> {}

// SAFETY:
// - `TrackClusterStatsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TrackClusterStatsMut<'_> {}

impl<'msg> ::protobuf::AsView for TrackClusterStatsMut<'msg> {
  type Proxied = TrackClusterStats;
  fn as_view(&self) -> ::protobuf::View<'_, TrackClusterStats> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TrackClusterStatsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TrackClusterStats>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TrackClusterStatsMut<'msg> {
  type MutProxied = TrackClusterStats;
  fn as_mut(&mut self) -> TrackClusterStatsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TrackClusterStatsMut<'msg> {
  fn into_mut<'shorter>(self) -> TrackClusterStatsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TrackClusterStats {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TrackClusterStats> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TrackClusterStatsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TrackClusterStatsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // timeout_budgets: optional bool
  pub fn timeout_budgets(&self) -> bool {
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
  pub fn set_timeout_budgets(&mut self, val: bool) {
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

  // request_response_sizes: optional bool
  pub fn request_response_sizes(&self) -> bool {
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
  pub fn set_request_response_sizes(&mut self, val: bool) {
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

  // per_endpoint_stats: optional bool
  pub fn per_endpoint_stats(&self) -> bool {
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
  pub fn set_per_endpoint_stats(&mut self, val: bool) {
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

}  // impl TrackClusterStats

impl ::std::ops::Drop for TrackClusterStats {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TrackClusterStats {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TrackClusterStats {
  type Proxied = Self;
  fn as_view(&self) -> TrackClusterStatsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TrackClusterStats {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TrackClusterStatsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TrackClusterStats {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__cluster__v3__TrackClusterStats_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__cluster__v3__TrackClusterStats_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__cluster__v3__TrackClusterStats_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TrackClusterStats {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TrackClusterStats {
  type Msg = TrackClusterStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrackClusterStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TrackClusterStats {
  type Msg = TrackClusterStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrackClusterStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TrackClusterStatsMut<'_> {
  type Msg = TrackClusterStats;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrackClusterStats> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TrackClusterStatsMut<'_> {
  type Msg = TrackClusterStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrackClusterStats> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TrackClusterStatsView<'_> {
  type Msg = TrackClusterStats;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TrackClusterStats> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TrackClusterStatsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



