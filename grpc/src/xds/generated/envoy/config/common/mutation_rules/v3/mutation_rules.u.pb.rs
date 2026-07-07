const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__mutation_0rules__v3__HeaderMutationRules_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderMutationRules {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderMutationRules>
}

impl ::protobuf::Message for HeaderMutationRules {
  type MessageView<'msg> = HeaderMutationRulesView<'msg>;
  type MessageMut<'msg> = HeaderMutationRulesMut<'msg>;
}

impl ::std::default::Default for HeaderMutationRules {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderMutationRules {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderMutationRules` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderMutationRulesMut`.
unsafe impl ::std::marker::Sync for HeaderMutationRules {}

// SAFETY:
// - `HeaderMutationRules` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderMutationRules {}

impl ::protobuf::Proxied for HeaderMutationRules {
  type View<'msg> = HeaderMutationRulesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderMutationRules {}

impl ::protobuf::MutProxied for HeaderMutationRules {
  type Mut<'msg> = HeaderMutationRulesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderMutationRulesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMutationRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderMutationRulesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderMutationRulesView<'msg> {
  type Message = HeaderMutationRules;
}

impl ::std::fmt::Debug for HeaderMutationRulesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderMutationRulesView<'_> {
  fn default() -> HeaderMutationRulesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMutationRules>> for HeaderMutationRulesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMutationRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderMutationRulesView<'msg> {

  pub fn to_owned(&self) -> HeaderMutationRules {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // allow_all_routing: optional message google.protobuf.BoolValue
  pub fn has_allow_all_routing(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn allow_all_routing_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_allow_all_routing().then(|| self.allow_all_routing())
  }
  pub fn allow_all_routing(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // allow_envoy: optional message google.protobuf.BoolValue
  pub fn has_allow_envoy(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn allow_envoy_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_allow_envoy().then(|| self.allow_envoy())
  }
  pub fn allow_envoy(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // disallow_system: optional message google.protobuf.BoolValue
  pub fn has_disallow_system(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn disallow_system_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_disallow_system().then(|| self.disallow_system())
  }
  pub fn disallow_system(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // disallow_all: optional message google.protobuf.BoolValue
  pub fn has_disallow_all(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn disallow_all_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_disallow_all().then(|| self.disallow_all())
  }
  pub fn disallow_all(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

  // allow_expression: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_allow_expression(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn allow_expression_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'msg>> {
    self.has_allow_expression().then(|| self.allow_expression())
  }
  pub fn allow_expression(self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView::default())
  }

  // disallow_expression: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_disallow_expression(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn disallow_expression_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'msg>> {
    self.has_disallow_expression().then(|| self.disallow_expression())
  }
  pub fn disallow_expression(self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView::default())
  }

  // disallow_is_error: optional message google.protobuf.BoolValue
  pub fn has_disallow_is_error(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn disallow_is_error_opt(self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'msg>> {
    self.has_disallow_is_error().then(|| self.disallow_is_error())
  }
  pub fn disallow_is_error(self) -> ::protobuf_well_known_types::BoolValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }

}

// SAFETY:
// - `HeaderMutationRulesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderMutationRulesView<'_> {}

// SAFETY:
// - `HeaderMutationRulesView` is `Send` because while its alive a `HeaderMutationRulesMut` cannot.
// - `HeaderMutationRulesView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderMutationRulesView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderMutationRulesView<'msg> {
  type Proxied = HeaderMutationRules;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderMutationRules> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderMutationRulesView<'msg> {
  fn into_view<'shorter>(self) -> HeaderMutationRulesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderMutationRules> for HeaderMutationRulesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderMutationRules {
    let mut dst = HeaderMutationRules::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderMutationRules> for HeaderMutationRulesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderMutationRules {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderMutationRules {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderMutationRulesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderMutationRulesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderMutationRulesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutationRules>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderMutationRulesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderMutationRulesMut<'msg> {
  type Message = HeaderMutationRules;
}

impl ::std::fmt::Debug for HeaderMutationRulesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutationRules>> for HeaderMutationRulesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutationRules>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderMutationRulesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutationRules> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderMutationRules {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // allow_all_routing: optional message google.protobuf.BoolValue
  pub fn has_allow_all_routing(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_allow_all_routing(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn allow_all_routing_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_allow_all_routing().then(|| self.allow_all_routing())
  }
  pub fn allow_all_routing(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn allow_all_routing_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_allow_all_routing(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // allow_envoy: optional message google.protobuf.BoolValue
  pub fn has_allow_envoy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_allow_envoy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn allow_envoy_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_allow_envoy().then(|| self.allow_envoy())
  }
  pub fn allow_envoy(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn allow_envoy_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_allow_envoy(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // disallow_system: optional message google.protobuf.BoolValue
  pub fn has_disallow_system(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_disallow_system(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn disallow_system_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_disallow_system().then(|| self.disallow_system())
  }
  pub fn disallow_system(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn disallow_system_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_disallow_system(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // disallow_all: optional message google.protobuf.BoolValue
  pub fn has_disallow_all(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_disallow_all(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn disallow_all_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_disallow_all().then(|| self.disallow_all())
  }
  pub fn disallow_all(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn disallow_all_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_disallow_all(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // allow_expression: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_allow_expression(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_allow_expression(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn allow_expression_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_>> {
    self.has_allow_expression().then(|| self.allow_expression())
  }
  pub fn allow_expression(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView::default())
  }
  pub fn allow_expression_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherMut<'_> {
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
  pub fn set_allow_expression(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // disallow_expression: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_disallow_expression(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_disallow_expression(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn disallow_expression_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_>> {
    self.has_disallow_expression().then(|| self.disallow_expression())
  }
  pub fn disallow_expression(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView::default())
  }
  pub fn disallow_expression_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherMut<'_> {
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
  pub fn set_disallow_expression(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // disallow_is_error: optional message google.protobuf.BoolValue
  pub fn has_disallow_is_error(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_disallow_is_error(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn disallow_is_error_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_disallow_is_error().then(|| self.disallow_is_error())
  }
  pub fn disallow_is_error(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn disallow_is_error_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_disallow_is_error(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

}

// SAFETY:
// - `HeaderMutationRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderMutationRulesMut<'_> {}

// SAFETY:
// - `HeaderMutationRulesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderMutationRulesMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderMutationRulesMut<'msg> {
  type Proxied = HeaderMutationRules;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderMutationRules> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderMutationRulesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderMutationRules>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderMutationRulesMut<'msg> {
  type MutProxied = HeaderMutationRules;
  fn as_mut(&mut self) -> HeaderMutationRulesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderMutationRulesMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderMutationRulesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderMutationRules {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderMutationRules> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderMutationRulesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderMutationRulesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // allow_all_routing: optional message google.protobuf.BoolValue
  pub fn has_allow_all_routing(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_allow_all_routing(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn allow_all_routing_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_allow_all_routing().then(|| self.allow_all_routing())
  }
  pub fn allow_all_routing(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn allow_all_routing_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_allow_all_routing(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // allow_envoy: optional message google.protobuf.BoolValue
  pub fn has_allow_envoy(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_allow_envoy(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn allow_envoy_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_allow_envoy().then(|| self.allow_envoy())
  }
  pub fn allow_envoy(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn allow_envoy_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_allow_envoy(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // disallow_system: optional message google.protobuf.BoolValue
  pub fn has_disallow_system(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_disallow_system(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn disallow_system_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_disallow_system().then(|| self.disallow_system())
  }
  pub fn disallow_system(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn disallow_system_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_disallow_system(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // disallow_all: optional message google.protobuf.BoolValue
  pub fn has_disallow_all(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_disallow_all(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn disallow_all_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_disallow_all().then(|| self.disallow_all())
  }
  pub fn disallow_all(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn disallow_all_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_disallow_all(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // allow_expression: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_allow_expression(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_allow_expression(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn allow_expression_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_>> {
    self.has_allow_expression().then(|| self.allow_expression())
  }
  pub fn allow_expression(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView::default())
  }
  pub fn allow_expression_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherMut<'_> {
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
  pub fn set_allow_expression(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // disallow_expression: optional message envoy.type.matcher.v3.RegexMatcher
  pub fn has_disallow_expression(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_disallow_expression(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn disallow_expression_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_>> {
    self.has_disallow_expression().then(|| self.disallow_expression())
  }
  pub fn disallow_expression(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherView::default())
  }
  pub fn disallow_expression_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcherMut<'_> {
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
  pub fn set_disallow_expression(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // disallow_is_error: optional message google.protobuf.BoolValue
  pub fn has_disallow_is_error(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_disallow_is_error(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn disallow_is_error_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::BoolValueView<'_>> {
    self.has_disallow_is_error().then(|| self.disallow_is_error())
  }
  pub fn disallow_is_error(&self) -> ::protobuf_well_known_types::BoolValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::BoolValueView::default())
  }
  pub fn disallow_is_error_mut(&mut self) -> ::protobuf_well_known_types::BoolValueMut<'_> {
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
  pub fn set_disallow_is_error(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::BoolValue>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

}  // impl HeaderMutationRules

impl ::std::ops::Drop for HeaderMutationRules {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderMutationRules {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderMutationRules {
  type Proxied = Self;
  fn as_view(&self) -> HeaderMutationRulesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderMutationRules {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderMutationRulesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderMutationRules {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__common__mutation_0rules__v3__HeaderMutationRules_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__common__mutation_0rules__v3__HeaderMutationRules_msg_init.0, &[<::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::regex::RegexMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::BoolValue as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__common__mutation_0rules__v3__HeaderMutationRules_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderMutationRules {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderMutationRules {
  type Msg = HeaderMutationRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutationRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMutationRules {
  type Msg = HeaderMutationRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutationRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderMutationRulesMut<'_> {
  type Msg = HeaderMutationRules;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutationRules> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMutationRulesMut<'_> {
  type Msg = HeaderMutationRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutationRules> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMutationRulesView<'_> {
  type Msg = HeaderMutationRules;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutationRules> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderMutationRulesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__mutation_0rules__v3__HeaderMutation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HeaderMutation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HeaderMutation>
}

impl ::protobuf::Message for HeaderMutation {
  type MessageView<'msg> = HeaderMutationView<'msg>;
  type MessageMut<'msg> = HeaderMutationMut<'msg>;
}

impl ::std::default::Default for HeaderMutation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HeaderMutation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HeaderMutation` is `Sync` because it does not implement interior mutability.
//    Neither does `HeaderMutationMut`.
unsafe impl ::std::marker::Sync for HeaderMutation {}

// SAFETY:
// - `HeaderMutation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HeaderMutation {}

impl ::protobuf::Proxied for HeaderMutation {
  type View<'msg> = HeaderMutationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HeaderMutation {}

impl ::protobuf::MutProxied for HeaderMutation {
  type Mut<'msg> = HeaderMutationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HeaderMutationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMutation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderMutationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HeaderMutationView<'msg> {
  type Message = HeaderMutation;
}

impl ::std::fmt::Debug for HeaderMutationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HeaderMutationView<'_> {
  fn default() -> HeaderMutationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMutation>> for HeaderMutationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HeaderMutation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderMutationView<'msg> {

  pub fn to_owned(&self) -> HeaderMutation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // remove: optional string
  pub fn has_remove(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn remove_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_remove().then(|| self.remove())
  }
  pub fn remove(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // append: optional message envoy.config.core.v3.HeaderValueOption
  pub fn has_append(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn append_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionView<'msg>> {
    self.has_append().then(|| self.append())
  }
  pub fn append(self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionView::default())
  }

  // remove_on_match: optional message envoy.config.common.mutation_rules.v3.HeaderMutation.RemoveOnMatch
  pub fn has_remove_on_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn remove_on_match_opt(self) -> ::std::option::Option<super::header_mutation::RemoveOnMatchView<'msg>> {
    self.has_remove_on_match().then(|| self.remove_on_match())
  }
  pub fn remove_on_match(self) -> super::header_mutation::RemoveOnMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::header_mutation::RemoveOnMatchView::default())
  }

  pub fn action(self) -> super::header_mutation::ActionOneof<'msg> {
    match self.action_case() {
      super::header_mutation::ActionCase::Remove =>
          super::header_mutation::ActionOneof::Remove(self.remove()),
      super::header_mutation::ActionCase::Append =>
          super::header_mutation::ActionOneof::Append(self.append()),
      super::header_mutation::ActionCase::RemoveOnMatch =>
          super::header_mutation::ActionOneof::RemoveOnMatch(self.remove_on_match()),
      _ => super::header_mutation::ActionOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn action_case(self) -> super::header_mutation::ActionCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::header_mutation::ActionCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HeaderMutationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HeaderMutationView<'_> {}

// SAFETY:
// - `HeaderMutationView` is `Send` because while its alive a `HeaderMutationMut` cannot.
// - `HeaderMutationView` does not use thread-local data.
unsafe impl ::std::marker::Send for HeaderMutationView<'_> {}

impl<'msg> ::protobuf::AsView for HeaderMutationView<'msg> {
  type Proxied = HeaderMutation;
  fn as_view(&self) -> ::protobuf::View<'msg, HeaderMutation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderMutationView<'msg> {
  fn into_view<'shorter>(self) -> HeaderMutationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderMutation> for HeaderMutationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderMutation {
    let mut dst = HeaderMutation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HeaderMutation> for HeaderMutationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HeaderMutation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HeaderMutation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderMutationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HeaderMutationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HeaderMutationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HeaderMutationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HeaderMutationMut<'msg> {
  type Message = HeaderMutation;
}

impl ::std::fmt::Debug for HeaderMutationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutation>> for HeaderMutationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HeaderMutationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HeaderMutation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HeaderMutation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // remove: optional string
  pub fn has_remove(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_remove(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn remove_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_remove().then(|| self.remove())
  }
  pub fn remove(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_remove(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // append: optional message envoy.config.core.v3.HeaderValueOption
  pub fn has_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn append_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionView<'_>> {
    self.has_append().then(|| self.append())
  }
  pub fn append(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionView::default())
  }
  pub fn append_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionMut<'_> {
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
  pub fn set_append(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // remove_on_match: optional message envoy.config.common.mutation_rules.v3.HeaderMutation.RemoveOnMatch
  pub fn has_remove_on_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_remove_on_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn remove_on_match_opt(&self) -> ::std::option::Option<super::header_mutation::RemoveOnMatchView<'_>> {
    self.has_remove_on_match().then(|| self.remove_on_match())
  }
  pub fn remove_on_match(&self) -> super::header_mutation::RemoveOnMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::header_mutation::RemoveOnMatchView::default())
  }
  pub fn remove_on_match_mut(&mut self) -> super::header_mutation::RemoveOnMatchMut<'_> {
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
  pub fn set_remove_on_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::header_mutation::RemoveOnMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn action(&self) -> super::header_mutation::ActionOneof<'_> {
    match &self.action_case() {
      super::header_mutation::ActionCase::Remove =>
          super::header_mutation::ActionOneof::Remove(self.remove()),
      super::header_mutation::ActionCase::Append =>
          super::header_mutation::ActionOneof::Append(self.append()),
      super::header_mutation::ActionCase::RemoveOnMatch =>
          super::header_mutation::ActionOneof::RemoveOnMatch(self.remove_on_match()),
      _ => super::header_mutation::ActionOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn action_case(&self) -> super::header_mutation::ActionCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::header_mutation::ActionCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `HeaderMutationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HeaderMutationMut<'_> {}

// SAFETY:
// - `HeaderMutationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HeaderMutationMut<'_> {}

impl<'msg> ::protobuf::AsView for HeaderMutationMut<'msg> {
  type Proxied = HeaderMutation;
  fn as_view(&self) -> ::protobuf::View<'_, HeaderMutation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HeaderMutationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HeaderMutation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HeaderMutationMut<'msg> {
  type MutProxied = HeaderMutation;
  fn as_mut(&mut self) -> HeaderMutationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HeaderMutationMut<'msg> {
  fn into_mut<'shorter>(self) -> HeaderMutationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HeaderMutation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HeaderMutation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HeaderMutationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HeaderMutationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // remove: optional string
  pub fn has_remove(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_remove(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn remove_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_remove().then(|| self.remove())
  }
  pub fn remove(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_remove(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // append: optional message envoy.config.core.v3.HeaderValueOption
  pub fn has_append(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_append(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn append_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionView<'_>> {
    self.has_append().then(|| self.append())
  }
  pub fn append(&self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionView::default())
  }
  pub fn append_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::HeaderValueOptionMut<'_> {
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
  pub fn set_append(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // remove_on_match: optional message envoy.config.common.mutation_rules.v3.HeaderMutation.RemoveOnMatch
  pub fn has_remove_on_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_remove_on_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn remove_on_match_opt(&self) -> ::std::option::Option<super::header_mutation::RemoveOnMatchView<'_>> {
    self.has_remove_on_match().then(|| self.remove_on_match())
  }
  pub fn remove_on_match(&self) -> super::header_mutation::RemoveOnMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::header_mutation::RemoveOnMatchView::default())
  }
  pub fn remove_on_match_mut(&mut self) -> super::header_mutation::RemoveOnMatchMut<'_> {
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
  pub fn set_remove_on_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::header_mutation::RemoveOnMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn action(&self) -> super::header_mutation::ActionOneof<'_> {
    match &self.action_case() {
      super::header_mutation::ActionCase::Remove =>
          super::header_mutation::ActionOneof::Remove(self.remove()),
      super::header_mutation::ActionCase::Append =>
          super::header_mutation::ActionOneof::Append(self.append()),
      super::header_mutation::ActionCase::RemoveOnMatch =>
          super::header_mutation::ActionOneof::RemoveOnMatch(self.remove_on_match()),
      _ => super::header_mutation::ActionOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn action_case(&self) -> super::header_mutation::ActionCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::header_mutation::ActionCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl HeaderMutation

impl ::std::ops::Drop for HeaderMutation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HeaderMutation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HeaderMutation {
  type Proxied = Self;
  fn as_view(&self) -> HeaderMutationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HeaderMutation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HeaderMutationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HeaderMutation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__common__mutation_0rules__v3__HeaderMutation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T33^!|#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__common__mutation_0rules__v3__HeaderMutation_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::header_mutation::RemoveOnMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__common__mutation_0rules__v3__HeaderMutation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderMutation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderMutation {
  type Msg = HeaderMutation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMutation {
  type Msg = HeaderMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HeaderMutationMut<'_> {
  type Msg = HeaderMutation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMutationMut<'_> {
  type Msg = HeaderMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HeaderMutationView<'_> {
  type Msg = HeaderMutation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HeaderMutation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HeaderMutationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod header_mutation {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__mutation_0rules__v3__HeaderMutation__RemoveOnMatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RemoveOnMatch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RemoveOnMatch>
}

impl ::protobuf::Message for RemoveOnMatch {
  type MessageView<'msg> = RemoveOnMatchView<'msg>;
  type MessageMut<'msg> = RemoveOnMatchMut<'msg>;
}

impl ::std::default::Default for RemoveOnMatch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RemoveOnMatch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RemoveOnMatch` is `Sync` because it does not implement interior mutability.
//    Neither does `RemoveOnMatchMut`.
unsafe impl ::std::marker::Sync for RemoveOnMatch {}

// SAFETY:
// - `RemoveOnMatch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RemoveOnMatch {}

impl ::protobuf::Proxied for RemoveOnMatch {
  type View<'msg> = RemoveOnMatchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RemoveOnMatch {}

impl ::protobuf::MutProxied for RemoveOnMatch {
  type Mut<'msg> = RemoveOnMatchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RemoveOnMatchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RemoveOnMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RemoveOnMatchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RemoveOnMatchView<'msg> {
  type Message = RemoveOnMatch;
}

impl ::std::fmt::Debug for RemoveOnMatchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RemoveOnMatchView<'_> {
  fn default() -> RemoveOnMatchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RemoveOnMatch>> for RemoveOnMatchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RemoveOnMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RemoveOnMatchView<'msg> {

  pub fn to_owned(&self) -> RemoveOnMatch {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // key_matcher: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_key_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn key_matcher_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_key_matcher().then(|| self.key_matcher())
  }
  pub fn key_matcher(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

}

// SAFETY:
// - `RemoveOnMatchView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RemoveOnMatchView<'_> {}

// SAFETY:
// - `RemoveOnMatchView` is `Send` because while its alive a `RemoveOnMatchMut` cannot.
// - `RemoveOnMatchView` does not use thread-local data.
unsafe impl ::std::marker::Send for RemoveOnMatchView<'_> {}

impl<'msg> ::protobuf::AsView for RemoveOnMatchView<'msg> {
  type Proxied = RemoveOnMatch;
  fn as_view(&self) -> ::protobuf::View<'msg, RemoveOnMatch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RemoveOnMatchView<'msg> {
  fn into_view<'shorter>(self) -> RemoveOnMatchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RemoveOnMatch> for RemoveOnMatchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RemoveOnMatch {
    let mut dst = RemoveOnMatch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RemoveOnMatch> for RemoveOnMatchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RemoveOnMatch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RemoveOnMatch {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RemoveOnMatchView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RemoveOnMatchMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RemoveOnMatchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RemoveOnMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RemoveOnMatchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RemoveOnMatchMut<'msg> {
  type Message = RemoveOnMatch;
}

impl ::std::fmt::Debug for RemoveOnMatchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RemoveOnMatch>> for RemoveOnMatchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RemoveOnMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RemoveOnMatchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RemoveOnMatch> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RemoveOnMatch {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // key_matcher: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_key_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_key_matcher().then(|| self.key_matcher())
  }
  pub fn key_matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn key_matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_key_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

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
// - `RemoveOnMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RemoveOnMatchMut<'_> {}

// SAFETY:
// - `RemoveOnMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RemoveOnMatchMut<'_> {}

impl<'msg> ::protobuf::AsView for RemoveOnMatchMut<'msg> {
  type Proxied = RemoveOnMatch;
  fn as_view(&self) -> ::protobuf::View<'_, RemoveOnMatch> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RemoveOnMatchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RemoveOnMatch>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RemoveOnMatchMut<'msg> {
  type MutProxied = RemoveOnMatch;
  fn as_mut(&mut self) -> RemoveOnMatchMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RemoveOnMatchMut<'msg> {
  fn into_mut<'shorter>(self) -> RemoveOnMatchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RemoveOnMatch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RemoveOnMatch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RemoveOnMatchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RemoveOnMatchMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // key_matcher: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_key_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_key_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn key_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_key_matcher().then(|| self.key_matcher())
  }
  pub fn key_matcher(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn key_matcher_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_key_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl RemoveOnMatch

impl ::std::ops::Drop for RemoveOnMatch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RemoveOnMatch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RemoveOnMatch {
  type Proxied = Self;
  fn as_view(&self) -> RemoveOnMatchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RemoveOnMatch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RemoveOnMatchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RemoveOnMatch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::header_mutation::envoy__config__common__mutation_0rules__v3__HeaderMutation__RemoveOnMatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::header_mutation::envoy__config__common__mutation_0rules__v3__HeaderMutation__RemoveOnMatch_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::header_mutation::envoy__config__common__mutation_0rules__v3__HeaderMutation__RemoveOnMatch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RemoveOnMatch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RemoveOnMatch {
  type Msg = RemoveOnMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoveOnMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RemoveOnMatch {
  type Msg = RemoveOnMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoveOnMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RemoveOnMatchMut<'_> {
  type Msg = RemoveOnMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoveOnMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RemoveOnMatchMut<'_> {
  type Msg = RemoveOnMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoveOnMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RemoveOnMatchView<'_> {
  type Msg = RemoveOnMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RemoveOnMatch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RemoveOnMatchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum ActionOneof<'msg> {
  Remove(&'msg ::protobuf::ProtoStr) = 1,
  Append(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::base::HeaderValueOption>) = 2,
  RemoveOnMatch(::protobuf::View<'msg, super::super::header_mutation::RemoveOnMatch>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum ActionCase {
  Remove = 1,
  Append = 2,
  RemoveOnMatch = 3,

  not_set = 0
}

impl ActionCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<ActionCase> {
    match v {
      0 => Some(ActionCase::not_set),
      1 => Some(ActionCase::Remove),
      2 => Some(ActionCase::Append),
      3 => Some(ActionCase::RemoveOnMatch),
      _ => None
    }
  }
}
}  // pub mod header_mutation


