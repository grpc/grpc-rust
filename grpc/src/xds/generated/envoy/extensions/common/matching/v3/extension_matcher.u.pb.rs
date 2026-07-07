const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__common__matching__v3__ExtensionWithMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtensionWithMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtensionWithMatcher>
}

impl ::protobuf::Message for ExtensionWithMatcher {
  type MessageView<'msg> = ExtensionWithMatcherView<'msg>;
  type MessageMut<'msg> = ExtensionWithMatcherMut<'msg>;
}

impl ::std::default::Default for ExtensionWithMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtensionWithMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtensionWithMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtensionWithMatcherMut`.
unsafe impl ::std::marker::Sync for ExtensionWithMatcher {}

// SAFETY:
// - `ExtensionWithMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionWithMatcher {}

impl ::protobuf::Proxied for ExtensionWithMatcher {
  type View<'msg> = ExtensionWithMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtensionWithMatcher {}

impl ::protobuf::MutProxied for ExtensionWithMatcher {
  type Mut<'msg> = ExtensionWithMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtensionWithMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionWithMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionWithMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtensionWithMatcherView<'msg> {
  type Message = ExtensionWithMatcher;
}

impl ::std::fmt::Debug for ExtensionWithMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtensionWithMatcherView<'_> {
  fn default() -> ExtensionWithMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionWithMatcher>> for ExtensionWithMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionWithMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionWithMatcherView<'msg> {

  pub fn to_owned(&self) -> ExtensionWithMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // matcher: optional message envoy.config.common.matcher.v3.Matcher
  pub fn has_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn matcher_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(self) -> crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherView::default())
  }

  // xds_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_xds_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn xds_matcher_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_xds_matcher().then(|| self.xds_matcher())
  }
  pub fn xds_matcher(self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }

  // extension_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_extension_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn extension_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_extension_config().then(|| self.extension_config())
  }
  pub fn extension_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

}

// SAFETY:
// - `ExtensionWithMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtensionWithMatcherView<'_> {}

// SAFETY:
// - `ExtensionWithMatcherView` is `Send` because while its alive a `ExtensionWithMatcherMut` cannot.
// - `ExtensionWithMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionWithMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionWithMatcherView<'msg> {
  type Proxied = ExtensionWithMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtensionWithMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionWithMatcherView<'msg> {
  fn into_view<'shorter>(self) -> ExtensionWithMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtensionWithMatcher> for ExtensionWithMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtensionWithMatcher {
    let mut dst = ExtensionWithMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtensionWithMatcher> for ExtensionWithMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtensionWithMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtensionWithMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionWithMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionWithMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtensionWithMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionWithMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionWithMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtensionWithMatcherMut<'msg> {
  type Message = ExtensionWithMatcher;
}

impl ::std::fmt::Debug for ExtensionWithMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionWithMatcher>> for ExtensionWithMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionWithMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionWithMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionWithMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtensionWithMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // matcher: optional message envoy.config.common.matcher.v3.Matcher
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::common::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // xds_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_xds_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_xds_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn xds_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_xds_matcher().then(|| self.xds_matcher())
  }
  pub fn xds_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn xds_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_xds_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // extension_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_extension_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_extension_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn extension_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_extension_config().then(|| self.extension_config())
  }
  pub fn extension_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn extension_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_extension_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

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
// - `ExtensionWithMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtensionWithMatcherMut<'_> {}

// SAFETY:
// - `ExtensionWithMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtensionWithMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionWithMatcherMut<'msg> {
  type Proxied = ExtensionWithMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, ExtensionWithMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionWithMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtensionWithMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtensionWithMatcherMut<'msg> {
  type MutProxied = ExtensionWithMatcher;
  fn as_mut(&mut self) -> ExtensionWithMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtensionWithMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtensionWithMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtensionWithMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtensionWithMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtensionWithMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtensionWithMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // matcher: optional message envoy.config.common.matcher.v3.Matcher
  pub fn has_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> crate::xds::generated::envoy::config::common::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::common::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // xds_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_xds_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_xds_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn xds_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_xds_matcher().then(|| self.xds_matcher())
  }
  pub fn xds_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn xds_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_xds_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // extension_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_extension_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_extension_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn extension_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_extension_config().then(|| self.extension_config())
  }
  pub fn extension_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn extension_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_extension_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl ExtensionWithMatcher

impl ::std::ops::Drop for ExtensionWithMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtensionWithMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtensionWithMatcher {
  type Proxied = Self;
  fn as_view(&self) -> ExtensionWithMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtensionWithMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtensionWithMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtensionWithMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__common__matching__v3__ExtensionWithMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__common__matching__v3__ExtensionWithMatcher_msg_init.0, &[<crate::xds::generated::envoy::config::common::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__common__matching__v3__ExtensionWithMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionWithMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionWithMatcher {
  type Msg = ExtensionWithMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionWithMatcher {
  type Msg = ExtensionWithMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionWithMatcherMut<'_> {
  type Msg = ExtensionWithMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionWithMatcherMut<'_> {
  type Msg = ExtensionWithMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionWithMatcherView<'_> {
  type Msg = ExtensionWithMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionWithMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__extensions__common__matching__v3__ExtensionWithMatcherPerRoute_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ExtensionWithMatcherPerRoute {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ExtensionWithMatcherPerRoute>
}

impl ::protobuf::Message for ExtensionWithMatcherPerRoute {
  type MessageView<'msg> = ExtensionWithMatcherPerRouteView<'msg>;
  type MessageMut<'msg> = ExtensionWithMatcherPerRouteMut<'msg>;
}

impl ::std::default::Default for ExtensionWithMatcherPerRoute {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ExtensionWithMatcherPerRoute {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ExtensionWithMatcherPerRoute` is `Sync` because it does not implement interior mutability.
//    Neither does `ExtensionWithMatcherPerRouteMut`.
unsafe impl ::std::marker::Sync for ExtensionWithMatcherPerRoute {}

// SAFETY:
// - `ExtensionWithMatcherPerRoute` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionWithMatcherPerRoute {}

impl ::protobuf::Proxied for ExtensionWithMatcherPerRoute {
  type View<'msg> = ExtensionWithMatcherPerRouteView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ExtensionWithMatcherPerRoute {}

impl ::protobuf::MutProxied for ExtensionWithMatcherPerRoute {
  type Mut<'msg> = ExtensionWithMatcherPerRouteMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ExtensionWithMatcherPerRouteView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionWithMatcherPerRoute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionWithMatcherPerRouteView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ExtensionWithMatcherPerRouteView<'msg> {
  type Message = ExtensionWithMatcherPerRoute;
}

impl ::std::fmt::Debug for ExtensionWithMatcherPerRouteView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ExtensionWithMatcherPerRouteView<'_> {
  fn default() -> ExtensionWithMatcherPerRouteView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionWithMatcherPerRoute>> for ExtensionWithMatcherPerRouteView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ExtensionWithMatcherPerRoute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionWithMatcherPerRouteView<'msg> {

  pub fn to_owned(&self) -> ExtensionWithMatcherPerRoute {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // xds_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_xds_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn xds_matcher_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg>> {
    self.has_xds_matcher().then(|| self.xds_matcher())
  }
  pub fn xds_matcher(self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }

}

// SAFETY:
// - `ExtensionWithMatcherPerRouteView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ExtensionWithMatcherPerRouteView<'_> {}

// SAFETY:
// - `ExtensionWithMatcherPerRouteView` is `Send` because while its alive a `ExtensionWithMatcherPerRouteMut` cannot.
// - `ExtensionWithMatcherPerRouteView` does not use thread-local data.
unsafe impl ::std::marker::Send for ExtensionWithMatcherPerRouteView<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionWithMatcherPerRouteView<'msg> {
  type Proxied = ExtensionWithMatcherPerRoute;
  fn as_view(&self) -> ::protobuf::View<'msg, ExtensionWithMatcherPerRoute> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionWithMatcherPerRouteView<'msg> {
  fn into_view<'shorter>(self) -> ExtensionWithMatcherPerRouteView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtensionWithMatcherPerRoute> for ExtensionWithMatcherPerRouteView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtensionWithMatcherPerRoute {
    let mut dst = ExtensionWithMatcherPerRoute::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ExtensionWithMatcherPerRoute> for ExtensionWithMatcherPerRouteMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ExtensionWithMatcherPerRoute {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ExtensionWithMatcherPerRoute {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionWithMatcherPerRouteView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ExtensionWithMatcherPerRouteMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ExtensionWithMatcherPerRouteMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionWithMatcherPerRoute>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ExtensionWithMatcherPerRouteMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ExtensionWithMatcherPerRouteMut<'msg> {
  type Message = ExtensionWithMatcherPerRoute;
}

impl ::std::fmt::Debug for ExtensionWithMatcherPerRouteMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionWithMatcherPerRoute>> for ExtensionWithMatcherPerRouteMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionWithMatcherPerRoute>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ExtensionWithMatcherPerRouteMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ExtensionWithMatcherPerRoute> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ExtensionWithMatcherPerRoute {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // xds_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_xds_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_xds_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn xds_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_xds_matcher().then(|| self.xds_matcher())
  }
  pub fn xds_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn xds_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_xds_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

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
// - `ExtensionWithMatcherPerRouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ExtensionWithMatcherPerRouteMut<'_> {}

// SAFETY:
// - `ExtensionWithMatcherPerRouteMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ExtensionWithMatcherPerRouteMut<'_> {}

impl<'msg> ::protobuf::AsView for ExtensionWithMatcherPerRouteMut<'msg> {
  type Proxied = ExtensionWithMatcherPerRoute;
  fn as_view(&self) -> ::protobuf::View<'_, ExtensionWithMatcherPerRoute> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ExtensionWithMatcherPerRouteMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ExtensionWithMatcherPerRoute>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ExtensionWithMatcherPerRouteMut<'msg> {
  type MutProxied = ExtensionWithMatcherPerRoute;
  fn as_mut(&mut self) -> ExtensionWithMatcherPerRouteMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ExtensionWithMatcherPerRouteMut<'msg> {
  fn into_mut<'shorter>(self) -> ExtensionWithMatcherPerRouteMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ExtensionWithMatcherPerRoute {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ExtensionWithMatcherPerRoute> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ExtensionWithMatcherPerRouteView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ExtensionWithMatcherPerRouteMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // xds_matcher: optional message xds.type.matcher.v3.Matcher
  pub fn has_xds_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_xds_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn xds_matcher_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_>> {
    self.has_xds_matcher().then(|| self.xds_matcher())
  }
  pub fn xds_matcher(&self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherView::default())
  }
  pub fn xds_matcher_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::matcher::MatcherMut<'_> {
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
  pub fn set_xds_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl ExtensionWithMatcherPerRoute

impl ::std::ops::Drop for ExtensionWithMatcherPerRoute {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ExtensionWithMatcherPerRoute {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ExtensionWithMatcherPerRoute {
  type Proxied = Self;
  fn as_view(&self) -> ExtensionWithMatcherPerRouteView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ExtensionWithMatcherPerRoute {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ExtensionWithMatcherPerRouteMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ExtensionWithMatcherPerRoute {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__extensions__common__matching__v3__ExtensionWithMatcherPerRoute_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__extensions__common__matching__v3__ExtensionWithMatcherPerRoute_msg_init.0, &[<crate::xds::generated::xds::r#type::matcher::v3::matcher::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__extensions__common__matching__v3__ExtensionWithMatcherPerRoute_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionWithMatcherPerRoute {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionWithMatcherPerRoute {
  type Msg = ExtensionWithMatcherPerRoute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcherPerRoute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionWithMatcherPerRoute {
  type Msg = ExtensionWithMatcherPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcherPerRoute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ExtensionWithMatcherPerRouteMut<'_> {
  type Msg = ExtensionWithMatcherPerRoute;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcherPerRoute> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionWithMatcherPerRouteMut<'_> {
  type Msg = ExtensionWithMatcherPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcherPerRoute> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ExtensionWithMatcherPerRouteView<'_> {
  type Msg = ExtensionWithMatcherPerRoute;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ExtensionWithMatcherPerRoute> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ExtensionWithMatcherPerRouteMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



