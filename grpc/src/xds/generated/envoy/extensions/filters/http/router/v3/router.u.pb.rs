const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__router__v3__Router_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Router {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Router>
}

impl ::protobuf::Message for Router {
  type MessageView<'msg> = RouterView<'msg>;
  type MessageMut<'msg> = RouterMut<'msg>;
}

impl ::std::default::Default for Router {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Router {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Router` is `Sync` because it does not implement interior mutability.
//    Neither does `RouterMut`.
unsafe impl ::std::marker::Sync for Router {}

// SAFETY:
// - `Router` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Router {}

impl ::protobuf::Proxied for Router {
  type View<'msg> = RouterView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Router {}

impl ::protobuf::MutProxied for Router {
  type Mut<'msg> = RouterMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RouterView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Router>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RouterView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RouterView<'msg> {
  type Message = Router;
}

impl ::std::fmt::Debug for RouterView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RouterView<'_> {
  fn default() -> RouterView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Router>> for RouterView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Router>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RouterView<'msg> {

  pub fn to_owned(&self) -> Router {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // dynamic_stats: optional message google.protobuf.BoolValue
  pub fn has_dynamic_stats(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn dynamic_stats_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_dynamic_stats().then(|| self.dynamic_stats())
  }
  pub fn dynamic_stats(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // start_child_span: optional bool
  pub fn start_child_span(self) -> bool {
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

  // upstream_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn upstream_log(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // upstream_log_options: optional message envoy.extensions.filters.http.router.v3.Router.UpstreamAccessLogOptions
  pub fn has_upstream_log_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn upstream_log_options_opt(self) -> ::std::option::Option<super::router::UpstreamAccessLogOptionsView<'msg>> {
    self.has_upstream_log_options().then(|| self.upstream_log_options())
  }
  pub fn upstream_log_options(self) -> super::router::UpstreamAccessLogOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::router::UpstreamAccessLogOptionsView::default())
  }

  // suppress_envoy_headers: optional bool
  pub fn suppress_envoy_headers(self) -> bool {
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

  // strict_check_headers: repeated string
  pub fn strict_check_headers(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
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

  // respect_expected_rq_timeout: optional bool
  pub fn respect_expected_rq_timeout(self) -> bool {
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

  // suppress_grpc_request_failure_code_stats: optional bool
  pub fn suppress_grpc_request_failure_code_stats(self) -> bool {
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

  // upstream_http_filters: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter
  pub fn upstream_http_filters(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // reject_connect_request_early_data: optional message google.protobuf.BoolValue
  pub fn has_reject_connect_request_early_data(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn reject_connect_request_early_data_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_reject_connect_request_early_data().then(|| self.reject_connect_request_early_data())
  }
  pub fn reject_connect_request_early_data(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

}

// SAFETY:
// - `RouterView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RouterView<'_> {}

// SAFETY:
// - `RouterView` is `Send` because while its alive a `RouterMut` cannot.
// - `RouterView` does not use thread-local data.
unsafe impl ::std::marker::Send for RouterView<'_> {}

impl<'msg> ::protobuf::AsView for RouterView<'msg> {
  type Proxied = Router;
  fn as_view(&self) -> ::protobuf::View<'msg, Router> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RouterView<'msg> {
  fn into_view<'shorter>(self) -> RouterView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Router> for RouterView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Router {
    let mut dst = Router::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Router> for RouterMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Router {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Router {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RouterView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RouterMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RouterMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Router>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RouterMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RouterMut<'msg> {
  type Message = Router;
}

impl ::std::fmt::Debug for RouterMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Router>> for RouterMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Router>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RouterMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Router> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Router {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // dynamic_stats: optional message google.protobuf.BoolValue
  pub fn has_dynamic_stats(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_dynamic_stats(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn dynamic_stats_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_dynamic_stats().then(|| self.dynamic_stats())
  }
  pub fn dynamic_stats(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn dynamic_stats_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_dynamic_stats(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // start_child_span: optional bool
  pub fn start_child_span(&self) -> bool {
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
  pub fn set_start_child_span(&mut self, val: bool) {
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

  // upstream_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn upstream_log(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upstream_log_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
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
  pub fn set_upstream_log(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // upstream_log_options: optional message envoy.extensions.filters.http.router.v3.Router.UpstreamAccessLogOptions
  pub fn has_upstream_log_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_upstream_log_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn upstream_log_options_opt(&self) -> ::std::option::Option<super::router::UpstreamAccessLogOptionsView<'_>> {
    self.has_upstream_log_options().then(|| self.upstream_log_options())
  }
  pub fn upstream_log_options(&self) -> super::router::UpstreamAccessLogOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::router::UpstreamAccessLogOptionsView::default())
  }
  pub fn upstream_log_options_mut(&mut self) -> super::router::UpstreamAccessLogOptionsMut<'_> {
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
  pub fn set_upstream_log_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::router::UpstreamAccessLogOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // suppress_envoy_headers: optional bool
  pub fn suppress_envoy_headers(&self) -> bool {
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
  pub fn set_suppress_envoy_headers(&mut self, val: bool) {
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

  // strict_check_headers: repeated string
  pub fn strict_check_headers(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn strict_check_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_strict_check_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // respect_expected_rq_timeout: optional bool
  pub fn respect_expected_rq_timeout(&self) -> bool {
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
  pub fn set_respect_expected_rq_timeout(&mut self, val: bool) {
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

  // suppress_grpc_request_failure_code_stats: optional bool
  pub fn suppress_grpc_request_failure_code_stats(&self) -> bool {
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
  pub fn set_suppress_grpc_request_failure_code_stats(&mut self, val: bool) {
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

  // upstream_http_filters: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter
  pub fn upstream_http_filters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upstream_http_filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter> {
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
  pub fn set_upstream_http_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // reject_connect_request_early_data: optional message google.protobuf.BoolValue
  pub fn has_reject_connect_request_early_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_reject_connect_request_early_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn reject_connect_request_early_data_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_reject_connect_request_early_data().then(|| self.reject_connect_request_early_data())
  }
  pub fn reject_connect_request_early_data(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn reject_connect_request_early_data_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_reject_connect_request_early_data(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

}

// SAFETY:
// - `RouterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RouterMut<'_> {}

// SAFETY:
// - `RouterMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RouterMut<'_> {}

impl<'msg> ::protobuf::AsView for RouterMut<'msg> {
  type Proxied = Router;
  fn as_view(&self) -> ::protobuf::View<'_, Router> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RouterMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Router>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RouterMut<'msg> {
  type MutProxied = Router;
  fn as_mut(&mut self) -> RouterMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RouterMut<'msg> {
  fn into_mut<'shorter>(self) -> RouterMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Router {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Router> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RouterView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RouterMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // dynamic_stats: optional message google.protobuf.BoolValue
  pub fn has_dynamic_stats(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_dynamic_stats(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn dynamic_stats_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_dynamic_stats().then(|| self.dynamic_stats())
  }
  pub fn dynamic_stats(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn dynamic_stats_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_dynamic_stats(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // start_child_span: optional bool
  pub fn start_child_span(&self) -> bool {
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
  pub fn set_start_child_span(&mut self, val: bool) {
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

  // upstream_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn upstream_log(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upstream_log_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
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
  pub fn set_upstream_log(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // upstream_log_options: optional message envoy.extensions.filters.http.router.v3.Router.UpstreamAccessLogOptions
  pub fn has_upstream_log_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_upstream_log_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn upstream_log_options_opt(&self) -> ::std::option::Option<super::router::UpstreamAccessLogOptionsView<'_>> {
    self.has_upstream_log_options().then(|| self.upstream_log_options())
  }
  pub fn upstream_log_options(&self) -> super::router::UpstreamAccessLogOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::router::UpstreamAccessLogOptionsView::default())
  }
  pub fn upstream_log_options_mut(&mut self) -> super::router::UpstreamAccessLogOptionsMut<'_> {
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
  pub fn set_upstream_log_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::router::UpstreamAccessLogOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // suppress_envoy_headers: optional bool
  pub fn suppress_envoy_headers(&self) -> bool {
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
  pub fn set_suppress_envoy_headers(&mut self, val: bool) {
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

  // strict_check_headers: repeated string
  pub fn strict_check_headers(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
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
  pub fn strict_check_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_strict_check_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // respect_expected_rq_timeout: optional bool
  pub fn respect_expected_rq_timeout(&self) -> bool {
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
  pub fn set_respect_expected_rq_timeout(&mut self, val: bool) {
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

  // suppress_grpc_request_failure_code_stats: optional bool
  pub fn suppress_grpc_request_failure_code_stats(&self) -> bool {
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
  pub fn set_suppress_grpc_request_failure_code_stats(&mut self, val: bool) {
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

  // upstream_http_filters: repeated message envoy.extensions.filters.network.http_connection_manager.v3.HttpFilter
  pub fn upstream_http_filters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn upstream_http_filters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter> {
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
  pub fn set_upstream_http_filters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // reject_connect_request_early_data: optional message google.protobuf.BoolValue
  pub fn has_reject_connect_request_early_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_reject_connect_request_early_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn reject_connect_request_early_data_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_reject_connect_request_early_data().then(|| self.reject_connect_request_early_data())
  }
  pub fn reject_connect_request_early_data(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn reject_connect_request_early_data_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_reject_connect_request_early_data(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

}  // impl Router

impl ::std::ops::Drop for Router {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Router {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Router {
  type Proxied = Self;
  fn as_view(&self) -> RouterView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Router {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RouterMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Router {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__router__v3__Router_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3/PG/PET/P/PG33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__router__v3__Router_msg_init.0, &[<::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::filters::network::http_connection_manager::v3::http_connection_manager::HttpFilter as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::router::UpstreamAccessLogOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__router__v3__Router_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Router {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Router {
  type Msg = Router;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Router> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Router {
  type Msg = Router;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Router> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RouterMut<'_> {
  type Msg = Router;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Router> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouterMut<'_> {
  type Msg = Router;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Router> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouterView<'_> {
  type Msg = Router;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Router> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RouterMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod router {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__router__v3__Router__UpstreamAccessLogOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UpstreamAccessLogOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UpstreamAccessLogOptions>
}

impl ::protobuf::Message for UpstreamAccessLogOptions {
  type MessageView<'msg> = UpstreamAccessLogOptionsView<'msg>;
  type MessageMut<'msg> = UpstreamAccessLogOptionsMut<'msg>;
}

impl ::std::default::Default for UpstreamAccessLogOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UpstreamAccessLogOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UpstreamAccessLogOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `UpstreamAccessLogOptionsMut`.
unsafe impl ::std::marker::Sync for UpstreamAccessLogOptions {}

// SAFETY:
// - `UpstreamAccessLogOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamAccessLogOptions {}

impl ::protobuf::Proxied for UpstreamAccessLogOptions {
  type View<'msg> = UpstreamAccessLogOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UpstreamAccessLogOptions {}

impl ::protobuf::MutProxied for UpstreamAccessLogOptions {
  type Mut<'msg> = UpstreamAccessLogOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpstreamAccessLogOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamAccessLogOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamAccessLogOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpstreamAccessLogOptionsView<'msg> {
  type Message = UpstreamAccessLogOptions;
}

impl ::std::fmt::Debug for UpstreamAccessLogOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpstreamAccessLogOptionsView<'_> {
  fn default() -> UpstreamAccessLogOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamAccessLogOptions>> for UpstreamAccessLogOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UpstreamAccessLogOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamAccessLogOptionsView<'msg> {

  pub fn to_owned(&self) -> UpstreamAccessLogOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // flush_upstream_log_on_upstream_stream: optional bool
  pub fn flush_upstream_log_on_upstream_stream(self) -> bool {
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

  // upstream_log_flush_interval: optional message google.protobuf.Duration
  pub fn has_upstream_log_flush_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn upstream_log_flush_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_upstream_log_flush_interval().then(|| self.upstream_log_flush_interval())
  }
  pub fn upstream_log_flush_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `UpstreamAccessLogOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpstreamAccessLogOptionsView<'_> {}

// SAFETY:
// - `UpstreamAccessLogOptionsView` is `Send` because while its alive a `UpstreamAccessLogOptionsMut` cannot.
// - `UpstreamAccessLogOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpstreamAccessLogOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamAccessLogOptionsView<'msg> {
  type Proxied = UpstreamAccessLogOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, UpstreamAccessLogOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamAccessLogOptionsView<'msg> {
  fn into_view<'shorter>(self) -> UpstreamAccessLogOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamAccessLogOptions> for UpstreamAccessLogOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamAccessLogOptions {
    let mut dst = UpstreamAccessLogOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UpstreamAccessLogOptions> for UpstreamAccessLogOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UpstreamAccessLogOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UpstreamAccessLogOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamAccessLogOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpstreamAccessLogOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpstreamAccessLogOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamAccessLogOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpstreamAccessLogOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpstreamAccessLogOptionsMut<'msg> {
  type Message = UpstreamAccessLogOptions;
}

impl ::std::fmt::Debug for UpstreamAccessLogOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamAccessLogOptions>> for UpstreamAccessLogOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamAccessLogOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpstreamAccessLogOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UpstreamAccessLogOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UpstreamAccessLogOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // flush_upstream_log_on_upstream_stream: optional bool
  pub fn flush_upstream_log_on_upstream_stream(&self) -> bool {
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
  pub fn set_flush_upstream_log_on_upstream_stream(&mut self, val: bool) {
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

  // upstream_log_flush_interval: optional message google.protobuf.Duration
  pub fn has_upstream_log_flush_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_upstream_log_flush_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn upstream_log_flush_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_upstream_log_flush_interval().then(|| self.upstream_log_flush_interval())
  }
  pub fn upstream_log_flush_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn upstream_log_flush_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_upstream_log_flush_interval(&mut self,
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
// - `UpstreamAccessLogOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpstreamAccessLogOptionsMut<'_> {}

// SAFETY:
// - `UpstreamAccessLogOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpstreamAccessLogOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for UpstreamAccessLogOptionsMut<'msg> {
  type Proxied = UpstreamAccessLogOptions;
  fn as_view(&self) -> ::protobuf::View<'_, UpstreamAccessLogOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpstreamAccessLogOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UpstreamAccessLogOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpstreamAccessLogOptionsMut<'msg> {
  type MutProxied = UpstreamAccessLogOptions;
  fn as_mut(&mut self) -> UpstreamAccessLogOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpstreamAccessLogOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> UpstreamAccessLogOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UpstreamAccessLogOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UpstreamAccessLogOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpstreamAccessLogOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpstreamAccessLogOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // flush_upstream_log_on_upstream_stream: optional bool
  pub fn flush_upstream_log_on_upstream_stream(&self) -> bool {
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
  pub fn set_flush_upstream_log_on_upstream_stream(&mut self, val: bool) {
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

  // upstream_log_flush_interval: optional message google.protobuf.Duration
  pub fn has_upstream_log_flush_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_upstream_log_flush_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn upstream_log_flush_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_upstream_log_flush_interval().then(|| self.upstream_log_flush_interval())
  }
  pub fn upstream_log_flush_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn upstream_log_flush_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
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
  pub fn set_upstream_log_flush_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl UpstreamAccessLogOptions

impl ::std::ops::Drop for UpstreamAccessLogOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UpstreamAccessLogOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UpstreamAccessLogOptions {
  type Proxied = Self;
  fn as_view(&self) -> UpstreamAccessLogOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UpstreamAccessLogOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpstreamAccessLogOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UpstreamAccessLogOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::router::envoy__extensions__filters__http__router__v3__Router__UpstreamAccessLogOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::router::envoy__extensions__filters__http__router__v3__Router__UpstreamAccessLogOptions_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::router::envoy__extensions__filters__http__router__v3__Router__UpstreamAccessLogOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamAccessLogOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamAccessLogOptions {
  type Msg = UpstreamAccessLogOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamAccessLogOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamAccessLogOptions {
  type Msg = UpstreamAccessLogOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamAccessLogOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpstreamAccessLogOptionsMut<'_> {
  type Msg = UpstreamAccessLogOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamAccessLogOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamAccessLogOptionsMut<'_> {
  type Msg = UpstreamAccessLogOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamAccessLogOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpstreamAccessLogOptionsView<'_> {
  type Msg = UpstreamAccessLogOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UpstreamAccessLogOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpstreamAccessLogOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod router


