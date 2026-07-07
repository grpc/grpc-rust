const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rbac__v3__RBAC_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RBAC {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RBAC>
}

impl ::protobuf::Message for RBAC {
  type MessageView<'msg> = RBACView<'msg>;
  type MessageMut<'msg> = RBACMut<'msg>;
}

impl ::std::default::Default for RBAC {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RBAC {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RBAC` is `Sync` because it does not implement interior mutability.
//    Neither does `RBACMut`.
unsafe impl ::std::marker::Sync for RBAC {}

// SAFETY:
// - `RBAC` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RBAC {}

impl ::protobuf::Proxied for RBAC {
  type View<'msg> = RBACView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RBAC {}

impl ::protobuf::MutProxied for RBAC {
  type Mut<'msg> = RBACMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RBACView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RBAC>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RBACView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RBACView<'msg> {
  type Message = RBAC;
}

impl ::std::fmt::Debug for RBACView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RBACView<'_> {
  fn default() -> RBACView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RBAC>> for RBACView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RBAC>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RBACView<'msg> {

  pub fn to_owned(&self) -> RBAC {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // rules: optional message envoy.config.rbac.v3.RBAC
  pub fn has_rules(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn rules_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'msg>> {
    self.has_rules().then(|| self.rules())
  }
  pub fn rules(self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView::default())
  }

  // rules_stat_prefix: optional string
  pub fn rules_stat_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn matcher_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }

  // shadow_rules: optional message envoy.config.rbac.v3.RBAC
  pub fn has_shadow_rules(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn shadow_rules_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'msg>> {
    self.has_shadow_rules().then(|| self.shadow_rules())
  }
  pub fn shadow_rules(self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView::default())
  }

  // shadow_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_shadow_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn shadow_matcher_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_shadow_matcher().then(|| self.shadow_matcher())
  }
  pub fn shadow_matcher(self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }

  // shadow_rules_stat_prefix: optional string
  pub fn shadow_rules_stat_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // track_per_rule_stats: optional bool
  pub fn track_per_rule_stats(self) -> bool {
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

}

// SAFETY:
// - `RBACView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RBACView<'_> {}

// SAFETY:
// - `RBACView` is `Send` because while its alive a `RBACMut` cannot.
// - `RBACView` does not use thread-local data.
unsafe impl ::std::marker::Send for RBACView<'_> {}

impl<'msg> ::protobuf::AsView for RBACView<'msg> {
  type Proxied = RBAC;
  fn as_view(&self) -> ::protobuf::View<'msg, RBAC> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RBACView<'msg> {
  fn into_view<'shorter>(self) -> RBACView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RBAC> for RBACView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RBAC {
    let mut dst = RBAC::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RBAC> for RBACMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RBAC {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RBAC {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RBACView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RBACMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RBACMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RBAC>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RBACMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RBACMut<'msg> {
  type Message = RBAC;
}

impl ::std::fmt::Debug for RBACMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RBAC>> for RBACMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RBAC>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RBACMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RBAC> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RBAC {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // rules: optional message envoy.config.rbac.v3.RBAC
  pub fn has_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn rules_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'_>> {
    self.has_rules().then(|| self.rules())
  }
  pub fn rules(&self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView::default())
  }
  pub fn rules_mut(&mut self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACMut<'_> {
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
  pub fn set_rules(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::rbac::v3::rbac::RBAC>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // rules_stat_prefix: optional string
  pub fn rules_stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_rules_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // shadow_rules: optional message envoy.config.rbac.v3.RBAC
  pub fn has_shadow_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_shadow_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn shadow_rules_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'_>> {
    self.has_shadow_rules().then(|| self.shadow_rules())
  }
  pub fn shadow_rules(&self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView::default())
  }
  pub fn shadow_rules_mut(&mut self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACMut<'_> {
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
  pub fn set_shadow_rules(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::rbac::v3::rbac::RBAC>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // shadow_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_shadow_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_shadow_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn shadow_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_shadow_matcher().then(|| self.shadow_matcher())
  }
  pub fn shadow_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn shadow_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_shadow_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // shadow_rules_stat_prefix: optional string
  pub fn shadow_rules_stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_shadow_rules_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // track_per_rule_stats: optional bool
  pub fn track_per_rule_stats(&self) -> bool {
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
  pub fn set_track_per_rule_stats(&mut self, val: bool) {
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

}

// SAFETY:
// - `RBACMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RBACMut<'_> {}

// SAFETY:
// - `RBACMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RBACMut<'_> {}

impl<'msg> ::protobuf::AsView for RBACMut<'msg> {
  type Proxied = RBAC;
  fn as_view(&self) -> ::protobuf::View<'_, RBAC> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RBACMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RBAC>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RBACMut<'msg> {
  type MutProxied = RBAC;
  fn as_mut(&mut self) -> RBACMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RBACMut<'msg> {
  fn into_mut<'shorter>(self) -> RBACMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RBAC {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RBAC> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RBACView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RBACMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // rules: optional message envoy.config.rbac.v3.RBAC
  pub fn has_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn rules_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'_>> {
    self.has_rules().then(|| self.rules())
  }
  pub fn rules(&self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView::default())
  }
  pub fn rules_mut(&mut self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACMut<'_> {
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
  pub fn set_rules(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::rbac::v3::rbac::RBAC>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // rules_stat_prefix: optional string
  pub fn rules_stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_rules_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // shadow_rules: optional message envoy.config.rbac.v3.RBAC
  pub fn has_shadow_rules(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_shadow_rules(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn shadow_rules_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'_>> {
    self.has_shadow_rules().then(|| self.shadow_rules())
  }
  pub fn shadow_rules(&self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::rbac::v3::rbac::RBACView::default())
  }
  pub fn shadow_rules_mut(&mut self) -> crate::xds::generated::envoy::config::rbac::v3::rbac::RBACMut<'_> {
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
  pub fn set_shadow_rules(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::rbac::v3::rbac::RBAC>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // shadow_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_shadow_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_shadow_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn shadow_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_shadow_matcher().then(|| self.shadow_matcher())
  }
  pub fn shadow_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn shadow_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_shadow_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // shadow_rules_stat_prefix: optional string
  pub fn shadow_rules_stat_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_shadow_rules_stat_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // track_per_rule_stats: optional bool
  pub fn track_per_rule_stats(&self) -> bool {
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
  pub fn set_track_per_rule_stats(&mut self, val: bool) {
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

}  // impl RBAC

impl ::std::ops::Drop for RBAC {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RBAC {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RBAC {
  type Proxied = Self;
  fn as_view(&self) -> RBACView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RBAC {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RBACMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RBAC {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__rbac__v3__RBAC_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$331X331X/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__rbac__v3__RBAC_msg_init.0, &[<crate::xds::generated::envoy::config::rbac::v3::rbac::RBAC as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::rbac::v3::rbac::RBAC as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__rbac__v3__RBAC_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RBAC {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RBAC {
  type Msg = RBAC;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RBAC {
  type Msg = RBAC;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RBACMut<'_> {
  type Msg = RBAC;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RBACMut<'_> {
  type Msg = RBAC;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RBACView<'_> {
  type Msg = RBAC;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBAC> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RBACMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__filters__http__rbac__v3__RBACPerRoute_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RBACPerRoute {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RBACPerRoute>
}

impl ::protobuf::Message for RBACPerRoute {
  type MessageView<'msg> = RBACPerRouteView<'msg>;
  type MessageMut<'msg> = RBACPerRouteMut<'msg>;
}

impl ::std::default::Default for RBACPerRoute {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RBACPerRoute {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RBACPerRoute` is `Sync` because it does not implement interior mutability.
//    Neither does `RBACPerRouteMut`.
unsafe impl ::std::marker::Sync for RBACPerRoute {}

// SAFETY:
// - `RBACPerRoute` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RBACPerRoute {}

impl ::protobuf::Proxied for RBACPerRoute {
  type View<'msg> = RBACPerRouteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RBACPerRoute {}

impl ::protobuf::MutProxied for RBACPerRoute {
  type Mut<'msg> = RBACPerRouteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RBACPerRouteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RBACPerRoute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RBACPerRouteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RBACPerRouteView<'msg> {
  type Message = RBACPerRoute;
}

impl ::std::fmt::Debug for RBACPerRouteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RBACPerRouteView<'_> {
  fn default() -> RBACPerRouteView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RBACPerRoute>> for RBACPerRouteView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RBACPerRoute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RBACPerRouteView<'msg> {

  pub fn to_owned(&self) -> RBACPerRoute {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // rbac: optional message envoy.extensions.filters.http.rbac.v3.RBAC
  pub fn has_rbac(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn rbac_opt(self) -> ::std::option::Option<super::RBACView<'msg>> {
    self.has_rbac().then(|| self.rbac())
  }
  pub fn rbac(self) -> super::RBACView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RBACView::default())
  }

}

// SAFETY:
// - `RBACPerRouteView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RBACPerRouteView<'_> {}

// SAFETY:
// - `RBACPerRouteView` is `Send` because while its alive a `RBACPerRouteMut` cannot.
// - `RBACPerRouteView` does not use thread-local data.
unsafe impl ::std::marker::Send for RBACPerRouteView<'_> {}

impl<'msg> ::protobuf::AsView for RBACPerRouteView<'msg> {
  type Proxied = RBACPerRoute;
  fn as_view(&self) -> ::protobuf::View<'msg, RBACPerRoute> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RBACPerRouteView<'msg> {
  fn into_view<'shorter>(self) -> RBACPerRouteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RBACPerRoute> for RBACPerRouteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RBACPerRoute {
    let mut dst = RBACPerRoute::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RBACPerRoute> for RBACPerRouteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RBACPerRoute {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RBACPerRoute {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RBACPerRouteView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RBACPerRouteMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RBACPerRouteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RBACPerRoute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RBACPerRouteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RBACPerRouteMut<'msg> {
  type Message = RBACPerRoute;
}

impl ::std::fmt::Debug for RBACPerRouteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RBACPerRoute>> for RBACPerRouteMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RBACPerRoute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RBACPerRouteMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RBACPerRoute> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RBACPerRoute {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // rbac: optional message envoy.extensions.filters.http.rbac.v3.RBAC
  pub fn has_rbac(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_rbac(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn rbac_opt(&self) -> ::std::option::Option<super::RBACView<'_>> {
    self.has_rbac().then(|| self.rbac())
  }
  pub fn rbac(&self) -> super::RBACView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RBACView::default())
  }
  pub fn rbac_mut(&mut self) -> super::RBACMut<'_> {
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
  pub fn set_rbac(&mut self,
    val: impl ::protobuf::IntoProxied<super::RBAC>) {

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
// - `RBACPerRouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RBACPerRouteMut<'_> {}

// SAFETY:
// - `RBACPerRouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RBACPerRouteMut<'_> {}

impl<'msg> ::protobuf::AsView for RBACPerRouteMut<'msg> {
  type Proxied = RBACPerRoute;
  fn as_view(&self) -> ::protobuf::View<'_, RBACPerRoute> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RBACPerRouteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RBACPerRoute>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RBACPerRouteMut<'msg> {
  type MutProxied = RBACPerRoute;
  fn as_mut(&mut self) -> RBACPerRouteMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RBACPerRouteMut<'msg> {
  fn into_mut<'shorter>(self) -> RBACPerRouteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RBACPerRoute {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RBACPerRoute> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RBACPerRouteView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RBACPerRouteMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // rbac: optional message envoy.extensions.filters.http.rbac.v3.RBAC
  pub fn has_rbac(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_rbac(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn rbac_opt(&self) -> ::std::option::Option<super::RBACView<'_>> {
    self.has_rbac().then(|| self.rbac())
  }
  pub fn rbac(&self) -> super::RBACView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RBACView::default())
  }
  pub fn rbac_mut(&mut self) -> super::RBACMut<'_> {
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
  pub fn set_rbac(&mut self,
    val: impl ::protobuf::IntoProxied<super::RBAC>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl RBACPerRoute

impl ::std::ops::Drop for RBACPerRoute {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RBACPerRoute {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RBACPerRoute {
  type Proxied = Self;
  fn as_view(&self) -> RBACPerRouteView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RBACPerRoute {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RBACPerRouteMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RBACPerRoute {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__filters__http__rbac__v3__RBACPerRoute_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$a3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__filters__http__rbac__v3__RBACPerRoute_msg_init.0, &[<super::RBAC as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__filters__http__rbac__v3__RBACPerRoute_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RBACPerRoute {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RBACPerRoute {
  type Msg = RBACPerRoute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBACPerRoute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RBACPerRoute {
  type Msg = RBACPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBACPerRoute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RBACPerRouteMut<'_> {
  type Msg = RBACPerRoute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBACPerRoute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RBACPerRouteMut<'_> {
  type Msg = RBACPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBACPerRoute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RBACPerRouteView<'_> {
  type Msg = RBACPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RBACPerRoute> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RBACPerRouteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



