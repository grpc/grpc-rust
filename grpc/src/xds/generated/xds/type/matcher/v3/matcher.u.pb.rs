const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut xds__type__matcher__v3__Matcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // matcher_list: optional message xds.type.matcher.v3.Matcher.MatcherList
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

  // matcher_tree: optional message xds.type.matcher.v3.Matcher.MatcherTree
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

  // on_no_match: optional message xds.type.matcher.v3.Matcher.OnMatch
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

  // matcher_list: optional message xds.type.matcher.v3.Matcher.MatcherList
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

  // matcher_tree: optional message xds.type.matcher.v3.Matcher.MatcherTree
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

  // on_no_match: optional message xds.type.matcher.v3.Matcher.OnMatch
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

  // matcher_list: optional message xds.type.matcher.v3.Matcher.MatcherList
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

  // matcher_tree: optional message xds.type.matcher.v3.Matcher.MatcherTree
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

  // on_no_match: optional message xds.type.matcher.v3.Matcher.OnMatch
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
        super::xds__type__matcher__v3__Matcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^!|#");
        super::matcher::xds__type__matcher__v3__Matcher__MatcherList_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        super::matcher::matcher_list::xds__type__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        super::matcher::xds__type__matcher__v3__Matcher__MatcherTree_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333^#|$|%");
        super::matcher::matcher_tree::xds__type__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        super::matcher::matcher_tree::match_map::xds__type__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        super::matcher::xds__type__matcher__v3__Matcher__OnMatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33/P^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::xds__type__matcher__v3__Matcher_msg_init.0, &[super::matcher::xds__type__matcher__v3__Matcher__MatcherList_msg_init.0,
            super::matcher::xds__type__matcher__v3__Matcher__MatcherTree_msg_init.0,
            super::matcher::xds__type__matcher__v3__Matcher__OnMatch_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::xds__type__matcher__v3__Matcher__MatcherList_msg_init.0, &[super::matcher::matcher_list::xds__type__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::matcher_list::xds__type__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init.0, &[<super::matcher::matcher_list::Predicate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::matcher::xds__type__matcher__v3__Matcher__OnMatch_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::xds__type__matcher__v3__Matcher__MatcherTree_msg_init.0, &[<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::matcher::matcher_tree::xds__type__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0,
            super::matcher::matcher_tree::xds__type__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0,
            <crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::matcher_tree::xds__type__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0, &[super::matcher::matcher_tree::match_map::xds__type__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::matcher_tree::match_map::xds__type__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init.0, &[super::matcher::xds__type__matcher__v3__Matcher__OnMatch_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::matcher::xds__type__matcher__v3__Matcher__OnMatch_msg_init.0, &[super::xds__type__matcher__v3__Matcher_msg_init.0,
            <crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::xds__type__matcher__v3__Matcher_msg_init.0)
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
pub(crate) static mut xds__type__matcher__v3__Matcher__OnMatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // matcher: optional message xds.type.matcher.v3.Matcher
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

  // action: optional message xds.core.v3.TypedExtensionConfig
  pub fn has_action(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn action_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_action().then(|| self.action())
  }
  pub fn action(self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
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

  // matcher: optional message xds.type.matcher.v3.Matcher
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

  // action: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn action_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_action().then(|| self.action())
  }
  pub fn action(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn action_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

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

  // matcher: optional message xds.type.matcher.v3.Matcher
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

  // action: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn action_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_action().then(|| self.action())
  }
  pub fn action(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn action_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

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
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::matcher::xds__type__matcher__v3__Matcher__OnMatch_msg_init.0)
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
  Action(::protobuf::View<'msg, crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) = 2,

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
pub(crate) static mut xds__type__matcher__v3__Matcher__MatcherList_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // matchers: repeated message xds.type.matcher.v3.Matcher.MatcherList.FieldMatcher
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

  // matchers: repeated message xds.type.matcher.v3.Matcher.MatcherList.FieldMatcher
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

  // matchers: repeated message xds.type.matcher.v3.Matcher.MatcherList.FieldMatcher
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
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::matcher::xds__type__matcher__v3__Matcher__MatcherList_msg_init.0)
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
pub(crate) static mut xds__type__matcher__v3__Matcher__MatcherList__Predicate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // single_predicate: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate.SinglePredicate
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

  // or_matcher: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
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

  // and_matcher: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
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

  // not_matcher: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate
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

  // single_predicate: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate.SinglePredicate
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

  // or_matcher: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
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

  // and_matcher: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
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

  // not_matcher: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate
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

  // single_predicate: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate.SinglePredicate
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

  // or_matcher: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
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

  // and_matcher: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate.PredicateList
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

  // not_matcher: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate
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
        super::super::super::matcher::matcher_list::xds__type__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333^!|#|$|%");
        super::super::super::matcher::matcher_list::predicate::xds__type__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::matcher::matcher_list::xds__type__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0, &[<super::super::super::matcher::matcher_list::predicate::SinglePredicate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            super::super::super::matcher::matcher_list::predicate::xds__type__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0,
            super::super::super::matcher::matcher_list::predicate::xds__type__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0,
            super::super::super::matcher::matcher_list::xds__type__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::matcher::matcher_list::predicate::xds__type__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0, &[super::super::super::matcher::matcher_list::xds__type__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0,
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::matcher::matcher_list::xds__type__matcher__v3__Matcher__MatcherList__Predicate_msg_init.0)
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
pub(crate) static mut xds__type__matcher__v3__Matcher__MatcherList__Predicate__SinglePredicate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // input: optional message xds.core.v3.TypedExtensionConfig
  pub fn has_input(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn input_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }

  // value_match: optional message xds.type.matcher.v3.StringMatcher
  pub fn has_value_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn value_match_opt(self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherView<'msg>> {
    self.has_value_match().then(|| self.value_match())
  }
  pub fn value_match(self) -> crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherView::default())
  }

  // custom_match: optional message xds.core.v3.TypedExtensionConfig
  pub fn has_custom_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn custom_match_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
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

  // input: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn input_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn input_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value_match: optional message xds.type.matcher.v3.StringMatcher
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
  pub fn value_match_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_value_match().then(|| self.value_match())
  }
  pub fn value_match(&self) -> crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn value_match_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // custom_match: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn custom_match_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_match_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

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

  // input: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn input_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn input_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // value_match: optional message xds.type.matcher.v3.StringMatcher
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
  pub fn value_match_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherView<'_>> {
    self.has_value_match().then(|| self.value_match())
  }
  pub fn value_match(&self) -> crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherView::default())
  }
  pub fn value_match_mut(&mut self) -> crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcherMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcher>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // custom_match: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn custom_match_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_match_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

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
        super::super::super::super::matcher::matcher_list::predicate::xds__type__matcher__v3__Matcher__MatcherList__Predicate__SinglePredicate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333^#|$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::super::matcher::matcher_list::predicate::xds__type__matcher__v3__Matcher__MatcherList__Predicate__SinglePredicate_msg_init.0, &[<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::matcher::matcher_list::predicate::xds__type__matcher__v3__Matcher__MatcherList__Predicate__SinglePredicate_msg_init.0)
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
  ValueMatch(::protobuf::View<'msg, crate::xds::generated::xds::r#type::matcher::v3::string::StringMatcher>) = 2,
  CustomMatch(::protobuf::View<'msg, crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) = 3,

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
pub(crate) static mut xds__type__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // predicate: repeated message xds.type.matcher.v3.Matcher.MatcherList.Predicate
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

  // predicate: repeated message xds.type.matcher.v3.Matcher.MatcherList.Predicate
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

  // predicate: repeated message xds.type.matcher.v3.Matcher.MatcherList.Predicate
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
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::matcher::matcher_list::predicate::xds__type__matcher__v3__Matcher__MatcherList__Predicate__PredicateList_msg_init.0)
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
pub(crate) static mut xds__type__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // predicate: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate
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

  // on_match: optional message xds.type.matcher.v3.Matcher.OnMatch
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

  // predicate: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate
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

  // on_match: optional message xds.type.matcher.v3.Matcher.OnMatch
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

  // predicate: optional message xds.type.matcher.v3.Matcher.MatcherList.Predicate
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

  // on_match: optional message xds.type.matcher.v3.Matcher.OnMatch
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
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::matcher::matcher_list::xds__type__matcher__v3__Matcher__MatcherList__FieldMatcher_msg_init.0)
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
pub(crate) static mut xds__type__matcher__v3__Matcher__MatcherTree_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // input: optional message xds.core.v3.TypedExtensionConfig
  pub fn has_input(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn input_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }

  // exact_match_map: optional message xds.type.matcher.v3.Matcher.MatcherTree.MatchMap
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

  // prefix_match_map: optional message xds.type.matcher.v3.Matcher.MatcherTree.MatchMap
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

  // custom_match: optional message xds.core.v3.TypedExtensionConfig
  pub fn has_custom_match(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn custom_match_opt(self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
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

  // input: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn input_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn input_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // exact_match_map: optional message xds.type.matcher.v3.Matcher.MatcherTree.MatchMap
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

  // prefix_match_map: optional message xds.type.matcher.v3.Matcher.MatcherTree.MatchMap
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

  // custom_match: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn custom_match_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_match_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

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

  // input: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn input_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_input().then(|| self.input())
  }
  pub fn input(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn input_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // exact_match_map: optional message xds.type.matcher.v3.Matcher.MatcherTree.MatchMap
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

  // prefix_match_map: optional message xds.type.matcher.v3.Matcher.MatcherTree.MatchMap
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

  // custom_match: optional message xds.core.v3.TypedExtensionConfig
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
  pub fn custom_match_opt(&self) -> ::std::option::Option<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_custom_match().then(|| self.custom_match())
  }
  pub fn custom_match(&self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn custom_match_mut(&mut self) -> crate::xds::generated::xds::core::v3::extension::TypedExtensionConfigMut<'_> {
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
    val: impl ::protobuf::IntoProxied<crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) {

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
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::matcher::xds__type__matcher__v3__Matcher__MatcherTree_msg_init.0)
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
pub(crate) static mut xds__type__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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

  // map: repeated message xds.type.matcher.v3.Matcher.MatcherTree.MatchMap.MapEntry
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

  // map: repeated message xds.type.matcher.v3.Matcher.MatcherTree.MatchMap.MapEntry
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

  // map: repeated message xds.type.matcher.v3.Matcher.MatcherTree.MatchMap.MapEntry
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
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::matcher::matcher_tree::xds__type__matcher__v3__Matcher__MatcherTree__MatchMap_msg_init.0)
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
pub(crate) static mut xds__type__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::super::matcher::matcher_tree::match_map::xds__type__matcher__v3__Matcher__MatcherTree__MatchMap__MapEntry_msg_init.0)
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
  CustomMatch(::protobuf::View<'msg, crate::xds::generated::xds::core::v3::extension::TypedExtensionConfig>) = 4,

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


