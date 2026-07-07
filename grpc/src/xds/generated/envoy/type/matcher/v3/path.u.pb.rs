const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__type__matcher__v3__PathMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PathMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PathMatcher>
}

impl ::protobuf::Message for PathMatcher {
  type MessageView<'msg> = PathMatcherView<'msg>;
  type MessageMut<'msg> = PathMatcherMut<'msg>;
}

impl ::std::default::Default for PathMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PathMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PathMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `PathMatcherMut`.
unsafe impl ::std::marker::Sync for PathMatcher {}

// SAFETY:
// - `PathMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PathMatcher {}

impl ::protobuf::Proxied for PathMatcher {
  type View<'msg> = PathMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PathMatcher {}

impl ::protobuf::MutProxied for PathMatcher {
  type Mut<'msg> = PathMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PathMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PathMatcherView<'msg> {
  type Message = PathMatcher;
}

impl ::std::fmt::Debug for PathMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PathMatcherView<'_> {
  fn default() -> PathMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PathMatcher>> for PathMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PathMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathMatcherView<'msg> {

  pub fn to_owned(&self) -> PathMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // path: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_path(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn path_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_path().then(|| self.path())
  }
  pub fn path(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

  pub fn rule(self) -> super::path_matcher::RuleOneof<'msg> {
    match self.rule_case() {
      super::path_matcher::RuleCase::Path =>
          super::path_matcher::RuleOneof::Path(self.path()),
      _ => super::path_matcher::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(self) -> super::path_matcher::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::path_matcher::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PathMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PathMatcherView<'_> {}

// SAFETY:
// - `PathMatcherView` is `Send` because while its alive a `PathMatcherMut` cannot.
// - `PathMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for PathMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for PathMatcherView<'msg> {
  type Proxied = PathMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, PathMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathMatcherView<'msg> {
  fn into_view<'shorter>(self) -> PathMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PathMatcher> for PathMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathMatcher {
    let mut dst = PathMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PathMatcher> for PathMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PathMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PathMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PathMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PathMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PathMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PathMatcherMut<'msg> {
  type Message = PathMatcher;
}

impl ::std::fmt::Debug for PathMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PathMatcher>> for PathMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PathMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PathMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PathMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PathMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // path: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn path_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_path().then(|| self.path())
  }
  pub fn path(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn path_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_path(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  pub fn rule(&self) -> super::path_matcher::RuleOneof<'_> {
    match &self.rule_case() {
      super::path_matcher::RuleCase::Path =>
          super::path_matcher::RuleOneof::Path(self.path()),
      _ => super::path_matcher::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::path_matcher::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::path_matcher::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PathMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PathMatcherMut<'_> {}

// SAFETY:
// - `PathMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PathMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for PathMatcherMut<'msg> {
  type Proxied = PathMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, PathMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PathMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PathMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PathMatcherMut<'msg> {
  type MutProxied = PathMatcher;
  fn as_mut(&mut self) -> PathMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PathMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> PathMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PathMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PathMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PathMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PathMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // path: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_path(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_path(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn path_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_path().then(|| self.path())
  }
  pub fn path(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn path_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_path(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  pub fn rule(&self) -> super::path_matcher::RuleOneof<'_> {
    match &self.rule_case() {
      super::path_matcher::RuleCase::Path =>
          super::path_matcher::RuleOneof::Path(self.path()),
      _ => super::path_matcher::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::path_matcher::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::path_matcher::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl PathMatcher

impl ::std::ops::Drop for PathMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PathMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PathMatcher {
  type Proxied = Self;
  fn as_view(&self) -> PathMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PathMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PathMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PathMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__type__matcher__v3__PathMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3^!");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__type__matcher__v3__PathMatcher_msg_init.0, &[<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__type__matcher__v3__PathMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathMatcher {
  type Msg = PathMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathMatcher {
  type Msg = PathMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PathMatcherMut<'_> {
  type Msg = PathMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathMatcherMut<'_> {
  type Msg = PathMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PathMatcherView<'_> {
  type Msg = PathMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PathMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PathMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod path_matcher {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum RuleOneof<'msg> {
  Path(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) = 1,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum RuleCase {
  Path = 1,

  not_set = 0
}

impl RuleCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<RuleCase> {
    match v {
      0 => Some(RuleCase::not_set),
      1 => Some(RuleCase::Path),
      _ => None
    }
  }
}
}  // pub mod path_matcher


