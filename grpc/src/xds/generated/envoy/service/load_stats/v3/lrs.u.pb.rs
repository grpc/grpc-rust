const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__load_0stats__v3__LoadStatsRequest_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LoadStatsRequest {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LoadStatsRequest>
}

impl ::protobuf::Message for LoadStatsRequest {
  type MessageView<'msg> = LoadStatsRequestView<'msg>;
  type MessageMut<'msg> = LoadStatsRequestMut<'msg>;
}

impl ::std::default::Default for LoadStatsRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LoadStatsRequest {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LoadStatsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `LoadStatsRequestMut`.
unsafe impl ::std::marker::Sync for LoadStatsRequest {}

// SAFETY:
// - `LoadStatsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LoadStatsRequest {}

impl ::protobuf::Proxied for LoadStatsRequest {
  type View<'msg> = LoadStatsRequestView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LoadStatsRequest {}

impl ::protobuf::MutProxied for LoadStatsRequest {
  type Mut<'msg> = LoadStatsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LoadStatsRequestView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LoadStatsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LoadStatsRequestView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LoadStatsRequestView<'msg> {
  type Message = LoadStatsRequest;
}

impl ::std::fmt::Debug for LoadStatsRequestView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LoadStatsRequestView<'_> {
  fn default() -> LoadStatsRequestView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LoadStatsRequest>> for LoadStatsRequestView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LoadStatsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LoadStatsRequestView<'msg> {

  pub fn to_owned(&self) -> LoadStatsRequest {
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

  // cluster_stats: repeated message envoy.config.endpoint.v3.ClusterStats
  pub fn cluster_stats(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `LoadStatsRequestView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LoadStatsRequestView<'_> {}

// SAFETY:
// - `LoadStatsRequestView` is `Send` because while its alive a `LoadStatsRequestMut` cannot.
// - `LoadStatsRequestView` does not use thread-local data.
unsafe impl ::std::marker::Send for LoadStatsRequestView<'_> {}

impl<'msg> ::protobuf::AsView for LoadStatsRequestView<'msg> {
  type Proxied = LoadStatsRequest;
  fn as_view(&self) -> ::protobuf::View<'msg, LoadStatsRequest> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LoadStatsRequestView<'msg> {
  fn into_view<'shorter>(self) -> LoadStatsRequestView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LoadStatsRequest> for LoadStatsRequestView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LoadStatsRequest {
    let mut dst = LoadStatsRequest::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LoadStatsRequest> for LoadStatsRequestMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LoadStatsRequest {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LoadStatsRequest {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LoadStatsRequestView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LoadStatsRequestMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LoadStatsRequestMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadStatsRequest>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LoadStatsRequestMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LoadStatsRequestMut<'msg> {
  type Message = LoadStatsRequest;
}

impl ::std::fmt::Debug for LoadStatsRequestMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LoadStatsRequest>> for LoadStatsRequestMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadStatsRequest>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LoadStatsRequestMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadStatsRequest> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LoadStatsRequest {
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

  // cluster_stats: repeated message envoy.config.endpoint.v3.ClusterStats
  pub fn cluster_stats(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn cluster_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats> {
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
  pub fn set_cluster_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `LoadStatsRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LoadStatsRequestMut<'_> {}

// SAFETY:
// - `LoadStatsRequestMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LoadStatsRequestMut<'_> {}

impl<'msg> ::protobuf::AsView for LoadStatsRequestMut<'msg> {
  type Proxied = LoadStatsRequest;
  fn as_view(&self) -> ::protobuf::View<'_, LoadStatsRequest> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LoadStatsRequestMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LoadStatsRequest>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LoadStatsRequestMut<'msg> {
  type MutProxied = LoadStatsRequest;
  fn as_mut(&mut self) -> LoadStatsRequestMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LoadStatsRequestMut<'msg> {
  fn into_mut<'shorter>(self) -> LoadStatsRequestMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LoadStatsRequest {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LoadStatsRequest> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LoadStatsRequestView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LoadStatsRequestMut<'_> {
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

  // cluster_stats: repeated message envoy.config.endpoint.v3.ClusterStats
  pub fn cluster_stats(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn cluster_stats_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats> {
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
  pub fn set_cluster_stats(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl LoadStatsRequest

impl ::std::ops::Drop for LoadStatsRequest {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LoadStatsRequest {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LoadStatsRequest {
  type Proxied = Self;
  fn as_view(&self) -> LoadStatsRequestView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LoadStatsRequest {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LoadStatsRequestMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LoadStatsRequest {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__load_0stats__v3__LoadStatsRequest_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__load_0stats__v3__LoadStatsRequest_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::Node as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::endpoint::v3::load_report::ClusterStats as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__load_0stats__v3__LoadStatsRequest_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LoadStatsRequest {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LoadStatsRequest {
  type Msg = LoadStatsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadStatsRequest {
  type Msg = LoadStatsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LoadStatsRequestMut<'_> {
  type Msg = LoadStatsRequest;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsRequest> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadStatsRequestMut<'_> {
  type Msg = LoadStatsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsRequest> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadStatsRequestView<'_> {
  type Msg = LoadStatsRequest;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsRequest> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LoadStatsRequestMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__service__load_0stats__v3__LoadStatsResponse_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LoadStatsResponse {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LoadStatsResponse>
}

impl ::protobuf::Message for LoadStatsResponse {
  type MessageView<'msg> = LoadStatsResponseView<'msg>;
  type MessageMut<'msg> = LoadStatsResponseMut<'msg>;
}

impl ::std::default::Default for LoadStatsResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LoadStatsResponse {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LoadStatsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `LoadStatsResponseMut`.
unsafe impl ::std::marker::Sync for LoadStatsResponse {}

// SAFETY:
// - `LoadStatsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LoadStatsResponse {}

impl ::protobuf::Proxied for LoadStatsResponse {
  type View<'msg> = LoadStatsResponseView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LoadStatsResponse {}

impl ::protobuf::MutProxied for LoadStatsResponse {
  type Mut<'msg> = LoadStatsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LoadStatsResponseView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LoadStatsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LoadStatsResponseView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LoadStatsResponseView<'msg> {
  type Message = LoadStatsResponse;
}

impl ::std::fmt::Debug for LoadStatsResponseView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LoadStatsResponseView<'_> {
  fn default() -> LoadStatsResponseView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LoadStatsResponse>> for LoadStatsResponseView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LoadStatsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LoadStatsResponseView<'msg> {

  pub fn to_owned(&self) -> LoadStatsResponse {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // clusters: repeated string
  pub fn clusters(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // send_all_clusters: optional bool
  pub fn send_all_clusters(self) -> bool {
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

  // load_reporting_interval: optional message google.protobuf.Duration
  pub fn has_load_reporting_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn load_reporting_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_load_reporting_interval().then(|| self.load_reporting_interval())
  }
  pub fn load_reporting_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // report_endpoint_granularity: optional bool
  pub fn report_endpoint_granularity(self) -> bool {
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
// - `LoadStatsResponseView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LoadStatsResponseView<'_> {}

// SAFETY:
// - `LoadStatsResponseView` is `Send` because while its alive a `LoadStatsResponseMut` cannot.
// - `LoadStatsResponseView` does not use thread-local data.
unsafe impl ::std::marker::Send for LoadStatsResponseView<'_> {}

impl<'msg> ::protobuf::AsView for LoadStatsResponseView<'msg> {
  type Proxied = LoadStatsResponse;
  fn as_view(&self) -> ::protobuf::View<'msg, LoadStatsResponse> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LoadStatsResponseView<'msg> {
  fn into_view<'shorter>(self) -> LoadStatsResponseView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LoadStatsResponse> for LoadStatsResponseView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LoadStatsResponse {
    let mut dst = LoadStatsResponse::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LoadStatsResponse> for LoadStatsResponseMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LoadStatsResponse {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LoadStatsResponse {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LoadStatsResponseView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LoadStatsResponseMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LoadStatsResponseMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadStatsResponse>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LoadStatsResponseMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LoadStatsResponseMut<'msg> {
  type Message = LoadStatsResponse;
}

impl ::std::fmt::Debug for LoadStatsResponseMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LoadStatsResponse>> for LoadStatsResponseMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadStatsResponse>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LoadStatsResponseMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LoadStatsResponse> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LoadStatsResponse {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // clusters: repeated string
  pub fn clusters(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // send_all_clusters: optional bool
  pub fn send_all_clusters(&self) -> bool {
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
  pub fn set_send_all_clusters(&mut self, val: bool) {
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

  // load_reporting_interval: optional message google.protobuf.Duration
  pub fn has_load_reporting_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_load_reporting_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn load_reporting_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_load_reporting_interval().then(|| self.load_reporting_interval())
  }
  pub fn load_reporting_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn load_reporting_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_load_reporting_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // report_endpoint_granularity: optional bool
  pub fn report_endpoint_granularity(&self) -> bool {
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
  pub fn set_report_endpoint_granularity(&mut self, val: bool) {
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
// - `LoadStatsResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LoadStatsResponseMut<'_> {}

// SAFETY:
// - `LoadStatsResponseMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LoadStatsResponseMut<'_> {}

impl<'msg> ::protobuf::AsView for LoadStatsResponseMut<'msg> {
  type Proxied = LoadStatsResponse;
  fn as_view(&self) -> ::protobuf::View<'_, LoadStatsResponse> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LoadStatsResponseMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LoadStatsResponse>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LoadStatsResponseMut<'msg> {
  type MutProxied = LoadStatsResponse;
  fn as_mut(&mut self) -> LoadStatsResponseMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LoadStatsResponseMut<'msg> {
  fn into_mut<'shorter>(self) -> LoadStatsResponseMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LoadStatsResponse {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LoadStatsResponse> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LoadStatsResponseView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LoadStatsResponseMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // clusters: repeated string
  pub fn clusters(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // send_all_clusters: optional bool
  pub fn send_all_clusters(&self) -> bool {
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
  pub fn set_send_all_clusters(&mut self, val: bool) {
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

  // load_reporting_interval: optional message google.protobuf.Duration
  pub fn has_load_reporting_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_load_reporting_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn load_reporting_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_load_reporting_interval().then(|| self.load_reporting_interval())
  }
  pub fn load_reporting_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn load_reporting_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_load_reporting_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // report_endpoint_granularity: optional bool
  pub fn report_endpoint_granularity(&self) -> bool {
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
  pub fn set_report_endpoint_granularity(&mut self, val: bool) {
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

}  // impl LoadStatsResponse

impl ::std::ops::Drop for LoadStatsResponse {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LoadStatsResponse {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LoadStatsResponse {
  type Proxied = Self;
  fn as_view(&self) -> LoadStatsResponseView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LoadStatsResponse {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LoadStatsResponseMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LoadStatsResponse {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__service__load_0stats__v3__LoadStatsResponse_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ET3/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__service__load_0stats__v3__LoadStatsResponse_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__service__load_0stats__v3__LoadStatsResponse_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LoadStatsResponse {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LoadStatsResponse {
  type Msg = LoadStatsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadStatsResponse {
  type Msg = LoadStatsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LoadStatsResponseMut<'_> {
  type Msg = LoadStatsResponse;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsResponse> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadStatsResponseMut<'_> {
  type Msg = LoadStatsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsResponse> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LoadStatsResponseView<'_> {
  type Msg = LoadStatsResponse;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LoadStatsResponse> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LoadStatsResponseMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



