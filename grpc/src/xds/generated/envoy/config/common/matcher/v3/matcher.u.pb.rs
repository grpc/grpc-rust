const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Matcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Matcher>
}

impl ::protobuf::Message for Matcher {
  type MessageView<'msg> = MatcherView<'msg>;
  type MessageMut<'msg> = MatcherMut<'msg>;
}

impl ::std::default::Default for Matcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Matcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Matcher` is `Sync` because it does not implement interior mutability.
//    Neither does `MatcherMut`.
unsafe impl ::std::marker::Sync for Matcher {}

// SAFETY:
// - `Matcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Matcher {}

impl ::protobuf::Proxied for Matcher {
  type View<'msg> = MatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Matcher {}

impl ::protobuf::MutProxied for Matcher {
  type Mut<'msg> = MatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Matcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MatcherView<'msg> {
  type Message = Matcher;
}

impl ::std::fmt::Debug for MatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MatcherView<'_> {
  fn default() -> MatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Matcher>> for MatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Matcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatcherView<'msg> {

  pub fn to_owned(&self) -> Matcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // matcher_list: optional message envoy.config.common.matcher.v3.Matcher.MatcherList
  pub fn has_matcher_list(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn matcher_list_opt(self) -> ::std::option::Option<super::matcher::MatcherListView<'msg>> {
    self.has_matcher_list().then(|| self.matcher_list())
  }
  pub fn matcher_list(self) -> super::matcher::MatcherListView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::matcher::MatcherListView::default())
  }

  // matcher_tree: optional message envoy.config.common.matcher.v3.Matcher.MatcherTree
  pub fn has_matcher_tree(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn matcher_tree_opt(self) -> ::std::option::Option<super::matcher::MatcherTreeView<'msg>> {
    self.has_matcher_tree().then(|| self.matcher_tree())
  }
  pub fn matcher_tree(self) -> super::matcher::MatcherTreeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::matcher::MatcherTreeView::default())
  }

  // on_no_match: optional message envoy.config.common.matcher.v3.Matcher.OnMatch
  pub fn has_on_no_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn on_no_match_opt(self) -> ::std::option::Option<super::matcher::OnMatchView<'msg>> {
    self.has_on_no_match().then(|| self.on_no_match())
  }
  pub fn on_no_match(self) -> super::matcher::OnMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::matcher::OnMatchView::default())
  }

  pub fn matcher_type(self) -> super::matcher::MatcherTypeOneof<'msg> {
    match self.matcher_type_case() {
      super::matcher::MatcherTypeCase::MatcherList =>
          super::matcher::MatcherTypeOneof::MatcherList(self.matcher_list()),
      super::matcher::MatcherTypeCase::MatcherTree =>
          super::matcher::MatcherTypeOneof::MatcherTree(self.matcher_tree()),
      _ => super::matcher::MatcherTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn matcher_type_case(self) -> super::matcher::MatcherTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::matcher::MatcherTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MatcherView<'_> {}

// SAFETY:
// - `MatcherView` is `Send` because while its alive a `MatcherMut` cannot.
// - `MatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for MatcherView<'_> {}

impl<'msg> ::protobuf::AsView for MatcherView<'msg> {
  type Proxied = Matcher;
  fn as_view(&self) -> ::protobuf::View<'msg, Matcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatcherView<'msg> {
  fn into_view<'shorter>(self) -> MatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Matcher> for MatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Matcher {
    let mut dst = Matcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Matcher> for MatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Matcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Matcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Matcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MatcherMut<'msg> {
  type Message = Matcher;
}

impl ::std::fmt::Debug for MatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Matcher>> for MatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Matcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Matcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Matcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // matcher_list: optional message envoy.config.common.matcher.v3.Matcher.MatcherList
  pub fn has_matcher_list(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_matcher_list(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn matcher_list_opt(&self) -> ::std::option::Option<super::matcher::MatcherListView<'_>> {
    self.has_matcher_list().then(|| self.matcher_list())
  }
  pub fn matcher_list(&self) -> super::matcher::MatcherListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::matcher::MatcherListView::default())
  }
  pub fn matcher_list_mut(&mut self) -> super::matcher::MatcherListMut<'_> {
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
  pub fn set_matcher_list(&mut self,
    val: impl ::protobuf::IntoProxied<super::matcher::MatcherList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // matcher_tree: optional message envoy.config.common.matcher.v3.Matcher.MatcherTree
  pub fn has_matcher_tree(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_matcher_tree(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn matcher_tree_opt(&self) -> ::std::option::Option<super::matcher::MatcherTreeView<'_>> {
    self.has_matcher_tree().then(|| self.matcher_tree())
  }
  pub fn matcher_tree(&self) -> super::matcher::MatcherTreeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::matcher::MatcherTreeView::default())
  }
  pub fn matcher_tree_mut(&mut self) -> super::matcher::MatcherTreeMut<'_> {
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
  pub fn set_matcher_tree(&mut self,
    val: impl ::protobuf::IntoProxied<super::matcher::MatcherTree>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // on_no_match: optional message envoy.config.common.matcher.v3.Matcher.OnMatch
  pub fn has_on_no_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_on_no_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn on_no_match_opt(&self) -> ::std::option::Option<super::matcher::OnMatchView<'_>> {
    self.has_on_no_match().then(|| self.on_no_match())
  }
  pub fn on_no_match(&self) -> super::matcher::OnMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::matcher::OnMatchView::default())
  }
  pub fn on_no_match_mut(&mut self) -> super::matcher::OnMatchMut<'_> {
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
  pub fn set_on_no_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::matcher::OnMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn matcher_type(&self) -> super::matcher::MatcherTypeOneof<'_> {
    match &self.matcher_type_case() {
      super::matcher::MatcherTypeCase::MatcherList =>
          super::matcher::MatcherTypeOneof::MatcherList(self.matcher_list()),
      super::matcher::MatcherTypeCase::MatcherTree =>
          super::matcher::MatcherTypeOneof::MatcherTree(self.matcher_tree()),
      _ => super::matcher::MatcherTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn matcher_type_case(&self) -> super::matcher::MatcherTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::matcher::MatcherTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MatcherMut<'_> {}

// SAFETY:
// - `MatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for MatcherMut<'msg> {
  type Proxied = Matcher;
  fn as_view(&self) -> ::protobuf::View<'_, Matcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Matcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MatcherMut<'msg> {
  type MutProxied = Matcher;
  fn as_mut(&mut self) -> MatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> MatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Matcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Matcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // matcher_list: optional message envoy.config.common.matcher.v3.Matcher.MatcherList
  pub fn has_matcher_list(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_matcher_list(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn matcher_list_opt(&self) -> ::std::option::Option<super::matcher::MatcherListView<'_>> {
    self.has_matcher_list().then(|| self.matcher_list())
  }
  pub fn matcher_list(&self) -> super::matcher::MatcherListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::matcher::MatcherListView::default())
  }
  pub fn matcher_list_mut(&mut self) -> super::matcher::MatcherListMut<'_> {
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
  pub fn set_matcher_list(&mut self,
    val: impl ::protobuf::IntoProxied<super::matcher::MatcherList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // matcher_tree: optional message envoy.config.common.matcher.v3.Matcher.MatcherTree
  pub fn has_matcher_tree(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_matcher_tree(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn matcher_tree_opt(&self) -> ::std::option::Option<super::matcher::MatcherTreeView<'_>> {
    self.has_matcher_tree().then(|| self.matcher_tree())
  }
  pub fn matcher_tree(&self) -> super::matcher::MatcherTreeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::matcher::MatcherTreeView::default())
  }
  pub fn matcher_tree_mut(&mut self) -> super::matcher::MatcherTreeMut<'_> {
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
  pub fn set_matcher_tree(&mut self,
    val: impl ::protobuf::IntoProxied<super::matcher::MatcherTree>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // on_no_match: optional message envoy.config.common.matcher.v3.Matcher.OnMatch
  pub fn has_on_no_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_on_no_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn on_no_match_opt(&self) -> ::std::option::Option<super::matcher::OnMatchView<'_>> {
    self.has_on_no_match().then(|| self.on_no_match())
  }
  pub fn on_no_match(&self) -> super::matcher::OnMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::matcher::OnMatchView::default())
  }
  pub fn on_no_match_mut(&mut self) -> super::matcher::OnMatchMut<'_> {
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
  pub fn set_on_no_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::matcher::OnMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn matcher_type(&self) -> super::matcher::MatcherTypeOneof<'_> {
    match &self.matcher_type_case() {
      super::matcher::MatcherTypeCase::MatcherList =>
          super::matcher::MatcherTypeOneof::MatcherList(self.matcher_list()),
      super::matcher::MatcherTypeCase::MatcherTree =>
          super::matcher::MatcherTypeOneof::MatcherTree(self.matcher_tree()),
      _ => super::matcher::MatcherTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn matcher_type_case(&self) -> super::matcher::MatcherTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::matcher::MatcherTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Matcher

impl ::std::ops::Drop for Matcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Matcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Matcher {
  type Proxied = Self;
  fn as_view(&self) -> MatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Matcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Matcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__common__matcher__v3__Matcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^!|#");
        super::matcher::envoy__config__common__matcher__v3__Matcher__MatcherList_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        super::matcher::matcher_list::envoy__config__common__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        super::matcher::envoy__config__common__matcher__v3__Matcher__MatcherTree_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333^#|$|%");
        super::matcher::matcher_tree::envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        super::matcher::matcher_tree::match_map::envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        super::matcher::envoy__config__common__matcher__v3__Matcher__OnMatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33/P^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__common__matcher__v3__Matcher_msg_init.0, &[super::matcher::envoy__config__common__matcher__v3__Matcher__MatcherList_msg_init.0,
            super::matcher::envoy__config__common__matcher__v3__Matcher__MatcherTree_msg_init.0,
            super::matcher::envoy__config__common__matcher__v3__Matcher__OnMatch_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::envoy__config__common__matcher__v3__Matcher__MatcherList_msg_init.0, &[super::matcher::matcher_list::envoy__config__common__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::matcher_list::envoy__config__common__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init.0, &[<super::matcher::matcher_list::Predicate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::matcher::envoy__config__common__matcher__v3__Matcher__OnMatch_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::envoy__config__common__matcher__v3__Matcher__MatcherTree_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::matcher::matcher_tree::envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0,
            super::matcher::matcher_tree::envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0,
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::matcher_tree::envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0, &[super::matcher::matcher_tree::match_map::envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::matcher_tree::match_map::envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init.0, &[super::matcher::envoy__config__common__matcher__v3__Matcher__OnMatch_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::envoy__config__common__matcher__v3__Matcher__OnMatch_msg_init.0, &[super::envoy__config__common__matcher__v3__Matcher_msg_init.0,
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__common__matcher__v3__Matcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Matcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Matcher {
  type Msg = Matcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Matcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Matcher {
  type Msg = Matcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Matcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatcherMut<'_> {
  type Msg = Matcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Matcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatcherMut<'_> {
  type Msg = Matcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Matcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatcherView<'_> {
  type Msg = Matcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Matcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod matcher {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher__OnMatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OnMatch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OnMatch>
}

impl ::protobuf::Message for OnMatch {
  type MessageView<'msg> = OnMatchView<'msg>;
  type MessageMut<'msg> = OnMatchMut<'msg>;
}

impl ::std::default::Default for OnMatch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OnMatch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OnMatch` is `Sync` because it does not implement interior mutability.
//    Neither does `OnMatchMut`.
unsafe impl ::std::marker::Sync for OnMatch {}

// SAFETY:
// - `OnMatch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OnMatch {}

impl ::protobuf::Proxied for OnMatch {
  type View<'msg> = OnMatchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OnMatch {}

impl ::protobuf::MutProxied for OnMatch {
  type Mut<'msg> = OnMatchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OnMatchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OnMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OnMatchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OnMatchView<'msg> {
  type Message = OnMatch;
}

impl ::std::fmt::Debug for OnMatchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OnMatchView<'_> {
  fn default() -> OnMatchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OnMatch>> for OnMatchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OnMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OnMatchView<'msg> {

  pub fn to_owned(&self) -> OnMatch {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // matcher: optional message envoy.config.common.matcher.v3.Matcher
  pub fn has_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn matcher_opt(self) -> ::std::option::Option<super::super::MatcherView<'msg>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(self) -> super::super::MatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::MatcherView::default())
  }

  // action: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_action(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn action_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_action().then(|| self.action())
  }
  pub fn action(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // keep_matching: optional bool
  pub fn keep_matching(self) -> bool {
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

  pub fn on_match(self) -> super::super::matcher::on_match::OnMatchOneof<'msg> {
    match self.on_match_case() {
      super::super::matcher::on_match::OnMatchCase::Matcher =>
          super::super::matcher::on_match::OnMatchOneof::Matcher(self.matcher()),
      super::super::matcher::on_match::OnMatchCase::Action =>
          super::super::matcher::on_match::OnMatchOneof::Action(self.action()),
      _ => super::super::matcher::on_match::OnMatchOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn on_match_case(self) -> super::super::matcher::on_match::OnMatchCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::matcher::on_match::OnMatchCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `OnMatchView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OnMatchView<'_> {}

// SAFETY:
// - `OnMatchView` is `Send` because while its alive a `OnMatchMut` cannot.
// - `OnMatchView` does not use thread-local data.
unsafe impl ::std::marker::Send for OnMatchView<'_> {}

impl<'msg> ::protobuf::AsView for OnMatchView<'msg> {
  type Proxied = OnMatch;
  fn as_view(&self) -> ::protobuf::View<'msg, OnMatch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OnMatchView<'msg> {
  fn into_view<'shorter>(self) -> OnMatchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OnMatch> for OnMatchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OnMatch {
    let mut dst = OnMatch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OnMatch> for OnMatchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OnMatch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OnMatch {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OnMatchView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OnMatchMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OnMatchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OnMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OnMatchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OnMatchMut<'msg> {
  type Message = OnMatch;
}

impl ::std::fmt::Debug for OnMatchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OnMatch>> for OnMatchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OnMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OnMatchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OnMatch> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OnMatch {
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
  pub fn matcher_opt(&self) -> ::std::option::Option<super::super::MatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> super::super::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::MatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> super::super::MatcherMut<'_> {
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
    val: impl ::protobuf::IntoProxied<super::super::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // action: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn action_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_action().then(|| self.action())
  }
  pub fn action(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn action_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_action(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // keep_matching: optional bool
  pub fn keep_matching(&self) -> bool {
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
  pub fn set_keep_matching(&mut self, val: bool) {
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

  pub fn on_match(&self) -> super::super::matcher::on_match::OnMatchOneof<'_> {
    match &self.on_match_case() {
      super::super::matcher::on_match::OnMatchCase::Matcher =>
          super::super::matcher::on_match::OnMatchOneof::Matcher(self.matcher()),
      super::super::matcher::on_match::OnMatchCase::Action =>
          super::super::matcher::on_match::OnMatchOneof::Action(self.action()),
      _ => super::super::matcher::on_match::OnMatchOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn on_match_case(&self) -> super::super::matcher::on_match::OnMatchCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::matcher::on_match::OnMatchCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `OnMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OnMatchMut<'_> {}

// SAFETY:
// - `OnMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OnMatchMut<'_> {}

impl<'msg> ::protobuf::AsView for OnMatchMut<'msg> {
  type Proxied = OnMatch;
  fn as_view(&self) -> ::protobuf::View<'_, OnMatch> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OnMatchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OnMatch>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OnMatchMut<'msg> {
  type MutProxied = OnMatch;
  fn as_mut(&mut self) -> OnMatchMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OnMatchMut<'msg> {
  fn into_mut<'shorter>(self) -> OnMatchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OnMatch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OnMatch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OnMatchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OnMatchMut<'_> {
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
  pub fn matcher_opt(&self) -> ::std::option::Option<super::super::MatcherView<'_>> {
    self.has_matcher().then(|| self.matcher())
  }
  pub fn matcher(&self) -> super::super::MatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::MatcherView::default())
  }
  pub fn matcher_mut(&mut self) -> super::super::MatcherMut<'_> {
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
    val: impl ::protobuf::IntoProxied<super::super::Matcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // action: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_action(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_action(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn action_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_action().then(|| self.action())
  }
  pub fn action(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn action_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_action(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // keep_matching: optional bool
  pub fn keep_matching(&self) -> bool {
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
  pub fn set_keep_matching(&mut self, val: bool) {
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

  pub fn on_match(&self) -> super::super::matcher::on_match::OnMatchOneof<'_> {
    match &self.on_match_case() {
      super::super::matcher::on_match::OnMatchCase::Matcher =>
          super::super::matcher::on_match::OnMatchOneof::Matcher(self.matcher()),
      super::super::matcher::on_match::OnMatchCase::Action =>
          super::super::matcher::on_match::OnMatchOneof::Action(self.action()),
      _ => super::super::matcher::on_match::OnMatchOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn on_match_case(&self) -> super::super::matcher::on_match::OnMatchCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::matcher::on_match::OnMatchCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl OnMatch

impl ::std::ops::Drop for OnMatch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OnMatch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OnMatch {
  type Proxied = Self;
  fn as_view(&self) -> OnMatchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OnMatch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OnMatchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OnMatch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::matcher::envoy__config__common__matcher__v3__Matcher__OnMatch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OnMatch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OnMatch {
  type Msg = OnMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OnMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OnMatch {
  type Msg = OnMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OnMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OnMatchMut<'_> {
  type Msg = OnMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OnMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OnMatchMut<'_> {
  type Msg = OnMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OnMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OnMatchView<'_> {
  type Msg = OnMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OnMatch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OnMatchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod on_match {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum OnMatchOneof<'msg> {
  Matcher(::protobuf::View<'msg, super::super::super::Matcher>) = 1,
  Action(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum OnMatchCase {
  Matcher = 1,
  Action = 2,

  not_set = 0
}

impl OnMatchCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<OnMatchCase> {
    match v {
      0 => Some(OnMatchCase::not_set),
      1 => Some(OnMatchCase::Matcher),
      2 => Some(OnMatchCase::Action),
      _ => None
    }
  }
}
}  // pub mod on_match

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher__MatcherList_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MatcherList {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MatcherList>
}

impl ::protobuf::Message for MatcherList {
  type MessageView<'msg> = MatcherListView<'msg>;
  type MessageMut<'msg> = MatcherListMut<'msg>;
}

impl ::std::default::Default for MatcherList {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MatcherList {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MatcherList` is `Sync` because it does not implement interior mutability.
//    Neither does `MatcherListMut`.
unsafe impl ::std::marker::Sync for MatcherList {}

// SAFETY:
// - `MatcherList` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MatcherList {}

impl ::protobuf::Proxied for MatcherList {
  type View<'msg> = MatcherListView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MatcherList {}

impl ::protobuf::MutProxied for MatcherList {
  type Mut<'msg> = MatcherListMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MatcherListView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatcherList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatcherListView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MatcherListView<'msg> {
  type Message = MatcherList;
}

impl ::std::fmt::Debug for MatcherListView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MatcherListView<'_> {
  fn default() -> MatcherListView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MatcherList>> for MatcherListView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatcherList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatcherListView<'msg> {

  pub fn to_owned(&self) -> MatcherList {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // matchers: repeated message envoy.config.common.matcher.v3.Matcher.MatcherList.FieldMatcher
  pub fn matchers(self) -> ::protobuf::RepeatedView<'msg, super::super::matcher::matcher_list::FieldMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::matcher::matcher_list::FieldMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `MatcherListView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MatcherListView<'_> {}

// SAFETY:
// - `MatcherListView` is `Send` because while its alive a `MatcherListMut` cannot.
// - `MatcherListView` does not use thread-local data.
unsafe impl ::std::marker::Send for MatcherListView<'_> {}

impl<'msg> ::protobuf::AsView for MatcherListView<'msg> {
  type Proxied = MatcherList;
  fn as_view(&self) -> ::protobuf::View<'msg, MatcherList> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatcherListView<'msg> {
  fn into_view<'shorter>(self) -> MatcherListView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MatcherList> for MatcherListView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatcherList {
    let mut dst = MatcherList::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MatcherList> for MatcherListMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatcherList {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MatcherList {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatcherListView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatcherListMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MatcherListMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatcherList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatcherListMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MatcherListMut<'msg> {
  type Message = MatcherList;
}

impl ::std::fmt::Debug for MatcherListMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MatcherList>> for MatcherListMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatcherList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatcherListMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MatcherList> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MatcherList {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // matchers: repeated message envoy.config.common.matcher.v3.Matcher.MatcherList.FieldMatcher
  pub fn matchers(&self) -> ::protobuf::RepeatedView<'_, super::super::matcher::matcher_list::FieldMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::matcher::matcher_list::FieldMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn matchers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::matcher::matcher_list::FieldMatcher> {
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
  pub fn set_matchers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::matcher::matcher_list::FieldMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `MatcherListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MatcherListMut<'_> {}

// SAFETY:
// - `MatcherListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MatcherListMut<'_> {}

impl<'msg> ::protobuf::AsView for MatcherListMut<'msg> {
  type Proxied = MatcherList;
  fn as_view(&self) -> ::protobuf::View<'_, MatcherList> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatcherListMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MatcherList>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MatcherListMut<'msg> {
  type MutProxied = MatcherList;
  fn as_mut(&mut self) -> MatcherListMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MatcherListMut<'msg> {
  fn into_mut<'shorter>(self) -> MatcherListMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MatcherList {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MatcherList> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MatcherListView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MatcherListMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // matchers: repeated message envoy.config.common.matcher.v3.Matcher.MatcherList.FieldMatcher
  pub fn matchers(&self) -> ::protobuf::RepeatedView<'_, super::super::matcher::matcher_list::FieldMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::matcher::matcher_list::FieldMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn matchers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::matcher::matcher_list::FieldMatcher> {
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
  pub fn set_matchers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::matcher::matcher_list::FieldMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl MatcherList

impl ::std::ops::Drop for MatcherList {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MatcherList {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MatcherList {
  type Proxied = Self;
  fn as_view(&self) -> MatcherListView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MatcherList {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MatcherListMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MatcherList {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::matcher::envoy__config__common__matcher__v3__Matcher__MatcherList_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatcherList {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatcherList {
  type Msg = MatcherList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatcherList {
  type Msg = MatcherList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatcherListMut<'_> {
  type Msg = MatcherList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatcherListMut<'_> {
  type Msg = MatcherList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatcherListView<'_> {
  type Msg = MatcherList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherList> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatcherListMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod matcher_list {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Predicate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Predicate>
}

impl ::protobuf::Message for Predicate {
  type MessageView<'msg> = PredicateView<'msg>;
  type MessageMut<'msg> = PredicateMut<'msg>;
}

impl ::std::default::Default for Predicate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Predicate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Predicate` is `Sync` because it does not implement interior mutability.
//    Neither does `PredicateMut`.
unsafe impl ::std::marker::Sync for Predicate {}

// SAFETY:
// - `Predicate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Predicate {}

impl ::protobuf::Proxied for Predicate {
  type View<'msg> = PredicateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Predicate {}

impl ::protobuf::MutProxied for Predicate {
  type Mut<'msg> = PredicateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PredicateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Predicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PredicateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PredicateView<'msg> {
  type Message = Predicate;
}

impl ::std::fmt::Debug for PredicateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PredicateView<'_> {
  fn default() -> PredicateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Predicate>> for PredicateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Predicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PredicateView<'msg> {

  pub fn to_owned(&self) -> Predicate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // single_predicate: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate.SinglePredicate
  pub fn has_single_predicate(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn single_predicate_opt(self) -> ::std::option::Option<super::super::super::matcher::matcher_list::predicate::SinglePredicateView<'msg>> {
    self.has_single_predicate().then(|| self.single_predicate())
  }
  pub fn single_predicate(self) -> super::super::super::matcher::matcher_list::predicate::SinglePredicateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::predicate::SinglePredicateView::default())
  }

  // or_matcher: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
  pub fn has_or_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn or_matcher_opt(self) -> ::std::option::Option<super::super::super::matcher::matcher_list::predicate::PredicateListView<'msg>> {
    self.has_or_matcher().then(|| self.or_matcher())
  }
  pub fn or_matcher(self) -> super::super::super::matcher::matcher_list::predicate::PredicateListView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::predicate::PredicateListView::default())
  }

  // and_matcher: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
  pub fn has_and_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn and_matcher_opt(self) -> ::std::option::Option<super::super::super::matcher::matcher_list::predicate::PredicateListView<'msg>> {
    self.has_and_matcher().then(|| self.and_matcher())
  }
  pub fn and_matcher(self) -> super::super::super::matcher::matcher_list::predicate::PredicateListView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::predicate::PredicateListView::default())
  }

  // not_matcher: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate
  pub fn has_not_matcher(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn not_matcher_opt(self) -> ::std::option::Option<super::super::super::matcher::matcher_list::PredicateView<'msg>> {
    self.has_not_matcher().then(|| self.not_matcher())
  }
  pub fn not_matcher(self) -> super::super::super::matcher::matcher_list::PredicateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::PredicateView::default())
  }

  pub fn match_type(self) -> super::super::super::matcher::matcher_list::predicate::MatchTypeOneof<'msg> {
    match self.match_type_case() {
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::SinglePredicate =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::SinglePredicate(self.single_predicate()),
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::OrMatcher =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::OrMatcher(self.or_matcher()),
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::AndMatcher =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::AndMatcher(self.and_matcher()),
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::NotMatcher =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::NotMatcher(self.not_matcher()),
      _ => super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_type_case(self) -> super::super::super::matcher::matcher_list::predicate::MatchTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PredicateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PredicateView<'_> {}

// SAFETY:
// - `PredicateView` is `Send` because while its alive a `PredicateMut` cannot.
// - `PredicateView` does not use thread-local data.
unsafe impl ::std::marker::Send for PredicateView<'_> {}

impl<'msg> ::protobuf::AsView for PredicateView<'msg> {
  type Proxied = Predicate;
  fn as_view(&self) -> ::protobuf::View<'msg, Predicate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PredicateView<'msg> {
  fn into_view<'shorter>(self) -> PredicateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Predicate> for PredicateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Predicate {
    let mut dst = Predicate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Predicate> for PredicateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Predicate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Predicate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PredicateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PredicateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PredicateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Predicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PredicateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PredicateMut<'msg> {
  type Message = Predicate;
}

impl ::std::fmt::Debug for PredicateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Predicate>> for PredicateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Predicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PredicateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Predicate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Predicate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // single_predicate: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate.SinglePredicate
  pub fn has_single_predicate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_single_predicate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn single_predicate_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::predicate::SinglePredicateView<'_>> {
    self.has_single_predicate().then(|| self.single_predicate())
  }
  pub fn single_predicate(&self) -> super::super::super::matcher::matcher_list::predicate::SinglePredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::predicate::SinglePredicateView::default())
  }
  pub fn single_predicate_mut(&mut self) -> super::super::super::matcher::matcher_list::predicate::SinglePredicateMut<'_> {
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
  pub fn set_single_predicate(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::predicate::SinglePredicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // or_matcher: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
  pub fn has_or_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_or_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn or_matcher_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::predicate::PredicateListView<'_>> {
    self.has_or_matcher().then(|| self.or_matcher())
  }
  pub fn or_matcher(&self) -> super::super::super::matcher::matcher_list::predicate::PredicateListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::predicate::PredicateListView::default())
  }
  pub fn or_matcher_mut(&mut self) -> super::super::super::matcher::matcher_list::predicate::PredicateListMut<'_> {
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
  pub fn set_or_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::predicate::PredicateList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // and_matcher: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
  pub fn has_and_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_and_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn and_matcher_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::predicate::PredicateListView<'_>> {
    self.has_and_matcher().then(|| self.and_matcher())
  }
  pub fn and_matcher(&self) -> super::super::super::matcher::matcher_list::predicate::PredicateListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::predicate::PredicateListView::default())
  }
  pub fn and_matcher_mut(&mut self) -> super::super::super::matcher::matcher_list::predicate::PredicateListMut<'_> {
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
  pub fn set_and_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::predicate::PredicateList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // not_matcher: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate
  pub fn has_not_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_not_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn not_matcher_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::PredicateView<'_>> {
    self.has_not_matcher().then(|| self.not_matcher())
  }
  pub fn not_matcher(&self) -> super::super::super::matcher::matcher_list::PredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::PredicateView::default())
  }
  pub fn not_matcher_mut(&mut self) -> super::super::super::matcher::matcher_list::PredicateMut<'_> {
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
  pub fn set_not_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::Predicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn match_type(&self) -> super::super::super::matcher::matcher_list::predicate::MatchTypeOneof<'_> {
    match &self.match_type_case() {
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::SinglePredicate =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::SinglePredicate(self.single_predicate()),
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::OrMatcher =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::OrMatcher(self.or_matcher()),
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::AndMatcher =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::AndMatcher(self.and_matcher()),
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::NotMatcher =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::NotMatcher(self.not_matcher()),
      _ => super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_type_case(&self) -> super::super::super::matcher::matcher_list::predicate::MatchTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `PredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PredicateMut<'_> {}

// SAFETY:
// - `PredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PredicateMut<'_> {}

impl<'msg> ::protobuf::AsView for PredicateMut<'msg> {
  type Proxied = Predicate;
  fn as_view(&self) -> ::protobuf::View<'_, Predicate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PredicateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Predicate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PredicateMut<'msg> {
  type MutProxied = Predicate;
  fn as_mut(&mut self) -> PredicateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PredicateMut<'msg> {
  fn into_mut<'shorter>(self) -> PredicateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Predicate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Predicate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PredicateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PredicateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // single_predicate: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate.SinglePredicate
  pub fn has_single_predicate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_single_predicate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn single_predicate_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::predicate::SinglePredicateView<'_>> {
    self.has_single_predicate().then(|| self.single_predicate())
  }
  pub fn single_predicate(&self) -> super::super::super::matcher::matcher_list::predicate::SinglePredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::predicate::SinglePredicateView::default())
  }
  pub fn single_predicate_mut(&mut self) -> super::super::super::matcher::matcher_list::predicate::SinglePredicateMut<'_> {
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
  pub fn set_single_predicate(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::predicate::SinglePredicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // or_matcher: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
  pub fn has_or_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_or_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn or_matcher_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::predicate::PredicateListView<'_>> {
    self.has_or_matcher().then(|| self.or_matcher())
  }
  pub fn or_matcher(&self) -> super::super::super::matcher::matcher_list::predicate::PredicateListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::predicate::PredicateListView::default())
  }
  pub fn or_matcher_mut(&mut self) -> super::super::super::matcher::matcher_list::predicate::PredicateListMut<'_> {
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
  pub fn set_or_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::predicate::PredicateList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // and_matcher: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
  pub fn has_and_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_and_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn and_matcher_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::predicate::PredicateListView<'_>> {
    self.has_and_matcher().then(|| self.and_matcher())
  }
  pub fn and_matcher(&self) -> super::super::super::matcher::matcher_list::predicate::PredicateListView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::predicate::PredicateListView::default())
  }
  pub fn and_matcher_mut(&mut self) -> super::super::super::matcher::matcher_list::predicate::PredicateListMut<'_> {
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
  pub fn set_and_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::predicate::PredicateList>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // not_matcher: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate
  pub fn has_not_matcher(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_not_matcher(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn not_matcher_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::PredicateView<'_>> {
    self.has_not_matcher().then(|| self.not_matcher())
  }
  pub fn not_matcher(&self) -> super::super::super::matcher::matcher_list::PredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::PredicateView::default())
  }
  pub fn not_matcher_mut(&mut self) -> super::super::super::matcher::matcher_list::PredicateMut<'_> {
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
  pub fn set_not_matcher(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::Predicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn match_type(&self) -> super::super::super::matcher::matcher_list::predicate::MatchTypeOneof<'_> {
    match &self.match_type_case() {
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::SinglePredicate =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::SinglePredicate(self.single_predicate()),
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::OrMatcher =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::OrMatcher(self.or_matcher()),
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::AndMatcher =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::AndMatcher(self.and_matcher()),
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::NotMatcher =>
          super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::NotMatcher(self.not_matcher()),
      _ => super::super::super::matcher::matcher_list::predicate::MatchTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn match_type_case(&self) -> super::super::super::matcher::matcher_list::predicate::MatchTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::matcher::matcher_list::predicate::MatchTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Predicate

impl ::std::ops::Drop for Predicate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Predicate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Predicate {
  type Proxied = Self;
  fn as_view(&self) -> PredicateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Predicate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PredicateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Predicate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::matcher::matcher_list::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333^!|#|$|%");
        super::super::super::matcher::matcher_list::predicate::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::matcher::matcher_list::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0, &[<super::super::super::matcher::matcher_list::predicate::SinglePredicate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::super::super::matcher::matcher_list::predicate::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0,
            super::super::super::matcher::matcher_list::predicate::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0,
            super::super::super::matcher::matcher_list::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::matcher::matcher_list::predicate::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0, &[super::super::super::matcher::matcher_list::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::matcher::matcher_list::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Predicate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Predicate {
  type Msg = Predicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Predicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Predicate {
  type Msg = Predicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Predicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PredicateMut<'_> {
  type Msg = Predicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Predicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PredicateMut<'_> {
  type Msg = Predicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Predicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PredicateView<'_> {
  type Msg = Predicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Predicate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PredicateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod predicate {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__SinglePredicate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SinglePredicate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SinglePredicate>
}

impl ::protobuf::Message for SinglePredicate {
  type MessageView<'msg> = SinglePredicateView<'msg>;
  type MessageMut<'msg> = SinglePredicateMut<'msg>;
}

impl ::std::default::Default for SinglePredicate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SinglePredicate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SinglePredicate` is `Sync` because it does not implement interior mutability.
//    Neither does `SinglePredicateMut`.
unsafe impl ::std::marker::Sync for SinglePredicate {}

// SAFETY:
// - `SinglePredicate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SinglePredicate {}

impl ::protobuf::Proxied for SinglePredicate {
  type View<'msg> = SinglePredicateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SinglePredicate {}

impl ::protobuf::MutProxied for SinglePredicate {
  type Mut<'msg> = SinglePredicateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SinglePredicateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SinglePredicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SinglePredicateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SinglePredicateView<'msg> {
  type Message = SinglePredicate;
}

impl ::std::fmt::Debug for SinglePredicateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SinglePredicateView<'_> {
  fn default() -> SinglePredicateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SinglePredicate>> for SinglePredicateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SinglePredicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SinglePredicateView<'msg> {

  pub fn to_owned(&self) -> SinglePredicate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // input: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_input(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn input_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // value_match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_value_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_match_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_value_match().then(|| self.value_match())
  }
  pub fn value_match(self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }

  // custom_match: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn custom_match_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  pub fn matcher(self) -> super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof<'msg> {
    match self.matcher_case() {
      super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase::ValueMatch =>
          super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof::ValueMatch(self.value_match()),
      super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase::CustomMatch =>
          super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof::CustomMatch(self.custom_match()),
      _ => super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn matcher_case(self) -> super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SinglePredicateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SinglePredicateView<'_> {}

// SAFETY:
// - `SinglePredicateView` is `Send` because while its alive a `SinglePredicateMut` cannot.
// - `SinglePredicateView` does not use thread-local data.
unsafe impl ::std::marker::Send for SinglePredicateView<'_> {}

impl<'msg> ::protobuf::AsView for SinglePredicateView<'msg> {
  type Proxied = SinglePredicate;
  fn as_view(&self) -> ::protobuf::View<'msg, SinglePredicate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SinglePredicateView<'msg> {
  fn into_view<'shorter>(self) -> SinglePredicateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SinglePredicate> for SinglePredicateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SinglePredicate {
    let mut dst = SinglePredicate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SinglePredicate> for SinglePredicateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SinglePredicate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SinglePredicate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SinglePredicateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SinglePredicateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SinglePredicateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SinglePredicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SinglePredicateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SinglePredicateMut<'msg> {
  type Message = SinglePredicate;
}

impl ::std::fmt::Debug for SinglePredicateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SinglePredicate>> for SinglePredicateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SinglePredicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SinglePredicateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SinglePredicate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SinglePredicate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // input: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_input(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_input(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn input_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn input_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_input(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value_match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_value_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_value_match().then(|| self.value_match())
  }
  pub fn value_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn value_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_value_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // custom_match: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_custom_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn custom_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_match_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn matcher(&self) -> super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof<'_> {
    match &self.matcher_case() {
      super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase::ValueMatch =>
          super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof::ValueMatch(self.value_match()),
      super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase::CustomMatch =>
          super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof::CustomMatch(self.custom_match()),
      _ => super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn matcher_case(&self) -> super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `SinglePredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SinglePredicateMut<'_> {}

// SAFETY:
// - `SinglePredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SinglePredicateMut<'_> {}

impl<'msg> ::protobuf::AsView for SinglePredicateMut<'msg> {
  type Proxied = SinglePredicate;
  fn as_view(&self) -> ::protobuf::View<'_, SinglePredicate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SinglePredicateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SinglePredicate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SinglePredicateMut<'msg> {
  type MutProxied = SinglePredicate;
  fn as_mut(&mut self) -> SinglePredicateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SinglePredicateMut<'msg> {
  fn into_mut<'shorter>(self) -> SinglePredicateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SinglePredicate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SinglePredicate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SinglePredicateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SinglePredicateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // input: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_input(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_input(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn input_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn input_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_input(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value_match: optional message envoy.type.matcher.v3.StringMatcher
  pub fn has_value_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_value_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn value_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_value_match().then(|| self.value_match())
  }
  pub fn value_match(&self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn value_match_mut(&mut self) -> crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
  pub fn set_value_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // custom_match: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_custom_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn custom_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_match_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  pub fn matcher(&self) -> super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof<'_> {
    match &self.matcher_case() {
      super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase::ValueMatch =>
          super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof::ValueMatch(self.value_match()),
      super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase::CustomMatch =>
          super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof::CustomMatch(self.custom_match()),
      _ => super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn matcher_case(&self) -> super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::super::super::matcher::matcher_list::predicate::single_predicate::MatcherCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl SinglePredicate

impl ::std::ops::Drop for SinglePredicate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SinglePredicate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SinglePredicate {
  type Proxied = Self;
  fn as_view(&self) -> SinglePredicateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SinglePredicate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SinglePredicateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SinglePredicate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::super::matcher::matcher_list::predicate::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__SinglePredicate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::matcher::matcher_list::predicate::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__SinglePredicate_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::matcher::matcher_list::predicate::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__SinglePredicate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SinglePredicate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SinglePredicate {
  type Msg = SinglePredicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SinglePredicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SinglePredicate {
  type Msg = SinglePredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SinglePredicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SinglePredicateMut<'_> {
  type Msg = SinglePredicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SinglePredicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SinglePredicateMut<'_> {
  type Msg = SinglePredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SinglePredicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SinglePredicateView<'_> {
  type Msg = SinglePredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SinglePredicate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SinglePredicateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod single_predicate {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MatcherOneof<'msg> {
  ValueMatch(::protobuf::View<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>) = 2,
  CustomMatch(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 3,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MatcherCase {
  ValueMatch = 2,
  CustomMatch = 3,

  not_set = 0
}

impl MatcherCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MatcherCase> {
    match v {
      0 => Some(MatcherCase::not_set),
      2 => Some(MatcherCase::ValueMatch),
      3 => Some(MatcherCase::CustomMatch),
      _ => None
    }
  }
}
}  // pub mod single_predicate

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct PredicateList {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PredicateList>
}

impl ::protobuf::Message for PredicateList {
  type MessageView<'msg> = PredicateListView<'msg>;
  type MessageMut<'msg> = PredicateListMut<'msg>;
}

impl ::std::default::Default for PredicateList {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for PredicateList {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `PredicateList` is `Sync` because it does not implement interior mutability.
//    Neither does `PredicateListMut`.
unsafe impl ::std::marker::Sync for PredicateList {}

// SAFETY:
// - `PredicateList` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for PredicateList {}

impl ::protobuf::Proxied for PredicateList {
  type View<'msg> = PredicateListView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PredicateList {}

impl ::protobuf::MutProxied for PredicateList {
  type Mut<'msg> = PredicateListMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PredicateListView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PredicateList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PredicateListView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PredicateListView<'msg> {
  type Message = PredicateList;
}

impl ::std::fmt::Debug for PredicateListView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PredicateListView<'_> {
  fn default() -> PredicateListView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, PredicateList>> for PredicateListView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PredicateList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PredicateListView<'msg> {

  pub fn to_owned(&self) -> PredicateList {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // predicate: repeated message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate
  pub fn predicate(self) -> ::protobuf::RepeatedView<'msg, super::super::super::super::matcher::matcher_list::Predicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::super::super::matcher::matcher_list::Predicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PredicateListView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PredicateListView<'_> {}

// SAFETY:
// - `PredicateListView` is `Send` because while its alive a `PredicateListMut` cannot.
// - `PredicateListView` does not use thread-local data.
unsafe impl ::std::marker::Send for PredicateListView<'_> {}

impl<'msg> ::protobuf::AsView for PredicateListView<'msg> {
  type Proxied = PredicateList;
  fn as_view(&self) -> ::protobuf::View<'msg, PredicateList> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PredicateListView<'msg> {
  fn into_view<'shorter>(self) -> PredicateListView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PredicateList> for PredicateListView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PredicateList {
    let mut dst = PredicateList::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PredicateList> for PredicateListMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PredicateList {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for PredicateList {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PredicateListView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PredicateListMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PredicateListMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PredicateList>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PredicateListMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PredicateListMut<'msg> {
  type Message = PredicateList;
}

impl ::std::fmt::Debug for PredicateListMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, PredicateList>> for PredicateListMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PredicateList>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PredicateListMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PredicateList> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> PredicateList {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // predicate: repeated message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate
  pub fn predicate(&self) -> ::protobuf::RepeatedView<'_, super::super::super::super::matcher::matcher_list::Predicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::super::super::matcher::matcher_list::Predicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn predicate_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::super::super::matcher::matcher_list::Predicate> {
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
  pub fn set_predicate(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::super::super::matcher::matcher_list::Predicate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `PredicateListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PredicateListMut<'_> {}

// SAFETY:
// - `PredicateListMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PredicateListMut<'_> {}

impl<'msg> ::protobuf::AsView for PredicateListMut<'msg> {
  type Proxied = PredicateList;
  fn as_view(&self) -> ::protobuf::View<'_, PredicateList> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PredicateListMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PredicateList>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PredicateListMut<'msg> {
  type MutProxied = PredicateList;
  fn as_mut(&mut self) -> PredicateListMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PredicateListMut<'msg> {
  fn into_mut<'shorter>(self) -> PredicateListMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PredicateList {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PredicateList> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PredicateListView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PredicateListMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // predicate: repeated message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate
  pub fn predicate(&self) -> ::protobuf::RepeatedView<'_, super::super::super::super::matcher::matcher_list::Predicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::super::super::matcher::matcher_list::Predicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn predicate_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::super::super::matcher::matcher_list::Predicate> {
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
  pub fn set_predicate(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::super::super::matcher::matcher_list::Predicate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl PredicateList

impl ::std::ops::Drop for PredicateList {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PredicateList {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PredicateList {
  type Proxied = Self;
  fn as_view(&self) -> PredicateListView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PredicateList {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PredicateListMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PredicateList {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::super::super::matcher::matcher_list::Predicate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::matcher::matcher_list::predicate::envoy__config__common__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PredicateList {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PredicateList {
  type Msg = PredicateList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PredicateList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PredicateList {
  type Msg = PredicateList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PredicateList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PredicateListMut<'_> {
  type Msg = PredicateList;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PredicateList> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PredicateListMut<'_> {
  type Msg = PredicateList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PredicateList> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PredicateListView<'_> {
  type Msg = PredicateList;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PredicateList> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PredicateListMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MatchTypeOneof<'msg> {
  SinglePredicate(::protobuf::View<'msg, super::super::super::super::matcher::matcher_list::predicate::SinglePredicate>) = 1,
  OrMatcher(::protobuf::View<'msg, super::super::super::super::matcher::matcher_list::predicate::PredicateList>) = 2,
  AndMatcher(::protobuf::View<'msg, super::super::super::super::matcher::matcher_list::predicate::PredicateList>) = 3,
  NotMatcher(::protobuf::View<'msg, super::super::super::super::matcher::matcher_list::Predicate>) = 4,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MatchTypeCase {
  SinglePredicate = 1,
  OrMatcher = 2,
  AndMatcher = 3,
  NotMatcher = 4,

  not_set = 0
}

impl MatchTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MatchTypeCase> {
    match v {
      0 => Some(MatchTypeCase::not_set),
      1 => Some(MatchTypeCase::SinglePredicate),
      2 => Some(MatchTypeCase::OrMatcher),
      3 => Some(MatchTypeCase::AndMatcher),
      4 => Some(MatchTypeCase::NotMatcher),
      _ => None
    }
  }
}
}  // pub mod predicate

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FieldMatcher {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FieldMatcher>
}

impl ::protobuf::Message for FieldMatcher {
  type MessageView<'msg> = FieldMatcherView<'msg>;
  type MessageMut<'msg> = FieldMatcherMut<'msg>;
}

impl ::std::default::Default for FieldMatcher {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FieldMatcher {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FieldMatcher` is `Sync` because it does not implement interior mutability.
//    Neither does `FieldMatcherMut`.
unsafe impl ::std::marker::Sync for FieldMatcher {}

// SAFETY:
// - `FieldMatcher` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FieldMatcher {}

impl ::protobuf::Proxied for FieldMatcher {
  type View<'msg> = FieldMatcherView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FieldMatcher {}

impl ::protobuf::MutProxied for FieldMatcher {
  type Mut<'msg> = FieldMatcherMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FieldMatcherView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldMatcherView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FieldMatcherView<'msg> {
  type Message = FieldMatcher;
}

impl ::std::fmt::Debug for FieldMatcherView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FieldMatcherView<'_> {
  fn default() -> FieldMatcherView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FieldMatcher>> for FieldMatcherView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FieldMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldMatcherView<'msg> {

  pub fn to_owned(&self) -> FieldMatcher {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // predicate: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate
  pub fn has_predicate(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn predicate_opt(self) -> ::std::option::Option<super::super::super::matcher::matcher_list::PredicateView<'msg>> {
    self.has_predicate().then(|| self.predicate())
  }
  pub fn predicate(self) -> super::super::super::matcher::matcher_list::PredicateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::PredicateView::default())
  }

  // on_match: optional message envoy.config.common.matcher.v3.Matcher.OnMatch
  pub fn has_on_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn on_match_opt(self) -> ::std::option::Option<super::super::super::matcher::OnMatchView<'msg>> {
    self.has_on_match().then(|| self.on_match())
  }
  pub fn on_match(self) -> super::super::super::matcher::OnMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::OnMatchView::default())
  }

}

// SAFETY:
// - `FieldMatcherView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FieldMatcherView<'_> {}

// SAFETY:
// - `FieldMatcherView` is `Send` because while its alive a `FieldMatcherMut` cannot.
// - `FieldMatcherView` does not use thread-local data.
unsafe impl ::std::marker::Send for FieldMatcherView<'_> {}

impl<'msg> ::protobuf::AsView for FieldMatcherView<'msg> {
  type Proxied = FieldMatcher;
  fn as_view(&self) -> ::protobuf::View<'msg, FieldMatcher> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldMatcherView<'msg> {
  fn into_view<'shorter>(self) -> FieldMatcherView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldMatcher> for FieldMatcherView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldMatcher {
    let mut dst = FieldMatcher::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FieldMatcher> for FieldMatcherMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FieldMatcher {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FieldMatcher {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FieldMatcherView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FieldMatcherMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FieldMatcherMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldMatcher>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FieldMatcherMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FieldMatcherMut<'msg> {
  type Message = FieldMatcher;
}

impl ::std::fmt::Debug for FieldMatcherMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FieldMatcher>> for FieldMatcherMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldMatcher>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FieldMatcherMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FieldMatcher> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FieldMatcher {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // predicate: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate
  pub fn has_predicate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_predicate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn predicate_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::PredicateView<'_>> {
    self.has_predicate().then(|| self.predicate())
  }
  pub fn predicate(&self) -> super::super::super::matcher::matcher_list::PredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::PredicateView::default())
  }
  pub fn predicate_mut(&mut self) -> super::super::super::matcher::matcher_list::PredicateMut<'_> {
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
  pub fn set_predicate(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::Predicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // on_match: optional message envoy.config.common.matcher.v3.Matcher.OnMatch
  pub fn has_on_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_on_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn on_match_opt(&self) -> ::std::option::Option<super::super::super::matcher::OnMatchView<'_>> {
    self.has_on_match().then(|| self.on_match())
  }
  pub fn on_match(&self) -> super::super::super::matcher::OnMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::OnMatchView::default())
  }
  pub fn on_match_mut(&mut self) -> super::super::super::matcher::OnMatchMut<'_> {
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
  pub fn set_on_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::OnMatch>) {

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
// - `FieldMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FieldMatcherMut<'_> {}

// SAFETY:
// - `FieldMatcherMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FieldMatcherMut<'_> {}

impl<'msg> ::protobuf::AsView for FieldMatcherMut<'msg> {
  type Proxied = FieldMatcher;
  fn as_view(&self) -> ::protobuf::View<'_, FieldMatcher> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FieldMatcherMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FieldMatcher>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FieldMatcherMut<'msg> {
  type MutProxied = FieldMatcher;
  fn as_mut(&mut self) -> FieldMatcherMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FieldMatcherMut<'msg> {
  fn into_mut<'shorter>(self) -> FieldMatcherMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FieldMatcher {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FieldMatcher> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FieldMatcherView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FieldMatcherMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // predicate: optional message envoy.config.common.matcher.v3.Matcher.MatcherList.Predicate
  pub fn has_predicate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_predicate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn predicate_opt(&self) -> ::std::option::Option<super::super::super::matcher::matcher_list::PredicateView<'_>> {
    self.has_predicate().then(|| self.predicate())
  }
  pub fn predicate(&self) -> super::super::super::matcher::matcher_list::PredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::matcher_list::PredicateView::default())
  }
  pub fn predicate_mut(&mut self) -> super::super::super::matcher::matcher_list::PredicateMut<'_> {
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
  pub fn set_predicate(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::matcher_list::Predicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // on_match: optional message envoy.config.common.matcher.v3.Matcher.OnMatch
  pub fn has_on_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_on_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn on_match_opt(&self) -> ::std::option::Option<super::super::super::matcher::OnMatchView<'_>> {
    self.has_on_match().then(|| self.on_match())
  }
  pub fn on_match(&self) -> super::super::super::matcher::OnMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::super::matcher::OnMatchView::default())
  }
  pub fn on_match_mut(&mut self) -> super::super::super::matcher::OnMatchMut<'_> {
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
  pub fn set_on_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::super::matcher::OnMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl FieldMatcher

impl ::std::ops::Drop for FieldMatcher {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FieldMatcher {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FieldMatcher {
  type Proxied = Self;
  fn as_view(&self) -> FieldMatcherView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FieldMatcher {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FieldMatcherMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FieldMatcher {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::super::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::matcher::matcher_list::envoy__config__common__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldMatcher {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldMatcher {
  type Msg = FieldMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldMatcher {
  type Msg = FieldMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FieldMatcherMut<'_> {
  type Msg = FieldMatcher;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMatcher> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldMatcherMut<'_> {
  type Msg = FieldMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMatcher> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FieldMatcherView<'_> {
  type Msg = FieldMatcher;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FieldMatcher> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FieldMatcherMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod matcher_list

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher__MatcherTree_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MatcherTree {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MatcherTree>
}

impl ::protobuf::Message for MatcherTree {
  type MessageView<'msg> = MatcherTreeView<'msg>;
  type MessageMut<'msg> = MatcherTreeMut<'msg>;
}

impl ::std::default::Default for MatcherTree {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MatcherTree {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MatcherTree` is `Sync` because it does not implement interior mutability.
//    Neither does `MatcherTreeMut`.
unsafe impl ::std::marker::Sync for MatcherTree {}

// SAFETY:
// - `MatcherTree` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MatcherTree {}

impl ::protobuf::Proxied for MatcherTree {
  type View<'msg> = MatcherTreeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MatcherTree {}

impl ::protobuf::MutProxied for MatcherTree {
  type Mut<'msg> = MatcherTreeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MatcherTreeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatcherTree>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatcherTreeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MatcherTreeView<'msg> {
  type Message = MatcherTree;
}

impl ::std::fmt::Debug for MatcherTreeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MatcherTreeView<'_> {
  fn default() -> MatcherTreeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MatcherTree>> for MatcherTreeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatcherTree>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatcherTreeView<'msg> {

  pub fn to_owned(&self) -> MatcherTree {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // input: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_input(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn input_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // exact_match_map: optional message envoy.config.common.matcher.v3.Matcher.MatcherTree.MatchMap
  pub fn has_exact_match_map(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn exact_match_map_opt(self) -> ::std::option::Option<super::super::matcher::matcher_tree::MatchMapView<'msg>> {
    self.has_exact_match_map().then(|| self.exact_match_map())
  }
  pub fn exact_match_map(self) -> super::super::matcher::matcher_tree::MatchMapView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::matcher::matcher_tree::MatchMapView::default())
  }

  // prefix_match_map: optional message envoy.config.common.matcher.v3.Matcher.MatcherTree.MatchMap
  pub fn has_prefix_match_map(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn prefix_match_map_opt(self) -> ::std::option::Option<super::super::matcher::matcher_tree::MatchMapView<'msg>> {
    self.has_prefix_match_map().then(|| self.prefix_match_map())
  }
  pub fn prefix_match_map(self) -> super::super::matcher::matcher_tree::MatchMapView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::matcher::matcher_tree::MatchMapView::default())
  }

  // custom_match: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn custom_match_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  pub fn tree_type(self) -> super::super::matcher::matcher_tree::TreeTypeOneof<'msg> {
    match self.tree_type_case() {
      super::super::matcher::matcher_tree::TreeTypeCase::ExactMatchMap =>
          super::super::matcher::matcher_tree::TreeTypeOneof::ExactMatchMap(self.exact_match_map()),
      super::super::matcher::matcher_tree::TreeTypeCase::PrefixMatchMap =>
          super::super::matcher::matcher_tree::TreeTypeOneof::PrefixMatchMap(self.prefix_match_map()),
      super::super::matcher::matcher_tree::TreeTypeCase::CustomMatch =>
          super::super::matcher::matcher_tree::TreeTypeOneof::CustomMatch(self.custom_match()),
      _ => super::super::matcher::matcher_tree::TreeTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn tree_type_case(self) -> super::super::matcher::matcher_tree::TreeTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::matcher::matcher_tree::TreeTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MatcherTreeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MatcherTreeView<'_> {}

// SAFETY:
// - `MatcherTreeView` is `Send` because while its alive a `MatcherTreeMut` cannot.
// - `MatcherTreeView` does not use thread-local data.
unsafe impl ::std::marker::Send for MatcherTreeView<'_> {}

impl<'msg> ::protobuf::AsView for MatcherTreeView<'msg> {
  type Proxied = MatcherTree;
  fn as_view(&self) -> ::protobuf::View<'msg, MatcherTree> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatcherTreeView<'msg> {
  fn into_view<'shorter>(self) -> MatcherTreeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MatcherTree> for MatcherTreeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatcherTree {
    let mut dst = MatcherTree::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MatcherTree> for MatcherTreeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatcherTree {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MatcherTree {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatcherTreeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatcherTreeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MatcherTreeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatcherTree>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatcherTreeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MatcherTreeMut<'msg> {
  type Message = MatcherTree;
}

impl ::std::fmt::Debug for MatcherTreeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MatcherTree>> for MatcherTreeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatcherTree>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatcherTreeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MatcherTree> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MatcherTree {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // input: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_input(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_input(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn input_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn input_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_input(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // exact_match_map: optional message envoy.config.common.matcher.v3.Matcher.MatcherTree.MatchMap
  pub fn has_exact_match_map(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_exact_match_map(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn exact_match_map_opt(&self) -> ::std::option::Option<super::super::matcher::matcher_tree::MatchMapView<'_>> {
    self.has_exact_match_map().then(|| self.exact_match_map())
  }
  pub fn exact_match_map(&self) -> super::super::matcher::matcher_tree::MatchMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::matcher::matcher_tree::MatchMapView::default())
  }
  pub fn exact_match_map_mut(&mut self) -> super::super::matcher::matcher_tree::MatchMapMut<'_> {
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
  pub fn set_exact_match_map(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::matcher::matcher_tree::MatchMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // prefix_match_map: optional message envoy.config.common.matcher.v3.Matcher.MatcherTree.MatchMap
  pub fn has_prefix_match_map(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_prefix_match_map(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn prefix_match_map_opt(&self) -> ::std::option::Option<super::super::matcher::matcher_tree::MatchMapView<'_>> {
    self.has_prefix_match_map().then(|| self.prefix_match_map())
  }
  pub fn prefix_match_map(&self) -> super::super::matcher::matcher_tree::MatchMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::matcher::matcher_tree::MatchMapView::default())
  }
  pub fn prefix_match_map_mut(&mut self) -> super::super::matcher::matcher_tree::MatchMapMut<'_> {
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
  pub fn set_prefix_match_map(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::matcher::matcher_tree::MatchMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // custom_match: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_custom_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn custom_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_match_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn tree_type(&self) -> super::super::matcher::matcher_tree::TreeTypeOneof<'_> {
    match &self.tree_type_case() {
      super::super::matcher::matcher_tree::TreeTypeCase::ExactMatchMap =>
          super::super::matcher::matcher_tree::TreeTypeOneof::ExactMatchMap(self.exact_match_map()),
      super::super::matcher::matcher_tree::TreeTypeCase::PrefixMatchMap =>
          super::super::matcher::matcher_tree::TreeTypeOneof::PrefixMatchMap(self.prefix_match_map()),
      super::super::matcher::matcher_tree::TreeTypeCase::CustomMatch =>
          super::super::matcher::matcher_tree::TreeTypeOneof::CustomMatch(self.custom_match()),
      _ => super::super::matcher::matcher_tree::TreeTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn tree_type_case(&self) -> super::super::matcher::matcher_tree::TreeTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::matcher::matcher_tree::TreeTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MatcherTreeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MatcherTreeMut<'_> {}

// SAFETY:
// - `MatcherTreeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MatcherTreeMut<'_> {}

impl<'msg> ::protobuf::AsView for MatcherTreeMut<'msg> {
  type Proxied = MatcherTree;
  fn as_view(&self) -> ::protobuf::View<'_, MatcherTree> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatcherTreeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MatcherTree>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MatcherTreeMut<'msg> {
  type MutProxied = MatcherTree;
  fn as_mut(&mut self) -> MatcherTreeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MatcherTreeMut<'msg> {
  fn into_mut<'shorter>(self) -> MatcherTreeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MatcherTree {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MatcherTree> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MatcherTreeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MatcherTreeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // input: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_input(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_input(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn input_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn input_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_input(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // exact_match_map: optional message envoy.config.common.matcher.v3.Matcher.MatcherTree.MatchMap
  pub fn has_exact_match_map(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_exact_match_map(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn exact_match_map_opt(&self) -> ::std::option::Option<super::super::matcher::matcher_tree::MatchMapView<'_>> {
    self.has_exact_match_map().then(|| self.exact_match_map())
  }
  pub fn exact_match_map(&self) -> super::super::matcher::matcher_tree::MatchMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::matcher::matcher_tree::MatchMapView::default())
  }
  pub fn exact_match_map_mut(&mut self) -> super::super::matcher::matcher_tree::MatchMapMut<'_> {
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
  pub fn set_exact_match_map(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::matcher::matcher_tree::MatchMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // prefix_match_map: optional message envoy.config.common.matcher.v3.Matcher.MatcherTree.MatchMap
  pub fn has_prefix_match_map(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_prefix_match_map(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn prefix_match_map_opt(&self) -> ::std::option::Option<super::super::matcher::matcher_tree::MatchMapView<'_>> {
    self.has_prefix_match_map().then(|| self.prefix_match_map())
  }
  pub fn prefix_match_map(&self) -> super::super::matcher::matcher_tree::MatchMapView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::matcher::matcher_tree::MatchMapView::default())
  }
  pub fn prefix_match_map_mut(&mut self) -> super::super::matcher::matcher_tree::MatchMapMut<'_> {
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
  pub fn set_prefix_match_map(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::matcher::matcher_tree::MatchMap>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // custom_match: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_custom_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_custom_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn custom_match_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_match_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
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
  pub fn set_custom_match(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  pub fn tree_type(&self) -> super::super::matcher::matcher_tree::TreeTypeOneof<'_> {
    match &self.tree_type_case() {
      super::super::matcher::matcher_tree::TreeTypeCase::ExactMatchMap =>
          super::super::matcher::matcher_tree::TreeTypeOneof::ExactMatchMap(self.exact_match_map()),
      super::super::matcher::matcher_tree::TreeTypeCase::PrefixMatchMap =>
          super::super::matcher::matcher_tree::TreeTypeOneof::PrefixMatchMap(self.prefix_match_map()),
      super::super::matcher::matcher_tree::TreeTypeCase::CustomMatch =>
          super::super::matcher::matcher_tree::TreeTypeOneof::CustomMatch(self.custom_match()),
      _ => super::super::matcher::matcher_tree::TreeTypeOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn tree_type_case(&self) -> super::super::matcher::matcher_tree::TreeTypeCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::super::matcher::matcher_tree::TreeTypeCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl MatcherTree

impl ::std::ops::Drop for MatcherTree {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MatcherTree {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MatcherTree {
  type Proxied = Self;
  fn as_view(&self) -> MatcherTreeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MatcherTree {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MatcherTreeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MatcherTree {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::matcher::envoy__config__common__matcher__v3__Matcher__MatcherTree_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatcherTree {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatcherTree {
  type Msg = MatcherTree;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherTree> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatcherTree {
  type Msg = MatcherTree;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherTree> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatcherTreeMut<'_> {
  type Msg = MatcherTree;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherTree> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatcherTreeMut<'_> {
  type Msg = MatcherTree;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherTree> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatcherTreeView<'_> {
  type Msg = MatcherTree;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatcherTree> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatcherTreeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod matcher_tree {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MatchMap {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MatchMap>
}

impl ::protobuf::Message for MatchMap {
  type MessageView<'msg> = MatchMapView<'msg>;
  type MessageMut<'msg> = MatchMapMut<'msg>;
}

impl ::std::default::Default for MatchMap {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MatchMap {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MatchMap` is `Sync` because it does not implement interior mutability.
//    Neither does `MatchMapMut`.
unsafe impl ::std::marker::Sync for MatchMap {}

// SAFETY:
// - `MatchMap` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MatchMap {}

impl ::protobuf::Proxied for MatchMap {
  type View<'msg> = MatchMapView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MatchMap {}

impl ::protobuf::MutProxied for MatchMap {
  type Mut<'msg> = MatchMapMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MatchMapView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatchMap>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatchMapView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MatchMapView<'msg> {
  type Message = MatchMap;
}

impl ::std::fmt::Debug for MatchMapView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MatchMapView<'_> {
  fn default() -> MatchMapView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MatchMap>> for MatchMapView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatchMap>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatchMapView<'msg> {

  pub fn to_owned(&self) -> MatchMap {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // map: repeated message envoy.config.common.matcher.v3.Matcher.MatcherTree.MatchMap.MapEntry
  pub fn map(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, super::super::super::matcher::OnMatch> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::super::super::matcher::OnMatch>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

}

// SAFETY:
// - `MatchMapView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MatchMapView<'_> {}

// SAFETY:
// - `MatchMapView` is `Send` because while its alive a `MatchMapMut` cannot.
// - `MatchMapView` does not use thread-local data.
unsafe impl ::std::marker::Send for MatchMapView<'_> {}

impl<'msg> ::protobuf::AsView for MatchMapView<'msg> {
  type Proxied = MatchMap;
  fn as_view(&self) -> ::protobuf::View<'msg, MatchMap> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatchMapView<'msg> {
  fn into_view<'shorter>(self) -> MatchMapView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MatchMap> for MatchMapView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatchMap {
    let mut dst = MatchMap::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MatchMap> for MatchMapMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatchMap {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MatchMap {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatchMapView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatchMapMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MatchMapMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchMap>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatchMapMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MatchMapMut<'msg> {
  type Message = MatchMap;
}

impl ::std::fmt::Debug for MatchMapMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MatchMap>> for MatchMapMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchMap>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatchMapMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchMap> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MatchMap {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // map: repeated message envoy.config.common.matcher.v3.Matcher.MatcherTree.MatchMap.MapEntry
  pub fn map(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::super::super::matcher::OnMatch> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::super::super::matcher::OnMatch>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn map_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::super::super::matcher::OnMatch> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_map(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::super::super::matcher::OnMatch>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `MatchMapMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MatchMapMut<'_> {}

// SAFETY:
// - `MatchMapMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MatchMapMut<'_> {}

impl<'msg> ::protobuf::AsView for MatchMapMut<'msg> {
  type Proxied = MatchMap;
  fn as_view(&self) -> ::protobuf::View<'_, MatchMap> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatchMapMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MatchMap>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MatchMapMut<'msg> {
  type MutProxied = MatchMap;
  fn as_mut(&mut self) -> MatchMapMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MatchMapMut<'msg> {
  fn into_mut<'shorter>(self) -> MatchMapMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MatchMap {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MatchMap> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MatchMapView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MatchMapMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // map: repeated message envoy.config.common.matcher.v3.Matcher.MatcherTree.MatchMap.MapEntry
  pub fn map(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, super::super::super::matcher::OnMatch> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(0)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, super::super::super::matcher::OnMatch>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn map_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, super::super::super::matcher::OnMatch> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          0, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_map(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, super::super::super::matcher::OnMatch>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl MatchMap

impl ::std::ops::Drop for MatchMap {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MatchMap {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MatchMap {
  type Proxied = Self;
  fn as_view(&self) -> MatchMapView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MatchMap {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MatchMapMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MatchMap {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::super::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::matcher::matcher_tree::envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatchMap {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatchMap {
  type Msg = MatchMap;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchMap> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchMap {
  type Msg = MatchMap;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchMap> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatchMapMut<'_> {
  type Msg = MatchMap;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchMap> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchMapMut<'_> {
  type Msg = MatchMap;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchMap> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchMapView<'_> {
  type Msg = MatchMap;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchMap> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatchMapMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod match_map {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct MapEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MapEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::super::super::Matcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::matcher::matcher_tree::match_map::envoy__config__common__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init.0)
      }).0
    }
  }
}

}  // pub mod match_map


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum TreeTypeOneof<'msg> {
  ExactMatchMap(::protobuf::View<'msg, super::super::super::matcher::matcher_tree::MatchMap>) = 2,
  PrefixMatchMap(::protobuf::View<'msg, super::super::super::matcher::matcher_tree::MatchMap>) = 3,
  CustomMatch(::protobuf::View<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) = 4,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum TreeTypeCase {
  ExactMatchMap = 2,
  PrefixMatchMap = 3,
  CustomMatch = 4,

  not_set = 0
}

impl TreeTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<TreeTypeCase> {
    match v {
      0 => Some(TreeTypeCase::not_set),
      2 => Some(TreeTypeCase::ExactMatchMap),
      3 => Some(TreeTypeCase::PrefixMatchMap),
      4 => Some(TreeTypeCase::CustomMatch),
      _ => None
    }
  }
}
}  // pub mod matcher_tree


#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum MatcherTypeOneof<'msg> {
  MatcherList(::protobuf::View<'msg, super::super::matcher::MatcherList>) = 1,
  MatcherTree(::protobuf::View<'msg, super::super::matcher::MatcherTree>) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum MatcherTypeCase {
  MatcherList = 1,
  MatcherTree = 2,

  not_set = 0
}

impl MatcherTypeCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<MatcherTypeCase> {
    match v {
      0 => Some(MatcherTypeCase::not_set),
      1 => Some(MatcherTypeCase::MatcherList),
      2 => Some(MatcherTypeCase::MatcherTree),
      _ => None
    }
  }
}
}  // pub mod matcher


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__MatchPredicate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MatchPredicate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MatchPredicate>
}

impl ::protobuf::Message for MatchPredicate {
  type MessageView<'msg> = MatchPredicateView<'msg>;
  type MessageMut<'msg> = MatchPredicateMut<'msg>;
}

impl ::std::default::Default for MatchPredicate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MatchPredicate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MatchPredicate` is `Sync` because it does not implement interior mutability.
//    Neither does `MatchPredicateMut`.
unsafe impl ::std::marker::Sync for MatchPredicate {}

// SAFETY:
// - `MatchPredicate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MatchPredicate {}

impl ::protobuf::Proxied for MatchPredicate {
  type View<'msg> = MatchPredicateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MatchPredicate {}

impl ::protobuf::MutProxied for MatchPredicate {
  type Mut<'msg> = MatchPredicateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MatchPredicateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatchPredicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatchPredicateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MatchPredicateView<'msg> {
  type Message = MatchPredicate;
}

impl ::std::fmt::Debug for MatchPredicateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MatchPredicateView<'_> {
  fn default() -> MatchPredicateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MatchPredicate>> for MatchPredicateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatchPredicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatchPredicateView<'msg> {

  pub fn to_owned(&self) -> MatchPredicate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // or_match: optional message envoy.config.common.matcher.v3.MatchPredicate.MatchSet
  pub fn has_or_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn or_match_opt(self) -> ::std::option::Option<super::match_predicate::MatchSetView<'msg>> {
    self.has_or_match().then(|| self.or_match())
  }
  pub fn or_match(self) -> super::match_predicate::MatchSetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::match_predicate::MatchSetView::default())
  }

  // and_match: optional message envoy.config.common.matcher.v3.MatchPredicate.MatchSet
  pub fn has_and_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn and_match_opt(self) -> ::std::option::Option<super::match_predicate::MatchSetView<'msg>> {
    self.has_and_match().then(|| self.and_match())
  }
  pub fn and_match(self) -> super::match_predicate::MatchSetView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::match_predicate::MatchSetView::default())
  }

  // not_match: optional message envoy.config.common.matcher.v3.MatchPredicate
  pub fn has_not_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn not_match_opt(self) -> ::std::option::Option<super::MatchPredicateView<'msg>> {
    self.has_not_match().then(|| self.not_match())
  }
  pub fn not_match(self) -> super::MatchPredicateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MatchPredicateView::default())
  }

  // any_match: optional bool
  pub fn has_any_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn any_match_opt(self) -> ::std::option::Option<bool> {
    self.has_any_match().then(|| self.any_match())
  }
  pub fn any_match(self) -> bool {
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

  // http_request_headers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_request_headers_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn http_request_headers_match_opt(self) -> ::std::option::Option<super::HttpHeadersMatchView<'msg>> {
    self.has_http_request_headers_match().then(|| self.http_request_headers_match())
  }
  pub fn http_request_headers_match(self) -> super::HttpHeadersMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }

  // http_request_trailers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_request_trailers_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn http_request_trailers_match_opt(self) -> ::std::option::Option<super::HttpHeadersMatchView<'msg>> {
    self.has_http_request_trailers_match().then(|| self.http_request_trailers_match())
  }
  pub fn http_request_trailers_match(self) -> super::HttpHeadersMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }

  // http_response_headers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_response_headers_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn http_response_headers_match_opt(self) -> ::std::option::Option<super::HttpHeadersMatchView<'msg>> {
    self.has_http_response_headers_match().then(|| self.http_response_headers_match())
  }
  pub fn http_response_headers_match(self) -> super::HttpHeadersMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }

  // http_response_trailers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_response_trailers_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn http_response_trailers_match_opt(self) -> ::std::option::Option<super::HttpHeadersMatchView<'msg>> {
    self.has_http_response_trailers_match().then(|| self.http_response_trailers_match())
  }
  pub fn http_response_trailers_match(self) -> super::HttpHeadersMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }

  // http_request_generic_body_match: optional message envoy.config.common.matcher.v3.HttpGenericBodyMatch
  pub fn has_http_request_generic_body_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn http_request_generic_body_match_opt(self) -> ::std::option::Option<super::HttpGenericBodyMatchView<'msg>> {
    self.has_http_request_generic_body_match().then(|| self.http_request_generic_body_match())
  }
  pub fn http_request_generic_body_match(self) -> super::HttpGenericBodyMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpGenericBodyMatchView::default())
  }

  // http_response_generic_body_match: optional message envoy.config.common.matcher.v3.HttpGenericBodyMatch
  pub fn has_http_response_generic_body_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn http_response_generic_body_match_opt(self) -> ::std::option::Option<super::HttpGenericBodyMatchView<'msg>> {
    self.has_http_response_generic_body_match().then(|| self.http_response_generic_body_match())
  }
  pub fn http_response_generic_body_match(self) -> super::HttpGenericBodyMatchView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpGenericBodyMatchView::default())
  }

  pub fn rule(self) -> super::match_predicate::RuleOneof<'msg> {
    match self.rule_case() {
      super::match_predicate::RuleCase::OrMatch =>
          super::match_predicate::RuleOneof::OrMatch(self.or_match()),
      super::match_predicate::RuleCase::AndMatch =>
          super::match_predicate::RuleOneof::AndMatch(self.and_match()),
      super::match_predicate::RuleCase::NotMatch =>
          super::match_predicate::RuleOneof::NotMatch(self.not_match()),
      super::match_predicate::RuleCase::AnyMatch =>
          super::match_predicate::RuleOneof::AnyMatch(self.any_match()),
      super::match_predicate::RuleCase::HttpRequestHeadersMatch =>
          super::match_predicate::RuleOneof::HttpRequestHeadersMatch(self.http_request_headers_match()),
      super::match_predicate::RuleCase::HttpRequestTrailersMatch =>
          super::match_predicate::RuleOneof::HttpRequestTrailersMatch(self.http_request_trailers_match()),
      super::match_predicate::RuleCase::HttpResponseHeadersMatch =>
          super::match_predicate::RuleOneof::HttpResponseHeadersMatch(self.http_response_headers_match()),
      super::match_predicate::RuleCase::HttpResponseTrailersMatch =>
          super::match_predicate::RuleOneof::HttpResponseTrailersMatch(self.http_response_trailers_match()),
      super::match_predicate::RuleCase::HttpRequestGenericBodyMatch =>
          super::match_predicate::RuleOneof::HttpRequestGenericBodyMatch(self.http_request_generic_body_match()),
      super::match_predicate::RuleCase::HttpResponseGenericBodyMatch =>
          super::match_predicate::RuleOneof::HttpResponseGenericBodyMatch(self.http_response_generic_body_match()),
      _ => super::match_predicate::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(self) -> super::match_predicate::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::match_predicate::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MatchPredicateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MatchPredicateView<'_> {}

// SAFETY:
// - `MatchPredicateView` is `Send` because while its alive a `MatchPredicateMut` cannot.
// - `MatchPredicateView` does not use thread-local data.
unsafe impl ::std::marker::Send for MatchPredicateView<'_> {}

impl<'msg> ::protobuf::AsView for MatchPredicateView<'msg> {
  type Proxied = MatchPredicate;
  fn as_view(&self) -> ::protobuf::View<'msg, MatchPredicate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatchPredicateView<'msg> {
  fn into_view<'shorter>(self) -> MatchPredicateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MatchPredicate> for MatchPredicateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatchPredicate {
    let mut dst = MatchPredicate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MatchPredicate> for MatchPredicateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatchPredicate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MatchPredicate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatchPredicateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatchPredicateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MatchPredicateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchPredicate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatchPredicateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MatchPredicateMut<'msg> {
  type Message = MatchPredicate;
}

impl ::std::fmt::Debug for MatchPredicateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MatchPredicate>> for MatchPredicateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchPredicate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatchPredicateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchPredicate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MatchPredicate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // or_match: optional message envoy.config.common.matcher.v3.MatchPredicate.MatchSet
  pub fn has_or_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_or_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn or_match_opt(&self) -> ::std::option::Option<super::match_predicate::MatchSetView<'_>> {
    self.has_or_match().then(|| self.or_match())
  }
  pub fn or_match(&self) -> super::match_predicate::MatchSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::match_predicate::MatchSetView::default())
  }
  pub fn or_match_mut(&mut self) -> super::match_predicate::MatchSetMut<'_> {
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
  pub fn set_or_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::match_predicate::MatchSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // and_match: optional message envoy.config.common.matcher.v3.MatchPredicate.MatchSet
  pub fn has_and_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_and_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn and_match_opt(&self) -> ::std::option::Option<super::match_predicate::MatchSetView<'_>> {
    self.has_and_match().then(|| self.and_match())
  }
  pub fn and_match(&self) -> super::match_predicate::MatchSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::match_predicate::MatchSetView::default())
  }
  pub fn and_match_mut(&mut self) -> super::match_predicate::MatchSetMut<'_> {
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
  pub fn set_and_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::match_predicate::MatchSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // not_match: optional message envoy.config.common.matcher.v3.MatchPredicate
  pub fn has_not_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_not_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn not_match_opt(&self) -> ::std::option::Option<super::MatchPredicateView<'_>> {
    self.has_not_match().then(|| self.not_match())
  }
  pub fn not_match(&self) -> super::MatchPredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MatchPredicateView::default())
  }
  pub fn not_match_mut(&mut self) -> super::MatchPredicateMut<'_> {
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
  pub fn set_not_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::MatchPredicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // any_match: optional bool
  pub fn has_any_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_any_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn any_match_opt(&self) -> ::std::option::Option<bool> {
    self.has_any_match().then(|| self.any_match())
  }
  pub fn any_match(&self) -> bool {
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
  pub fn set_any_match(&mut self, val: bool) {
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

  // http_request_headers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_request_headers_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_http_request_headers_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn http_request_headers_match_opt(&self) -> ::std::option::Option<super::HttpHeadersMatchView<'_>> {
    self.has_http_request_headers_match().then(|| self.http_request_headers_match())
  }
  pub fn http_request_headers_match(&self) -> super::HttpHeadersMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }
  pub fn http_request_headers_match_mut(&mut self) -> super::HttpHeadersMatchMut<'_> {
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
  pub fn set_http_request_headers_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeadersMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // http_request_trailers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_request_trailers_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_http_request_trailers_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn http_request_trailers_match_opt(&self) -> ::std::option::Option<super::HttpHeadersMatchView<'_>> {
    self.has_http_request_trailers_match().then(|| self.http_request_trailers_match())
  }
  pub fn http_request_trailers_match(&self) -> super::HttpHeadersMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }
  pub fn http_request_trailers_match_mut(&mut self) -> super::HttpHeadersMatchMut<'_> {
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
  pub fn set_http_request_trailers_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeadersMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // http_response_headers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_response_headers_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_http_response_headers_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn http_response_headers_match_opt(&self) -> ::std::option::Option<super::HttpHeadersMatchView<'_>> {
    self.has_http_response_headers_match().then(|| self.http_response_headers_match())
  }
  pub fn http_response_headers_match(&self) -> super::HttpHeadersMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }
  pub fn http_response_headers_match_mut(&mut self) -> super::HttpHeadersMatchMut<'_> {
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
  pub fn set_http_response_headers_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeadersMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // http_response_trailers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_response_trailers_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_http_response_trailers_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn http_response_trailers_match_opt(&self) -> ::std::option::Option<super::HttpHeadersMatchView<'_>> {
    self.has_http_response_trailers_match().then(|| self.http_response_trailers_match())
  }
  pub fn http_response_trailers_match(&self) -> super::HttpHeadersMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }
  pub fn http_response_trailers_match_mut(&mut self) -> super::HttpHeadersMatchMut<'_> {
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
  pub fn set_http_response_trailers_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeadersMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // http_request_generic_body_match: optional message envoy.config.common.matcher.v3.HttpGenericBodyMatch
  pub fn has_http_request_generic_body_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_http_request_generic_body_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn http_request_generic_body_match_opt(&self) -> ::std::option::Option<super::HttpGenericBodyMatchView<'_>> {
    self.has_http_request_generic_body_match().then(|| self.http_request_generic_body_match())
  }
  pub fn http_request_generic_body_match(&self) -> super::HttpGenericBodyMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpGenericBodyMatchView::default())
  }
  pub fn http_request_generic_body_match_mut(&mut self) -> super::HttpGenericBodyMatchMut<'_> {
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
  pub fn set_http_request_generic_body_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpGenericBodyMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // http_response_generic_body_match: optional message envoy.config.common.matcher.v3.HttpGenericBodyMatch
  pub fn has_http_response_generic_body_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_http_response_generic_body_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn http_response_generic_body_match_opt(&self) -> ::std::option::Option<super::HttpGenericBodyMatchView<'_>> {
    self.has_http_response_generic_body_match().then(|| self.http_response_generic_body_match())
  }
  pub fn http_response_generic_body_match(&self) -> super::HttpGenericBodyMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpGenericBodyMatchView::default())
  }
  pub fn http_response_generic_body_match_mut(&mut self) -> super::HttpGenericBodyMatchMut<'_> {
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
  pub fn set_http_response_generic_body_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpGenericBodyMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  pub fn rule(&self) -> super::match_predicate::RuleOneof<'_> {
    match &self.rule_case() {
      super::match_predicate::RuleCase::OrMatch =>
          super::match_predicate::RuleOneof::OrMatch(self.or_match()),
      super::match_predicate::RuleCase::AndMatch =>
          super::match_predicate::RuleOneof::AndMatch(self.and_match()),
      super::match_predicate::RuleCase::NotMatch =>
          super::match_predicate::RuleOneof::NotMatch(self.not_match()),
      super::match_predicate::RuleCase::AnyMatch =>
          super::match_predicate::RuleOneof::AnyMatch(self.any_match()),
      super::match_predicate::RuleCase::HttpRequestHeadersMatch =>
          super::match_predicate::RuleOneof::HttpRequestHeadersMatch(self.http_request_headers_match()),
      super::match_predicate::RuleCase::HttpRequestTrailersMatch =>
          super::match_predicate::RuleOneof::HttpRequestTrailersMatch(self.http_request_trailers_match()),
      super::match_predicate::RuleCase::HttpResponseHeadersMatch =>
          super::match_predicate::RuleOneof::HttpResponseHeadersMatch(self.http_response_headers_match()),
      super::match_predicate::RuleCase::HttpResponseTrailersMatch =>
          super::match_predicate::RuleOneof::HttpResponseTrailersMatch(self.http_response_trailers_match()),
      super::match_predicate::RuleCase::HttpRequestGenericBodyMatch =>
          super::match_predicate::RuleOneof::HttpRequestGenericBodyMatch(self.http_request_generic_body_match()),
      super::match_predicate::RuleCase::HttpResponseGenericBodyMatch =>
          super::match_predicate::RuleOneof::HttpResponseGenericBodyMatch(self.http_response_generic_body_match()),
      _ => super::match_predicate::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::match_predicate::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::match_predicate::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `MatchPredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MatchPredicateMut<'_> {}

// SAFETY:
// - `MatchPredicateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MatchPredicateMut<'_> {}

impl<'msg> ::protobuf::AsView for MatchPredicateMut<'msg> {
  type Proxied = MatchPredicate;
  fn as_view(&self) -> ::protobuf::View<'_, MatchPredicate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatchPredicateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MatchPredicate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MatchPredicateMut<'msg> {
  type MutProxied = MatchPredicate;
  fn as_mut(&mut self) -> MatchPredicateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MatchPredicateMut<'msg> {
  fn into_mut<'shorter>(self) -> MatchPredicateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MatchPredicate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MatchPredicate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MatchPredicateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MatchPredicateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // or_match: optional message envoy.config.common.matcher.v3.MatchPredicate.MatchSet
  pub fn has_or_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_or_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn or_match_opt(&self) -> ::std::option::Option<super::match_predicate::MatchSetView<'_>> {
    self.has_or_match().then(|| self.or_match())
  }
  pub fn or_match(&self) -> super::match_predicate::MatchSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::match_predicate::MatchSetView::default())
  }
  pub fn or_match_mut(&mut self) -> super::match_predicate::MatchSetMut<'_> {
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
  pub fn set_or_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::match_predicate::MatchSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // and_match: optional message envoy.config.common.matcher.v3.MatchPredicate.MatchSet
  pub fn has_and_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_and_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn and_match_opt(&self) -> ::std::option::Option<super::match_predicate::MatchSetView<'_>> {
    self.has_and_match().then(|| self.and_match())
  }
  pub fn and_match(&self) -> super::match_predicate::MatchSetView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::match_predicate::MatchSetView::default())
  }
  pub fn and_match_mut(&mut self) -> super::match_predicate::MatchSetMut<'_> {
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
  pub fn set_and_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::match_predicate::MatchSet>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // not_match: optional message envoy.config.common.matcher.v3.MatchPredicate
  pub fn has_not_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_not_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn not_match_opt(&self) -> ::std::option::Option<super::MatchPredicateView<'_>> {
    self.has_not_match().then(|| self.not_match())
  }
  pub fn not_match(&self) -> super::MatchPredicateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MatchPredicateView::default())
  }
  pub fn not_match_mut(&mut self) -> super::MatchPredicateMut<'_> {
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
  pub fn set_not_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::MatchPredicate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // any_match: optional bool
  pub fn has_any_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_any_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn any_match_opt(&self) -> ::std::option::Option<bool> {
    self.has_any_match().then(|| self.any_match())
  }
  pub fn any_match(&self) -> bool {
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
  pub fn set_any_match(&mut self, val: bool) {
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

  // http_request_headers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_request_headers_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_http_request_headers_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn http_request_headers_match_opt(&self) -> ::std::option::Option<super::HttpHeadersMatchView<'_>> {
    self.has_http_request_headers_match().then(|| self.http_request_headers_match())
  }
  pub fn http_request_headers_match(&self) -> super::HttpHeadersMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }
  pub fn http_request_headers_match_mut(&mut self) -> super::HttpHeadersMatchMut<'_> {
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
  pub fn set_http_request_headers_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeadersMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // http_request_trailers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_request_trailers_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_http_request_trailers_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn http_request_trailers_match_opt(&self) -> ::std::option::Option<super::HttpHeadersMatchView<'_>> {
    self.has_http_request_trailers_match().then(|| self.http_request_trailers_match())
  }
  pub fn http_request_trailers_match(&self) -> super::HttpHeadersMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }
  pub fn http_request_trailers_match_mut(&mut self) -> super::HttpHeadersMatchMut<'_> {
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
  pub fn set_http_request_trailers_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeadersMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // http_response_headers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_response_headers_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_http_response_headers_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn http_response_headers_match_opt(&self) -> ::std::option::Option<super::HttpHeadersMatchView<'_>> {
    self.has_http_response_headers_match().then(|| self.http_response_headers_match())
  }
  pub fn http_response_headers_match(&self) -> super::HttpHeadersMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }
  pub fn http_response_headers_match_mut(&mut self) -> super::HttpHeadersMatchMut<'_> {
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
  pub fn set_http_response_headers_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeadersMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // http_response_trailers_match: optional message envoy.config.common.matcher.v3.HttpHeadersMatch
  pub fn has_http_response_trailers_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_http_response_trailers_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn http_response_trailers_match_opt(&self) -> ::std::option::Option<super::HttpHeadersMatchView<'_>> {
    self.has_http_response_trailers_match().then(|| self.http_response_trailers_match())
  }
  pub fn http_response_trailers_match(&self) -> super::HttpHeadersMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpHeadersMatchView::default())
  }
  pub fn http_response_trailers_match_mut(&mut self) -> super::HttpHeadersMatchMut<'_> {
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
  pub fn set_http_response_trailers_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpHeadersMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // http_request_generic_body_match: optional message envoy.config.common.matcher.v3.HttpGenericBodyMatch
  pub fn has_http_request_generic_body_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_http_request_generic_body_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn http_request_generic_body_match_opt(&self) -> ::std::option::Option<super::HttpGenericBodyMatchView<'_>> {
    self.has_http_request_generic_body_match().then(|| self.http_request_generic_body_match())
  }
  pub fn http_request_generic_body_match(&self) -> super::HttpGenericBodyMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpGenericBodyMatchView::default())
  }
  pub fn http_request_generic_body_match_mut(&mut self) -> super::HttpGenericBodyMatchMut<'_> {
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
  pub fn set_http_request_generic_body_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpGenericBodyMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // http_response_generic_body_match: optional message envoy.config.common.matcher.v3.HttpGenericBodyMatch
  pub fn has_http_response_generic_body_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_http_response_generic_body_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn http_response_generic_body_match_opt(&self) -> ::std::option::Option<super::HttpGenericBodyMatchView<'_>> {
    self.has_http_response_generic_body_match().then(|| self.http_response_generic_body_match())
  }
  pub fn http_response_generic_body_match(&self) -> super::HttpGenericBodyMatchView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::HttpGenericBodyMatchView::default())
  }
  pub fn http_response_generic_body_match_mut(&mut self) -> super::HttpGenericBodyMatchMut<'_> {
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
  pub fn set_http_response_generic_body_match(&mut self,
    val: impl ::protobuf::IntoProxied<super::HttpGenericBodyMatch>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  pub fn rule(&self) -> super::match_predicate::RuleOneof<'_> {
    match &self.rule_case() {
      super::match_predicate::RuleCase::OrMatch =>
          super::match_predicate::RuleOneof::OrMatch(self.or_match()),
      super::match_predicate::RuleCase::AndMatch =>
          super::match_predicate::RuleOneof::AndMatch(self.and_match()),
      super::match_predicate::RuleCase::NotMatch =>
          super::match_predicate::RuleOneof::NotMatch(self.not_match()),
      super::match_predicate::RuleCase::AnyMatch =>
          super::match_predicate::RuleOneof::AnyMatch(self.any_match()),
      super::match_predicate::RuleCase::HttpRequestHeadersMatch =>
          super::match_predicate::RuleOneof::HttpRequestHeadersMatch(self.http_request_headers_match()),
      super::match_predicate::RuleCase::HttpRequestTrailersMatch =>
          super::match_predicate::RuleOneof::HttpRequestTrailersMatch(self.http_request_trailers_match()),
      super::match_predicate::RuleCase::HttpResponseHeadersMatch =>
          super::match_predicate::RuleOneof::HttpResponseHeadersMatch(self.http_response_headers_match()),
      super::match_predicate::RuleCase::HttpResponseTrailersMatch =>
          super::match_predicate::RuleOneof::HttpResponseTrailersMatch(self.http_response_trailers_match()),
      super::match_predicate::RuleCase::HttpRequestGenericBodyMatch =>
          super::match_predicate::RuleOneof::HttpRequestGenericBodyMatch(self.http_request_generic_body_match()),
      super::match_predicate::RuleCase::HttpResponseGenericBodyMatch =>
          super::match_predicate::RuleOneof::HttpResponseGenericBodyMatch(self.http_response_generic_body_match()),
      _ => super::match_predicate::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::match_predicate::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::match_predicate::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl MatchPredicate

impl ::std::ops::Drop for MatchPredicate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MatchPredicate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MatchPredicate {
  type Proxied = Self;
  fn as_view(&self) -> MatchPredicateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MatchPredicate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MatchPredicateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MatchPredicate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__common__matcher__v3__MatchPredicate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333/333333^!|#|$|%|&|(|)|*|+|,");
        super::match_predicate::envoy__config__common__matcher__v3__MatchPredicate__MatchSet_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__common__matcher__v3__MatchPredicate_msg_init.0, &[super::match_predicate::envoy__config__common__matcher__v3__MatchPredicate__MatchSet_msg_init.0,
            super::match_predicate::envoy__config__common__matcher__v3__MatchPredicate__MatchSet_msg_init.0,
            super::envoy__config__common__matcher__v3__MatchPredicate_msg_init.0,
            <super::HttpHeadersMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpHeadersMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpHeadersMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpHeadersMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpGenericBodyMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::HttpGenericBodyMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::match_predicate::envoy__config__common__matcher__v3__MatchPredicate__MatchSet_msg_init.0, &[super::envoy__config__common__matcher__v3__MatchPredicate_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__common__matcher__v3__MatchPredicate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatchPredicate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatchPredicate {
  type Msg = MatchPredicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchPredicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchPredicate {
  type Msg = MatchPredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchPredicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatchPredicateMut<'_> {
  type Msg = MatchPredicate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchPredicate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchPredicateMut<'_> {
  type Msg = MatchPredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchPredicate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchPredicateView<'_> {
  type Msg = MatchPredicate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchPredicate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatchPredicateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod match_predicate {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__MatchPredicate__MatchSet_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MatchSet {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MatchSet>
}

impl ::protobuf::Message for MatchSet {
  type MessageView<'msg> = MatchSetView<'msg>;
  type MessageMut<'msg> = MatchSetMut<'msg>;
}

impl ::std::default::Default for MatchSet {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MatchSet {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MatchSet` is `Sync` because it does not implement interior mutability.
//    Neither does `MatchSetMut`.
unsafe impl ::std::marker::Sync for MatchSet {}

// SAFETY:
// - `MatchSet` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MatchSet {}

impl ::protobuf::Proxied for MatchSet {
  type View<'msg> = MatchSetView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MatchSet {}

impl ::protobuf::MutProxied for MatchSet {
  type Mut<'msg> = MatchSetMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MatchSetView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatchSet>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatchSetView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MatchSetView<'msg> {
  type Message = MatchSet;
}

impl ::std::fmt::Debug for MatchSetView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MatchSetView<'_> {
  fn default() -> MatchSetView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MatchSet>> for MatchSetView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MatchSet>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatchSetView<'msg> {

  pub fn to_owned(&self) -> MatchSet {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // rules: repeated message envoy.config.common.matcher.v3.MatchPredicate
  pub fn rules(self) -> ::protobuf::RepeatedView<'msg, super::super::MatchPredicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::MatchPredicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `MatchSetView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MatchSetView<'_> {}

// SAFETY:
// - `MatchSetView` is `Send` because while its alive a `MatchSetMut` cannot.
// - `MatchSetView` does not use thread-local data.
unsafe impl ::std::marker::Send for MatchSetView<'_> {}

impl<'msg> ::protobuf::AsView for MatchSetView<'msg> {
  type Proxied = MatchSet;
  fn as_view(&self) -> ::protobuf::View<'msg, MatchSet> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatchSetView<'msg> {
  fn into_view<'shorter>(self) -> MatchSetView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MatchSet> for MatchSetView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatchSet {
    let mut dst = MatchSet::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MatchSet> for MatchSetMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MatchSet {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MatchSet {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatchSetView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MatchSetMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MatchSetMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchSet>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MatchSetMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MatchSetMut<'msg> {
  type Message = MatchSet;
}

impl ::std::fmt::Debug for MatchSetMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MatchSet>> for MatchSetMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchSet>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MatchSetMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MatchSet> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MatchSet {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // rules: repeated message envoy.config.common.matcher.v3.MatchPredicate
  pub fn rules(&self) -> ::protobuf::RepeatedView<'_, super::super::MatchPredicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::MatchPredicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn rules_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::MatchPredicate> {
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
  pub fn set_rules(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::MatchPredicate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `MatchSetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MatchSetMut<'_> {}

// SAFETY:
// - `MatchSetMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MatchSetMut<'_> {}

impl<'msg> ::protobuf::AsView for MatchSetMut<'msg> {
  type Proxied = MatchSet;
  fn as_view(&self) -> ::protobuf::View<'_, MatchSet> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MatchSetMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MatchSet>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MatchSetMut<'msg> {
  type MutProxied = MatchSet;
  fn as_mut(&mut self) -> MatchSetMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MatchSetMut<'msg> {
  fn into_mut<'shorter>(self) -> MatchSetMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MatchSet {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MatchSet> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MatchSetView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MatchSetMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // rules: repeated message envoy.config.common.matcher.v3.MatchPredicate
  pub fn rules(&self) -> ::protobuf::RepeatedView<'_, super::super::MatchPredicate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::MatchPredicate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn rules_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::MatchPredicate> {
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
  pub fn set_rules(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::MatchPredicate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl MatchSet

impl ::std::ops::Drop for MatchSet {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MatchSet {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MatchSet {
  type Proxied = Self;
  fn as_view(&self) -> MatchSetView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MatchSet {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MatchSetMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MatchSet {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        <super::super::MatchPredicate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table();
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::match_predicate::envoy__config__common__matcher__v3__MatchPredicate__MatchSet_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatchSet {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatchSet {
  type Msg = MatchSet;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchSet {
  type Msg = MatchSet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MatchSetMut<'_> {
  type Msg = MatchSet;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchSetMut<'_> {
  type Msg = MatchSet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MatchSetView<'_> {
  type Msg = MatchSet;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MatchSet> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MatchSetMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum RuleOneof<'msg> {
  OrMatch(::protobuf::View<'msg, super::super::match_predicate::MatchSet>) = 1,
  AndMatch(::protobuf::View<'msg, super::super::match_predicate::MatchSet>) = 2,
  NotMatch(::protobuf::View<'msg, super::super::MatchPredicate>) = 3,
  AnyMatch(bool) = 4,
  HttpRequestHeadersMatch(::protobuf::View<'msg, super::super::HttpHeadersMatch>) = 5,
  HttpRequestTrailersMatch(::protobuf::View<'msg, super::super::HttpHeadersMatch>) = 6,
  HttpResponseHeadersMatch(::protobuf::View<'msg, super::super::HttpHeadersMatch>) = 7,
  HttpResponseTrailersMatch(::protobuf::View<'msg, super::super::HttpHeadersMatch>) = 8,
  HttpRequestGenericBodyMatch(::protobuf::View<'msg, super::super::HttpGenericBodyMatch>) = 9,
  HttpResponseGenericBodyMatch(::protobuf::View<'msg, super::super::HttpGenericBodyMatch>) = 10,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum RuleCase {
  OrMatch = 1,
  AndMatch = 2,
  NotMatch = 3,
  AnyMatch = 4,
  HttpRequestHeadersMatch = 5,
  HttpRequestTrailersMatch = 6,
  HttpResponseHeadersMatch = 7,
  HttpResponseTrailersMatch = 8,
  HttpRequestGenericBodyMatch = 9,
  HttpResponseGenericBodyMatch = 10,

  not_set = 0
}

impl RuleCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<RuleCase> {
    match v {
      0 => Some(RuleCase::not_set),
      1 => Some(RuleCase::OrMatch),
      2 => Some(RuleCase::AndMatch),
      3 => Some(RuleCase::NotMatch),
      4 => Some(RuleCase::AnyMatch),
      5 => Some(RuleCase::HttpRequestHeadersMatch),
      6 => Some(RuleCase::HttpRequestTrailersMatch),
      7 => Some(RuleCase::HttpResponseHeadersMatch),
      8 => Some(RuleCase::HttpResponseTrailersMatch),
      9 => Some(RuleCase::HttpRequestGenericBodyMatch),
      10 => Some(RuleCase::HttpResponseGenericBodyMatch),
      _ => None
    }
  }
}
}  // pub mod match_predicate


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__HttpHeadersMatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpHeadersMatch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpHeadersMatch>
}

impl ::protobuf::Message for HttpHeadersMatch {
  type MessageView<'msg> = HttpHeadersMatchView<'msg>;
  type MessageMut<'msg> = HttpHeadersMatchMut<'msg>;
}

impl ::std::default::Default for HttpHeadersMatch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpHeadersMatch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpHeadersMatch` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpHeadersMatchMut`.
unsafe impl ::std::marker::Sync for HttpHeadersMatch {}

// SAFETY:
// - `HttpHeadersMatch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpHeadersMatch {}

impl ::protobuf::Proxied for HttpHeadersMatch {
  type View<'msg> = HttpHeadersMatchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpHeadersMatch {}

impl ::protobuf::MutProxied for HttpHeadersMatch {
  type Mut<'msg> = HttpHeadersMatchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpHeadersMatchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpHeadersMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpHeadersMatchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpHeadersMatchView<'msg> {
  type Message = HttpHeadersMatch;
}

impl ::std::fmt::Debug for HttpHeadersMatchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpHeadersMatchView<'_> {
  fn default() -> HttpHeadersMatchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpHeadersMatch>> for HttpHeadersMatchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpHeadersMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpHeadersMatchView<'msg> {

  pub fn to_owned(&self) -> HttpHeadersMatch {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // headers: repeated message envoy.config.route.v3.HeaderMatcher
  pub fn headers(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `HttpHeadersMatchView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpHeadersMatchView<'_> {}

// SAFETY:
// - `HttpHeadersMatchView` is `Send` because while its alive a `HttpHeadersMatchMut` cannot.
// - `HttpHeadersMatchView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpHeadersMatchView<'_> {}

impl<'msg> ::protobuf::AsView for HttpHeadersMatchView<'msg> {
  type Proxied = HttpHeadersMatch;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpHeadersMatch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpHeadersMatchView<'msg> {
  fn into_view<'shorter>(self) -> HttpHeadersMatchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpHeadersMatch> for HttpHeadersMatchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpHeadersMatch {
    let mut dst = HttpHeadersMatch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpHeadersMatch> for HttpHeadersMatchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpHeadersMatch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpHeadersMatch {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpHeadersMatchView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpHeadersMatchMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpHeadersMatchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHeadersMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpHeadersMatchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpHeadersMatchMut<'msg> {
  type Message = HttpHeadersMatch;
}

impl ::std::fmt::Debug for HttpHeadersMatchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHeadersMatch>> for HttpHeadersMatchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHeadersMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpHeadersMatchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpHeadersMatch> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpHeadersMatch {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // headers: repeated message envoy.config.route.v3.HeaderMatcher
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `HttpHeadersMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpHeadersMatchMut<'_> {}

// SAFETY:
// - `HttpHeadersMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpHeadersMatchMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpHeadersMatchMut<'msg> {
  type Proxied = HttpHeadersMatch;
  fn as_view(&self) -> ::protobuf::View<'_, HttpHeadersMatch> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpHeadersMatchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpHeadersMatch>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpHeadersMatchMut<'msg> {
  type MutProxied = HttpHeadersMatch;
  fn as_mut(&mut self) -> HttpHeadersMatchMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpHeadersMatchMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpHeadersMatchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpHeadersMatch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpHeadersMatch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpHeadersMatchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpHeadersMatchMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // headers: repeated message envoy.config.route.v3.HeaderMatcher
  pub fn headers(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher> {
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
  pub fn set_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl HttpHeadersMatch

impl ::std::ops::Drop for HttpHeadersMatch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpHeadersMatch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpHeadersMatch {
  type Proxied = Self;
  fn as_view(&self) -> HttpHeadersMatchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpHeadersMatch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpHeadersMatchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpHeadersMatch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__common__matcher__v3__HttpHeadersMatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__common__matcher__v3__HttpHeadersMatch_msg_init.0, &[<crate::xds::generated::envoy::config::route::v3::route_components::HeaderMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__common__matcher__v3__HttpHeadersMatch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpHeadersMatch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpHeadersMatch {
  type Msg = HttpHeadersMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeadersMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpHeadersMatch {
  type Msg = HttpHeadersMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeadersMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpHeadersMatchMut<'_> {
  type Msg = HttpHeadersMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeadersMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpHeadersMatchMut<'_> {
  type Msg = HttpHeadersMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeadersMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpHeadersMatchView<'_> {
  type Msg = HttpHeadersMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpHeadersMatch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpHeadersMatchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__HttpGenericBodyMatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct HttpGenericBodyMatch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<HttpGenericBodyMatch>
}

impl ::protobuf::Message for HttpGenericBodyMatch {
  type MessageView<'msg> = HttpGenericBodyMatchView<'msg>;
  type MessageMut<'msg> = HttpGenericBodyMatchMut<'msg>;
}

impl ::std::default::Default for HttpGenericBodyMatch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for HttpGenericBodyMatch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `HttpGenericBodyMatch` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpGenericBodyMatchMut`.
unsafe impl ::std::marker::Sync for HttpGenericBodyMatch {}

// SAFETY:
// - `HttpGenericBodyMatch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for HttpGenericBodyMatch {}

impl ::protobuf::Proxied for HttpGenericBodyMatch {
  type View<'msg> = HttpGenericBodyMatchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for HttpGenericBodyMatch {}

impl ::protobuf::MutProxied for HttpGenericBodyMatch {
  type Mut<'msg> = HttpGenericBodyMatchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpGenericBodyMatchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpGenericBodyMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpGenericBodyMatchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for HttpGenericBodyMatchView<'msg> {
  type Message = HttpGenericBodyMatch;
}

impl ::std::fmt::Debug for HttpGenericBodyMatchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for HttpGenericBodyMatchView<'_> {
  fn default() -> HttpGenericBodyMatchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, HttpGenericBodyMatch>> for HttpGenericBodyMatchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, HttpGenericBodyMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpGenericBodyMatchView<'msg> {

  pub fn to_owned(&self) -> HttpGenericBodyMatch {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bytes_limit: optional uint32
  pub fn bytes_limit(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // patterns: repeated message envoy.config.common.matcher.v3.HttpGenericBodyMatch.GenericTextMatch
  pub fn patterns(self) -> ::protobuf::RepeatedView<'msg, super::http_generic_body_match::GenericTextMatch> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http_generic_body_match::GenericTextMatch>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `HttpGenericBodyMatchView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for HttpGenericBodyMatchView<'_> {}

// SAFETY:
// - `HttpGenericBodyMatchView` is `Send` because while its alive a `HttpGenericBodyMatchMut` cannot.
// - `HttpGenericBodyMatchView` does not use thread-local data.
unsafe impl ::std::marker::Send for HttpGenericBodyMatchView<'_> {}

impl<'msg> ::protobuf::AsView for HttpGenericBodyMatchView<'msg> {
  type Proxied = HttpGenericBodyMatch;
  fn as_view(&self) -> ::protobuf::View<'msg, HttpGenericBodyMatch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpGenericBodyMatchView<'msg> {
  fn into_view<'shorter>(self) -> HttpGenericBodyMatchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpGenericBodyMatch> for HttpGenericBodyMatchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpGenericBodyMatch {
    let mut dst = HttpGenericBodyMatch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<HttpGenericBodyMatch> for HttpGenericBodyMatchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> HttpGenericBodyMatch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for HttpGenericBodyMatch {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpGenericBodyMatchView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for HttpGenericBodyMatchMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpGenericBodyMatchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpGenericBodyMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for HttpGenericBodyMatchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for HttpGenericBodyMatchMut<'msg> {
  type Message = HttpGenericBodyMatch;
}

impl ::std::fmt::Debug for HttpGenericBodyMatchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, HttpGenericBodyMatch>> for HttpGenericBodyMatchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpGenericBodyMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> HttpGenericBodyMatchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, HttpGenericBodyMatch> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> HttpGenericBodyMatch {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bytes_limit: optional uint32
  pub fn bytes_limit(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bytes_limit(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // patterns: repeated message envoy.config.common.matcher.v3.HttpGenericBodyMatch.GenericTextMatch
  pub fn patterns(&self) -> ::protobuf::RepeatedView<'_, super::http_generic_body_match::GenericTextMatch> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http_generic_body_match::GenericTextMatch>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn patterns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::http_generic_body_match::GenericTextMatch> {
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
  pub fn set_patterns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::http_generic_body_match::GenericTextMatch>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `HttpGenericBodyMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for HttpGenericBodyMatchMut<'_> {}

// SAFETY:
// - `HttpGenericBodyMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for HttpGenericBodyMatchMut<'_> {}

impl<'msg> ::protobuf::AsView for HttpGenericBodyMatchMut<'msg> {
  type Proxied = HttpGenericBodyMatch;
  fn as_view(&self) -> ::protobuf::View<'_, HttpGenericBodyMatch> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for HttpGenericBodyMatchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, HttpGenericBodyMatch>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for HttpGenericBodyMatchMut<'msg> {
  type MutProxied = HttpGenericBodyMatch;
  fn as_mut(&mut self) -> HttpGenericBodyMatchMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for HttpGenericBodyMatchMut<'msg> {
  fn into_mut<'shorter>(self) -> HttpGenericBodyMatchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl HttpGenericBodyMatch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, HttpGenericBodyMatch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> HttpGenericBodyMatchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> HttpGenericBodyMatchMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bytes_limit: optional uint32
  pub fn bytes_limit(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bytes_limit(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // patterns: repeated message envoy.config.common.matcher.v3.HttpGenericBodyMatch.GenericTextMatch
  pub fn patterns(&self) -> ::protobuf::RepeatedView<'_, super::http_generic_body_match::GenericTextMatch> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::http_generic_body_match::GenericTextMatch>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn patterns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::http_generic_body_match::GenericTextMatch> {
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
  pub fn set_patterns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::http_generic_body_match::GenericTextMatch>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl HttpGenericBodyMatch

impl ::std::ops::Drop for HttpGenericBodyMatch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for HttpGenericBodyMatch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for HttpGenericBodyMatch {
  type Proxied = Self;
  fn as_view(&self) -> HttpGenericBodyMatchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for HttpGenericBodyMatch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> HttpGenericBodyMatchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for HttpGenericBodyMatch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__common__matcher__v3__HttpGenericBodyMatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__common__matcher__v3__HttpGenericBodyMatch_msg_init.0, &[<super::http_generic_body_match::GenericTextMatch as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__common__matcher__v3__HttpGenericBodyMatch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpGenericBodyMatch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpGenericBodyMatch {
  type Msg = HttpGenericBodyMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpGenericBodyMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpGenericBodyMatch {
  type Msg = HttpGenericBodyMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpGenericBodyMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for HttpGenericBodyMatchMut<'_> {
  type Msg = HttpGenericBodyMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpGenericBodyMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpGenericBodyMatchMut<'_> {
  type Msg = HttpGenericBodyMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpGenericBodyMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for HttpGenericBodyMatchView<'_> {
  type Msg = HttpGenericBodyMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<HttpGenericBodyMatch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for HttpGenericBodyMatchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod http_generic_body_match {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__common__matcher__v3__HttpGenericBodyMatch__GenericTextMatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GenericTextMatch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GenericTextMatch>
}

impl ::protobuf::Message for GenericTextMatch {
  type MessageView<'msg> = GenericTextMatchView<'msg>;
  type MessageMut<'msg> = GenericTextMatchMut<'msg>;
}

impl ::std::default::Default for GenericTextMatch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GenericTextMatch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GenericTextMatch` is `Sync` because it does not implement interior mutability.
//    Neither does `GenericTextMatchMut`.
unsafe impl ::std::marker::Sync for GenericTextMatch {}

// SAFETY:
// - `GenericTextMatch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GenericTextMatch {}

impl ::protobuf::Proxied for GenericTextMatch {
  type View<'msg> = GenericTextMatchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GenericTextMatch {}

impl ::protobuf::MutProxied for GenericTextMatch {
  type Mut<'msg> = GenericTextMatchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GenericTextMatchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GenericTextMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GenericTextMatchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GenericTextMatchView<'msg> {
  type Message = GenericTextMatch;
}

impl ::std::fmt::Debug for GenericTextMatchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GenericTextMatchView<'_> {
  fn default() -> GenericTextMatchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GenericTextMatch>> for GenericTextMatchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GenericTextMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GenericTextMatchView<'msg> {

  pub fn to_owned(&self) -> GenericTextMatch {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // string_match: optional string
  pub fn has_string_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn string_match_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_string_match().then(|| self.string_match())
  }
  pub fn string_match(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // binary_match: optional bytes
  pub fn has_binary_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn binary_match_opt(self) -> ::std::option::Option<&'msg [u8]> {
    self.has_binary_match().then(|| self.binary_match())
  }
  pub fn binary_match(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  pub fn rule(self) -> super::super::http_generic_body_match::generic_text_match::RuleOneof<'msg> {
    match self.rule_case() {
      super::super::http_generic_body_match::generic_text_match::RuleCase::StringMatch =>
          super::super::http_generic_body_match::generic_text_match::RuleOneof::StringMatch(self.string_match()),
      super::super::http_generic_body_match::generic_text_match::RuleCase::BinaryMatch =>
          super::super::http_generic_body_match::generic_text_match::RuleOneof::BinaryMatch(self.binary_match()),
      _ => super::super::http_generic_body_match::generic_text_match::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(self) -> super::super::http_generic_body_match::generic_text_match::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::http_generic_body_match::generic_text_match::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `GenericTextMatchView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GenericTextMatchView<'_> {}

// SAFETY:
// - `GenericTextMatchView` is `Send` because while its alive a `GenericTextMatchMut` cannot.
// - `GenericTextMatchView` does not use thread-local data.
unsafe impl ::std::marker::Send for GenericTextMatchView<'_> {}

impl<'msg> ::protobuf::AsView for GenericTextMatchView<'msg> {
  type Proxied = GenericTextMatch;
  fn as_view(&self) -> ::protobuf::View<'msg, GenericTextMatch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GenericTextMatchView<'msg> {
  fn into_view<'shorter>(self) -> GenericTextMatchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GenericTextMatch> for GenericTextMatchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GenericTextMatch {
    let mut dst = GenericTextMatch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GenericTextMatch> for GenericTextMatchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GenericTextMatch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GenericTextMatch {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GenericTextMatchView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GenericTextMatchMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GenericTextMatchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GenericTextMatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GenericTextMatchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GenericTextMatchMut<'msg> {
  type Message = GenericTextMatch;
}

impl ::std::fmt::Debug for GenericTextMatchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GenericTextMatch>> for GenericTextMatchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GenericTextMatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GenericTextMatchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GenericTextMatch> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GenericTextMatch {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // string_match: optional string
  pub fn has_string_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_string_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn string_match_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_match().then(|| self.string_match())
  }
  pub fn string_match(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_match(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // binary_match: optional bytes
  pub fn has_binary_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_binary_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn binary_match_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_binary_match().then(|| self.binary_match())
  }
  pub fn binary_match(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_binary_match(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn rule(&self) -> super::super::http_generic_body_match::generic_text_match::RuleOneof<'_> {
    match &self.rule_case() {
      super::super::http_generic_body_match::generic_text_match::RuleCase::StringMatch =>
          super::super::http_generic_body_match::generic_text_match::RuleOneof::StringMatch(self.string_match()),
      super::super::http_generic_body_match::generic_text_match::RuleCase::BinaryMatch =>
          super::super::http_generic_body_match::generic_text_match::RuleOneof::BinaryMatch(self.binary_match()),
      _ => super::super::http_generic_body_match::generic_text_match::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::super::http_generic_body_match::generic_text_match::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::http_generic_body_match::generic_text_match::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `GenericTextMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GenericTextMatchMut<'_> {}

// SAFETY:
// - `GenericTextMatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GenericTextMatchMut<'_> {}

impl<'msg> ::protobuf::AsView for GenericTextMatchMut<'msg> {
  type Proxied = GenericTextMatch;
  fn as_view(&self) -> ::protobuf::View<'_, GenericTextMatch> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GenericTextMatchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GenericTextMatch>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GenericTextMatchMut<'msg> {
  type MutProxied = GenericTextMatch;
  fn as_mut(&mut self) -> GenericTextMatchMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GenericTextMatchMut<'msg> {
  fn into_mut<'shorter>(self) -> GenericTextMatchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GenericTextMatch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GenericTextMatch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GenericTextMatchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GenericTextMatchMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // string_match: optional string
  pub fn has_string_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_string_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn string_match_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_string_match().then(|| self.string_match())
  }
  pub fn string_match(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_string_match(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // binary_match: optional bytes
  pub fn has_binary_match(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_binary_match(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn binary_match_opt(&self) -> ::std::option::Option<&'_ [u8]> {
    self.has_binary_match().then(|| self.binary_match())
  }
  pub fn binary_match(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_binary_match(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn rule(&self) -> super::super::http_generic_body_match::generic_text_match::RuleOneof<'_> {
    match &self.rule_case() {
      super::super::http_generic_body_match::generic_text_match::RuleCase::StringMatch =>
          super::super::http_generic_body_match::generic_text_match::RuleOneof::StringMatch(self.string_match()),
      super::super::http_generic_body_match::generic_text_match::RuleCase::BinaryMatch =>
          super::super::http_generic_body_match::generic_text_match::RuleOneof::BinaryMatch(self.binary_match()),
      _ => super::super::http_generic_body_match::generic_text_match::RuleOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn rule_case(&self) -> super::super::http_generic_body_match::generic_text_match::RuleCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::http_generic_body_match::generic_text_match::RuleCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl GenericTextMatch

impl ::std::ops::Drop for GenericTextMatch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GenericTextMatch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GenericTextMatch {
  type Proxied = Self;
  fn as_view(&self) -> GenericTextMatchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GenericTextMatch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GenericTextMatchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GenericTextMatch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::http_generic_body_match::envoy__config__common__matcher__v3__HttpGenericBodyMatch__GenericTextMatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1T0^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::http_generic_body_match::envoy__config__common__matcher__v3__HttpGenericBodyMatch__GenericTextMatch_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::http_generic_body_match::envoy__config__common__matcher__v3__HttpGenericBodyMatch__GenericTextMatch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GenericTextMatch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GenericTextMatch {
  type Msg = GenericTextMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericTextMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GenericTextMatch {
  type Msg = GenericTextMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericTextMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GenericTextMatchMut<'_> {
  type Msg = GenericTextMatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericTextMatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GenericTextMatchMut<'_> {
  type Msg = GenericTextMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericTextMatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GenericTextMatchView<'_> {
  type Msg = GenericTextMatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GenericTextMatch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GenericTextMatchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod generic_text_match {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum RuleOneof<'msg> {
  StringMatch(&'msg ::protobuf::ProtoStr) = 1,
  BinaryMatch(&'msg [u8]) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum RuleCase {
  StringMatch = 1,
  BinaryMatch = 2,

  not_set = 0
}

impl RuleCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<RuleCase> {
    match v {
      0 => Some(RuleCase::not_set),
      1 => Some(RuleCase::StringMatch),
      2 => Some(RuleCase::BinaryMatch),
      _ => None
    }
  }
}
}  // pub mod generic_text_match


}  // pub mod http_generic_body_match


